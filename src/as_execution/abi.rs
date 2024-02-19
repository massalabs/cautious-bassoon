//! *abi_impl.rs* contains all the implementation (and some tools as
//! abi_bail!) of the massa abi.
//!
//! The ABIs are the imported function / object declared in the webassembly
//! module. You can look at the other side of the mirror in `massa.ts` and the
//! rust side in `execution_impl.rs`.

use as_ffi_bindings::{BufferPtr, Read as ASRead, StringPtr, Write as ASWrite};
use function_name::named;
use std::ops::Add;
use wasmer::{AsStoreMut, AsStoreRef, FunctionEnvMut, Memory};

use super::env::{get_remaining_points, sub_remaining_gas_abi, ASEnv};
use crate::settings;
#[cfg(feature = "execution-trace")]
use crate::types::{AbiTrace, AbiTraceType};

use super::common::{call_module, create_sc, function_exists, local_call};
use super::error::{abi_bail, ABIResult};

macro_rules! get_memory {
    ($env:ident) => {
        match $env.get_ffi_env().memory.as_ref() {
            Some(mem) => mem,
            _ => abi_bail!("AssemblyScript memory is missing from the environment"),
        }
    };
}

/// Retrieves the AssemblyScript environment.
///
/// Fails during instantiation to avoid gas manipulation in the WASM start
/// function.
pub(crate) fn get_env(ctx: &FunctionEnvMut<ASEnv>) -> ABIResult<ASEnv> {
    let env = ctx.data().clone();
    if !(env.abi_enabled.load(std::sync::atomic::Ordering::Relaxed)) {
        abi_bail!("ABI calls are not available during instantiation");
    } else {
        Ok(env)
    }
}

/// Get the coins that have been made available for a specific purpose for the
/// current call.
#[named]
pub(crate) fn assembly_script_get_call_coins(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let res = env.get_interface().get_call_coins()? as i64;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::I64(res),
        sub_calls: None,
    });
    Ok(res)
}

/// Transfer an amount from the address on the current call stack to a target
/// address.
#[named]
pub(crate) fn assembly_script_transfer_coins(
    mut ctx: FunctionEnvMut<ASEnv>,
    to_address: i32,
    raw_amount: i64,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    if raw_amount.is_negative() {
        abi_bail!("Negative raw amount.");
    }
    let memory = get_memory!(env);
    let to_address = read_string(memory, &ctx, to_address)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, to_address.len(), true);
    // }
    env.get_interface()
        .transfer_coins(&to_address, raw_amount as u64)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(to_address),
            AbiTraceType::I64(raw_amount),
        ],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// Transfer an amount from the specified address to a target address.
#[named]
pub(crate) fn assembly_script_transfer_coins_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    from_address: i32,
    to_address: i32,
    raw_amount: i64,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    if raw_amount.is_negative() {
        abi_bail!("Negative raw amount.");
    }
    let memory = get_memory!(env);
    let from_address = read_string(memory, &ctx, from_address)?;
    let to_address = read_string(memory, &ctx, to_address)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, from_address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, to_address.len(), true);
    // }
    env.get_interface()
        .transfer_coins_for(&from_address, &to_address, raw_amount as u64)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(from_address),
            AbiTraceType::String(to_address),
            AbiTraceType::I64(raw_amount),
        ],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

#[named]
pub(crate) fn assembly_script_get_balance(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let res = env.get_interface().get_balance()? as i64;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::I64(res),
        sub_calls: None,
    });
    Ok(res)
}

#[named]
pub(crate) fn assembly_script_get_balance_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
) -> ABIResult<i64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    // }
    let res = env.get_interface().get_balance_for(&address)? as i64;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(address)],
        return_value: AbiTraceType::I64(res),
        sub_calls: None,
    });
    Ok(res)
}

/// Raw call that have the right type signature to be able to be call a module
/// directly form AssemblyScript:
#[named]
pub(crate) fn assembly_script_call(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    function: i32,
    param: i32,
    call_coins: i64,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let function = read_string(memory, &ctx, function)?;
    let param = read_buffer(memory, &ctx, param)?;

    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, function.len(), true);
    //     let fname = format!("massa.{}:2", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, param.len(), true);
    // }

    let response = call_module(&mut ctx, &address, &function, &param, call_coins)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(address.clone()),
            AbiTraceType::String(function.clone()),
            AbiTraceType::ByteArray(param),
            AbiTraceType::I64(call_coins),
        ],
        return_value: AbiTraceType::ByteArray(response.ret.clone()),
        sub_calls: Some(response.trace),
    });
    match BufferPtr::alloc(&response.ret, env.get_ffi_env(), &mut ctx) {
        Ok(ret) => Ok(ret.offset() as i32),
        _ => abi_bail!(format!(
            "Cannot allocate response in call {}::{}",
            address, function
        )),
    }
}

#[named]
pub(crate) fn assembly_script_get_remaining_gas(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let res = get_remaining_points(&env, &mut ctx)? as i64;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::I64(res),
        sub_calls: None,
    });
    Ok(res)
}

