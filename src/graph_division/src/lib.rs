//#![allow(dead_code)]

extern crate obddimal as bdd;
use bdd::dimacs::{self, Instance};

use node_removal::NodeRemoval;

use std::collections::{HashMap, HashSet};

use petgraph::algo;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{EdgeReference, Graph, UnGraph, NodeIndex};
use petgraph::visit::EdgeRef;

pub mod node_removal;

pub struct InstanceGraph {
    ins: Instance,
    adjacency: Vec<Vec<bool>>,
    node_in_clauses: HashMap<u32, HashSet<u32>>,
    g: UnGraph<u32, ()>,
}

impl InstanceGraph {
    pub fn new(instance: Instance) -> InstanceGraph {
        let adjacency_mat = Self::adjacency_matrix(&instance);
        let node_clauses = Self::nodes_in_clauses(&instance);
        InstanceGraph {
            ins: instance,
            g: Self::graph(&adjacency_mat),
            adjacency: adjacency_mat,
            node_in_clauses: node_clauses
        }
    }

    pub fn get_instance(&self) -> &Instance {
        &self.ins
    }

    pub fn get_graph(&self) -> &UnGraph<u32, ()> {
        &self.g
    }

    pub fn print_graph(&self) {
        println!("{:?}", Dot::with_config(&self.g, &[Config::EdgeNoLabel]));
    }

    pub fn remove_top_occurring_nodes_by_quota(&mut self, quota: Option<f64>) -> Vec<NodeRemoval> {
        self.remove_nodes(vec![])
    }

    pub fn remove_top_occurring_nodes_by_growth(&mut self, factor: Option<f64>) -> Vec<NodeRemoval> {
        self.remove_nodes(vec![])
    }

    fn remove_nodes(&mut self, nodes: Vec<u32>) -> Vec<NodeRemoval> {
        let mut result: Vec<NodeRemoval> = Vec::new();
        let mut clauses = HashSet::new();

        for v in &nodes {
            let edges: HashSet<(u32, u32)> = HashSet::from_iter(self.g
                                                .edges(NodeIndex::new(*v as usize))
                                                .map(|e| (e.source().index() as u32, e.target().index() as u32)));
            clauses.extend(self.node_in_clauses.get(v).unwrap());
            self.g.remove_node(NodeIndex::new(*v as usize));
            result.push(
                NodeRemoval{
                    removed_node: *v,
                    removed_edges: edges,
                    affected_clauses: clauses.clone(),
                    number_components: algo::connected_components(&self.g) as u32
                });
        }

        result
    }

    fn remove_most_occurring_nodes(&mut self, nodes: Vec<u32>) -> Vec<NodeRemoval> {
        self.remove_nodes(vec![])
    }

    fn variable_occurrences(&self) -> HashMap<u32, u32> {
        let mut var_occs: HashMap<u32, u32> = HashMap::new();

        for clause in &self.ins.clauses {
            for var in clause {
                let x = var.abs() as u32;
                let count = var_occs.entry(x).or_insert(0);
                *count += 1;
            }
        }
        var_occs
    }

    fn adjacency_matrix(instance: &Instance) -> Vec<Vec<bool>> {
        let n = instance.no_variables as usize;
        let mut result: Vec<Vec<bool>> = vec![vec![false; n + 1]; n + 1];

        for clause in &instance.clauses {
            for (j, var) in clause.iter().enumerate() {
                let a = var.abs() as usize;
                for y in &clause[j + 1..] {
                    let b = y.abs() as usize;
                    result[a][b] = true;
                    result[b][a] = true;
                }
            }
        }

        result
    }

    fn nodes_in_clauses(instance: &Instance) -> HashMap<u32, HashSet<u32>> {
        let mut result: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (i, clause) in instance.clauses.iter().enumerate() {
            for var in clause.iter() {
                let a = var.abs() as u32;
                let mut set = result.entry(a).or_insert(HashSet::new());
                set.insert(i as u32);
            }
        }

        result
    }

