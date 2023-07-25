use anyhow::{anyhow, bail, Result};
use std::{
    collections::{BTreeSet, HashMap},
    path::PathBuf,
};

use crate::execution::RuntimeModule;

/// That's what is returned when a module is executed correctly since the end
#[derive(Debug)]
pub struct Response {
    /// returned value from the module call
    pub ret: Vec<u8>,
    /// number of gas that remain after the execution (metering)
    pub remaining_gas: u64,
    /// number of gas required for the instance creation
    pub init_gas_cost: u64,
}

pub trait InterfaceClone {
    fn clone_box(&self) -> Box<dyn Interface>;
}

impl Clone for Box<dyn Interface> {
    fn clone(&self) -> Box<dyn Interface> {
        self.clone_box()
    }
}

macro_rules! unimplemented {
    ($fn: expr) => {
        bail!(format!("unimplemented function {} in interface", $fn))
    };
}

#[derive(Clone, Debug)]
pub struct GasCosts {
    pub(crate) operator_cost: u64,
    pub(crate) launch_cost: u64,
    pub(crate) abi_costs: HashMap<String, u64>,
    pub sp_compilation_cost: u64,
}

impl GasCosts {
    pub fn new(abi_cost_file: PathBuf, wasm_abi_file: PathBuf) -> Result<Self> {
        let abi_cost_file = std::fs::read_to_string(abi_cost_file)?;
        let mut abi_costs: HashMap<String, u64> = serde_json::from_str(&abi_cost_file)?;
        abi_costs.iter_mut().for_each(|(_, v)| {
            let unit_digit = *v % 10;
            if unit_digit > 5 {
                *v += 10 - unit_digit;
            } else {
                *v -= unit_digit;
            }
        });
        let wasm_abi_file = std::fs::read_to_string(wasm_abi_file)?;
        let wasm_costs: HashMap<String, u64> = serde_json::from_str(&wasm_abi_file)?;
        Ok(Self {
            operator_cost: wasm_costs.values().copied().sum::<u64>() / wasm_costs.len() as u64,
            launch_cost: *abi_costs
                .get("launch")
                .ok_or_else(|| anyhow!("launch cost not found in ABI gas cost file."))?,
            sp_compilation_cost: *abi_costs
                .get("sp_compilation_cost")
                .ok_or_else(|| anyhow!("sp_compilation_cost not found in ABI gas cost file."))?,
            abi_costs,
        })
    }
}

