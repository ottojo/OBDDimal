#![allow(dead_code)]

extern crate obddimal as bdd;
use bdd::dimacs;
use bdd::dimacs::Instance;

use graph_division::InstanceGraph;
use graph_division::node_removal::NodeRemoval;

use petgraph::graph::{NodeIndex, UnGraph, Graph};
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};
use petgraph::Undirected;
use petgraph::graph::EdgeReference;
use petgraph::visit::EdgeRef;

fn main() {
    let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
        let mut sandwich_graph = InstanceGraph::new(&instance);
        let edges = sandwich_graph.get_graph().edges(NodeIndex::new(8));
        for i in edges {
            println!("Source: {:?}", i.source());
        }
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
