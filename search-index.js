var searchIndex = JSON.parse('{\
"massa_sc_runtime":{"doc":"","t":"NNENDNIIDENENKKKLKLLLLLLLLLLKKKKKKKKKKKKMLLLLKLLLLKKKKLKKKKKKLLLLLLLLLLLLLLLLKLLLLLKKKKKLLLLLLLLLLKKKKKKKKLLKKKKKLLKKKKKKKKKKKKKKKKKKKKLLLLLKKMLLLLLKLLLLLMKKLLKLLLLLKKKKKKKKKKKKKMMFFKKKKLKLLKKMKLLLLLKKKLLLLLLLLLLLLLLLKKKLLLLLLLLLLLLLLLKMM","n":["ASModule","CL","Compiler","ExecutionError","GasCosts","InstanceError","Interface","InterfaceClone","Response","RuntimeModule","SP","VMError","WasmV1Module","add_native_amount_wasmv1","address_from_public_key","append_ds_value_wasmv1","as_error","base58_check_to_bytes_wasmv1","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","bytes_to_base58_check_wasmv1","caller_has_write_access","chain_id","check_address_wasmv1","check_native_amount_wasmv1","check_pubkey_wasmv1","check_signature_wasmv1","checked_add_native_time_wasmv1","checked_div_native_time_wasmv1","checked_mul_native_time_wasmv1","checked_scalar_div_native_time_wasmv1","checked_sub_native_time_wasmv1","cl_compilation_cost","clone","clone","clone","clone","clone_box","clone_into","clone_into","clone_into","clone_into","compare_address_wasmv1","compare_native_amount_wasmv1","compare_native_time_wasmv1","compare_pub_key_wasmv1","compiler","create_module","deferred_call_cancel","deferred_call_exists","deferred_call_quote","deferred_call_register","delete_ds_entry_wasmv1","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","div_rem_native_amount_wasmv1","drop","drop","drop","drop","drop","ds_entry_exists_wasmv1","evm_get_address_from_pubkey","evm_get_pubkey_from_signature","evm_signature_verify","finish_call","fmt","fmt","fmt","fmt","from","from","from","from","from","from","generate_event","generate_event_wasmv1","get_address_category_wasmv1","get_address_version_wasmv1","get_balance","get_balance_for","get_balance_wasmv1","get_bytecode_wasmv1","get_call_coins","get_call_coins","get_call_coins_wasmv1","get_call_stack","get_current_period","get_current_slot","get_current_thread","get_data","get_data_for","get_ds_keys_wasmv1","get_ds_value_wasmv1","get_keys","get_keys_for","get_module","get_op_data","get_op_keys","get_op_keys_wasmv1","get_origin_operation_id","get_owned_addresses","get_pubkey_version_wasmv1","get_signature_version_wasmv1","get_time","get_tmp_module","has_data","has_data_for","hash","hash_blake3","hash_keccak256","hash_sha256","init","init","init","init","init","init_call","init_call_wasmv1","init_gas_cost","into","into","into","into","into","is_address_eoa","layout_raw","layout_raw","layout_raw","layout_raw","layout_raw","max_instance_cost","native_amount_from_str_wasmv1","native_amount_to_string_wasmv1","new","new","op_entry_exists","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","print","raw_append_data","raw_append_data_for","raw_delete_data","raw_delete_data_for","raw_get_bytecode","raw_get_bytecode_for","raw_get_data","raw_get_data_for","raw_set_bytecode","raw_set_bytecode_for","raw_set_data","raw_set_data_for","remaining_gas","ret","run_function","run_main","save_gas_remaining_before_subexecution","scalar_div_rem_native_amount_wasmv1","scalar_mul_native_amount_wasmv1","send_message","serialize","set_bytecode_wasmv1","set_data","set_data_for","set_ds_value_wasmv1","signature_verify","sp_compilation_cost","sub_native_amount_wasmv1","to_owned","to_owned","to_owned","to_owned","to_string","transfer_coins","transfer_coins_for","transfer_coins_wasmv1","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","unsafe_random","unsafe_random_f64","unsafe_random_wasmv1","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_box","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_mut","upcast_any_ref","upcast_any_ref","upcast_any_ref","upcast_any_ref","upcast_any_ref","validate_address","error","init_gas_cost"],"q":[[0,"massa_sc_runtime"],[236,"massa_sc_runtime::VMError"],[238,"massa_proto_rs::massa::model::v1"],[239,"anyhow"],[240,"alloc::string"],[241,"core::option"],[242,"core::error"],[243,"alloc::vec"],[244,"massa_proto_rs::massa::model::v1"],[245,"massa_proto_rs::massa::model::v1"],[246,"core::result"],[247,"core::fmt"],[248,"core::fmt"],[249,"alloc::collections::btree::set"],[250,"core::alloc::layout"],[251,"core::alloc::layout"],[252,"serde::ser"],[253,"core::any"],[254,"alloc::alloc"],[255,"core::any"]],"d":["","","Enum listing the available compilers","VM execution error: {error}","","VM instance error: {0}","","","That’s what is returned when a module is executed …","","","","","","Convert a public key to an address","","","","","","","","","","","","","","","Check whether or not the caller has write access in the …","","","","","","","","","","","","","","","","","","","","","","","","","Used compiler for the current module","Requires a new address that contains the sent &amp;u8","","","","","","","","","","","","","","","","","","","Deserialize a RuntimeModule","","","","","","","","","","Get address from public key (EVM)","Get public key from signature (EVM)","Verify signature (EVM)","Finish a call","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Generate a smart contract event","Generate a smart contract event","","","Get the SCE ledger balance for the current address. …","Get the SCE ledger balance for an address. Defaults to …","","","Get the amount of coins that have been made available for …","Get the amount of coins that have been made available for …","Get the native amount of coins that have been made …","Expect to return a list of addresses in the call stack","Returns the period of the current execution slot","Returns the current execution slot","Returns the thread of the current execution slot","","","","","Return datastore keys Will only return keys with a given …","Return datastore keys Will only return keys with a given …","For the given bytecode:","Return operation datastore data for a given key","Return operation datastore keys","","","Expect to return a list of owned addresses","","","Returns the current time (millisecond unix timestamp)","Compile a temportary module from the given bytecode","Requires to replace the data in the current address","Check if a datastore entry exists","Hash data","Returns the blake3 hash of the given bytes","","","","","","","","Prepare the execution of a module at the given address and …","Prepare the execution of a module at the given address and …","number of gas required for the instance creation","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Return true if the address is a User address, false if it …","","","","","","","","","Dispatch module creation corresponding to the first …","","Check if operation in datastore exists","","","","","","Print function for examples","Append a value to the current datastore value for the …","Append a value to the current datastore value for the …","Delete a datastore entry","Delete a datastore entry at of the given address","Returns bytecode of the current address","Returns bytecode of the target address","Return the datastore value of the corresponding key","Requires the data at the address","Sets the executable bytecode at a current address.","Sets the executable bytecode at a target address. The …","Set the datastore value for the corresponding key","Set the datastore value for the corresponding key of the …","number of gas that remain after the execution (metering)","returned value from the module call","Library Input, take a <code>module</code> wasm built with the massa …","Library Input, take a <code>module</code> wasm built with the massa …","","","","Sends an async message","Serialize a RuntimeModule, prepending its byte id","","","","","Verify signature","","","","","","","","Transfer an amount from the address on the current call …","Transfer an amount from the specified address to a target …","","","","","","","","","","","","","","","","","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","Returns a random number (unsafe: can be predicted and …","","","","","","","","","","","","","","","","Validate an address","",""],"i":[16,15,0,14,0,14,0,0,0,0,15,0,16,18,18,18,14,18,14,15,16,26,17,14,15,16,26,17,18,18,18,18,18,18,18,18,18,18,18,18,17,14,15,16,17,42,14,15,16,17,18,18,18,18,16,18,18,18,18,18,18,14,15,16,26,17,14,15,16,26,17,14,15,16,16,26,17,18,14,15,16,26,17,18,18,18,18,18,14,14,26,17,14,14,15,16,26,17,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,14,15,16,26,17,18,18,26,14,15,16,26,17,18,14,15,16,26,17,17,18,18,16,17,18,14,15,16,26,17,18,18,18,18,18,18,18,18,18,18,18,18,18,26,26,0,0,18,18,18,18,16,18,18,18,18,18,17,18,14,15,16,17,14,18,18,18,14,15,16,26,17,14,15,16,26,17,14,15,16,26,17,18,18,18,14,15,16,26,17,14,15,16,26,17,14,15,16,26,17,18,43,43],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[[-1,1,1],[[2,[1]]],[]],[[-1,3],[[2,[4]]],[]],[[-1,[6,[5]],[6,[5]],[7,[4]]],[[2,[8]]],[]],[-1,9,[]],[[-1,3],[[2,[[10,[5]]]]],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[[-1,[6,[5]]],4,[]],[-1,[[2,[11]]],[]],[-1,[[2,[12]]],[]],[[-1,3],[[2,[11]]],[]],[[-1,1],[[2,[11]]],[]],[[-1,3],[[2,[11]]],[]],[[-1,3],[[2,[11]]],[]],[[-1,13,13],[[2,[13]]],[]],[[-1,13,13],[[2,[[8,[12,13]]]]],[]],[[-1,13,12],[[2,[13]]],[]],[[-1,13,12],[[2,[[8,[13,13]]]]],[]],[[-1,13,13],[[2,[13]]],[]],0,[14,14],[15,15],[16,16],[17,17],[-1,[[19,[18]]],[]],[[-1,-2],8,[],[]],[[-1,-2],8,[],[]],[[-1,-2],8,[],[]],[[-1,-2],8,[],[]],[[-1,3,3],[[2,[20]]],[]],[[-1,1,1],[[2,[20]]],[]],[[-1,13,13],[[2,[20]]],[]],[[-1,3,3],[[2,[20]]],[]],[16,15],[[-1,[6,[5]]],[[2,[4]]],[]],[[-1,3],[[2,[11]]],[]],[[-1,3],[[2,[11]]],[]],[[-1,[8,[12,5]],12],[[2,[[8,[11,12]]]]],[]],[[-1,3,3,[8,[12,5]],12,[6,[5]],12],[[2,[4]]],[]],[[-1,[6,[5]],[7,[4]]],[[2,[8]]],[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[21,-1,[]],[[-1,-2],[[23,[[22,[-3,-4]]]]],[],[],[],[]],[[-1,-2],[[23,[[22,[-3,-4]]]]],[],[],[],[]],[[-1,-2],[[23,[[22,[-3,-4]]]]],[],[],[],[]],[[[6,[5]],12,17],[[2,[16]]]],[[-1,-2],[[23,[[22,[-3,-4]]]]],[],[],[],[]],[[-1,-2],[[23,[[22,[-3,-4]]]]],[],[],[],[]],[[-1,1,1],[[2,[[8,[12,1]]]]],[]],[21,8],[21,8],[21,8],[21,8],[21,8],[[-1,[6,[5]],[7,[4]]],[[2,[11]]],[]],[[-1,[6,[5]]],[[2,[[10,[5]]]]],[]],[[-1,[6,[5]],[6,[5]]],[[2,[[10,[5]]]]],[]],[[-1,[6,[5]],[6,[5]],[6,[5]]],[[2,[11]]],[]],[-1,[[2,[8]]],[]],[[14,24],25],[[14,24],25],[[26,24],25],[[17,24],25],[27,14],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[[-1,4],[[2,[8]]],[]],[[-1,[10,[5]]],[[2,[8]]],[]],[[-1,3],[[2,[28]]],[]],[[-1,3],[[2,[12]]],[]],[-1,[[2,[12]]],[]],[[-1,3],[[2,[12]]],[]],[[-1,[7,[4]]],[[2,[1]]],[]],[[-1,[7,[4]]],[[2,[[10,[5]]]]],[]],[-1,[[2,[12]]],[]],[-1,[[2,[12]]],[]],[-1,[[2,[1]]],[]],[-1,[[2,[[10,[4]]]]],[]],[-1,[[2,[12]]],[]],[-1,[[2,[29]]],[]],[-1,[[2,[5]]],[]],[[18,[6,[5]]],[[2,[-1]]],30],[[18,3,[6,[5]]],[[2,[-1]]],30],[[-1,[6,[5]],[7,[4]]],[[2,[[31,[[10,[5]]]]]]],[]],[[-1,[6,[5]],[7,[4]]],[[2,[[10,[5]]]]],[]],[[-1,[7,[[6,[5]]]]],[[2,[[31,[[10,[5]]]]]]],[]],[[-1,3,[7,[[6,[5]]]]],[[2,[[31,[[10,[5]]]]]]],[]],[[-1,[6,[5]],12],[[2,[16]]],[]],[[-1,[6,[5]]],[[2,[[10,[5]]]]],[]],[[-1,[7,[[6,[5]]]]],[[2,[[10,[[10,[5]]]]]]],[]],[[-1,[6,[5]]],[[2,[[10,[[10,[5]]]]]]],[]],[-1,[[2,[[7,[4]]]]],[]],[-1,[[2,[[10,[4]]]]],[]],[[-1,3],[[2,[12]]],[]],[[-1,3],[[2,[12]]],[]],[-1,[[2,[12]]],[]],[[-1,[6,[5]],12],[[2,[16]]],[]],[[-1,[6,[5]]],[[2,[11]]],[]],[[-1,3,[6,[5]]],[[2,[11]]],[]],[[-1,[6,[5]]],[[2,[[32,[5]]]]],[]],[[-1,[6,[5]]],[[2,[[32,[5]]]]],[]],[[-1,[6,[5]]],[[2,[[32,[5]]]]],[]],[[-1,[6,[5]]],[[2,[[32,[5]]]]],[]],[[],21],[[],21],[[],21],[[],21],[[],21],[[-1,3,12],[[2,[[10,[5]]]]],[]],[[-1,3,1],[[2,[[10,[5]]]]],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[[-1,3],[[2,[11]]],[]],[[],[[23,[33,34]]]],[[],[[23,[33,34]]]],[[],[[23,[33,34]]]],[[],[[23,[33,34]]]],[[],[[23,[33,34]]]],0,[[-1,3],[[2,[1]]],[]],[[-1,1],[[2,[4]]],[]],[[[6,[5]],17,15],[[2,[16]]]],[[35,35],[[2,[17]]]],[[-1,[6,[5]]],[[2,[11]]],[]],[[]],[[]],[[]],[[]],[[]],[[-1,3],[[2,[8]]],[]],[[-1,[6,[5]],[6,[5]]],[[2,[8]]],[]],[[-1,3,[6,[5]],[6,[5]]],[[2,[8]]],[]],[[-1,[6,[5]]],[[2,[8]]],[]],[[-1,3,[6,[5]]],[[2,[8]]],[]],[-1,[[2,[[10,[5]]]]],[]],[[-1,3],[[2,[[10,[5]]]]],[]],[[-1,[6,[5]]],[[2,[[10,[5]]]]],[]],[[-1,3,[6,[5]]],[[2,[[10,[5]]]]],[]],[[-1,[6,[5]]],[[2,[8]]],[]],[[-1,3,[6,[5]]],[[2,[8]]],[]],[[-1,[6,[5]],[6,[5]]],[[2,[8]]],[]],[[-1,3,[6,[5]],[6,[5]]],[[2,[8]]],[]],0,0,[[18,16,3,[6,[5]],12,17],[[23,[26,14]]]],[[18,16,12,17],[[23,[26,14]]]],[[-1,12],8,[]],[[-1,1,12],[[2,[[8,[1,1]]]]],[]],[[-1,1,12],[[2,[1]]],[]],[[-1,3,3,[8,[12,5]],[8,[12,5]],12,12,12,[6,[5]],[7,[[8,[3,[7,[[6,[5]]]]]]]]],[[2,[8]]],[]],[16,[[2,[[10,[5]]]]]],[[-1,[6,[5]],[7,[4]]],[[2,[8]]],[]],[[18,[6,[5]],-1],[[2,[8]]],36],[[18,3,[6,[5]],-1],[[2,[8]]],36],[[-1,[6,[5]],[6,[5]],[7,[4]]],[[2,[8]]],[]],[[-1,[6,[5]],3,3],[[2,[11]]],[]],0,[[-1,1,1],[[2,[1]]],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,4,[]],[[-1,3,12],[[2,[8]]],[]],[[-1,3,3,12],[[2,[8]]],[]],[[-1,4,1,[7,[4]]],[[2,[8]]],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,[[23,[-2]]],[],[]],[-1,37,[]],[-1,37,[]],[-1,37,[]],[-1,37,[]],[-1,37,[]],[-1,[[2,[38]]],[]],[-1,[[2,[39]]],[]],[[-1,12],[[2,[[10,[5]]]]],[]],[[[19,[-1,40]]],[[19,[41,40]]],[]],[[[19,[-1,40]]],[[19,[41,40]]],[]],[[[19,[-1,40]]],[[19,[41,40]]],[]],[[[19,[-1,40]]],[[19,[41,40]]],[]],[[[19,[-1,40]]],[[19,[41,40]]],[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[-1,41,[]],[[-1,3],[[2,[11]]],[]],0,0],"c":[],"p":[[3,"NativeAmount",238],[6,"Result",239],[15,"str"],[3,"String",240],[15,"u8"],[15,"slice"],[4,"Option",241],[15,"tuple"],[8,"Error",242],[3,"Vec",243],[15,"bool"],[15,"u64"],[3,"NativeTime",238],[4,"VMError",0],[4,"Compiler",0],[4,"RuntimeModule",0],[3,"GasCosts",0],[8,"Interface",0],[3,"Box",244],[4,"ComparisonResult",238],[15,"usize"],[3,"With",245],[4,"Result",246],[3,"Formatter",247],[6,"Result",247],[3,"Response",0],[3,"Error",239],[4,"AddressCategory",238],[3,"Slot",238],[8,"DeserializeOwned",248],[3,"BTreeSet",249],[15,"array"],[3,"Layout",250],[3,"LayoutError",250],[3,"PathBuf",251],[8,"Serialize",252],[3,"TypeId",253],[15,"i64"],[15,"f64"],[3,"Global",254],[8,"Any",253],[8,"InterfaceClone",0],[13,"ExecutionError",236]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