    fn graph(matrix: &Vec<Vec<bool>>) -> UnGraph<u32, ()> {
        let mut edges: Vec<(u32, u32)> = Vec::new();
        let mut graph = Graph::new_undirected();

        for i in 0..matrix.len() {
            graph.add_node(i as u32);
        }
        for i in 0..matrix.len() {
            for j in i..matrix[i].len() {
                if matrix[i][j] {
                    edges.push((i as u32, j as u32));
                }
            }
        }
        graph.extend_with_edges(&edges);
        graph.remove_node(NodeIndex::new(0));

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_nodes_from_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(instance);
        sandwich_graph.remove_nodes(vec![2, 9, 10]);
    }

    #[test]
    fn test_clauses_of_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(instance);
        for i in vec![1,2,3,21] {
            assert!(sandwich_graph.node_in_clauses.get(&2).unwrap().contains(&i));
        }    
    }

    #[test]
    fn test_adjacency_matrix_of_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(instance);
        assert!(sandwich_graph.adjacency[8][7]);
        assert!(sandwich_graph.adjacency[8][1]);
        assert!(sandwich_graph.adjacency[8][12]);
        assert!(sandwich_graph.adjacency[8][10]);

        assert!(!sandwich_graph.adjacency[8][3]);
        assert!(!sandwich_graph.adjacency[8][18]);
        assert!(!sandwich_graph.adjacency[8][8]);
        assert!(!sandwich_graph.adjacency[8][5]);
    }

    #[test]
    fn test_remove_nodes_from_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(instance);
        let removed_nodes = sandwich_graph.remove_nodes(vec![2,9,12]);

        assert_eq!(2, removed_nodes[0].removed_node);
        assert!(removed_nodes[0].removed_edges.contains(&(2, 10)) 
                || removed_nodes[0].removed_edges.contains(&(10, 2)));
        assert!(removed_nodes[0].removed_edges.contains(&(2, 11)) 
                || removed_nodes[0].removed_edges.contains(&(11, 2)));
        assert!(removed_nodes[0].removed_edges.contains(&(2, 16)) 
                || removed_nodes[0].removed_edges.contains(&(16, 2)));
        assert_eq!(4, removed_nodes[0].affected_clauses.len());
        assert_eq!(4, removed_nodes[0].number_components);

        assert_eq!(9, removed_nodes[1].removed_node);
        assert!(removed_nodes[1].removed_edges.contains(&(9, 10)) 
                || removed_nodes[1].removed_edges.contains(&(10, 9)));
        assert!(removed_nodes[1].removed_edges.contains(&(9, 6)) 
                || removed_nodes[1].removed_edges.contains(&(6, 9)));
        assert!(removed_nodes[1].removed_edges.contains(&(9, 17)) 
                || removed_nodes[1].removed_edges.contains(&(17, 9)));
        assert_eq!(9, removed_nodes[1].affected_clauses.len());
        assert_eq!(5, removed_nodes[1].number_components);

        assert_eq!(12, removed_nodes[2].removed_node);
        assert!(removed_nodes[2].removed_edges.contains(&(12, 8)) 
                || removed_nodes[2].removed_edges.contains(&(8, 12)));
        assert!(removed_nodes[2].removed_edges.contains(&(12, 1)) 
                || removed_nodes[2].removed_edges.contains(&(1, 12)));
        assert!(removed_nodes[2].removed_edges.contains(&(12, 7)) 
                || removed_nodes[2].removed_edges.contains(&(7, 12)));
        assert_eq!(13, removed_nodes[2].affected_clauses.len());
        assert_eq!(5, removed_nodes[2].number_components);
    }

    #[test]
    fn test_remove_top_occurring_nodes_by_quota_from_sandwich_default() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(instance);
        let removed_nodes = sandwich_graph.remove_top_occurring_nodes_by_quota(None);

        assert_eq!(8, removed_nodes[0].removed_node);

        assert_eq!(10, removed_nodes[1].removed_node);
    }

    fn test_remove_top_occurring_nodes_by_quota_from_busybox_five() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/busybox.dimacs");
        let mut sandwich_graph = InstanceGraph::new(instance);
        let removed_nodes = sandwich_graph.remove_top_occurring_nodes_by_quota(Some(0.05));
    }

    fn test_remove_top_occurring_nodes_by_quota_from_busybox_twenty() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/busybox.dimacs");
        let mut sandwich_graph = InstanceGraph::new(instance);
        let removed_nodes = sandwich_graph.remove_top_occurring_nodes_by_quota(Some(0.2));
    }
}
