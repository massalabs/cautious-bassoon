mod as_abi;
mod as_execution;
mod common;

use anyhow::{bail, Result};
use std::sync::Arc;
use wasmer::{
    wasmparser::Operator, BaseTunables, EngineBuilder, FunctionEnv, Imports, Pages, Target,
};
use wasmer::{CompilerConfig, Features, Instance, InstantiationError, Module, Store};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;
use wasmer_types::TrapCode;

use crate::middlewares::gas_calibration::GasCalibration;
use crate::settings::max_number_of_pages;
use crate::tunable_memory::LimitingTunables;
use crate::{GasCosts, Interface, Response};

use crate::env::ASEnv;
pub(crate) use as_execution::*;
pub(crate) use common::*;

pub(crate) trait MassaModule: Send + Sync {
    fn init(interface: &dyn Interface, bytecode: &[u8], gas_costs: GasCosts) -> Self;
    /// Closure for the execution allowing us to handle a gas error
    fn execution(
        &self,
        instance: &Instance,
        store: &mut Store,
        function: &str,
        param: &[u8],
    ) -> Result<Response>;
    fn has_function(&self, instance: &Instance, function: &str) -> bool;
    fn resolver(&self, store: &mut Store) -> (Imports, FunctionEnv<ASEnv>);
    fn init_with_instance(
        &mut self,
        instance: &Instance,
        store: &mut Store,
        fenv: &mut FunctionEnv<ASEnv>,
    ) -> Result<()>;
    fn get_bytecode(&self) -> &Vec<u8>;
    fn get_gas_costs(&self) -> GasCosts;
}

/// Create an instance of VM from a module with a given interface, an operation
/// number limit and a webassembly module
pub(crate) fn create_instance(
    limit: u64,
    as_module: &mut impl MassaModule,
) -> Result<(Instance, Store)> {
    // We use the Singlepass compiler because it is fast and adapted to blockchains
    // See https://docs.rs/wasmer-compiler-singlepass/latest/wasmer_compiler_singlepass/
    let mut compiler_config = Singlepass::new();

    // Turning-off sources of potential non-determinism,
    // see https://github.com/WebAssembly/design/blob/037c6fe94151eb13e30d174f5f7ce851be0a573e/Nondeterminism.md

    // Turning-off in the compiler:

    // Canonicalize NaN.
    compiler_config.canonicalize_nans(true);

    // Default: Turning-off all wasmer feature flags
    // Exception(s):
    // * bulk_memory:
    //   * https://docs.rs/wasmer/latest/wasmer/struct.Features.html: now fully standardized - wasm 2.0
    //   * See also: https://github.com/paritytech/substrate/issues/12216
    const FEATURES: Features = Features {
        threads: false, // disable threads
        reference_types: false,
        simd: false,           // turn off experimental SIMD feature
        bulk_memory: true,     // enabled in order to use ArrayBuffer in AS
        multi_value: false,    // turn off multi value, not support for SinglePass (default: true)
        tail_call: false,      // experimental
        module_linking: false, // experimental
        multi_memory: false,   // experimental
        memory64: false,       // experimental
        exceptions: false,
        relaxed_simd: false, // experimental
        extended_const: false,
    };

    let operator_cost = as_module.get_gas_costs().operator_cost;

    if cfg!(feature = "gas_calibration") {
        // Add gas calibration middleware
        let gas_calibration = Arc::new(GasCalibration::new());
        compiler_config.push_middleware(gas_calibration);
    } else {
        // Add metering middleware
        let metering = Arc::new(Metering::new(limit, move |_: &Operator| -> u64 {
            operator_cost
        }));
        compiler_config.push_middleware(metering);
    }

    let base = BaseTunables::for_target(&Target::default());
    let tunables = LimitingTunables::new(base, Pages(max_number_of_pages()));
    let engine = EngineBuilder::new(compiler_config)
        .set_features(Some(FEATURES))
        .engine();
    let mut store = Store::new_with_tunables(&engine, tunables);

    let module = &Module::new(&store, as_module.get_bytecode())?;
    let (imports, mut fenv) = as_module.resolver(&mut store);

    match Instance::new(&mut store, module, &imports) {
        Ok(i) => {
            as_module.init_with_instance(&i, &mut store, &mut fenv)?;
            Ok((i, store))
        }
        Err(err) => {
            // We filter the error created by the metering middleware when there is not enough gas at initialization.
            if let InstantiationError::Start(ref e) = err {
                if let Some(trap) = e.clone().to_trap() {
                    if trap == TrapCode::UnreachableCodeReached && e.trace().is_empty() {
                        bail!("RuntimeError: Not enough gas, limit reached at initialization");
                    }
                }
            }
            Err(err.into())
        }
    }
}

/// Dispatch module corresponding to the first bytecode.
/// 1: target AssemblyScript
/// 2: todo: another target
/// _: target AssemblyScript and use the full bytecode
pub(crate) fn get_module(
    interface: &dyn Interface,
    bytecode: &[u8],
    gas_costs: GasCosts,
) -> Result<impl MassaModule> {
    if bytecode.is_empty() {
        bail!("error: module is empty")
    }
    Ok(match bytecode[0] {
        1 => ASModule::init(interface, &bytecode[1..], gas_costs),
        _ => ASModule::init(interface, bytecode, gas_costs),
    })
}