/// Create an instance of VM from a module with a
/// given interface, an operation number limit and a webassembly module
///
/// An utility print function to write on stdout directly from AssemblyScript:
#[named]
pub(crate) fn assembly_script_print(mut ctx: FunctionEnvMut<ASEnv>, arg: i32) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let message = read_string(memory, &ctx, arg)?;

    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, message.len(), true);
    // }

    env.get_interface().print(&message)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(message)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// Get the operation datastore keys (aka entries)
#[named]
pub(crate) fn assembly_script_get_op_keys(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    match env.get_interface().get_op_keys(None) {
        Err(err) => abi_bail!(err),
        Ok(keys) => {
            let fmt_keys =
                ser_bytearray_vec(&keys, keys.len(), settings::max_op_datastore_entry_count())?;
            let ptr = pointer_from_bytearray(&env, &mut ctx, &fmt_keys)?.offset();

            #[cfg(feature = "execution-trace")]
            ctx.data_mut().trace.push(AbiTrace {
                name: function_name!().to_string(),
                params: vec![AbiTraceType::None],
                return_value: AbiTraceType::ByteArray(fmt_keys),
                sub_calls: None,
            });
            Ok(ptr as i32)
        }
    }
}

/// Get the operation datastore keys (aka entries)
#[named]
pub(crate) fn assembly_script_get_op_keys_prefix(
    mut ctx: FunctionEnvMut<ASEnv>,
    prefix: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let prefix = read_buffer(memory, &ctx, prefix)?;
    let prefix_opt = if !prefix.is_empty() {
        Some(prefix.as_ref())
    } else {
        None
    };
    match env.get_interface().get_op_keys(prefix_opt) {
        Err(err) => abi_bail!(err),
        Ok(keys) => {
            let fmt_keys =
                ser_bytearray_vec(&keys, keys.len(), settings::max_op_datastore_entry_count())?;
            let ptr = pointer_from_bytearray(&env, &mut ctx, &fmt_keys)?.offset();

            #[cfg(feature = "execution-trace")]
            ctx.data_mut().trace.push(AbiTrace {
                name: function_name!().to_string(),
                params: vec![AbiTraceType::ByteArray(prefix)],
                return_value: AbiTraceType::ByteArray(fmt_keys),
                sub_calls: None,
            });
            Ok(ptr as i32)
        }
    }
}

/// Check if a key is present in operation datastore
#[named]
pub(crate) fn assembly_script_has_op_key(
    mut ctx: FunctionEnvMut<ASEnv>,
    key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let env = get_env(&ctx)?;
    let memory = get_memory!(env);
    let key_bytes = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key_bytes.len(), true);
    // }

    match env.get_interface().op_entry_exists(&key_bytes) {
        Err(err) => abi_bail!(err),
        Ok(b) => {
            // https://doc.rust-lang.org/reference/types/boolean.html
            // 'true' is explicitly defined as: 0x01 while 'false' is: 0x00
            let b_vec: Vec<u8> = vec![b as u8];
            let a = pointer_from_bytearray(&env, &mut ctx, &b_vec)?.offset();

            #[cfg(feature = "execution-trace")]
            ctx.data_mut().trace.push(AbiTrace {
                name: function_name!().to_string(),
                params: vec![AbiTraceType::ByteArray(key_bytes)],
                return_value: AbiTraceType::ByteArray(b_vec),
                sub_calls: None,
            });

            Ok(a as i32)
        }
    }
}

/// Get the operation datastore value associated to given key
#[named]
pub(crate) fn assembly_script_get_op_data(
    mut ctx: FunctionEnvMut<ASEnv>,
    key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key_bytes = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key_bytes.len(), true);
    // }
    let data = env.get_interface().get_op_data(&key_bytes)?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &data)?.offset() as i32;

    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(key_bytes)],
        return_value: AbiTraceType::ByteArray(data),
        sub_calls: None,
    });
    Ok(ptr)
}

/// Read a bytecode string, representing the webassembly module binary encoded
/// with in base64.
#[named]
pub(crate) fn assembly_script_create_sc(
    mut ctx: FunctionEnvMut<ASEnv>,
    bytecode: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let bytecode: Vec<u8> = read_buffer(memory, &ctx, bytecode)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, bytecode.len(), true);
    // }
    let address = create_sc(&mut ctx, &bytecode)?;
    let ptr = StringPtr::alloc(&address, env.get_ffi_env(), &mut ctx)?.offset() as i32;

    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(bytecode)],
        return_value: AbiTraceType::String(address.clone()),
        sub_calls: None,
    });
    Ok(ptr)
}

/// performs a hash on a bytearray and returns the hash
#[named]
pub(crate) fn assembly_script_hash(mut ctx: FunctionEnvMut<ASEnv>, value: i32) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let bytes = read_buffer(memory, &ctx, value)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, value.len(), true);
    // }
    let hash = env.get_interface().hash(&bytes)?.to_vec();
    let ptr = pointer_from_bytearray(&env, &mut ctx, &hash)?.offset();
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(bytes)],
        return_value: AbiTraceType::ByteArray(hash),
        sub_calls: None,
    });
    Ok(ptr as i32)
}

/// performs a hash on a bytearray and returns the hash
#[named]
pub(crate) fn assembly_script_keccak256_hash(
    mut ctx: FunctionEnvMut<ASEnv>,
    value: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let bytes = read_buffer(memory, &ctx, value)?;
    let hash = env.get_interface().hash_keccak256(&bytes)?.to_vec();
    let ptr = pointer_from_bytearray(&env, &mut ctx, &hash)?.offset();

    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(bytes)],
        return_value: AbiTraceType::ByteArray(hash),
        sub_calls: None,
    });

    Ok(ptr as i32)
}

