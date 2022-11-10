#![allow(dead_code)]

extern crate obddimal as bdd;
use bdd::dimacs::{self, Instance};
use node_removal::NodeRemoval;

use std::collections::{HashMap, HashSet};

use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, UnGraph, NodeIndex};

pub mod node_removal;

pub struct InstanceGraph<'a> {
    ins: &'a Instance,
    adjacency: Vec<Vec<bool>>,
    // node_in_clause[i][j] indicates, whether node i is in clause j
    node_in_clauses: Vec<Vec<bool>>, // Alternatively, HashMap<u32, HashSet<u32>>?
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

    pub fn remove_nodes(&mut self, nodes: Vec<u32>) -> Vec<NodeRemoval> {
        let mut result: Vec<NodeRemoval> = Vec::new();

        for v in &nodes {
            self.g.remove_node(NodeIndex::new(*v as usize));
        }

        result
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

    fn nodes_in_clauses(instance: &'a Instance) -> Vec<Vec<bool>> {
        let m = instance.no_variables as usize; // rows
        let n = instance.no_clauses as usize; // columns
        let mut result: Vec<Vec<bool>> = vec![vec![false; n]; m + 1];

        for (i, clause) in instance.clauses.iter().enumerate() {
            for var in clause.iter() {
                let a = var.abs() as usize;
                result[a][i] = true;
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

/*
fn get_adjacency_matrix_and_nodes_clauses(instance: &Instance) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let n = instance.no_variables as usize;
    let mut result: Vec<Vec<i32>> = vec![vec![0; n+1]; n+1];
    let mut node_clauses: Vec<Vec<i32>> = vec![Vec::new(); n+1];
    for (i, clause) in instance.clauses.iter().enumerate() {
        for (j, x) in clause.iter().enumerate() {
            let a = x.abs() as usize;
            let mut clause_vector = &mut node_clauses[a];
            clause_vector.push(i as i32);
            for y in &clause[j+1..] {
                let b = y.abs() as usize;
                result[a][b] += 1;
                result[b][a] += 1;
            }
        }
    }
    (result, node_clauses)
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_nodes_from_sandwich() {
        let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(&instance);
        sandwich_graph.remove_nodes(vec![2, 9, 10]);
        
    }
}
