use std::collections::HashMap;

use crate::bdd_node::{DDNode, NodeID, VarID};

use super::{DDManager, ZERO};

impl DDManager {
    /// Generate graphviz for the provided function, not including any graph nodes not part of the function.
    /// TODO: Graphviz of all functions in DDManager
    pub fn graphviz(&self, f: NodeID) -> String {
        let nodes = self.collect_nodes(f);

        let mut by_var: HashMap<VarID, Vec<DDNode>> = HashMap::default();
        for id in nodes.iter() {
            let node = self.nodes.get(id).unwrap();
            by_var.entry(node.var()).or_default().push(*node);
        }

        let mut graph = String::new();
        graph += "digraph G {\n";
        graph += "\"1\" [shape = \"box\"];\n";
        graph += "\"0\" [shape = \"box\"];\n";

        let mut edges = String::new();
        for (var, nodes) in by_var {
            if var == ZERO.var() {
                continue;
            }
            graph += format!("subgraph cluster_{} {{\nrank=same;\n", var.0).as_str();
            for node in nodes {
                graph += format!(
                    "\"{}\" [label=\"{}\\n{}\" color=\"{}\"]\n",
                    node.id().0,
                    var.0,
                    node.id().0,
                    "black",
                )
                .as_str();
                edges += format!(
                    "\"{}\" -> \"{}\" [style = \"dotted\"]\n",
                    node.id().0,
                    node.low().0
                )
                .as_str();
                edges += format!("\"{}\" -> \"{}\"\n", node.id().0, node.high().0).as_str();
            }
            graph += "}\n\n";
        }

        graph += edges.as_str();
        graph += "}\n";
        graph
    }
}
