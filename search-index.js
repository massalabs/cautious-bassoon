var searchIndex = JSON.parse('{\
"massa_sc_runtime":{"doc":"","t":"NNEDNDNIIDENENLLLLLKKKLKLLLLLLLLLLLLKKKKKKKKKKKKMLLLLLKLLLLLKKKKLKLKLLLLLLLLLLLLLLLLLLLKLLLLLLKKKKKLLLLLLLLLLLLKKKKKKKKLLKKKKKLLKKKKKKKKKKKKKKKKKKKKKLLLLLLKKMLLLLLLKLLLLLLMMMMMMMMMMMMMMMMKKLLKLLLLLLKKKKKKKKKKKKKMMFFKKKKLKLLKKMKLLLLLLKKKLLLLLLLLLLLLLLLLLLKKKLLLLLLLLLLLLLLLLLLKMM","n":["ASModule","CL","Compiler","CondomLimits","ExecutionError","GasCosts","InstanceError","Interface","InterfaceClone","Response","RuntimeModule","SP","VMError","WasmV1Module","__clone_box","__clone_box","__clone_box","__clone_box","__clone_box","add_native_amount_wasmv1","address_from_public_key","append_ds_value_wasmv1","as_error","base58_check_to_bytes_wasmv1","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","bytes_to_base58_check_wasmv1","caller_has_write_access","chain_id","check_address_wasmv1","check_native_amount_wasmv1","check_pubkey_wasmv1","check_signature_wasmv1","checked_add_native_time_wasmv1","checked_div_native_time_wasmv1","checked_mul_native_time_wasmv1","checked_scalar_div_native_time_wasmv1","checked_sub_native_time_wasmv1","cl_compilation_cost","clone","clone","clone","clone","clone","clone_box","clone_into","clone_into","clone_into","clone_into","clone_into","compare_address_wasmv1","compare_native_amount_wasmv1","compare_native_time_wasmv1","compare_pub_key_wasmv1","compiler","create_module","default","delete_ds_entry_wasmv1","deref","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","div_rem_native_amount_wasmv1","drop","drop","drop","drop","drop","drop","ds_entry_exists_wasmv1","evm_get_address_from_pubkey","evm_get_pubkey_from_signature","evm_signature_verify","finish_call","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","generate_event","generate_event_wasmv1","get_address_category_wasmv1","get_address_version_wasmv1","get_balance","get_balance_for","get_balance_wasmv1","get_bytecode_wasmv1","get_call_coins","get_call_coins","get_call_coins_wasmv1","get_call_stack","get_current_period","get_current_slot","get_current_thread","get_data","get_data_for","get_ds_keys_wasmv1","get_ds_value_wasmv1","get_interface_version","get_keys","get_keys_for","get_module","get_op_data","get_op_keys","get_op_keys_wasmv1","get_origin_operation_id","get_owned_addresses","get_pubkey_version_wasmv1","get_signature_version_wasmv1","get_time","get_tmp_module","has_data","has_data_for","hash","hash_blake3","hash_keccak256","hash_sha256","init","init","init","init","init","init","init_call","init_call_wasmv1","init_gas_cost","into","into","into","into","into","into","is_address_eoa","layout_raw","layout_raw","layout_raw","layout_raw","layout_raw","layout_raw","max_custom_sections_data_len","max_custom_sections_len","max_exports","max_function_names_len","max_functions","max_global_initializers_len","max_globals_len","max_imports_len","max_instance_cost","max_memories_len","max_name_len","max_passive_data_len","max_passive_elements_len","max_signature_len","max_table_initializers_len","max_tables_count","native_amount_from_str_wasmv1","native_amount_to_string_wasmv1","new","new","op_entry_exists","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","print","raw_append_data","raw_append_data_for","raw_delete_data","raw_delete_data_for","raw_get_bytecode","raw_get_bytecode_for","raw_get_data","raw_get_data_for","raw_set_bytecode","raw_set_bytecode_for","raw_set_data","raw_set_data_for","remaining_gas","ret","run_function","run_main","save_gas_remaining_before_subexecution","scalar_div_rem_native_amount_wasmv1","scalar_mul_native_amount_wasmv1","send_message","serialize","set_bytecode_wasmv1","set_data","set_data_for","set_ds_value_wasmv1","signature_verify","sp_compilation_cost","sub_native_amount_wasmv1","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","transfer_coins","transfer_coins_for","transfer_coins_wasmv1","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","unsafe_random","unsafe_random_f64","unsafe_random_wasmv1","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_ref","upcast_any_ref","upcast_any_ref","upcast_any_ref","upcast_any_ref","upcast_any_ref","validate_address","error","init_gas_cost"],"q":[[0,"massa_sc_runtime"],[276,"massa_sc_runtime::VMError"],[278,"dyn_clone::sealed"],[279,"massa_proto_rs::massa::model::v1"],[280,"anyhow"],[281,"alloc::string"],[282,"core::option"],[283,"core::error"],[284,"alloc::vec"],[285,"massa_proto_rs::massa::model::v1"],[286,"massa_proto_rs::massa::model::v1"],[287,"core::result"],[288,"core::fmt"],[289,"core::fmt"],[290,"alloc::collections::btree::set"],[291,"core::alloc::layout"],[292,"core::alloc::layout"],[293,"serde::ser"],[294,"core::any"],[295,"alloc::alloc"],[296,"core::any"]],"d":["","","Enum listing the available compilers","","VM execution error: {error}","","VM instance error: {0}","","","That’s what is returned when a module is executed …","","","","","","","","","","","Convert a public key to an address","","","","","","","","","","","","","","","","","Check whether or not the caller has write access in the …","","","","","","","","","","","","","","","","","","","","","","","","","","","Used compiler for the current module","Requires a new address that contains the sent &amp;u8","","","","","","","","","","","","","","","","","Deserialize a RuntimeModule","","","","","","","","","","","","","Get address from public key (EVM)","Get public key from signature (EVM)","Verify signature (EVM)","Finish a call","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Generate a smart contract event","Generate a smart contract event","","","Get the SCE ledger balance for the current address. …","Get the SCE ledger balance for an address. Defaults to …","","","Get the amount of coins that have been made available for …","Get the amount of coins that have been made available for …","Get the native amount of coins that have been made …","Expect to return a list of addresses in the call stack","Returns the period of the current execution slot","Returns the current execution slot","Returns the thread of the current execution slot","","","","","","Return datastore keys Will only return keys with a given …","Return datastore keys Will only return keys with a given …","For the given bytecode:","Return operation datastore data for a given key","Return operation datastore keys","","","Expect to return a list of owned addresses","","","Returns the current time (millisecond unix timestamp)","Compile a temportary module from the given bytecode","Requires to replace the data in the current address","Check if a datastore entry exists","Hash data","Returns the blake3 hash of the given bytes","","","","","","","","","Prepare the execution of a module at the given address and …","Prepare the execution of a module at the given address and …","number of gas required for the instance creation","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Return true if the address is a User address, false if it …","","","","","","","","","","","","","","","","","","","","","","","","","Dispatch module creation corresponding to the first …","","Check if operation in datastore exists","","","","","","","Print function for examples","Append a value to the current datastore value for the …","Append a value to the current datastore value for the …","Delete a datastore entry","Delete a datastore entry at of the given address","Returns bytecode of the current address","Returns bytecode of the target address","Return the datastore value of the corresponding key","Requires the data at the address","Sets the executable bytecode at a current address.","Sets the executable bytecode at a target address. The …","Set the datastore value for the corresponding key","Set the datastore value for the corresponding key of the …","number of gas that remain after the execution (metering)","returned value from the module call","Library Input, take a <code>module</code> wasm built with the massa …","Library Input, take a <code>module</code> wasm built with the massa …","","","","Sends an async message","Serialize a RuntimeModule, prepending its byte id","","","","","Verify signature","","","","","","","","","Transfer an amount from the address on the current call …","Transfer an amount from the specified address to a target …","","","","","","","","","","","","","","","","","","","","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","","","","","","","","","","","","","","","","","","","Validate an address","",""],"i":[17,16,0,0,15,0,15,0,0,0,0,16,0,17,15,16,17,18,19,20,20,20,15,20,15,16,17,28,18,19,15,16,17,28,18,19,20,20,20,20,20,20,20,20,20,20,20,20,19,15,16,17,18,19,45,15,16,17,18,19,20,20,20,20,17,20,18,20,15,16,17,28,18,19,15,16,17,28,18,19,15,16,17,17,28,18,19,20,15,16,17,28,18,19,20,20,20,20,20,15,15,28,18,19,15,15,16,17,28,18,19,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,15,16,17,28,18,19,20,20,28,15,16,17,28,18,19,20,15,16,17,28,18,19,18,18,18,18,18,18,18,18,19,18,18,18,18,18,18,18,20,20,17,19,20,15,16,17,28,18,19,20,20,20,20,20,20,20,20,20,20,20,20,20,28,28,0,0,20,20,20,20,17,20,20,20,20,20,19,20,15,16,17,18,19,15,20,20,20,15,16,17,28,18,19,15,16,17,28,18,19,15,16,17,28,18,19,20,20,20,15,16,17,28,18,19,15,16,17,28,18,19,15,16,17,28,18,19,20,46,46],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[-1,1],2,[]],[[-1,1],2,[]],[[-1,1],2,[]],[[-1,1],2,[]],[[-1,1],2,[]],[[-1,3,3],[[4,[3]]],[]],[[-1,5],[[4,[6]]],[]],[[-1,[8,[7]],[8,[7]],[9,[6]]],[[4,[2]]],[]],[-1,10,[]],[[-1,5],[[4,[[11,[7]]]]],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[[-1,[8,[7]]],6,[]],[-1,[[4,[12]]],[]],[-1,[[4,[13]]],[]],[[-1,5],[[4,[12]]],[]],[[-1,3],[[4,[12]]],[]],[[-1,5],[[4,[12]]],[]],[[-1,5],[[4,[12]]],[]],[[-1,14,14],[[4,[14]]],[]],[[-1,14,14],[[4,[[2,[13,14]]]]],[]],[[-1,14,13],[[4,[14]]],[]],[[-1,14,13],[[4,[[2,[14,14]]]]],[]],[[-1,14,14],[[4,[14]]],[]],0,[15,15],[16,16],[17,17],[18,18],[19,19],[-1,[[21,[20]]],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,5,5],[[4,[22]]],[]],[[-1,3,3],[[4,[22]]],[]],[[-1,14,14],[[4,[22]]],[]],[[-1,5,5],[[4,[22]]],[]],[17,16],[[-1,[8,[7]]],[[4,[6]]],[]],[[],18],[[-1,[8,[7]],[9,[6]]],[[4,[2]]],[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[23,-1,[]],[[-1,-2],[[25,[[24,[-3,-4]]]]],[],[],[],[]],[[-1,-2],[[25,[[24,[-3,-4]]]]],[],[],[],[]],[[[8,[7]],13,19,18],[[4,[17]]]],[[-1,-2],[[25,[[24,[-3,-4]]]]],[],[],[],[]],[[-1,-2],[[25,[[24,[-3,-4]]]]],[],[],[],[]],[[-1,-2],[[25,[[24,[-3,-4]]]]],[],[],[],[]],[[-1,-2],[[25,[[24,[-3,-4]]]]],[],[],[],[]],[[-1,3,3],[[4,[[2,[13,3]]]]],[]],[23,2],[23,2],[23,2],[23,2],[23,2],[23,2],[[-1,[8,[7]],[9,[6]]],[[4,[12]]],[]],[[-1,[8,[7]]],[[4,[[11,[7]]]]],[]],[[-1,[8,[7]],[8,[7]]],[[4,[[11,[7]]]]],[]],[[-1,[8,[7]],[8,[7]],[8,[7]]],[[4,[12]]],[]],[-1,[[4,[2]]],[]],[[15,26],27],[[15,26],27],[[28,26],27],[[18,26],27],[[19,26],27],[29,15],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[[-1,6],[[4,[2]]],[]],[[-1,[11,[7]]],[[4,[2]]],[]],[[-1,5],[[4,[30]]],[]],[[-1,5],[[4,[13]]],[]],[-1,[[4,[13]]],[]],[[-1,5],[[4,[13]]],[]],[[-1,[9,[6]]],[[4,[3]]],[]],[[-1,[9,[6]]],[[4,[[11,[7]]]]],[]],[-1,[[4,[13]]],[]],[-1,[[4,[13]]],[]],[-1,[[4,[3]]],[]],[-1,[[4,[[11,[6]]]]],[]],[-1,[[4,[13]]],[]],[-1,[[4,[31]]],[]],[-1,[[4,[7]]],[]],[[20,[8,[7]]],[[4,[-1]]],32],[[20,5,[8,[7]]],[[4,[-1]]],32],[[-1,[8,[7]],[9,[6]]],[[4,[[33,[[11,[7]]]]]]],[]],[[-1,[8,[7]],[9,[6]]],[[4,[[11,[7]]]]],[]],[-1,[[4,[34]]],[]],[[-1,[9,[[8,[7]]]]],[[4,[[33,[[11,[7]]]]]]],[]],[[-1,5,[9,[[8,[7]]]]],[[4,[[33,[[11,[7]]]]]]],[]],[[-1,[8,[7]],13],[[4,[17]]],[]],[[-1,[8,[7]]],[[4,[[11,[7]]]]],[]],[[-1,[9,[[8,[7]]]]],[[4,[[11,[[11,[7]]]]]]],[]],[[-1,[8,[7]]],[[4,[[11,[[11,[7]]]]]]],[]],[-1,[[4,[[9,[6]]]]],[]],[-1,[[4,[[11,[6]]]]],[]],[[-1,5],[[4,[13]]],[]],[[-1,5],[[4,[13]]],[]],[-1,[[4,[13]]],[]],[[-1,[8,[7]],13],[[4,[17]]],[]],[[-1,[8,[7]]],[[4,[12]]],[]],[[-1,5,[8,[7]]],[[4,[12]]],[]],[[-1,[8,[7]]],[[4,[[35,[7]]]]],[]],[[-1,[8,[7]]],[[4,[[35,[7]]]]],[]],[[-1,[8,[7]]],[[4,[[35,[7]]]]],[]],[[-1,[8,[7]]],[[4,[[35,[7]]]]],[]],[[],23],[[],23],[[],23],[[],23],[[],23],[[],23],[[-1,5,13],[[4,[[11,[7]]]]],[]],[[-1,5,3],[[4,[[11,[7]]]]],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[[-1,5],[[4,[12]]],[]],[[],[[25,[36,37]]]],[[],[[25,[36,37]]]],[[],[[25,[36,37]]]],[[],[[25,[36,37]]]],[[],[[25,[36,37]]]],[[],[[25,[36,37]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[-1,5],[[4,[3]]],[]],[[-1,3],[[4,[6]]],[]],[[[8,[7]],19,16,18],[[4,[17]]]],[[38,38],[[4,[19]]]],[[-1,[8,[7]]],[[4,[12]]],[]],[[]],[[]],[[]],[[]],[[]],[[]],[[-1,5],[[4,[2]]],[]],[[-1,[8,[7]],[8,[7]]],[[4,[2]]],[]],[[-1,5,[8,[7]],[8,[7]]],[[4,[2]]],[]],[[-1,[8,[7]]],[[4,[2]]],[]],[[-1,5,[8,[7]]],[[4,[2]]],[]],[-1,[[4,[[11,[7]]]]],[]],[[-1,5],[[4,[[11,[7]]]]],[]],[[-1,[8,[7]]],[[4,[[11,[7]]]]],[]],[[-1,5,[8,[7]]],[[4,[[11,[7]]]]],[]],[[-1,[8,[7]]],[[4,[2]]],[]],[[-1,5,[8,[7]]],[[4,[2]]],[]],[[-1,[8,[7]],[8,[7]]],[[4,[2]]],[]],[[-1,5,[8,[7]],[8,[7]]],[[4,[2]]],[]],0,0,[[20,17,5,[8,[7]],13,19,18],[[25,[28,15]]]],[[20,17,13,19,18],[[25,[28,15]]]],[[-1,13],2,[]],[[-1,3,13],[[4,[[2,[3,3]]]]],[]],[[-1,3,13],[[4,[3]]],[]],[[-1,5,5,[2,[13,7]],[2,[13,7]],13,13,13,[8,[7]],[9,[[2,[5,[9,[[8,[7]]]]]]]]],[[4,[2]]],[]],[17,[[4,[[11,[7]]]]]],[[-1,[8,[7]],[9,[6]]],[[4,[2]]],[]],[[20,[8,[7]],-1],[[4,[2]]],39],[[20,5,[8,[7]],-1],[[4,[2]]],39],[[-1,[8,[7]],[8,[7]],[9,[6]]],[[4,[2]]],[]],[[-1,[8,[7]],5,5],[[4,[12]]],[]],0,[[-1,3,3],[[4,[3]]],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,6,[]],[[-1,5,13],[[4,[2]]],[]],[[-1,5,5,13],[[4,[2]]],[]],[[-1,6,3,[9,[6]]],[[4,[2]]],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,[[25,[-2]]],[],[]],[-1,40,[]],[-1,40,[]],[-1,40,[]],[-1,40,[]],[-1,40,[]],[-1,40,[]],[-1,[[4,[41]]],[]],[-1,[[4,[42]]],[]],[[-1,13],[[4,[[11,[7]]]]],[]],[[[21,[-1,43]]],[[21,[44,43]]],[]],[[[21,[-1,43]]],[[21,[44,43]]],[]],[[[21,[-1,43]]],[[21,[44,43]]],[]],[[[21,[-1,43]]],[[21,[44,43]]],[]],[[[21,[-1,43]]],[[21,[44,43]]],[]],[[[21,[-1,43]]],[[21,[44,43]]],[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[-1,44,[]],[[-1,5],[[4,[12]]],[]],0,0],"c":[],"p":[[3,"Private",278],[15,"tuple"],[3,"NativeAmount",279],[6,"Result",280],[15,"str"],[3,"String",281],[15,"u8"],[15,"slice"],[4,"Option",282],[8,"Error",283],[3,"Vec",284],[15,"bool"],[15,"u64"],[3,"NativeTime",279],[4,"VMError",0],[4,"Compiler",0],[4,"RuntimeModule",0],[3,"CondomLimits",0],[3,"GasCosts",0],[8,"Interface",0],[3,"Box",285],[4,"ComparisonResult",279],[15,"usize"],[3,"With",286],[4,"Result",287],[3,"Formatter",288],[6,"Result",288],[3,"Response",0],[3,"Error",280],[4,"AddressCategory",279],[3,"Slot",279],[8,"DeserializeOwned",289],[3,"BTreeSet",290],[15,"u32"],[15,"array"],[3,"Layout",291],[3,"LayoutError",291],[3,"PathBuf",292],[8,"Serialize",293],[3,"TypeId",294],[15,"i64"],[15,"f64"],[3,"Global",295],[8,"Any",294],[8,"InterfaceClone",0],[13,"ExecutionError",276]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
