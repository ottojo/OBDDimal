use rustc_hash::FxHashMap as HashMap;

use crate::{
    bdd_manager::{order_to_layernames, ZERO},
    bdd_node::{DDNode, NodeID, VarID},
};

use super::DDManager;

impl DDManager {
    pub(crate) fn reduce(&mut self, v: NodeID) -> NodeID {
        log::debug!("reducing");

        let mut vlist: Vec<Vec<NodeID>> = vec![Vec::new(); self.order[0] as usize];

        for (id, node) in self.nodes.iter() {
            vlist[node.var().0 as usize].push(*id);
        }

        let mut nextid = 0;

        #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
        enum Key {
            Terminal(bool),
            LowHigh(NodeID, NodeID),
            Unmatchable,
        }

        // This will be modified, such that old_nodes[id].id != id may be true, if a node is redundant.
        let mut old_nodes = self.nodes.clone();
        let mut new_nodes: HashMap<NodeID, DDNode> = HashMap::default();

        // Graph layers, bottom to top
        for i in order_to_layernames(&self.order).iter().rev() {
            log::debug!("Handling var {:?}", i);

            #[allow(non_snake_case)]
            let mut Q: Vec<(Key, NodeID)> = Vec::new();

            for u in vlist[i.0 as usize].iter() {
                //log::debug!(" Node {:?}", u);
                let node_var = old_nodes.get(u).unwrap().var();
                let node_id = old_nodes.get(u).unwrap().id();
                let low_ptr = old_nodes.get(u).unwrap().low();
                let high_ptr = old_nodes.get(u).unwrap().high();
                let low_real_id = old_nodes.get(&low_ptr).unwrap().id();
                let high_real_id = old_nodes.get(&high_ptr).unwrap().id();
                if node_var == VarID(0) {
                    //log::debug!("  terminal node. Adding to Q");
                    let key = if node_id == ZERO.id {
                        Key::Terminal(false)
                    } else {
                        Key::Terminal(true)
                    };

                    Q.push((key, *u));
                } else if low_real_id == high_real_id {
                    //log::debug!("  Redundant to only child {:?}!", node.low());
                    old_nodes.get_mut(u).unwrap().id = low_real_id;
                } else {
                    //log::debug!("  Normal node, adding to Q");
                    Q.push((Key::LowHigh(low_real_id, high_real_id), node_id));
                }
            }

            Q.sort_by_key(|k| k.0);

            log::debug!(" Iterating over Q...");
            let mut oldkey = Key::Unmatchable;
            for (key, u) in Q {
                log::debug!("  <{:?}, {:?}>", key, u);
                if key == oldkey {
                    log::debug!(
                        "  Repeated key -> Duplicate node. Assigning same ID as last ({:?})",
                        NodeID(nextid)
                    );
                    let node = old_nodes.get_mut(&u).unwrap();
                    node.id = NodeID(nextid);
                } else {
                    log::debug!("  New node");
                    nextid = {
                        match key {
                            Key::Terminal(true) => 1,
                            Key::Terminal(false) => 0,
                            _ => nextid + 1,
                        }
                    };
                    {
                        let node = old_nodes.get_mut(&u).unwrap();
                        log::debug!("  Assigning ID {:?}", nextid);
                        node.id = NodeID(nextid);
                    }

                    let (low_ptr, high_ptr) = {
                        let node = old_nodes.get(&u).unwrap();
                        (node.low, node.high)
                    };

                    log::debug!("  Visiting low and high child to see if ID changed");
                    let lownode_id = old_nodes
                        .get(&low_ptr)
                        .unwrap_or_else(|| {
                            panic!("Low child at {:?} not found in old nodes list!", low_ptr)
                        })
                        .id();
                    let highnode_id = old_nodes
                        .get(&high_ptr)
                        .unwrap_or_else(|| {
                            panic!("High child at {:?} not found in old nodes list!", high_ptr)
                        })
                        .id();

                    log::debug!(
                        "  Low, High were ({:?},{:?}), are now ({:?},{:?})",
                        low_ptr,
                        high_ptr,
                        lownode_id,
                        highnode_id
                    );

                    //log::debug!("  Updating node");
                    let node = old_nodes.get_mut(&u).unwrap();
                    node.low = lownode_id;
                    node.high = highnode_id;

                    new_nodes.insert(node.id(), *node);

                    oldkey = key;
                }
            }
        }

        self.nodes = new_nodes;

        // Rebuild unique-table
        for v in self.var2nodes.iter_mut() {
            v.clear();
        }

        for (_id, node) in self.nodes.iter() {
            self.var2nodes[node.var().0 as usize]
                .insert((node.var(), node.low(), node.high()), node.id());
        }

        // Return updated ID of function (Changes due to renumbering, but this
        // is unavoidable since v may have been redundant or duplicate)
        old_nodes.get(&v).unwrap().id()
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap as HashMap;

    use crate::bdd_node::{DDNode, NodeID, VarID};

    use super::DDManager;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn reduce_redundant() {
        init();

        let mut man = DDManager::default();
        #[allow(clippy::field_reassign_with_default)]
        {
            man.order = vec![4, 1, 2, 3];
        }
        man.var2nodes.resize(5, HashMap::default());

        man.nodes.insert(
            NodeID(2),
            DDNode::new(NodeID(2), VarID(3), NodeID(0), NodeID(1)),
        );
        man.var2nodes[3].insert((VarID(3), NodeID(0), NodeID(1)), NodeID(2));

        man.nodes.insert(
            NodeID(3),
            DDNode::new(NodeID(3), VarID(2), NodeID(2), NodeID(2)),
        );
        man.var2nodes[2].insert((VarID(2), NodeID(2), NodeID(2)), NodeID(3));

        man.nodes.insert(
            NodeID(4),
            DDNode::new(NodeID(4), VarID(1), NodeID(3), NodeID(3)),
        );
        man.var2nodes[1].insert((VarID(1), NodeID(3), NodeID(3)), NodeID(4));

        let f = man.reduce(NodeID(4));
        let f_node = man.nodes.get(&f).unwrap();
        assert!(f_node.low() == NodeID(0));
        assert!(f_node.high() == NodeID(1));
    }

    #[test]
    fn reduce_duplicate() {
        init();

        let mut man = DDManager::default();
        #[allow(clippy::field_reassign_with_default)]
        {
            man.order = vec![4, 1, 2, 3];
        }
        man.var2nodes.resize(5, HashMap::default());

        man.nodes.insert(
            NodeID(2),
            DDNode::new(NodeID(2), VarID(3), NodeID(0), NodeID(1)),
        );
        man.var2nodes[3].insert((VarID(3), NodeID(0), NodeID(1)), NodeID(2));

        // Duplicate node
        man.nodes.insert(
            NodeID(3),
            DDNode::new(NodeID(3), VarID(3), NodeID(0), NodeID(1)),
        );

        man.nodes.insert(
            NodeID(4),
            DDNode::new(NodeID(4), VarID(2), NodeID(1), NodeID(2)),
        );
        man.var2nodes[2].insert((VarID(2), NodeID(1), NodeID(2)), NodeID(4));

        man.nodes.insert(
            NodeID(5),
            DDNode::new(NodeID(5), VarID(1), NodeID(3), NodeID(4)),
        );
        man.var2nodes[1].insert((VarID(1), NodeID(3), NodeID(4)), NodeID(5));

        let f = NodeID(5);
        let f = man.reduce(f);
        let f_node = man.nodes.get(&f).unwrap();

        let t_node_id = f_node.high();
        let t_node = man.nodes.get(&t_node_id).unwrap();
        assert!(f_node.low() == t_node.high());
    }
}
