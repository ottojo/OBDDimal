use std::collections::HashSet;

pub struct NodeRemoval {
    pub node: u32,
    pub clauses: HashSet<u32>,
    pub number_components: u32
}

