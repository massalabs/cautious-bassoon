var searchIndex = JSON.parse('{\
"massa_sc_runtime":{"doc":"","t":"NDIIDELLLLLLLLLLLLKLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMFFLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["ASModule","GasCosts","Interface","InterfaceClone","Response","RuntimeModule","address_from_public_key","address_from_public_key","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","caller_has_write_access","caller_has_write_access","clone","clone","clone_box","clone_into","clone_into","create_module","create_module","deref","deref","deref","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","drop","drop","drop","finish_call","finish_call","fmt","fmt","from","from","from","generate_event","generate_event","get_balance","get_balance","get_balance_for","get_balance_for","get_call_coins","get_call_coins","get_call_stack","get_call_stack","get_current_period","get_current_period","get_current_thread","get_current_thread","get_data","get_data_for","get_keys","get_keys","get_keys_for","get_keys_for","get_matching_keys_for","get_matching_keys_for","get_module","get_module","get_op_data","get_op_data","get_op_keys","get_op_keys","get_owned_addresses","get_owned_addresses","get_time","get_time","has_data","has_data","has_data_for","has_data_for","has_op_key","has_op_key","hash","hash","hash_sha256","hash_sha256","init","init","init","init_call","init_call","init_cost","into","into","into","new","new","pointer_metadata","pointer_metadata","pointer_metadata","print","print","raw_append_data","raw_append_data","raw_append_data_for","raw_append_data_for","raw_delete_data","raw_delete_data","raw_delete_data_for","raw_delete_data_for","raw_get_bytecode","raw_get_bytecode","raw_get_bytecode_for","raw_get_bytecode_for","raw_get_data","raw_get_data","raw_get_data_for","raw_get_data_for","raw_set_bytecode","raw_set_bytecode","raw_set_bytecode_for","raw_set_bytecode_for","raw_set_data","raw_set_data","raw_set_data_for","raw_set_data_for","remaining_gas","ret","run_function","run_main","send_message","send_message","set_data","set_data_for","signature_verify","signature_verify","to_owned","to_owned","transfer_coins","transfer_coins","transfer_coins_for","transfer_coins_for","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","unsafe_random","unsafe_random","unsafe_random_f64","unsafe_random_f64","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_ref","upcast_any_ref","upcast_any_ref"],"q":["massa_sc_runtime","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","That’s what is returned when a module is executed …","","","","","","","","","","Check whether or not the caller has write access in the …","Check whether or not the caller has write access in the …","","","","","","Requires a new address that contains the sent &amp;u8","Requires a new address that contains the sent &amp;u8","","","","","","","","","","","","","Finish a call","Finish a call","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Generate a smart contract event","Generate a smart contract event","Get the SCE ledger balance for the current address. …","Get the SCE ledger balance for the current address. …","Get the SCE ledger balance for an address. Defaults to …","Get the SCE ledger balance for an address. Defaults to …","Get the amount of coins that have been made available for …","Get the amount of coins that have been made available for …","Expect to return a list of addresses in the call stack","Expect to return a list of addresses in the call stack","Returns the period of the current execution slot","Returns the period of the current execution slot","Returns the thread of the current execution slot","Returns the thread of the current execution slot","","","Return datastore keys","Return datastore keys","Return datastore keys","Return datastore keys","Return datastore keys matching the given prefix","Return datastore keys matching the given prefix","For the given bytecode:","For the given bytecode:","Return operation datastore data for a given key","Return operation datastore data for a given key","Return operation datastore keys","Return operation datastore keys","Expect to return a list of owned addresses","Expect to return a list of owned addresses","Returns the current time (millisecond unix timestamp)","Returns the current time (millisecond unix timestamp)","Requires to replace the data in the current address","Requires to replace the data in the current address","Check if a datastore entry exists","Check if a datastore entry exists","Check if key is in operation datastore","Check if key is in operation datastore","","","","","","","","Prepare the execution of a module at the given address and …","Prepare the execution of a module at the given address and …","number of gas required for the instance creation","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","TODO: Dispatch module creation corresponding to the first …","","","","","Print function for examples","Print function for examples","Append a value to the current datastore value for the …","Append a value to the current datastore value for the …","Append a value to the current datastore value for the …","Append a value to the current datastore value for the …","Delete a datastore entry","Delete a datastore entry","Delete a datastore entry at of the given address","Delete a datastore entry at of the given address","Returns bytecode of the current address","Returns bytecode of the current address","Returns bytecode of the target address","Returns bytecode of the target address","Return the datastore value of the corresponding key","Return the datastore value of the corresponding key","Requires the data at the address","Requires the data at the address","Sets the executable bytecode at a current address.","Sets the executable bytecode at a current address.","Sets the executable bytecode at a target address. The …","Sets the executable bytecode at a target address. The …","Set the datastore value for the corresponding key","Set the datastore value for the corresponding key","Set the datastore value for the corresponding key of the …","Set the datastore value for the corresponding key of the …","number of gas that remain after the execution (metering)","returned value from the module call","Library Input, take a <code>module</code> wasm built with the massa …","Library Input, take a <code>module</code> wasm built with the massa …","Sends an async message","Sends an async message","","","","","","","Transfer an amount from the address on the current call …","Transfer an amount from the address on the current call …","Transfer an amount from the specified address to a target …","Transfer an amount from the specified address to a target …","","","","","","","","","","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","","","","","","","","",""],"i":[5,0,0,0,0,0,7,7,5,12,6,5,12,6,7,7,5,6,27,5,6,7,7,5,12,6,5,12,6,5,12,6,5,12,6,7,7,12,6,5,12,6,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,5,12,6,7,7,12,5,12,6,5,6,5,12,6,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,12,12,0,0,7,7,7,7,7,7,5,6,7,7,7,7,5,12,6,5,12,6,5,12,6,7,7,7,7,5,12,6,5,12,6,5,12,6],"f":[0,0,0,0,0,0,[1,[[3,[2]]]],[1,[[3,[2]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[[3,[4]]]],[[],[[3,[4]]]],[5,5],[6,6],[[],[[8,[7]]]],[[]],[[]],[[],[[3,[2]]]],[[],[[3,[2]]]],[9],[9],[9],[9],[9],[9],[[],[[11,[10]]]],[[],[[11,[10]]]],[[],[[11,[10]]]],[9],[9],[9],[[],3],[[],3],[[12,13],14],[[6,13],14],[[]],[[]],[[]],[2,3],[2,3],[[],[[3,[15]]]],[[],[[3,[15]]]],[1,[[3,[15]]]],[1,[[3,[15]]]],[[],[[3,[15]]]],[[],[[3,[15]]]],[[],[[3,[[16,[2]]]]]],[[],[[3,[[16,[2]]]]]],[[],[[3,[15]]]],[[],[[3,[15]]]],[[],[[3,[17]]]],[[],[[3,[17]]]],[7,[[3,[18]]]],[[7,1],[[3,[18]]]],[[],[[3,[[19,[[16,[17]]]]]]]],[[],[[3,[[19,[[16,[17]]]]]]]],[1,[[3,[[19,[[16,[17]]]]]]]],[1,[[3,[[19,[[16,[17]]]]]]]],[[1,1],[[3,[[19,[[16,[17]]]]]]]],[[1,1],[[3,[[19,[[16,[17]]]]]]]],[15,[[3,[5]]]],[15,[[3,[5]]]],[[],[[3,[[16,[17]]]]]],[[],[[3,[[16,[17]]]]]],[[],[[3,[[16,[[16,[17]]]]]]]],[[],[[3,[[16,[[16,[17]]]]]]]],[[],[[3,[[16,[2]]]]]],[[],[[3,[[16,[2]]]]]],[[],[[3,[15]]]],[[],[[3,[15]]]],[[],[[3,[4]]]],[[],[[3,[4]]]],[1,[[3,[4]]]],[1,[[3,[4]]]],[[],[[3,[4]]]],[[],[[3,[4]]]],[[],3],[[],3],[[],3],[[],3],[[],9],[[],9],[[],9],[[1,15],[[3,[[16,[17]]]]]],[[1,15],[[3,[[16,[17]]]]]],0,[[]],[[]],[[]],[[15,6],[[3,[5]]]],[[20,20],[[3,[6]]]],[[]],[[]],[[]],[1,3],[1,3],[[],3],[[],3],[1,3],[1,3],[[],3],[[],3],[1,3],[1,3],[[],[[3,[[16,[17]]]]]],[[],[[3,[[16,[17]]]]]],[1,[[3,[[16,[17]]]]]],[1,[[3,[[16,[17]]]]]],[[],[[3,[[16,[17]]]]]],[[],[[3,[[16,[17]]]]]],[1,[[3,[[16,[17]]]]]],[1,[[3,[[16,[17]]]]]],[[],3],[[],3],[1,3],[1,3],[[],3],[[],3],[1,3],[1,3],0,0,[[7,5,1,15,6],[[3,[12]]]],[[7,5,15,6],[[3,[12]]]],[[1,1,15,15,15,21],3],[[1,1,15,15,15,21],3],[7,3],[[7,1],3],[[1,1],[[3,[4]]]],[[1,1],[[3,[4]]]],[[]],[[]],[[1,15],3],[[1,15],3],[[1,1,15],3],[[1,1,15],3],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],22],[[],22],[[],22],[[],[[3,[23]]]],[[],[[3,[23]]]],[[],[[3,[24]]]],[[],[[3,[24]]]],[[[8,[25]]],[[8,[26,25]]]],[[[8,[25]]],[[8,[26,25]]]],[[[8,[25]]],[[8,[26,25]]]],[[],26],[[],26],[[],26],[[],26],[[],26],[[],26]],"p":[[15,"str"],[3,"String"],[6,"Result"],[15,"bool"],[4,"RuntimeModule"],[3,"GasCosts"],[8,"Interface"],[3,"Box"],[15,"usize"],[3,"With"],[4,"Result"],[3,"Response"],[3,"Formatter"],[6,"Result"],[15,"u64"],[3,"Vec"],[15,"u8"],[8,"DeserializeOwned"],[3,"BTreeSet"],[3,"PathBuf"],[4,"Option"],[3,"TypeId"],[15,"i64"],[15,"f64"],[3,"Global"],[8,"Any"],[8,"InterfaceClone"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
