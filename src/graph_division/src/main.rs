extern crate obddimal as bdd;
use bdd::dimacs;
use bdd::dimacs::Instance;

use std::collections::HashMap;

use petgraph::graph::{NodeIndex, UnGraph, Graph};
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};
use petgraph::Undirected;

fn main() {
    let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
    println!("variables = {}, clauses = {}", instance.no_variables, instance.no_clauses);
    let nodes = count_variable_occurences(&instance);
    println!("{:?}", nodes);

    //let cand = get_candidates(nodes.values().cloned().collect(), 0.4);

}

fn count_variable_occurences(instance: &Instance) -> HashMap<i32, i32> {
    let mut occurrences = vec![0; (instance.no_variables + 1) as usize];
    let mut var_occs = HashMap::new();
    var_occs.insert(0, 0);

    for clause in &instance.clauses {
        for var in clause {
            let x = var.abs();
            let mut count = var_occs.entry(x).or_insert(0);
            *count += 1;
        }
    }
    var_occs
}

// TODO: Implement with graph
fn delete_nodes(instance: &Instance, number: i32, candidates: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut deleted_nodes = Vec::new();
    let mut affected_clauses = Vec::new();

    (deleted_nodes, affected_clauses)
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