#[cfg(any(test, feature = "gas_calibration", feature = "testing"))]
impl Default for GasCosts {
    fn default() -> Self {
        let mut abi_costs = HashMap::new();
        abi_costs.insert(String::from("assembly_script_address_from_public_key"), 147);
        abi_costs.insert(String::from("assembly_script_validate_address"), 4);
        abi_costs.insert(String::from("assembly_script_append_data"), 162);
        abi_costs.insert(String::from("assembly_script_append_data_for"), 200);
        abi_costs.insert(String::from("assembly_script_call"), 30466);
        abi_costs.insert(String::from("assembly_script_create_sc"), 160);
        abi_costs.insert(String::from("assembly_script_delete_data"), 78);
        abi_costs.insert(String::from("assembly_script_delete_data_for"), 120);
        abi_costs.insert(String::from("assembly_script_generate_event"), 36);
        abi_costs.insert(String::from("assembly_script_get_balance"), 4);
        abi_costs.insert(String::from("assembly_script_get_balance_for"), 41);
        abi_costs.insert(String::from("assembly_script_get_call_coins"), 9);
        abi_costs.insert(String::from("assembly_script_get_call_stack"), 56);
        abi_costs.insert(String::from("assembly_script_get_current_period"), 9);
        abi_costs.insert(String::from("assembly_script_get_current_thread"), 8);
        abi_costs.insert(String::from("assembly_script_get_data"), 85);
        abi_costs.insert(String::from("assembly_script_get_data_for"), 139);
        abi_costs.insert(String::from("assembly_script_get_keys"), 26);
        abi_costs.insert(String::from("assembly_script_get_keys_for"), 48);
        abi_costs.insert(String::from("assembly_script_get_op_data"), 71);
        abi_costs.insert(String::from("assembly_script_get_op_keys"), 138);
        abi_costs.insert(String::from("assembly_script_get_owned_addresses"), 52);
        abi_costs.insert(String::from("assembly_script_get_remaining_gas"), 7);
        abi_costs.insert(String::from("assembly_script_get_time"), 4);
        abi_costs.insert(String::from("assembly_script_has_data"), 69);
        abi_costs.insert(String::from("assembly_script_has_data_for"), 115);
        abi_costs.insert(String::from("assembly_script_has_op_key"), 78);
        abi_costs.insert(String::from("assembly_script_hash"), 83);
        abi_costs.insert(String::from("assembly_script_hash_sha256"), 83);
        abi_costs.insert(String::from("assembly_script_keccak256_hash"), 83);
        abi_costs.insert(String::from("assembly_script_print"), 35);
        abi_costs.insert(String::from("assembly_script_send_message"), 316);
        abi_costs.insert(String::from("assembly_script_get_origin_operation_id"), 200);
        abi_costs.insert(String::from("assembly_script_set_bytecode"), 74);
        abi_costs.insert(String::from("assembly_script_set_bytecode_for"), 129);
        abi_costs.insert(String::from("assembly_script_set_data"), 158);
        abi_costs.insert(String::from("assembly_script_set_data_for"), 165);
        abi_costs.insert(String::from("assembly_script_signature_verify"), 98);
        abi_costs.insert(String::from("assembly_script_evm_signature_verify"), 264);
        abi_costs.insert(
            String::from("assembly_script_evm_get_address_from_pubkey"),
            11,
        );
        abi_costs.insert(
            String::from("assembly_script_evm_get_pubkey_from_signature"),
            11,
        );
        abi_costs.insert(String::from("assembly_script_is_address_eoa"), 11);
        abi_costs.insert(String::from("assembly_script_transfer_coins"), 62);
        abi_costs.insert(String::from("assembly_script_transfer_coins_for"), 102);
        abi_costs.insert(String::from("assembly_script_unsafe_random"), 11);
        abi_costs.insert(String::from("assembly_script_call"), 11);
        abi_costs.insert(String::from("assembly_script_local_call"), 11);
        abi_costs.insert(String::from("assembly_script_local_execution"), 11);
        abi_costs.insert(String::from("assembly_script_get_bytecode"), 11);
        abi_costs.insert(String::from("assembly_script_get_bytecode_for"), 11);
        abi_costs.insert(String::from("assembly_script_caller_has_write_access"), 11);
        abi_costs.insert(String::from("assembly_script_function_exists"), 11);
        abi_costs.insert(String::from("assembly_script_seed"), 11);
        abi_costs.insert(String::from("assembly_script_abort"), 11);
        abi_costs.insert(String::from("assembly_script_date_now"), 11);
        abi_costs.insert(String::from("assembly_script_console_log"), 36); // same cost as for generate_event
        abi_costs.insert(String::from("assembly_script_console_info"), 36);
        abi_costs.insert(String::from("assembly_script_console_debug"), 36);
        abi_costs.insert(String::from("assembly_script_console_warn"), 36);
        abi_costs.insert(String::from("assembly_script_console_error"), 36);
        abi_costs.insert(String::from("assembly_script_trace"), 36);
        Self {
            operator_cost: 1,
            launch_cost: 10_000,
            abi_costs,
            sp_compilation_cost: 10_000,
        }
    }
}

#[allow(unused_variables)]
pub trait Interface: Send + Sync + InterfaceClone {
    /// Prepare the execution of a module at the given address and transfer a given amount of coins
    fn init_call(&self, address: &str, raw_coins: u64) -> Result<Vec<u8>> {
        unimplemented!("init_call")
    }

