use crate::{utils::cut_addr, Ipv4Cidr};
use std::{collections::HashSet, net::Ipv4Addr};

use crate::RoutingTable;

pub struct HashRoutingTable {
    cidrs: HashSet<Ipv4Cidr>,
}

impl HashRoutingTable {
    pub fn new() -> Self {
        Self {
            cidrs: HashSet::new(),
        }
    }
}

impl RoutingTable for HashRoutingTable {
    fn add_cidr(&mut self, cidr: Ipv4Cidr) {
        self.cidrs.insert(cidr);
    }

    fn remove_cidr(&mut self, cidr: Ipv4Cidr) {
        self.cidrs.remove(&cidr);
    }

    fn find_exact_cidr(&self, addr: Ipv4Addr) -> Option<Ipv4Cidr> {
        for len in (0..=32).rev() {
            let new_addr = cut_addr(addr, len).expect("Len is correct");
            let cidr =
                Ipv4Cidr::new(new_addr, len).expect("Len and Ipv4Addr should always be valid.");

            if self.cidrs.contains(&cidr) {
                return Some(cidr);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::HashRoutingTable;
    use crate::routing_table::tests::{complex_test, empty_test, simple_test};

    #[test]
    fn test_hash_empty_case() {
        empty_test(Box::new(HashRoutingTable::new()));
    }

    #[test]
    fn test_hash_simple() {
        simple_test(Box::new(HashRoutingTable::new()));
    }

    #[test]
    fn test_hash_complex() {
        complex_test(Box::new(HashRoutingTable::new()))
    }
}
