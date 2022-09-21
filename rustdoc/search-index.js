var searchIndex = JSON.parse('{\
"obddimal":{"doc":"","t":[0,0,0,14,0,3,17,17,11,5,11,11,11,11,12,11,11,11,11,11,0,0,11,11,11,0,11,0,11,0,11,11,11,11,11,11,11,12,5,11,11,0,11,0,12,11,0,11,0,11,11,11,11,0,11,0,11,11,11,11,0,12,11,11,11,11,11,11,3,13,3,13,3,13,8,4,3,13,3,13,3,13,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,12,12,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,12,12,12,12,12,12,6,6,3,11,11,11,12,11,11,12,11,11,11,11,11,11,5,5,12,12,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,3,11,11,12,11,11,5,11,11,11,11,12,12,5,11,11,11,11,11,5,5,5,5,5,5],"n":["bdd_manager","bdd_node","dimacs","if_some","static_ordering","DDManager","ONE","ZERO","add_node","align_clauses","and","bootstrap","borrow","borrow_mut","c_table","clone","clone_into","collect_nodes","count_active","default","dvo","dvo_schedules","ensure_order","fmt","from","from_dimacs","from_instance","graphviz","graphviz","hash_select","into","is_sat","ite","ith_var","min_by_order","nith_var","node_get_or_create","nodes","normalize_ite_args","not","one","options","or","order","order","purge_retain","reduce","reduce","sat","sat_count","sat_count_rec","sift_all_vars","sift_single_var","swap","swap","test","to_owned","try_from","try_into","type_id","util","var2nodes","var_at_level","verify","vzip","xor","xor_prim","zero","AlwaysOnce","AlwaysOnce","AlwaysUntilConvergence","AlwaysUntilConvergence","AtThreshold","AtThreshold","DVOSchedule","DVOScheduleEnum","NoDVOSchedule","NoDVOSchedule","SiftingAtThreshold","SiftingAtThreshold","TimeSizeLimit","TimeSizeLimit","active_nodes_threshold","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","default","default","default","default","from","from","from","from","from","from","from","from","from","from","from","from","from","interval","into","into","into","into","into","into","into","last_dvo","limit","new","new","run_dvo","run_dvo","run_dvo","run_dvo","run_dvo","run_dvo","run_dvo","run_dvo","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","underlying","underlying_schedule","underlying_schedule","vzip","vzip","vzip","vzip","vzip","vzip","vzip","0","0","0","0","0","0","HashMap","HashSet","Options","borrow","borrow_mut","default","dvo","from","into","progressbars","try_from","try_into","type_id","vzip","with_dvo","with_progressbars","check_order","order_to_layernames","0","0","DDNode","NodeID","VarID","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","cmp","cmp","eq","eq","eq","fmt","fmt","fmt","from","from","from","hash","hash","hash","high","id","into","into","into","low","ne","ne","partial_cmp","partial_cmp","restrict","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","var","vzip","vzip","vzip","Instance","borrow","borrow_mut","clauses","clone","clone_into","file_readlines","fmt","from","into","new","no_clauses","no_variables","parse_dimacs","to_owned","try_from","try_into","type_id","vzip","calc_center_of_gravity","calc_span","force","keep","order_dist","rand"],"q":["obddimal","","","","","obddimal::bdd_manager","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","obddimal::bdd_manager::dvo_schedules","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","obddimal::bdd_manager::dvo_schedules::DVOScheduleEnum","","","","","","obddimal::bdd_manager::hash_select","","obddimal::bdd_manager::options","","","","","","","","","","","","","","obddimal::bdd_manager::order","","obddimal::bdd_node","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","obddimal::dimacs","","","","","","","","","","","","","","","","","","","obddimal::static_ordering","","","","",""],"d":["All BDD building and manipulation functionality","Module containing type definitions for the elements of the …","Parsing of DIMACS input file format","Shortcut for","Implementations of different static variable ordering …","Container combining the nodes list, unique tables, …","Terminal node “one”","Terminal node “zero”","Insert Node. ID is assigned for nonterminal nodes (var != …","Determine order in which clauses should be added to BDD","","Initialize the BDD with zero and one constant nodes","","","Computed Table: maps (f,g,h) to ite(f,g,h)","","","Collect all nodes that are part of the specified function","","","Implementation of dynamic variable ordering techniques","Strategies for when and how to run DVO during BDD …","Ensure order vec is valid up to specified variable","","Returns the argument unchanged.","BDD building from CNF","Builds a BDD from a CNF read from DIMACS.","Utilities for saving the current BDD as a graphviz file …","Generate graphviz for the provided function, not including …","Type aliases for HashMap and HashSet to be used throuout …","Calls <code>U::from(self)</code>.","","","","Find top variable: Highest in tree according to order","","Search for Node, create if it doesnt exist","Node List","Bring ITE calls of the form ite(f,f,h) = ite(f,1,h) = …","","","Options for BDD building","","Utility functions related to variable order","Variable ordering: order[v.0] is the depth of variable v …","","BDD reduction","Reduces the BDD. This changes Node IDs, the new Node ID of …","Satisfyability count, active nodes count","","","Perform sifting for every layer containing at least one …","Swap layer containing specified variable first to the …","Implementation of BDD layer swap","Swaps graph layers of variables a and b. Requires a to be …","Utilities for BDD testing","","","","","Utility functions/macros","Unique Table for each variable. Note that the order …","Find the variable at specified level","","","","Creates an XOR “ladder”","","Run one iteration of sifting for all variables, every time …","","Always perform sifting of all variables until the number of","","Call the underlying strategy if the node count exceeds the …","","Implements run_dvo()","This contains all available DVO implementations","Dummy DVO implementation that does nothing","","Performs sifting until the number of nodes does not change …","","Calls the underlying DVO mode if the specified duration …","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","This gets called after a CNF clause has been integrated. …","","","","","","","This gets called after a CNF clause has been integrated. …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Type alias for a hashmap using the <code>fx</code> hash algorithm.","Type alias for a hashmap using the <code>fx</code> hash algorithm.","","","","","DVO strategy: When and how to run DVO","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Display progress bars for BDD building and DVO progress","","","","","","","Checks if a specified variable ordering is valid for the …","Returns the variable order as list of VarID top to bottom","","","Element of a BDD. Note that the Node contains its own ID. …","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Node ID. Special values: 0 and 1 for terminal nodes","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","Returns the function resulting when setting the specified …","","","","","","","","","","","","","Variable number. Special variable 0 == terminal nodes","","","","Logic formula in conjunctive normal form, parsed from a …","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Number of clauses as specified in the DIMACS header line","Number of variables as specified in the DIMACS header line","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,1,0,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,0,1,0,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,0,1,1,0,1,0,1,1,1,1,0,1,0,1,1,1,1,0,1,1,1,1,1,1,1,0,2,0,2,0,2,0,0,0,2,0,2,0,2,3,3,4,5,6,7,8,2,3,4,5,6,7,8,2,6,7,8,2,3,4,5,6,7,8,2,2,2,2,2,2,2,5,3,4,5,6,7,8,2,5,5,4,5,9,3,4,5,6,7,8,2,3,4,5,6,7,8,2,3,4,5,6,7,8,2,2,2,2,2,2,2,3,4,5,6,7,8,2,4,3,5,3,4,5,6,7,8,2,10,11,12,13,14,15,0,0,0,16,16,16,16,16,16,16,16,16,16,16,16,16,0,0,17,18,0,0,0,17,18,19,17,18,19,17,18,19,17,18,19,17,18,17,18,19,17,18,19,17,18,19,17,18,19,19,19,17,18,19,19,17,18,17,18,19,17,18,19,17,18,19,17,18,19,17,18,19,19,17,18,19,0,20,20,20,20,20,0,20,20,20,20,20,20,0,20,20,20,20,20,0,0,0,0,0,0],"f":[null,null,null,null,null,null,null,null,[[["ddmanager",3],["ddnode",3]],["nodeid",3]],[[],["vec",3,[["usize",0]]]],[[["ddmanager",3],["nodeid",3],["nodeid",3]],["nodeid",3]],[[["ddmanager",3]]],[[["",0]],["",0]],[[["",0]],["",0]],null,[[["ddmanager",3]],["ddmanager",3]],[[["",0],["",0]]],[[["ddmanager",3],["nodeid",3]],["hashset",6,[["nodeid",3]]]],[[["ddmanager",3],["nodeid",3]],["u32",0]],[[],["ddmanager",3]],null,null,[[["ddmanager",3],["varid",3]]],[[["ddmanager",3],["formatter",3]],["result",6]],[[]],null,[[["instance",3],["option",4,[["vec",3,[["u32",0]]]]],["options",3]],["result",4,[["string",3]]]],null,[[["ddmanager",3],["nodeid",3]],["string",3]],null,[[]],[[["ddmanager",3],["u32",0]],["bool",0]],[[["ddmanager",3],["nodeid",3],["nodeid",3],["nodeid",3]],["nodeid",3]],[[["ddmanager",3],["varid",3]],["nodeid",3]],[[["ddmanager",3],["varid",3],["varid",3],["varid",3]],["varid",3]],[[["ddmanager",3],["varid",3]],["nodeid",3]],[[["ddmanager",3],["ddnode",3]],["nodeid",3]],null,[[["nodeid",3],["nodeid",3],["nodeid",3]]],[[["ddmanager",3],["nodeid",3]],["nodeid",3]],[[["ddmanager",3]],["nodeid",3]],null,[[["ddmanager",3],["nodeid",3],["nodeid",3]],["nodeid",3]],null,null,[[["ddmanager",3],["nodeid",3]]],null,[[["ddmanager",3],["nodeid",3]],["nodeid",3]],null,[[["ddmanager",3],["nodeid",3]],["biguint",3]],[[["ddmanager",3],["nodeid",3],["hashmap",6]],["biguint",3]],[[["ddmanager",3],["nodeid",3],["bool",0]],["nodeid",3]],[[["ddmanager",3],["varid",3],["option",4,[["u32",0]]],["nodeid",3]],["nodeid",3]],null,[[["ddmanager",3],["varid",3],["varid",3],["nodeid",3]],["nodeid",3]],null,[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,[[["ddmanager",3],["u32",0]],["option",4,[["varid",3]]]],[[["ddmanager",3],["nodeid",3]],["bool",0]],[[]],[[["ddmanager",3],["nodeid",3],["nodeid",3]],["nodeid",3]],[[["ddmanager",3],["vec",3,[["u32",0]]]],["u32",0]],[[["ddmanager",3]],["nodeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[],["nodvoschedule",3]],[[],["alwaysuntilconvergence",3]],[[],["alwaysonce",3]],[[],["dvoscheduleenum",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["atthreshold",3]],["dvoscheduleenum",4]],[[["alwaysuntilconvergence",3]],["dvoscheduleenum",4]],[[["siftingatthreshold",3]],["dvoscheduleenum",4]],[[["timesizelimit",3]],["dvoscheduleenum",4]],[[["alwaysonce",3]],["dvoscheduleenum",4]],[[["nodvoschedule",3]],["dvoscheduleenum",4]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[["u32",0]],["siftingatthreshold",3]],[[["duration",3],["usize",0],["box",3,[["dvoscheduleenum",4]]]],["timesizelimit",3]],[[["",0],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[["atthreshold",3],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[["siftingatthreshold",3],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[["timesizelimit",3],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[["nodvoschedule",3],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[["alwaysuntilconvergence",3],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[["alwaysonce",3],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[["dvoscheduleenum",4],["usize",0],["ddmanager",3],["nodeid",3],["option",4]],["nodeid",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["dvoscheduleenum",4]],["result",4,[["alwaysonce",3]]]],[[["dvoscheduleenum",4]],["result",4,[["atthreshold",3]]]],[[["dvoscheduleenum",4]],["result",4,[["nodvoschedule",3]]]],[[["dvoscheduleenum",4]],["result",4,[["timesizelimit",3]]]],[[["dvoscheduleenum",4]],["result",4,[["alwaysuntilconvergence",3]]]],[[],["result",4]],[[["dvoscheduleenum",4]],["result",4,[["siftingatthreshold",3]]]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[],["options",3]],null,[[]],[[]],null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]],[[["options",3],["dvoscheduleenum",4]],["options",3]],[[["options",3]],["options",3]],[[["instance",3]],["result",4,[["string",3]]]],[[],["vec",3,[["varid",3]]]],null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["nodeid",3]],["nodeid",3]],[[["varid",3]],["varid",3]],[[["ddnode",3]],["ddnode",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["nodeid",3],["nodeid",3]],["ordering",4]],[[["varid",3],["varid",3]],["ordering",4]],[[["nodeid",3],["nodeid",3]],["bool",0]],[[["varid",3],["varid",3]],["bool",0]],[[["ddnode",3],["ddnode",3]],["bool",0]],[[["nodeid",3],["formatter",3]],["result",6]],[[["varid",3],["formatter",3]],["result",6]],[[["ddnode",3],["formatter",3]],["result",6]],[[]],[[]],[[]],[[["nodeid",3],["",0]]],[[["varid",3],["",0]]],[[["ddnode",3],["",0]]],null,null,[[]],[[]],[[]],null,[[["nodeid",3],["nodeid",3]],["bool",0]],[[["varid",3],["varid",3]],["bool",0]],[[["nodeid",3],["nodeid",3]],["option",4,[["ordering",4]]]],[[["varid",3],["varid",3]],["option",4,[["ordering",4]]]],[[["ddnode",3],["varid",3],["bool",0]],["nodeid",3]],[[["",0]]],[[["",0]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,[[]],[[]],[[]],null,[[["",0]],["",0]],[[["",0]],["",0]],null,[[["instance",3]],["instance",3]],[[["",0],["",0]]],[[["asref",8,[["path",3]]]],["vec",3,[["string",3]]]],[[["instance",3],["formatter",3]],["result",6]],[[]],[[]],[[["u32",0],["u32",0],["vec",3,[["vec",3,[["i32",0]]]]]],["instance",3]],null,null,[[["asref",8,[["path",3]]]],["instance",3]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]],[[],["f64",0]],[[],["u32",0]],[[["instance",3]],["vec",3,[["u32",0]]]],[[["instance",3]],["vec",3,[["u32",0]]]],[[["instance",3]],["vec",3,[["u32",0]]]],[[["instance",3]],["vec",3,[["u32",0]]]]],"p":[[3,"DDManager"],[4,"DVOScheduleEnum"],[3,"AtThreshold"],[3,"SiftingAtThreshold"],[3,"TimeSizeLimit"],[3,"NoDVOSchedule"],[3,"AlwaysUntilConvergence"],[3,"AlwaysOnce"],[8,"DVOSchedule"],[13,"NoDVOSchedule"],[13,"AlwaysUntilConvergence"],[13,"AtThreshold"],[13,"SiftingAtThreshold"],[13,"TimeSizeLimit"],[13,"AlwaysOnce"],[3,"Options"],[3,"NodeID"],[3,"VarID"],[3,"DDNode"],[3,"Instance"]]},\
"testcase_generation":{"doc":"","t":[5],"n":["main"],"q":["testcase_generation"],"d":["This prints the one-columns of a random truth table with 8 …"],"i":[0],"f":[[[]]],"p":[]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
