var searchIndex = JSON.parse('{\
"massa_sc_runtime":{"doc":"","t":"NNENDNIIDENENLLLLLLLLLLLLLLLLLLLLLKLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLMMFFLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMM","n":["ASModule","CL","Compiler","ExecutionError","GasCosts","InstanceError","Interface","InterfaceClone","Response","RuntimeModule","SP","VMError","WasmV1Module","address_from_public_key","address_from_public_key","append_ds_value","append_ds_value","as_error","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","caller_has_write_access","caller_has_write_access","clone","clone","clone","clone","clone_box","clone_into","clone_into","clone_into","clone_into","compiler","create_module","create_module","delete_ds_entry","delete_ds_entry","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","drop","drop","drop","drop","drop","ds_entry_exists","ds_entry_exists","finish_call","finish_call","fmt","fmt","fmt","fmt","from","from","from","from","from","from","generate_event","generate_event","get_balance","get_balance","get_bytecode","get_bytecode","get_call_coins","get_call_coins","get_call_stack","get_call_stack","get_current_period","get_current_period","get_current_thread","get_current_thread","get_ds_keys","get_ds_keys","get_ds_value","get_ds_value","get_module","get_module","get_op_keys","get_op_keys","get_op_value","get_op_value","get_owned_addresses","get_owned_addresses","get_time","get_time","hash","hash","hash_sha256","hash_sha256","init","init","init","init","init","init_call","init_call","init_gas_cost","into","into","into","into","into","layout_raw","layout_raw","layout_raw","layout_raw","layout_raw","new","new","op_entry_exists","op_entry_exists","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","provide","remaining_gas","ret","run_function","run_main","send_message","send_message","serialize","set_bytecode","set_bytecode","set_ds_value","set_ds_value","signature_verify","signature_verify","sp_compilation_cost","to_owned","to_owned","to_owned","to_owned","to_string","transfer_coins","transfer_coins","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","unsafe_random","unsafe_random","unsafe_random_f64","unsafe_random_f64","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_ref","upcast_any_ref","upcast_any_ref","upcast_any_ref","upcast_any_ref","validate_address","validate_address","error","init_gas_cost"],"q":[[0,"massa_sc_runtime"],[196,"massa_sc_runtime::VMError"]],"d":["","","Enum listing the available compilers","VM execution error: {error} | Init cost is {init_gas_cost}","","VM instance error: {0}","","","That’s what is returned when a module is executed …","","","","","","","Append a value to the current datastore value for the …","Append a value to the current datastore value for the …","","","","","","","","","","","","Check whether or not the caller has write access in the …","Check whether or not the caller has write access in the …","","","","","","","","","","Used compiler for the current module","Requires a new address that contains the sent &amp;u8","Requires a new address that contains the sent &amp;u8","Delete a datastore entry at of the given address","Delete a datastore entry at of the given address","","","","","","","","","","","","","Deserialize a RuntimeModule","","","","","","","","","Check if a datastore entry exists","Check if a datastore entry exists","Finish a call","Finish a call","","","","","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Generate a smart contract event","Generate a smart contract event","Get the SCE ledger balance for an address. Defaults to …","Get the SCE ledger balance for an address. Defaults to …","Returns bytecode of the target address","Returns bytecode of the target address","Get the amount of coins that have been made available for …","Get the amount of coins that have been made available for …","Expect to return a list of addresses in the call stack","Expect to return a list of addresses in the call stack","Returns the period of the current execution slot","Returns the period of the current execution slot","Returns the thread of the current execution slot","Returns the thread of the current execution slot","Return datastore keys Will only return keys with a given …","Return datastore keys Will only return keys with a given …","Requires the data at the address","Requires the data at the address","For the given bytecode:","For the given bytecode:","Return operation datastore keys","Return operation datastore keys","Return operation datastore data for a given key","Return operation datastore data for a given key","Expect to return a list of owned addresses","Expect to return a list of owned addresses","Returns the current time (millisecond unix timestamp)","Returns the current time (millisecond unix timestamp)","","","","","","","","","","Prepare the execution of a module at the given address and …","Prepare the execution of a module at the given address and …","number of gas required for the instance creation","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","Dispatch module creation corresponding to the first …","","Check if key is in operation datastore","Check if key is in operation datastore","","","","","","","number of gas that remain after the execution (metering)","returned value from the module call","Library Input, take a <code>module</code> wasm built with the massa …","Library Input, take a <code>module</code> wasm built with the massa …","Sends an async message","Sends an async message","Serialize a RuntimeModule, prepending its byte id","Sets the executable bytecode at a target address. The …","Sets the executable bytecode at a target address. The …","Set the datastore value for the corresponding key of the …","Set the datastore value for the corresponding key of the …","","","","","","","","","Transfer an amount from the specified address to a target …","Transfer an amount from the specified address to a target …","","","","","","","","","","","","","","","","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","","","","","","","","","","","","","","","","","","",""],"i":[10,9,0,8,0,8,0,0,0,0,9,0,10,12,12,12,12,8,8,9,10,20,11,8,9,10,20,11,12,12,8,9,10,11,35,8,9,10,11,10,12,12,12,12,8,9,10,20,11,8,9,10,20,11,8,9,10,10,20,11,8,9,10,20,11,12,12,12,12,8,8,20,11,8,8,9,10,20,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,8,9,10,20,11,12,12,20,8,9,10,20,11,8,9,10,20,11,10,11,12,12,8,9,10,20,11,8,20,20,0,0,12,12,10,12,12,12,12,12,12,11,8,9,10,11,8,12,12,8,9,10,20,11,8,9,10,20,11,8,9,10,20,11,12,12,12,12,8,9,10,20,11,8,9,10,20,11,8,9,10,20,11,12,12,36,36],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[1,[[3,[2]]]],[1,[[3,[2]]]],[[1,[5,[4]],[5,[4]]],3],[[1,[5,[4]],[5,[4]]],3],[[],6],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[[3,[7]]]],[[],[[3,[7]]]],[8,8],[9,9],[10,10],[11,11],[[],[[13,[12]]]],[[]],[[]],[[]],[[]],[10,9],[[[5,[4]]],[[3,[2]]]],[[[5,[4]]],[[3,[2]]]],[[1,[5,[4]]],3],[[1,[5,[4]]],3],[14],[14],[14],[14],[14],[14],[14],[14],[14],[14],[[],[[16,[15]]]],[[],[[16,[15]]]],[[[5,[4]],17,11],[[3,[10]]]],[[],[[16,[15]]]],[[],[[16,[15]]]],[[],[[16,[15]]]],[14],[14],[14],[14],[14],[[1,[5,[4]]],[[3,[7]]]],[[1,[5,[4]]],[[3,[7]]]],[[],3],[[],3],[[8,18],19],[[8,18],19],[[20,18],19],[[11,18],19],[[]],[21,8],[[]],[[]],[[]],[[]],[2,3],[2,3],[1,[[3,[17]]]],[1,[[3,[17]]]],[1,[[3,[[22,[4]]]]]],[1,[[3,[[22,[4]]]]]],[[],[[3,[17]]]],[[],[[3,[17]]]],[[],[[3,[[22,[2]]]]]],[[],[[3,[[22,[2]]]]]],[[],[[3,[17]]]],[[],[[3,[17]]]],[[],[[3,[4]]]],[[],[[3,[4]]]],[[1,[5,[4]]],[[3,[[23,[[22,[4]]]]]]]],[[1,[5,[4]]],[[3,[[23,[[22,[4]]]]]]]],[[1,[5,[4]]],[[3,[[22,[4]]]]]],[[1,[5,[4]]],[[3,[[22,[4]]]]]],[[[5,[4]],17],[[3,[10]]]],[[[5,[4]],17],[[3,[10]]]],[[[5,[4]]],[[3,[[23,[[22,[4]]]]]]]],[[[5,[4]]],[[3,[[23,[[22,[4]]]]]]]],[[[5,[4]]],[[3,[[22,[4]]]]]],[[[5,[4]]],[[3,[[22,[4]]]]]],[[],[[3,[[22,[2]]]]]],[[],[[3,[[22,[2]]]]]],[[],[[3,[17]]]],[[],[[3,[17]]]],[[[5,[4]]],[[3,[[24,[4]]]]]],[[[5,[4]]],[[3,[[24,[4]]]]]],[[[5,[4]]],[[3,[[24,[4]]]]]],[[[5,[4]]],[[3,[[24,[4]]]]]],[[],14],[[],14],[[],14],[[],14],[[],14],[[1,17],[[3,[[22,[4]]]]]],[[1,17],[[3,[[22,[4]]]]]],0,[[]],[[]],[[]],[[]],[[]],[[],[[16,[25,26]]]],[[],[[16,[25,26]]]],[[],[[16,[25,26]]]],[[],[[16,[25,26]]]],[[],[[16,[25,26]]]],[[[5,[4]],17,11,9],[[3,[10]]]],[[27,27],[[3,[11]]]],[[[5,[4]]],[[3,[7]]]],[[[5,[4]]],[[3,[7]]]],[[]],[[]],[[]],[[]],[[]],[28],0,0,[[12,10,1,[5,[4]],17,11],[[16,[20,8]]]],[[12,10,17,11],[[16,[20,8]]]],[[1,1,17,17,17,[5,[4]],29],3],[[1,1,17,17,17,[5,[4]],29],3],[10,[[3,[[22,[4]]]]]],[[1,[5,[4]]],3],[[1,[5,[4]]],3],[[1,[5,[4]],[5,[4]]],3],[[1,[5,[4]],[5,[4]]],3],[[[5,[4]],1,1],[[3,[7]]]],[[[5,[4]],1,1],[[3,[7]]]],0,[[]],[[]],[[]],[[]],[[],2],[[1,1,17],3],[[1,1,17],3],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],30],[[],30],[[],30],[[],30],[[],30],[[],[[3,[31]]]],[[],[[3,[31]]]],[[],[[3,[32]]]],[[],[[3,[32]]]],[[[13,[33]]],[[13,[34,33]]]],[[[13,[33]]],[[13,[34,33]]]],[[[13,[33]]],[[13,[34,33]]]],[[[13,[33]]],[[13,[34,33]]]],[[[13,[33]]],[[13,[34,33]]]],[[],34],[[],34],[[],34],[[],34],[[],34],[[],34],[[],34],[[],34],[[],34],[[],34],[1,[[3,[7]]]],[1,[[3,[7]]]],0,0],"c":[],"p":[[15,"str"],[3,"String"],[6,"Result"],[15,"u8"],[15,"slice"],[8,"Error"],[15,"bool"],[4,"VMError"],[4,"Compiler"],[4,"RuntimeModule"],[3,"GasCosts"],[8,"Interface"],[3,"Box"],[15,"usize"],[3,"With"],[4,"Result"],[15,"u64"],[3,"Formatter"],[6,"Result"],[3,"Response"],[3,"Error"],[3,"Vec"],[3,"BTreeSet"],[15,"array"],[3,"Layout"],[3,"LayoutError"],[3,"PathBuf"],[3,"Demand"],[4,"Option"],[3,"TypeId"],[15,"i64"],[15,"f64"],[3,"Global"],[8,"Any"],[8,"InterfaceClone"],[13,"ExecutionError"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
