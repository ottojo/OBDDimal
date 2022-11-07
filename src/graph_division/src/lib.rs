#![allow(unused_imports)]

extern crate obddimal as bdd;
use bdd::dimacs;
use bdd::dimacs::Instance;

use std::collections::HashMap;

use petgraph::graph::{NodeIndex, UnGraph, Graph};
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};
use petgraph::Undirected;


struct InstanceGraph<'a> {
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
            node_in_clauses: Self::nodes_in_clauses(&instance)
            
        }
    }

    pub fn get_instance(&self) -> &'a Instance {
        self.ins
    }

    fn adjacency_matrix(instance: &'a Instance) -> Vec<Vec<bool>> {
        let n = instance.no_variables as usize;
        let mut result: Vec<Vec<bool>> = vec![vec![false; n+1]; n+1];

        for (i, clause) in instance.clauses.iter().enumerate() {
            for (j, var) in clause.iter().enumerate() {
                let a = var.abs() as usize;
                for y in &clause[j+1..] {
                    let b = y.abs() as usize;
                    result[a][b] = true;
                    result[b][a] = true;
                }
            }
        }

        result
    }

    fn nodes_in_clauses(instance: &'a Instance) -> Vec<Vec<bool>> {
        let n = instance.no_variables as usize;
        let mut result: Vec<Vec<bool>> = vec![vec![false; n]; n+1];

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
        for i in 0..matrix.len() { graph.add_node(i as u32); }
        for i in 0..matrix.len() {
            for j in i..matrix[i].len() {
                if matrix[i][j] { edges.push((i as u32, j as u32)); }
            }
        }
        graph.extend_with_edges(&edges);
        //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

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

}
