//#![allow(dead_code)]

extern crate obddimal as bdd;
use bdd::dimacs::{self, Instance};

use node_removal::NodeRemoval;

use std::collections::{HashMap, HashSet};

use petgraph::algo;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, UnGraph, NodeIndex};

pub mod node_removal;

pub struct InstanceGraph<'a> {
    ins: &'a Instance,
    adjacency: Vec<Vec<bool>>,
    node_in_clauses: HashMap<u32, HashSet<u32>>,
    g: UnGraph<u32, ()>,
}

impl<'a> InstanceGraph<'a> {
    pub fn new(instance: &'a Instance) -> InstanceGraph<'a> {
        let adjacency_mat = Self::adjacency_matrix(&instance);
        InstanceGraph {
            ins: instance,
            g: Self::graph(&adjacency_mat),
            adjacency: adjacency_mat,
            node_in_clauses: Self::nodes_in_clauses(&instance),
        }
    }

    pub fn get_instance(&self) -> &'a Instance {
        self.ins
    }

    pub fn get_graph(&self) -> &UnGraph<u32, ()> {
        &self.g
    }

    pub fn print_graph(&self) {
        println!("{:?}", Dot::with_config(&self.g, &[Config::EdgeNoLabel]));
    }

    fn remove_nodes(&mut self, nodes: Vec<u32>) -> Vec<NodeRemoval> {
        let mut result: Vec<NodeRemoval> = Vec::new();
        let mut clauses = HashSet::new();

        for v in &nodes {
            let clauses: HashSet<u32> = clauses
                                            .union(&self.node_in_clauses.get(v).unwrap())
                                            .cloned()
                                            .collect();
            result.push(
                NodeRemoval{
                    removed_node: *v,
                    removed_edges: HashSet::new()/*from_iter(self.g.edges(NodeIndex::new(*v as usize)))*/,
                    affected_clauses: clauses.clone(),
                    number_components: algo::connected_components(&self.g) as u32
                });
            self.g.remove_node(NodeIndex::new(*v as usize));
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

    fn adjacency_matrix(instance: &'a Instance) -> Vec<Vec<bool>> {
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

    fn nodes_in_clauses(instance: &'a Instance) -> HashMap<u32, HashSet<u32>> {
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

        // Weights are only used as labels for debugging using Graphviz, therefore should not be
        // interpreted as actual weights.
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

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_nodes_from_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(&instance);
        sandwich_graph.remove_nodes(vec![2, 9, 10]);
    }

    #[test]
    fn test_clauses_of_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(&instance);
        for i in vec![1,2,3,21] {
            assert!(sandwich_graph.node_in_clauses.get(&2).unwrap().contains(&i));
        }    
    }

    #[test]
    fn test_adjacency_matrix_of_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(&instance);
        assert!(sandwich_graph.adjacency[8][7]);
        assert!(sandwich_graph.adjacency[8][1]);
        assert!(sandwich_graph.adjacency[8][12]);
        assert!(sandwich_graph.adjacency[8][10]);

        assert!(!sandwich_graph.adjacency[8][3]);
        assert!(!sandwich_graph.adjacency[8][18]);
    }

    #[test]
    fn test_remove_nodes_from_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(&instance);
        let removed_nodes = sandwich_graph.remove_nodes(vec![2,9,12]);

        assert_eq!(2, removed_nodes[0].removed_node);
        

        assert_eq!(9, removed_nodes[1].removed_node);

        assert_eq!(12, removed_nodes[2].removed_node);
    }
}