/// Get keys (aka entries) in the datastore
#[named]
pub(crate) fn assembly_script_get_keys(
    mut ctx: FunctionEnvMut<ASEnv>,
    prefix: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let prefix = read_buffer(memory, &ctx, prefix)?;
    let prefix_opt = if !prefix.is_empty() {
        Some(prefix.as_ref())
    } else {
        None
    };
    let keys = env.get_interface().get_keys(prefix_opt)?;
    let fmt_keys = ser_bytearray_vec(&keys, keys.len(), settings::max_datastore_entry_count())?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &fmt_keys)?.offset();

    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(prefix)],
        return_value: AbiTraceType::ByteArray(fmt_keys),
        sub_calls: None,
    });
    Ok(ptr as i32)
}

/// Get keys (aka entries) in the datastore
#[named]
pub(crate) fn assembly_script_get_keys_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    prefix: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let prefix = read_buffer(memory, &ctx, prefix)?;
    let prefix_opt = if !prefix.is_empty() {
        Some(prefix.as_ref())
    } else {
        None
    };
    let keys = env.get_interface().get_keys_for(&address, prefix_opt)?;
    let fmt_keys = ser_bytearray_vec(&keys, keys.len(), settings::max_datastore_entry_count())?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &fmt_keys)?.offset();

    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(address),
            AbiTraceType::ByteArray(prefix),
        ],
        return_value: AbiTraceType::ByteArrays(keys.iter().cloned().collect()),
        sub_calls: None,
    });
    Ok(ptr as i32)
}

/// sets a key-indexed data entry in the datastore, overwriting existing values
/// if any
#[named]
pub(crate) fn assembly_script_set_data(
    mut ctx: FunctionEnvMut<ASEnv>,
    key: i32,
    value: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key = read_buffer(memory, &ctx, key)?;
    let value = read_buffer(memory, &ctx, value)?;

    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     param_size_update(
    //         &env,
    //         &mut ctx,
    //         "massa.assembly_script_set_data:0",
    //         key.len(),
    //         false,
    //     );
    //     param_size_update(
    //         &env,
    //         &mut ctx,
    //         "massa.assembly_script_set_data:1",
    //         value.len(),
    //         false,
    //     );
    // }

    env.get_interface().raw_set_data(&key, &value)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(key), AbiTraceType::ByteArray(value)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// appends data to a key-indexed data entry in the datastore, fails if the
/// entry does not exist
#[named]
pub(crate) fn assembly_script_append_data(
    mut ctx: FunctionEnvMut<ASEnv>,
    key: i32,
    value: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key = read_buffer(memory, &ctx, key)?;
    let value = read_buffer(memory, &ctx, value)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, value.len(), true);
    // }
    env.get_interface().raw_append_data(&key, &value)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(key), AbiTraceType::ByteArray(value)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// gets a key-indexed data entry in the datastore, failing if non-existent
#[named]
pub(crate) fn assembly_script_get_data(mut ctx: FunctionEnvMut<ASEnv>, key: i32) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    // }
    let data = env.get_interface().raw_get_data(&key)?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &data)?.offset() as i32;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(key)],
        return_value: AbiTraceType::ByteArray(data.clone()),
        sub_calls: None,
    });
    Ok(ptr)
}

/// checks if a key-indexed data entry exists in the datastore
#[named]
pub(crate) fn assembly_script_has_data(mut ctx: FunctionEnvMut<ASEnv>, key: i32) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    // }
    let res = env.get_interface().has_data(&key)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(key)],
        return_value: AbiTraceType::Bool(res),
        sub_calls: None,
    });
    Ok(res as i32)
}

/// deletes a key-indexed data entry in the datastore of the current address,
/// fails if the entry is absent
#[named]
pub(crate) fn assembly_script_delete_data(
    mut ctx: FunctionEnvMut<ASEnv>,
    key: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    // }
    env.get_interface().raw_delete_data(&key)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(key)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// Sets the value of a datastore entry of an arbitrary address, creating the
/// entry if it does not exist. Fails if the address does not exist.
#[named]
pub(crate) fn assembly_script_set_data_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    key: i32,
    value: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key = read_buffer(memory, &ctx, key)?;
    let value = read_buffer(memory, &ctx, value)?;
    let address = read_string(memory, &ctx, address)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    //     let fname = format!("massa.{}:2", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, value.len(), true);
    // }
    env.get_interface()
        .raw_set_data_for(&address, &key, &value)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(address),
            AbiTraceType::ByteArray(key),
            AbiTraceType::ByteArray(value),
        ],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// Appends data to the value of a datastore entry of an arbitrary address,
/// fails if the entry or address does not exist.
#[named]
pub(crate) fn assembly_script_append_data_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    key: i32,
    value: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let key = read_buffer(memory, &ctx, key)?;
    let value = read_buffer(memory, &ctx, value)?;
    let address = read_string(memory, &ctx, address)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    //     let fname = format!("massa.{}:2", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, value.len(), true);
    // }
    env.get_interface()
        .raw_append_data_for(&address, &key, &value)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(address),
            AbiTraceType::ByteArray(key),
            AbiTraceType::ByteArray(value),
        ],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// Gets the value of a datastore entry for an arbitrary address, fails if the
