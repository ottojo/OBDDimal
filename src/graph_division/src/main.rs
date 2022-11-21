extern crate obddimal as bdd;
use bdd::dimacs;
use bdd::dimacs::Instance;

use graph_division::InstanceGraph;

use petgraph::algo;

// TODO: Create example?
fn main() {
    let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
    let mut sandwich_graph = InstanceGraph::new(instance);
    sandwich_graph.print_graph();

    /* Removes at least 30% of the most occurring nodes */
    sandwich_graph.remove_top_occurring_nodes_by_quota(Some(0.3));

    /* Observe how the graph splits */
    sandwich_graph.print_graph();
    println!(
        "Number of components: {}",
        algo::connected_components(sandwich_graph.get_graph())
    );
}
