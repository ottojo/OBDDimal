use std::collections::HashSet;

/// Tracks removed nodes from InstanceGraph with impact
/// on edges, clauses and components.
pub struct NodeRemoval {
    pub removed_node: u32,
    pub removed_edges: HashSet<(u32, u32)>,
    pub affected_clauses: HashSet<u32>,
    pub number_components: u32,
}