/// entry or address does not exist
#[named]
pub(crate) fn assembly_script_get_data_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let key = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    // }

    let data = env.get_interface().raw_get_data_for(&address, &key)?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &data)?.offset() as i32;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(address), AbiTraceType::ByteArray(key)],
        return_value: AbiTraceType::ByteArray(data),
        sub_calls: None,
    });
    Ok(ptr)
}

/// Deletes a datastore entry for an address. Fails if the entry or address does
/// not exist.
#[named]
pub(crate) fn assembly_script_delete_data_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    key: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let key = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    // }
    env.get_interface().raw_delete_data_for(&address, &key)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(address), AbiTraceType::ByteArray(key)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

#[named]
pub(crate) fn assembly_script_has_data_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let key = read_buffer(memory, &ctx, key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, key.len(), true);
    // }
    let res = env.get_interface().has_data_for(&address, &key)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(address), AbiTraceType::ByteArray(key)],
        return_value: AbiTraceType::Bool(res),
        sub_calls: None,
    });
    Ok(res as i32)
}

#[named]
pub(crate) fn assembly_script_get_owned_addresses(
    mut ctx: FunctionEnvMut<ASEnv>,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let data = env.get_interface().get_owned_addresses()?;
    // prevent data.clone() when enabling execution-trace
    #[allow(clippy::let_and_return)]
    let ptr = alloc_string_array(&mut ctx, &data);
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::Strings(data),
        sub_calls: None,
    });
    ptr
}

#[named]
pub(crate) fn assembly_script_get_call_stack(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let data = env.get_interface().get_call_stack()?;
    // prevent data.clone() when enabling execution-trace
    #[allow(clippy::let_and_return)]
    let ptr = alloc_string_array(&mut ctx, &data);
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::Strings(data),
        sub_calls: None,
    });
    ptr
}

#[named]
pub(crate) fn assembly_script_generate_event(
    mut ctx: FunctionEnvMut<ASEnv>,
    event: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let event = read_string(memory, &ctx, event)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, event.len(), true);
    // }
    env.get_interface().generate_event(event.clone())?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(event)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// verify a signature of data given a public key. Returns Ok(1) if correctly
/// verified, otherwise Ok(0)
#[named]
pub(crate) fn assembly_script_signature_verify(
    mut ctx: FunctionEnvMut<ASEnv>,
    data: i32,
    signature: i32,
    public_key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let data = read_string(memory, &ctx, data)?;
    let signature = read_string(memory, &ctx, signature)?;
    let public_key = read_string(memory, &ctx, public_key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, data.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, signature.len(), true);
    //     let fname = format!("massa.{}:2", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, public_key.len(), true);
    // }
    let res = env
        .get_interface()
        .signature_verify(data.as_bytes(), &signature, &public_key)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::ByteArray(data.as_bytes().to_vec()),
            AbiTraceType::String(signature),
            AbiTraceType::String(public_key),
        ],
        return_value: AbiTraceType::Bool(res),
        sub_calls: None,
    });
    Ok(res as i32)
}

/// Verify an EVM signature.
/// Returns Ok(1) if correctly verified, Ok(0) otherwise.
#[named]
pub(crate) fn assembly_script_evm_signature_verify(
    mut ctx: FunctionEnvMut<ASEnv>,
    data: i32,
    signature: i32,
    public_key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let data = read_buffer(memory, &ctx, data)?;
    let signature = read_buffer(memory, &ctx, signature)?;
    let public_key = read_buffer(memory, &ctx, public_key)?;
    let res = env
        .get_interface()
        .evm_signature_verify(&data, &signature, &public_key)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::ByteArray(data),
            AbiTraceType::ByteArray(signature),
            AbiTraceType::ByteArray(public_key),
        ],
        return_value: AbiTraceType::Bool(res),
        sub_calls: None,
    });
    Ok(res as i32)
}

/// Get address from public key (EVM)
#[named]
pub(crate) fn assembly_script_evm_get_address_from_pubkey(
    mut ctx: FunctionEnvMut<ASEnv>,
    public_key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let public_key = read_buffer(memory, &ctx, public_key)?;
    let address = env
        .get_interface()
        .evm_get_address_from_pubkey(&public_key)?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &address)?.offset();
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(public_key)],
        return_value: AbiTraceType::ByteArray(address),
        sub_calls: None,
    });
    Ok(ptr as i32)
}

/// Get public key from signature (EVM)
#[named]
pub(crate) fn assembly_script_evm_get_pubkey_from_signature(
    mut ctx: FunctionEnvMut<ASEnv>,
    data: i32,
    signature: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let data = read_buffer(memory, &ctx, data)?;
    let signature = read_buffer(memory, &ctx, signature)?;
    let public_key = env
        .get_interface()
        .evm_get_pubkey_from_signature(&data, &signature)?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &public_key)?.offset();
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::ByteArray(data),
            AbiTraceType::ByteArray(signature),
        ],
        return_value: AbiTraceType::ByteArray(public_key),
        sub_calls: None,
    });
    Ok(ptr as i32)
}

