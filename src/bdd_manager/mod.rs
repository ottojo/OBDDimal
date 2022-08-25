use super::bdd_node::{DDNode, NodeID, VarID};
use super::dimacs::Instance;

use rand::Rng;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

mod graphviz;
mod reduce;
mod sat;

lazy_static! {
    pub static ref ZERO: DDNode = DDNode::new(NodeID(0), VarID(0), NodeID(0), NodeID(0));
    pub static ref ONE: DDNode = DDNode::new(NodeID(1), VarID(0), NodeID(1), NodeID(1),);
}

fn normalize_ite_args(mut f: NodeID, mut g: NodeID, mut h: NodeID) -> (NodeID, NodeID, NodeID) {
    if f == g {
        g = ONE.id();
    } else if f == h {
        h = ZERO.id()
    }

    fn order(a: NodeID, b: NodeID) -> (NodeID, NodeID) {
        // TODO: "Efficient implementation of a BDD package" orders by top variable first, is this relevant?
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    if g == ONE.id() {
        (f, h) = order(f, h);
    }
    if h == ZERO.id() {
        (f, g) = order(f, g);
    }

    (f, g, h)
}

pub struct DDManager {
    /// Node List
    pub nodes: HashMap<NodeID, DDNode>,

    /// Variable ordering: order[v.0] is the depth of variable v in the tree.
    /// Top level is 1.
    /// See [check_order] for requirements
    order: Vec<u32>,
    /// Unique Table for each variable. This works because the Hash impl for
    /// DDNode only hashes variable and high and low edge, not the ID.
    /// node.var(), node.low(), node.high()
    ///
    /// Index is VarID
    var2nodes: Vec<HashMap<(VarID, NodeID, NodeID), NodeID>>,
    /// Computed Table: ite(f,g,h) cache
    c_table: HashMap<(NodeID, NodeID, NodeID), NodeID>,
}

impl Default for DDManager {
    fn default() -> Self {
        let mut man = DDManager {
            nodes: HashMap::default(),
            order: Vec::new(),
            var2nodes: Vec::new(),
            c_table: HashMap::default(),
        };

        man.bootstrap();
        man
    }
}

/// Determine order in which clauses should be added to BDD
fn align_clauses(clauses: &[Vec<i32>]) -> Vec<usize> {
    let mut shuffle: Vec<(usize, f32)> = Vec::default();

    for (i, clause) in clauses.iter().enumerate() {
        let min = clause.iter().map(|x| x.abs()).min().unwrap();
        let max = clause.iter().map(|x| x.abs()).max().unwrap();

        shuffle.push((i, (clause.len() as f32 * (max - min) as f32)));
    }

    shuffle.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
    shuffle.iter().map(|(x, _)| *x).collect::<Vec<usize>>()
}

/// Checks if a specified variable ordering is valid for the CNF instance.
/// Returns `OK(())` or `Err("error message")`.
fn check_order(cnf: &Instance, order: &[u32]) -> Result<(), String> {
    if order.len() != cnf.no_variables as usize + 1 {
        return Err(format!(
            "Invalid size of ordering: Size was {}, expected {} (nr of variables + 1)",
            order.len(),
            cnf.no_variables + 1
        ));
    }

    if order[0] != cnf.no_variables + 1 {
        return Err(format!(
            "Depth of terminal nodes (index 0) is specified as {}, but should be {} (nr of variables + 1)",order[0], cnf.no_variables+1
        ));
    }

    let max_depth = *order.iter().max().unwrap();
    if order[0] != max_depth {
        return Err(format!(
            "A variable is specified to have depth of {} which is below depth \
            of terminal nodes ({}, index 0)",
            max_depth, order[0]
        ));
    }

    let mut var_map = vec![0; cnf.no_variables as usize + 1];
    for (var, depth) in order.iter().enumerate() {
        if *depth < 1 {
            return Err(format!(
                "Variable {} specified at depth {} which is < 1",
                var, depth
            ));
        }

        if *depth > cnf.no_variables && var != 0 {
            return Err(format!(
                "Variable {} specified at depth {} which is greater than the number of variables",
                var, depth
            ));
        }

        var_map[*depth as usize - 1] = var;
    }

    for (depth, var) in var_map.iter().enumerate() {
        if *var == 0 && depth != cnf.no_variables as usize {
            return Err(format!("No variable at depth {}", depth + 1));
        }
    }

    Ok(())
}

fn order_to_layernames(order: &[u32]) -> Vec<VarID> {
    let mut res = vec![VarID(0); *order.iter().max().unwrap() as usize];
    for (var_num, var_pos) in order.iter().enumerate() {
        res[*var_pos as usize - 1] = VarID(var_num as u32);
    }
    res
}

impl DDManager {
    pub fn from_instance(
        instance: &mut Instance,
        order: Option<Vec<u32>>,
    ) -> Result<(DDManager, NodeID), String> {
        let mut man = DDManager::default();
        let clause_order = align_clauses(&instance.clauses);
        if let Some(o) = order {
            check_order(instance, &o)?;
            man.order = o;
        }

        man.var2nodes
            .resize(instance.no_variables as usize + 1, HashMap::default());

        log::info!("Initial order: {:?}", man.order);

        let mut bdd = man.one();

        let mut n = 1;
        for i in clause_order.iter() {
            let clause = &instance.clauses[*i];

            log::info!("Integrating clause: {:?}", clause);

            let mut cbdd = man.zero();
            for x in clause {
                let node = if *x < 0_i32 {
                    man.nith_var(VarID(-x as u32))
                } else {
                    man.ith_var(VarID(*x as u32))
                };

                cbdd = man.or(node, cbdd);
            }

            bdd = man.and(cbdd, bdd);

            man.purge_retain(bdd);

            log::info!(
                "Nr. Nodes: {:?} ({:?}/{:?} clauses integrated)",
                &man.nodes.len(),
                n,
                &instance.clauses.len()
            );
            n += 1;
        }

        bdd = man.dvo_sifting(bdd);
        bdd = man.reduce(bdd);

        log::info!(
            "Final order: {:?} (layers: {:?})",
            man.order,
            order_to_layernames(&man.order)
        );

        Ok((man, bdd))
    }

    /// Initialize the BDD with zero and one constant nodes
    fn bootstrap(&mut self) {
        self.add_node(*ZERO);
        self.add_node(*ONE);
    }

    /// Collect all nodes that are part of the specified function
    fn collect_nodes(&self, f: NodeID) -> HashSet<NodeID> {
        let mut h = HashSet::default();

        fn collect_impl(man: &DDManager, f: NodeID, h: &mut HashSet<NodeID>) {
            if h.contains(&f) {
                return;
            }

            let inserted = h.insert(f);
            assert!(inserted);

            let node = man
                .nodes
                .get(&f)
                .unwrap_or_else(|| panic!("Node {:?} not in nodes list!", f));
            collect_impl(man, node.low(), h);
            collect_impl(man, node.high(), h);
        }

        collect_impl(self, f, &mut h);
        h
    }

    fn nr_nodes(&self, f: NodeID) -> usize {
        self.collect_nodes(f).len()
    }

    /// Find the variable at specified level
    fn var_at_level(&self, level: u32) -> Option<VarID> {
        self.order
            .iter()
            .enumerate()
            .find(|(_, &l)| l == level)
            .map(|(v, _)| VarID(v as u32))
    }

    fn format_node(&self, id: NodeID) -> String {
        let node = self.nodes.get(&id).unwrap();
        format!(
            "{{Node {:?}: {:?} high: {:?}, low: {:?} }}",
            id,
            node.var(),
            node.high(),
            node.low()
        )
    }

    /// Swaps graph layers of variables a and b. Requires a to be directly above b.
    fn swap(&mut self, a: VarID, b: VarID) {
        log::info!(
            "Swapping variables {:?} and {:?} (layers {} and {})",
            a,
            b,
            self.order[a.0 as usize],
            self.order[b.0 as usize]
        );
        assert!(a.0 != 0 && b.0 != 0);
        assert!(self.order[b.0 as usize] == self.order[a.0 as usize] + 1);
        let ids = self.var2nodes[a.0 as usize]
            .values()
            .cloned()
            .collect::<Vec<NodeID>>();

        self.var2nodes[a.0 as usize].clear();
        // Unique Table: Will not be valid. Directly modified node might be
        // degenerate. Reduction after reduce will restore canonical form.

        for id in ids {
            let f_id = id;

            let old_f_node = self.nodes[&f_id];

            log::debug!("Replacing node {:?} old_f_node={:?}", f_id, old_f_node);

            let f_1_id = old_f_node.high();
            let f_0_id = old_f_node.low();

            let f_0_node = self.nodes[&f_0_id];
            let f_1_node = self.nodes[&f_1_id];

            if f_0_node.var() != b && f_1_node.var() != b {
                // "If neither child of the node for f is labeled b, then the
                // node is moved to the other subtable; otherwise swapping
                // proceeds as described above"
                let new_f_node = DDNode::new(f_id, b, f_0_id, f_1_id);
                *self.nodes.get_mut(&f_id).unwrap() = new_f_node;
                continue;
            }

            let (f_01_id, f_00_id) = (f_0_node.high(), f_0_node.low());
            let (f_11_id, f_10_id) = (f_1_node.high(), f_1_node.low());

            let new_then_id = self.node_get_or_create(a, f_01_id, f_11_id);
            let new_else_id = self.node_get_or_create(a, f_00_id, f_10_id);

            log::debug!(
                "New children: high {} low {}",
                self.format_node(new_then_id),
                self.format_node(new_else_id)
            );

            // Replace F node
            let new_f_node = DDNode::new(f_id, b, new_else_id, new_then_id);

            log::debug!("new_f_node={:?}", new_f_node);

            *self.nodes.get_mut(&f_id).unwrap() = new_f_node;

            let old = self.var2nodes[b.0 as usize].insert((b, new_else_id, new_then_id), f_id);

            if let Some(old_node_id) = old {
                log::warn!( "The unique table for {:?} seems to already contain a node ({:?} {:?} {:?}) with ID {:?}", b,b,new_else_id,new_then_id,old_node_id)
            }

            log::debug!("Replaced node {:?} with {:?}", f_id, self.nodes[&f_id]);
        }
        self.order.swap(a.0 as usize, b.0 as usize);
        self.c_table.clear();
        log::debug!(
            "Order is now: {:?} (layers: {:?})",
            self.order,
            order_to_layernames(&self.order)
        );
    }

    fn dvo_sifting(&mut self, mut f: NodeID) -> NodeID {
        let var_ids: Vec<VarID> = self
            .var2nodes
            .iter()
            .enumerate()
            .filter(|(i, nodes)| *i != 0 && !nodes.is_empty())
            .map(|(i, _)| i)
            .map(|i| VarID(i as u32))
            .collect();

        for var in var_ids {
            self.purge_retain(f);
            f = self.reduce(f);
            self.purge_retain(f);
            let starting_pos = self.order[var.0 as usize];

            let mut best_position = starting_pos;
            let mut best_graphsize = self.nodes.len();

            log::info!(
                "Sifting variable {:?}, starting from level {} (graph size {}).",
                var,
                starting_pos,
                best_graphsize
            );

            // Move variable to the bottom
            let terminal_node_level = self.order[ZERO.var().0 as usize];

            log::debug!("Moving down...");
            for level in starting_pos + 1..terminal_node_level {
                log::debug!("Trying level {}", level);
                // Swap var at level-1 (our variable) with var at level
                self.swap(
                    self.var_at_level(level - 1).unwrap(),
                    self.var_at_level(level).unwrap(),
                );
                self.purge_retain(f);
                f = self.reduce(f);
                self.purge_retain(f);

                let new_size = self.nodes.len();
                if new_size < best_graphsize {
                    best_graphsize = new_size;
                    best_position = level;
                }
            }

            // Level is now bottom (terminal-1). Move variable to the top
            log::debug!("Moving up...");

            for level in (1..terminal_node_level - 1).rev() {
                log::debug!("Trying level {}", level);
                // Swap var at level+1 (our variable) with var at level
                self.swap(
                    self.var_at_level(level).unwrap(),
                    self.var_at_level(level + 1).unwrap(),
                );
                self.purge_retain(f);
                f = self.reduce(f);
                self.purge_retain(f);
                let new_size = self.nodes.len();
                if new_size < best_graphsize {
                    best_graphsize = new_size;
                    best_position = level;
                }
            }

            // Level is now top (1). Move variable down to best location

            log::info!(
                "The best result was graph size of {} at level {}. Moving there...",
                best_graphsize,
                best_position
            );

            for level in 2..best_position + 1 {
                // Swap var at level-1 (our variable) with var at level
                self.swap(
                    self.var_at_level(level - 1).unwrap(),
                    self.var_at_level(level).unwrap(),
                );
                self.purge_retain(f);
                f = self.reduce(f);
                self.purge_retain(f);
            }

            log::info!("Size is now  {}", self.nodes.len());
        }
        f
    }

    /// Ensure order vec is valid up to specified variable
    fn ensure_order(&mut self, target: VarID) {
        let old_size = self.order.len();

        if (target.0 as usize) < old_size {
            // order[target] exists an contains tree depth of target
            return;
        }

        // Ensure there is space for order[target]
        self.order.resize((target.0 + 1) as usize, 0);

        // Fill newly created space:
        let mut y = old_size;
        for x in old_size..self.order.len() {
            // order[x] = x
            self.order[x] = y as u32;
            y += 1;
        }

        // VarID 0 (terminal nodes) at the very bottom of the tree
        self.order[0] = y as u32;
    }

    /// Insert Node. ID is assigned for nonterminal nodes (var != 0).
    /// This does not check the unique table, you should do so before using!
    fn add_node(&mut self, node: DDNode) -> NodeID {
        if node.id().0 != 0 && node.id().0 != 1 {
            panic!("Adding node With ID > 1: {:?}", node);
        }

        if node.var().0 != 0 && node.id().0 != 0 {
            panic!("Trying to add node with predefined ID that is not a terminal node");
        }

        //log::debug!("\tCreating new node");
        let mut new_id = node.id();

        if node.var() != VarID(0) {
            if node.low() == node.high() {
                // log::warn!("Trying to add non-reduced node: low==high");
            }

            // Assign new node ID
            new_id = NodeID(rand::thread_rng().gen::<u32>());

            while self.nodes.get(&new_id).is_some() {
                new_id = NodeID(rand::thread_rng().gen::<u32>());
            }
        }
        //log::debug!("\t\tassigned ID {:?}", node.id());

        let node = DDNode::new(new_id, node.var(), node.low(), node.high());

        let id = node.id();
        let var = node.var();

        self.nodes.insert(id, node);

        while self.var2nodes.len() <= (var.0 as usize) {
            self.var2nodes.push(HashMap::default())
        }

        self.ensure_order(var);

        let was_inserted =
            self.var2nodes[var.0 as usize].insert((node.var(), node.low(), node.high()), id);
        if was_inserted != None {
            panic!("Node is already in unique table!");
        }

        id
    }

    /// Search for Node, create if it doesnt exist
    fn node_get_or_create(&mut self, var: VarID, low: NodeID, high: NodeID) -> NodeID {
        if self.var2nodes.len() <= (var.0 as usize) {
            // Unique table does not contain any entries for this variable. Create new Node.
            return self.add_node(DDNode::new(NodeID(0), var, low, high));
        }

        // Lookup in variable-specific unique-table
        let res = self.var2nodes[var.0 as usize].get(&(var, low, high));

        match res {
            Some(nodeid) => *nodeid, // An existing node was found
            None => self.add_node(DDNode::new(NodeID(0), var, low, high)), // No existing node found -> create new
        }
    }

    //------------------------------------------------------------------------//
    // Constants

    fn zero(&self) -> NodeID {
        NodeID(0)
    }

    fn one(&self) -> NodeID {
        NodeID(1)
    }

    //------------------------------------------------------------------------//
    // Variables

    pub fn ith_var(&mut self, var: VarID) -> NodeID {
        let v = DDNode::new(NodeID(0), var, NodeID(0), NodeID(1));

        if self.var2nodes.len() > (var.0 as usize) {
            let x = self.var2nodes[var.0 as usize].get(&(v.var(), v.low(), v.high()));

            if let Some(x) = x {
                return *x;
            }
        }

        self.add_node(v)
    }

    pub fn nith_var(&mut self, var: VarID) -> NodeID {
        let v = DDNode::new(NodeID(0), var, NodeID(1), NodeID(0));

        if self.var2nodes.len() > (var.0 as usize) {
            let x = self.var2nodes[var.0 as usize].get(&(v.var(), v.low(), v.high()));

            if let Some(x) = x {
                return *x;
            }
        }

        self.add_node(v)
    }

    //------------------------------------------------------------------------//
    // Unitary Operations

    fn not(&mut self, f: NodeID) -> NodeID {
        self.ite(f, NodeID(0), NodeID(1))
    }

    //------------------------------------------------------------------------//
    // Binary Operations

    pub fn and(&mut self, f: NodeID, g: NodeID) -> NodeID {
        self.ite(f, g, NodeID(0))
    }

    pub fn or(&mut self, f: NodeID, g: NodeID) -> NodeID {
        self.ite(f, NodeID(1), g)
    }

    #[allow(dead_code)]
    fn xor(&mut self, f: NodeID, g: NodeID) -> NodeID {
        let ng = self.not(g);

        self.ite(f, ng, g)
    }

    //------------------------------------------------------------------------//
    // N-ary Operations

    /// Find top variable: Highest in tree according to order
    fn min_by_order(&self, fvar: VarID, gvar: VarID, hvar: VarID) -> VarID {
        let list = [fvar, gvar, hvar];

        // Tree depths
        let tlist = [
            self.order[fvar.0 as usize],
            self.order[gvar.0 as usize],
            self.order[hvar.0 as usize],
        ];

        // Minimum tree depth
        let min = *tlist.iter().min().unwrap();
        // Index of Var with minimum tree depth
        let index = tlist.iter().position(|&x| x == min).unwrap();

        list[index]
    }

    fn ite(&mut self, f: NodeID, g: NodeID, h: NodeID) -> NodeID {
        let (f, g, h) = normalize_ite_args(f, g, h);
        match (f, g, h) {
            (_, NodeID(1), NodeID(0)) => f, // ite(f,1,0)
            (NodeID(1), _, _) => g,         // ite(1,g,h)
            (NodeID(0), _, _) => h,         // ite(0,g,h)
            (_, t, e) if t == e => t,       // ite(f,g,g)
            (_, _, _) => {
                let cache = self.c_table.get(&(f, g, h));

                if let Some(cached) = cache {
                    return *cached;
                }

                let fnode = self.nodes.get(&f).unwrap();
                let gnode = self.nodes.get(&g).unwrap();
                let hnode = self.nodes.get(&h).unwrap();

                let top = self.min_by_order(fnode.var(), gnode.var(), hnode.var());

                let fxt = fnode.restrict(top, &self.order, true);
                let gxt = gnode.restrict(top, &self.order, true);
                let hxt = hnode.restrict(top, &self.order, true);

                let fxf = fnode.restrict(top, &self.order, false);
                let gxf = gnode.restrict(top, &self.order, false);
                let hxf = hnode.restrict(top, &self.order, false);

                let high = self.ite(fxt, gxt, hxt);
                let low = self.ite(fxf, gxf, hxf);

                if low == high {
                    return low;
                }

                let out = self.node_get_or_create(top, low, high);

                self.c_table.insert((f, g, h), out);

                out
            }
        }
    }

    //------------------------------------------------------------------------//
    // Builders

    /// Creates an XOR "ladder"
    ///
    ///
    #[allow(dead_code)]
    fn xor_prim(&mut self, _vars: Vec<u32>) -> u32 {
        todo!();
    }

    #[allow(dead_code)]
    fn verify(&self, f: NodeID, trues: Vec<u32>) -> bool {
        let mut values: Vec<bool> = vec![false; self.var2nodes.len() + 1];

        for x in trues {
            let x: usize = x as usize;

            if x < values.len() {
                values[x] = true;
            } else {
                values[x] = false;
            }
        }

        let mut node_id = f;

        while node_id.0 >= 2 {
            let node = &self.nodes.get(&node_id).unwrap();

            if values[node.var().0 as usize] {
                node_id = node.high();
            } else {
                node_id = node.low();
            }
        }

        node_id.0 == 1
    }

    pub fn purge_retain(&mut self, f: NodeID) {
        let mut keep = HashSet::default();

        let mut stack = vec![f];

        while !stack.is_empty() {
            let x = stack.pop().unwrap();

            if keep.contains(&x) {
                continue;
            }

            let node = self.nodes.get(&x).unwrap();

            stack.push(node.low());
            stack.push(node.high());
            keep.insert(x);
        }

        let mut garbage = self.nodes.clone();

        garbage.retain(|&x, _| !keep.contains(&x) && x.0 > 1);

        for x in &garbage {
            self.var2nodes[x.1.var().0 as usize].remove(&(x.1.var(), x.1.low(), x.1.high()));
            self.nodes.remove(x.0);
        }

        self.c_table.retain(|_, x| keep.contains(x));
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap as HashMap;

    use crate::bdd_node::{DDNode, NodeID, VarID};

    use super::DDManager;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn build_trivial_bdd() -> (DDManager, NodeID) {
        let mut man = DDManager::default();
        #[allow(clippy::field_reassign_with_default)]
        {
            man.order = vec![3, 1, 2];
        }
        man.var2nodes
            .resize(man.order[0] as usize, HashMap::default());

        man.nodes.insert(
            NodeID(2),
            DDNode::new(NodeID(2), VarID(2), NodeID(0), NodeID(1)),
        );
        man.var2nodes[2].insert((VarID(2), NodeID(0), NodeID(1)), NodeID(2));

        man.nodes.insert(
            NodeID(3),
            DDNode::new(NodeID(3), VarID(1), NodeID(0), NodeID(2)),
        );
        man.var2nodes[1].insert((VarID(1), NodeID(0), NodeID(2)), NodeID(3));

        let f = NodeID(3);

        (man, f)
    }

    #[test]
    fn node_count() {
        init();

        let (man, f) = build_trivial_bdd();

        // Test that node-count works in trivial case
        assert!(man.nr_nodes(f) == 4);
    }

    #[test]
    fn node_count_multiroot() {
        init();

        let (mut man, f) = build_trivial_bdd();

        man.nodes.insert(
            NodeID(4),
            DDNode::new(NodeID(4), VarID(1), NodeID(1), NodeID(2)),
        );
        man.var2nodes[1].insert((VarID(1), NodeID(1), NodeID(2)), NodeID(4));

        // Test that node-count works even if unrelated nodes are present
        assert!(man.nr_nodes(f) == 4);
    }

    #[test]
    fn node_count_trivial_reduce() {
        init();

        let (mut man, f) = build_trivial_bdd();

        assert!(man.nr_nodes(f) == 4);

        let f = man.reduce(f);

        // Test that node count remains after reducing already-reduced graph
        assert!(man.nr_nodes(f) == 4);
    }

    #[test]
    fn sift_easy() {
        init();
        let (mut man, f) = build_trivial_bdd();
        assert!(man.nr_nodes(f) == 4);
        let f = man.dvo_sifting(f);
        assert!(man.nr_nodes(f) <= 4);
    }
}