    /// Finish a call
    fn finish_call(&self) -> Result<()> {
        unimplemented!("finish_call")
    }

    /// Get the SCE ledger balance for an address.
    /// Defaults to zero if the address is not found.
    fn get_balance(&self, address: &str) -> Result<u64> {
        unimplemented!("get_balance_for")
    }

    /// Transfer an amount from the specified address to a target address.
    fn transfer_coins(&self, from_address: &str, to_address: &str, raw_amount: u64) -> Result<()> {
        unimplemented!("transfer_coins_for")
    }

    /// Get the amount of coins that have been made available for use by the caller of the currently executing code.
    fn get_call_coins(&self) -> Result<u64> {
        bail!("unimplemented function get_call_coins_for in interface")
    }

    /// Sets the executable bytecode at a target address.
    /// The target address must exist and the current context must have access rights.
    fn set_bytecode(&self, address: &str, bytecode: &[u8]) -> Result<()> {
        unimplemented!("set_bytecode")
    }

    /// Requires a new address that contains the sent &[u8]
    fn create_module(&self, module: &[u8]) -> Result<String> {
        unimplemented!("create_module")
    }

    /// Return datastore keys
    /// Will only return keys with a given prefix if provided in args
    fn get_ds_keys(&self, address: &str, prefix: &[u8]) -> Result<BTreeSet<Vec<u8>>> {
        unimplemented!("get_ds_keys")
    }

    /// Requires the data at the address
    fn get_ds_value(&self, address: &str, key: &[u8]) -> Result<Vec<u8>> {
        unimplemented!("get_ds_value")
    }

    /// Set the datastore value for the corresponding key of the given address
    fn set_ds_value(&self, address: &str, key: &[u8], value: &[u8]) -> Result<()> {
        unimplemented!("set_ds_value")
    }

    /// Append a value to the current datastore value for the corresponding key and the given address
    fn append_ds_value(&self, address: &str, key: &[u8], value: &[u8]) -> Result<()> {
        unimplemented!("append_ds_value")
    }

    /// Delete a datastore entry at of the given address
    fn delete_ds_entry(&self, address: &str, key: &[u8]) -> Result<()> {
        unimplemented!("delete_ds_entry")
    }

    /// Check if a datastore entry exists
    fn ds_entry_exists(&self, address: &str, key: &[u8]) -> Result<bool> {
        unimplemented!("ds_entry_exists")
    }

    /// Returns bytecode of the target address
    fn get_bytecode(&self, address: &str) -> Result<Vec<u8>> {
        unimplemented!("get_bytecode")
    }

    /// Return operation datastore keys
    fn get_op_keys(&self, prefix: &[u8]) -> Result<BTreeSet<Vec<u8>>> {
        unimplemented!("get_op_keys")
    }

    /// Check if key is in operation datastore
    fn op_entry_exists(&self, key: &[u8]) -> Result<bool> {
        unimplemented!("op_entry_exists")
    }

    /// Return operation datastore data for a given key
    fn get_op_value(&self, key: &[u8]) -> Result<Vec<u8>> {
        unimplemented!("get_op_value")
    }

    /// Check whether or not the caller has write access in the current context
    fn caller_has_write_access(&self) -> Result<bool> {
        unimplemented!("caller_has_write_access")
    }

    /// Hash data
    fn hash(&self, data: &[u8]) -> Result<[u8; 32]> {
        unimplemented!("hash")
    }

    /// Verify signature
    fn signature_verify(&self, data: &[u8], signature: &str, public_key: &str) -> Result<bool> {
        unimplemented!("signature_verify")
    }

    /// Verify signature (EVM)
    fn evm_signature_verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool> {
        unimplemented!("evm_signature_verify")
    }

    /// Get address from public key (EVM)
    fn evm_get_address_from_pubkey(&self, public_key: &[u8]) -> Result<Vec<u8>> {
        unimplemented!("evm_get_address_from_pubkey")
    }