#[named]
/// Return Ok(1) if the address is a User address, Ok(0) if it is an SC address
pub(crate) fn assembly_script_is_address_eoa(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let res = env.get_interface().is_address_eoa(&address)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(address)],
        return_value: AbiTraceType::Bool(res),
        sub_calls: None,
    });
    Ok(res as i32)
}

/// converts a public key to an address
#[named]
pub(crate) fn assembly_script_address_from_public_key(
    mut ctx: FunctionEnvMut<ASEnv>,
    public_key: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let public_key = read_string(memory, &ctx, public_key)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, public_key.len(), true);
    // }
    let addr = env.get_interface().address_from_public_key(&public_key)?;
    let ptr = pointer_from_string(&env, &mut ctx, &addr)?.offset() as i32;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(public_key)],
        return_value: AbiTraceType::String(addr),
        sub_calls: None,
    });
    Ok(ptr)
}

/// Validates an address is correct
#[named]
pub(crate) fn assembly_script_validate_address(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
) -> ABIResult<i32> {
    let env = ctx.data().clone();
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let res = env.get_interface().validate_address(&address)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(address)],
        return_value: AbiTraceType::Bool(res),
        sub_calls: None,
    });
    Ok(res as i32)
}

/// generates an unsafe random number
#[named]
pub(crate) fn assembly_script_unsafe_random(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let res = env.get_interface().unsafe_random()?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::I64(res),
        sub_calls: None,
    });
    Ok(res)
}

/// gets the current unix timestamp in milliseconds
#[named]
pub(crate) fn assembly_script_get_time(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let res = env.get_interface().get_time()?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::U64(res),
        sub_calls: None,
    });
    Ok(res as i64)
}

/// sends an async message
#[allow(clippy::too_many_arguments)]
#[named]
pub(crate) fn assembly_script_send_message(
    mut ctx: FunctionEnvMut<ASEnv>,
    target_address: i32,
    target_handler: i32,
    validity_start_period: i64,
    validity_start_thread: i32,
    validity_end_period: i64,
    validity_end_thread: i32,
    max_gas: i64,
    raw_fee: i64,
    raw_coins: i64,
    data: i32,
    filter_address: i32,
    filter_datastore_key: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let validity_start: (u64, u8) = match (
        validity_start_period.try_into(),
        validity_start_thread.try_into(),
    ) {
        (Ok(p), Ok(t)) => (p, t),
        (Err(_), _) => abi_bail!("negative validity start period"),
        (_, Err(_)) => abi_bail!("invalid validity start thread"),
    };
    let validity_end: (u64, u8) = match (
        validity_end_period.try_into(),
        validity_end_thread.try_into(),
    ) {
        (Ok(p), Ok(t)) => (p, t),
        (Err(_), _) => abi_bail!("negative validity end period"),
        (_, Err(_)) => abi_bail!("invalid validity end thread"),
    };
    if max_gas.is_negative() {
        abi_bail!("negative max gas");
    }
    if raw_fee.is_negative() {
        abi_bail!("negative raw_fee");
    }
    if raw_coins.is_negative() {
        abi_bail!("negative coins")
    }
    let memory = get_memory!(env);
    let target_address = read_string(memory, &ctx, target_address)?;
    let target_handler = read_string(memory, &ctx, target_handler)?;
    let data = read_buffer(memory, &ctx, data)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, target_address.len(),
    // true);     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, target_handler.len(),
    // true);     let fname = format!("massa.{}:2", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, data.len(), true);
    // }
    let filter_address_string = read_string(memory, &ctx, filter_address)?;
    let key = read_buffer(memory, &ctx, filter_datastore_key)?;
    let filter = match (filter_address_string.as_str(), key.as_slice()) {
        ("", _) => None,
        (addr, &[]) => Some((addr, None)),
        (addr, key) => Some((addr, Some(key))),
    };

    env.get_interface().send_message(
        &target_address,
        &target_handler,
        validity_start,
        validity_end,
        max_gas as u64,
        raw_fee as u64,
        raw_coins as u64,
        &data,
        filter,
    )?;

    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(target_address),
            AbiTraceType::String(target_handler),
            AbiTraceType::Slot(validity_start),
            AbiTraceType::Slot(validity_end),
            AbiTraceType::U64(max_gas as u64),
            AbiTraceType::U64(raw_fee as u64),
            AbiTraceType::U64(raw_coins as u64),
            AbiTraceType::ByteArray(data),
            AbiTraceType::String(filter_address_string),
            AbiTraceType::ByteArray(key),
        ],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });

    Ok(())
}

/// converts a public key to an address
#[named]
pub(crate) fn assembly_script_get_origin_operation_id(
    mut ctx: FunctionEnvMut<ASEnv>,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let operation_id = env
        .get_interface()
        .get_origin_operation_id()?
        .unwrap_or_default();
    let ptr = pointer_from_string(&env, &mut ctx, &operation_id)?.offset() as i32;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::String(operation_id),
        sub_calls: None,
    });
    Ok(ptr)
}

/// gets the period of the current execution slot
#[named]
pub(crate) fn assembly_script_get_current_period(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let current_period = env.get_interface().get_current_period()?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::U64(current_period),
        sub_calls: None,
    });
    Ok(current_period as i64)
}

