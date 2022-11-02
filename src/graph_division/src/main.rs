extern crate obddimal as bdd;
use bdd::dimacs;
use bdd::dimacs::Instance;

use std::any::type_name;

use petgraph::graph::{NodeIndex, UnGraph, Graph};
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};
use petgraph::Undirected;

fn main() {
    let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
    println!("variables = {}, clauses = {}", instance.no_variables, instance.no_clauses);
    let nodes = count_variable_occurences(&instance);
    let cand = get_candidates(nodes, 0.4);
    let (matrix, node_clauses) = get_adjacency_matrix_and_nodes_clauses(&instance);
    graph_from_matrix(&matrix);
}

fn count_variable_occurences(instance: &Instance) -> Vec<i32> {
    let mut occurrences = vec![0; (instance.no_variables + 1) as usize];

    for clause in &instance.clauses {
        for var in clause {
            let x = var.abs();
            occurrences[x as usize] += 1;
        }
    }

    return occurrences;
}

fn delete_nodes(instance: &Instance, number: i32, candidates: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut deleted_nodes = Vec::new();
    let mut affected_clauses = Vec::new();

    (deleted_nodes, affected_clauses)
}

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

fn get_candidates(metric: Vec<i32>, percentage: f32) -> Vec<i32> {
    let mut candidates: Vec<i32> = Vec::new();
    let n = (percentage * metric.len() as f32) as i32;

    let metric_iterator = metric.iter().enumerate()/*.sort_by(|_, x| x)*/;
    let mut tuples: Vec<(i32, i32)> = Vec::new();
    for a in metric_iterator {
        let (f, g) = a;
        let x = f;
        let y = g.clone();
        tuples.push((x as i32, y));
    }
    tuples.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", tuples);

    while candidates.len() <= n as usize {
        if let Some((x,y)) = tuples.pop() {
            candidates.push(x);
            while tuples[0].1 == y {
                if let Some((a, b)) = tuples.pop() { candidates.push(a); }
            }
        }
    }

    candidates
}

fn graph_from_matrix(matrix: &Vec<Vec<i32>>) -> UnGraph::<i32, ()> {
    let mut edges: Vec<(u32, u32)> = Vec::new();
    edges.push((5,6));
    let mut graph = Graph::new_undirected();
    let edges2 = vec![(3,4), (4,5)];
    graph.extend_with_edges(&[(1,2),(2,3)]);
    graph.extend_with_edges(&edges);
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    graph
}
