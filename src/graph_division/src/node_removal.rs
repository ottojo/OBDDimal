use std::collections::HashSet;

pub struct NodeRemoval {
    pub removed_node: u32,
    pub removed_edges: HashSet<(u32, u32)>,
    pub affected_clauses: HashSet<u32>, // total
    pub number_components: u32
}