/// gets the thread of the current execution slot
#[named]
pub(crate) fn assembly_script_get_current_thread(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let current_thread = env.get_interface().get_current_thread()?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::U8(current_thread),
        sub_calls: None,
    });
    Ok(current_thread as i32)
}

/// sets the executable bytecode of an arbitrary address
#[named]
pub(crate) fn assembly_script_set_bytecode_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    bytecode: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let bytecode_raw = read_buffer(memory, &ctx, bytecode)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, address.len(), true);
    //     let fname = format!("massa.{}:1", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, bytecode_raw.len(), true);
    // }
    env.get_interface()
        .raw_set_bytecode_for(&address, &bytecode_raw)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(address),
            AbiTraceType::ByteArray(bytecode_raw),
        ],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// sets the executable bytecode of the current address
#[named]
pub(crate) fn assembly_script_set_bytecode(
    mut ctx: FunctionEnvMut<ASEnv>,
    bytecode: i32,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let bytecode_raw = read_buffer(memory, &ctx, bytecode)?;
    // Do not remove this. It could be used for gas_calibration in future.
    // if cfg!(feature = "gas_calibration") {
    //     let fname = format!("massa.{}:0", function_name!());
    //     param_size_update(&env, &mut ctx, &fname, bytecode_raw.len(), true);
    // }
    env.get_interface().raw_set_bytecode(&bytecode_raw)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::ByteArray(bytecode_raw)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// get bytecode of the current address
#[named]
pub(crate) fn assembly_script_get_bytecode(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let data = env.get_interface().raw_get_bytecode()?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &data)?.offset() as i32;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::ByteArray(data),
        sub_calls: None,
    });
    Ok(ptr)
}

/// get bytecode of the target address
#[named]
pub(crate) fn assembly_script_get_bytecode_for(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = read_string(memory, &ctx, address)?;
    let data = env.get_interface().raw_get_bytecode_for(&address)?;
    let ptr = pointer_from_bytearray(&env, &mut ctx, &data)?.offset() as i32;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(address)],
        return_value: AbiTraceType::ByteArray(data),
        sub_calls: None,
    });
    Ok(ptr)
}

/// execute `function` of the given bytecode in the current context
#[named]
pub(crate) fn assembly_script_local_execution(
    mut ctx: FunctionEnvMut<ASEnv>,
    bytecode: i32,
    function: i32,
    param: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);

    let bytecode = read_buffer(memory, &ctx, bytecode)?;
    let function = read_string(memory, &ctx, function)?;
    let param = read_buffer(memory, &ctx, param)?;
    let response = local_call(&mut ctx, &bytecode, &function, &param, true)?;
    let res = match BufferPtr::alloc(&response.ret, env.get_ffi_env(), &mut ctx) {
        Ok(ret) => Ok(ret.offset() as i32),
        _ => abi_bail!(format!(
            "Cannot allocate response in local call of {}",
            function
        )),
    };
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::ByteArray(bytecode),
            AbiTraceType::String(function),
            AbiTraceType::ByteArray(param),
        ],
        return_value: AbiTraceType::ByteArray(response.ret.clone()),
        sub_calls: Some(response.trace),
    });
    res
}

/// execute `function` of the bytecode located at `address` in the current
/// context
#[named]
pub(crate) fn assembly_script_local_call(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    function: i32,
    param: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);

    let address = &read_string(memory, &ctx, address)?;
    let bytecode = env.get_interface().raw_get_bytecode_for(address)?;
    let function = read_string(memory, &ctx, function)?;
    let param = read_buffer(memory, &ctx, param)?;

    let response = local_call(&mut ctx, &bytecode, &function, &param, false)?;
    let res = match BufferPtr::alloc(&response.ret, env.get_ffi_env(), &mut ctx) {
        Ok(ret) => Ok(ret.offset() as i32),
        _ => abi_bail!(format!(
            "Cannot allocate response in local call of {}",
            function
        )),
    };
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::ByteArray(bytecode),
            AbiTraceType::String(function),
            AbiTraceType::ByteArray(param),
        ],
        return_value: AbiTraceType::ByteArray(response.ret.clone()),
        sub_calls: Some(response.trace),
    });

    res
}

/// Check whether or not the caller has write access in the current context
#[named]
pub fn assembly_script_caller_has_write_access(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let has_write_access = env.get_interface().caller_has_write_access()?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::Bool(has_write_access),
        sub_calls: None,
    });
    Ok(has_write_access as i32)
}

/// Check whether the given function exists at the given address
#[named]
pub fn assembly_script_function_exists(
    mut ctx: FunctionEnvMut<ASEnv>,
    address: i32,
    function: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let address = &read_string(memory, &ctx, address)?;
    let function = &read_string(memory, &ctx, function)?;
    let function_exists = function_exists(&mut ctx, address, function)?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::Bool(function_exists),
        sub_calls: None,
    });
    Ok(function_exists as i32)
}

/// Return current chain id
#[named]
pub(crate) fn assembly_script_chain_id(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<u64> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let chain_id = env.get_interface().chain_id()?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::U64(chain_id),
        sub_calls: None,
    });
    Ok(chain_id as u64)
}