    /// Get public key from signature (EVM)
    fn evm_get_pubkey_from_signature(&self, hash: &[u8], signature: &[u8]) -> Result<Vec<u8>> {
        unimplemented!("evm_get_pubkey_from_signature")
    }

    /// Return true if the address is a User address, false if it is an SC address
    fn is_address_eoa(&self, address: &str) -> Result<bool> {
        unimplemented!("is_address_eoa")
    }

    /// Convert a public key to an address
    fn address_from_public_key(&self, public_key: &str) -> Result<String> {
        unimplemented!("address_from_public_key")
    }

    /// Validate an address
    fn validate_address(&self, address: &str) -> Result<bool> {
        unimplemented!("validate_address")
    }

    /// Returns the current time (millisecond unix timestamp)
    fn get_time(&self) -> Result<u64> {
        unimplemented!("get_time")
    }

    /// Returns a random number (unsafe: can be predicted and manipulated)
    fn unsafe_random(&self) -> Result<i64> {
        unimplemented!("unsafe_random")
    }

    /// Returns a random number (unsafe: can be predicted and manipulated)
    fn unsafe_random_f64(&self) -> Result<f64> {
        unimplemented!("unsafe_random_f64")
    }

    /// Returns the period of the current execution slot
    fn get_current_period(&self) -> Result<u64> {
        unimplemented!("get_current_period")
    }

    /// Returns the thread of the current execution slot
    fn get_current_thread(&self) -> Result<u8> {
        unimplemented!("get_current_thread")
    }

    /// Expect to return a list of owned addresses
    ///
    /// Required on smart-contract execute the imported function
    /// `assembly_script_get_owned_addresses`
    fn get_owned_addresses(&self) -> Result<Vec<String>> {
        unimplemented!("get_owned_addresses")
    }

    /// Expect to return a list of addresses in the call stack
    ///
    /// Required on smart-contract execute the imported function
    /// `assembly_script_get_call_stack`
    fn get_call_stack(&self) -> Result<Vec<String>> {
        unimplemented!("get_call_stack")
    }

    /// Generate a smart contract event
    fn generate_event(&self, _event: String) -> Result<()> {
        unimplemented!("generate_event")
    }

    /// For the given bytecode:
    ///
    /// * Get the corresponding runtime module if it already exists
    /// * Compile it if not
    fn get_module(&self, bytecode: &[u8], limit: u64) -> Result<RuntimeModule> {
        unimplemented!("get_module")
    }

    /// Sends an async message
    ///
    /// # Arguments
    ///
    /// * `target_address` - Destination address hash in format string
    /// * `target_handler` - Name of the message handling function
    /// * `validity_start` - Tuple containing the period and thread of the validity start slot
    /// * `validity_end` - Tuple containing the period and thread of the validity end slot
    /// * `max_gas` - Maximum gas for the message execution
    /// * `raw_fee` - Fee to be paid for message execution
    /// * `coins` - Coins of the sender
    /// * `data` - Message data
    ///
    #[allow(clippy::too_many_arguments)]
    fn send_message(
        &self,
        target_address: &str,
        target_handler: &str,
        validity_start: (u64, u8),
        validity_end: (u64, u8),
        max_gas: u64,
        raw_fee: u64,
        raw_coins: u64,
        data: &[u8],
        filter: Option<(&str, Option<&[u8]>)>,
    ) -> Result<()> {
        unimplemented!("send_message")
    }

    // Returns the operation id that originated the current execution if there is one
    fn get_origin_operation_id(&self) -> Result<Option<String>> {
        unimplemented!("get_origin_operation_id")
    }

    // Sha256 hash bytes
    fn hash_sha256(&self, bytes: &[u8]) -> Result<[u8; 32]> {
        unimplemented!("hash_sha256")
    }

    // Keccak256 hash bytes
    fn hash_keccak256(&self, bytes: &[u8]) -> Result<[u8; 32]> {
        unimplemented!("hash_keccak256")
    }
}
