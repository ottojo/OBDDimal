//! Satisfyability count, active nodes count

use std::collections::hash_map::Entry::{Occupied, Vacant};

use num_bigint::BigUint;
use num_traits::{One, Zero};

use super::{hash_select::HashMap, DDManager};
use crate::bdd_node::{NodeID, VarID};

impl DDManager {
    #[allow(dead_code)]
    fn is_sat(&self, node: u32) -> bool {
        node != 0
    }

    pub fn sat_count(&self, f: NodeID) -> BigUint {
        self.sat_count_rec(f, &mut HashMap::default())
    }

    fn sat_count_rec(&self, f: NodeID, cache: &mut HashMap<NodeID, BigUint>) -> BigUint {
        let mut total: BigUint = Zero::zero();
        let node_id = f;

        if node_id == NodeID(0) {
            return Zero::zero();
        } else if node_id == NodeID(1) {
            return One::one();
        } else {
            let node = &self.nodes.get(&node_id).unwrap();

            let low = &self.nodes.get(&node.low).unwrap();
            let high = &self.nodes.get(&node.high).unwrap();

            let low_jump = if low.var == VarID(0) {
                self.order.len() as u32 - self.order[node.var.0 as usize] - 1
            } else {
                self.order[low.var.0 as usize] - self.order[node.var.0 as usize] - 1
            };

            let high_jump = if high.var == VarID(0) {
                self.order.len() as u32 - self.order[node.var.0 as usize] - 1
            } else {
                self.order[high.var.0 as usize] - self.order[node.var.0 as usize] - 1
            };

            let low_fac = BigUint::parse_bytes(b"2", 10).unwrap().pow(low_jump);
            let high_fac = BigUint::parse_bytes(b"2", 10).unwrap().pow(high_jump);

            total += match cache.get(&node.low) {
                Some(x) => x * low_fac,
                None => self.sat_count_rec(node.low, cache) * low_fac,
            };

            total += match cache.get(&node.high) {
                Some(x) => x * high_fac,
                None => self.sat_count_rec(node.high, cache) * high_fac,
            };
        };

        cache.insert(f, total.clone());

        total
    }

    pub fn count_active(&mut self, f: NodeID) -> u32 {
        let mut cnt = 0;

        let mut to_count = vec![f];

        while !to_count.is_empty() {
            let mut node = self.nodes.get_mut(&to_count.pop().unwrap()).unwrap();
            if node.visited_flag {
                continue;
            }
            node.visited_flag = true;
            to_count.push(node.high);
            to_count.push(node.low);
            cnt += 1;
        }

        for (_id, node) in self.nodes.iter_mut() {
            node.visited_flag = false;
        }

        cnt
    }
}