/// Assembly script builtin `abort` function.
///
/// It prints the origin filename, an error messag, the line and column.
#[allow(unused_macros)]
#[allow(unused_mut)]
#[named]
pub fn assembly_script_abort(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
    filename: StringPtr,
    line: i32,
    col: i32,
) -> ABIResult<()> {
    let memory = ctx
        .data()
        .get_ffi_env()
        .memory
        .as_ref()
        .expect("Failed to get memory on env")
        .clone();
    let message_ = message
        .read(&memory, &ctx)
        .map_err(|e| wasmer::RuntimeError::new(e.to_string()));
    let filename_ = filename
        .read(&memory, &ctx)
        .map_err(|e| wasmer::RuntimeError::new(e.to_string()));

    if message_.is_err() || filename_.is_err() {
        abi_bail!("aborting failed to load message or filename")
    }
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![
            AbiTraceType::String(message_.clone().unwrap_or_default()),
            AbiTraceType::String(filename_.clone().unwrap_or_default()),
            AbiTraceType::I32(line),
            AbiTraceType::I32(col),
        ],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    abi_bail!(format!(
        "error: {} at {}:{} col: {}",
        message_.unwrap(),
        filename_.unwrap(),
        line,
        col
    ));
}

/// Assembly script builtin `seed` function
#[named]
pub fn assembly_script_seed(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<f64> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }
    let seed = match env.interface.unsafe_random_f64() {
        Ok(ret) => ret,
        _ => abi_bail!("failed to get random from interface"),
    };
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::F64(seed),
        sub_calls: None,
    });
    Ok(seed)
}

/// Assembly script builtin `Date.now()`
#[named]
pub fn assembly_script_date_now(mut ctx: FunctionEnvMut<ASEnv>) -> ABIResult<f64> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }
    let utime = match env.interface.get_time() {
        Ok(time) => time,
        _ => abi_bail!("failed to get time from interface"),
    };
    let ret = utime as f64;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![],
        return_value: AbiTraceType::F64(ret),
        sub_calls: None,
    });
    Ok(ret)
}

/// Assembly script builtin `console.log()`.
#[named]
pub fn assembly_script_console_log(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }

    assembly_script_console(ctx, message, "LOG")
}

/// Assembly script builtin `console.info()`.
#[named]
pub fn assembly_script_console_info(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }
    assembly_script_console(ctx, message, "INFO")
}

/// Assembly script builtin `console.warn()`.
#[named]
pub fn assembly_script_console_warn(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }
    assembly_script_console(ctx, message, "WARN")
}

/// Assembly script builtin `console.debug()`.
#[named]
pub fn assembly_script_console_debug(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }

    assembly_script_console(ctx, message, "DEBUG")
}

/// Assembly script builtin `console.error()`.
#[named]
pub fn assembly_script_console_error(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }
    assembly_script_console(ctx, message, "ERROR")
}

/// Assembly script console functions
#[allow(unused_macros)]
#[allow(unused_mut)]
#[named]
fn assembly_script_console(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
    prefix: &str,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;

    let memory = ctx
        .data()
        .get_ffi_env()
        .memory
        .as_ref()
        .expect("Failed to get memory on env")
        .clone();
    let message = prefix
        .to_string()
        .add(" | ")
        .add(&message.read(&memory, &ctx)?);

    env.get_interface().generate_event(message.clone())?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(message)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// Assembly script builtin `trace()`.
#[named]
#[allow(clippy::too_many_arguments)]
pub fn assembly_script_trace(
    mut ctx: FunctionEnvMut<ASEnv>,
    message: StringPtr,
    n: i32,
    a0: f64,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
) -> ABIResult<()> {
    let env = get_env(&ctx)?;
    if cfg!(not(feature = "gas_calibration")) {
        sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    }

    let memory = ctx
        .data()
        .get_ffi_env()
        .memory
        .as_ref()
        .expect("Failed to get memory on env")
        .clone();

    let message = message.read(&memory, &ctx)?;

    let message_for_event = match n {
        1 => format!("msg: {}, a0: {}", message, a0),
        2 => format!("msg: {}, a0: {}, a1: {}", message, a0, a1),
        3 => format!("msg: {}, a0: {}, a1: {}, a2: {}", message, a0, a1, a2),
        4 => format!(
            "msg: {}, a0: {}, a1: {}, a2: {}, a3: {}",
            message, a0, a1, a2, a3
        ),
        5 => format!(
            "msg: {}, a0: {}, a1: {}, a2: {}, a3: {}, a4: {}",
            message, a0, a1, a2, a3, a4
        ),
        _ => message, // Should we warn here or return an error?
    };

    env.get_interface()
        .generate_event(message_for_event.clone())?;
    #[cfg(feature = "execution-trace")]
    ctx.data_mut().trace.push(AbiTrace {
        name: function_name!().to_string(),
        params: vec![AbiTraceType::String(message_for_event)],
        return_value: AbiTraceType::None,
        sub_calls: None,
    });
    Ok(())
}

/// Assembly script builtin `process.exit()`.
pub fn assembly_script_process_exit(_ctx: FunctionEnvMut<ASEnv>, exit_code: i32) -> ABIResult<()> {
    abi_bail!(format!("exit with code: {}", exit_code));
}

/// Tooling, return a StringPtr allocated from a String
fn pointer_from_string(
    env: &ASEnv,
    ctx: &mut impl AsStoreMut,
    value: &str,
) -> ABIResult<StringPtr> {
    Ok(*StringPtr::alloc(&value.into(), env.get_ffi_env(), ctx)?)
}

/// Tooling, return a BufferPtr allocated from bytes
fn pointer_from_bytearray(
    env: &ASEnv,
    ctx: &mut impl AsStoreMut,
    value: &Vec<u8>,
) -> ABIResult<BufferPtr> {
    Ok(*BufferPtr::alloc(value, env.get_ffi_env(), ctx)?)
}

/// Tooling that reads a buffer (Vec<u8>) in memory
fn read_buffer(memory: &Memory, store: &impl AsStoreRef, offset: i32) -> ABIResult<Vec<u8>> {
    Ok(BufferPtr::new(offset as u32).read(memory, store)?)
}

/// Tooling, return a string from a given offset
fn read_string(memory: &Memory, store: &impl AsStoreRef, ptr: i32) -> ABIResult<String> {
    Ok(StringPtr::new(ptr as u32).read(memory, store)?)
}

/// Tooling, return a pointer offset of a serialized list in json
fn alloc_string_array(ctx: &mut FunctionEnvMut<ASEnv>, vec: &[String]) -> ABIResult<i32> {
    let env = get_env(ctx)?;
    let addresses = serde_json::to_string(vec)?;
    Ok(StringPtr::alloc(&addresses, env.get_ffi_env(), ctx)?.offset() as i32)
}

/// Flatten a Vec<Vec<u8>> (or anything that can be turned into an iterator) to
/// a Vec<u8> with the format: L (32 bits LE) V1_L (8 bits) V1 (8bits * V1_L),
/// V2_L ... VN (8 bits * VN_L)
fn ser_bytearray_vec<'a, I>(data: I, data_len: usize, max_length: usize) -> ABIResult<Vec<u8>>
where
    I: IntoIterator<Item = &'a Vec<u8>>,
{
    if data_len == 0 {
        return Ok(Vec::new());
    }

    if data_len > max_length {
        abi_bail!("Too many entries in the datastore");
    }

    // pre alloc with max capacity
    let mut buffer = Vec::with_capacity(4 + (data_len * (1 + 255)));

    let entry_count = u32::try_from(data_len).unwrap();
    buffer.extend_from_slice(&entry_count.to_le_bytes());

    for key in data.into_iter() {
        let k_len = match u8::try_from(key.len()) {
            Ok(l) => l,
            Err(_) => abi_bail!("Some Datastore keys are too long"),
        };
        buffer.push(k_len);
        buffer.extend_from_slice(&key[..]);
    }

    Ok(buffer)
}

/// performs a sha256 hash on byte array and returns the hash as byte array
#[named]
pub(crate) fn assembly_script_hash_sha256(
    mut ctx: FunctionEnvMut<ASEnv>,
    bytes: i32,
) -> ABIResult<i32> {
    let env = get_env(&ctx)?;
    sub_remaining_gas_abi(&env, &mut ctx, function_name!())?;
    let memory = get_memory!(env);
    let bytes = read_buffer(memory, &ctx, bytes)?;
    let hash = env.get_interface().hash_sha256(&bytes)?.to_vec();
    let ptr = pointer_from_bytearray(&env, &mut ctx, &hash)?.offset();
    Ok(ptr as i32)
}

#[cfg(test)]
mod tests {
    use crate::as_execution::abi::ser_bytearray_vec;

    #[test]
    fn test_ser() {
        let vb: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![255]];

        let vb_ser = ser_bytearray_vec(&vb, vb.len(), 10).unwrap();
        assert_eq!(vb_ser, [2, 0, 0, 0, 3, 1, 2, 3, 1, 255]);
    }

    #[test]
    fn test_ser_edge_cases() {
        // FIXME: should we support theses edge cases or bail?

        let vb: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![]];

        let vb_ser = ser_bytearray_vec(&vb, vb.len(), 10).unwrap();
        assert_eq!(vb_ser, [2, 0, 0, 0, 3, 1, 2, 3, 0]);

        let vb_ser = ser_bytearray_vec(&vb, vb.len(), 1);
        assert!(vb_ser.is_err());

        let vb: Vec<Vec<u8>> = vec![];
        let vb_ser = ser_bytearray_vec(&vb, vb.len(), 10).unwrap();
        let empty_vec: Vec<u8> = vec![];
        assert_eq!(vb_ser, empty_vec);

        // A really big vec to serialize
        let vb: Vec<Vec<u8>> = (0..=u8::MAX)
            .cycle()
            .take(u16::MAX as usize)
            .map(|i| vec![i])
            .collect();
        assert_eq!(vb.len(), u16::MAX as usize);

        let vb_ser = ser_bytearray_vec(&vb, vb.len(), u16::MAX as usize).unwrap();
        assert_eq!(vb_ser[0..4], [255, 255, 0, 0]);
        assert_eq!(vb_ser[4], 1);
        assert_eq!(vb_ser[4 + 1], 0);
        assert_eq!(vb_ser[4 + 2], 1);
        assert_eq!(vb_ser[4 + 3], 1);
        assert_eq!(vb_ser[vb_ser.len() - 2], 1);
        assert_eq!(vb_ser[vb_ser.len() - 1], 254);
    }
}
