use crate::{utils::cut_addr, Ipv4Cidr, RoutingTable};
use std::{collections::HashSet, net::Ipv4Addr};

pub struct HashRoutingTable {
    cidrs: Vec<HashSet<u32>>,
}

impl HashRoutingTable {
    pub fn new() -> Self {
        let mut cidrs = Vec::with_capacity(33);
        for _ in 0..=32 {
            cidrs.push(HashSet::new());
        }

        Self { cidrs }
    }
}

impl RoutingTable for HashRoutingTable {
    fn add_cidr(&mut self, cidr: Ipv4Cidr) {
        self.cidrs[cidr.prefix_len() as usize].insert(u32::from(cidr.min()));
    }

    fn remove_cidr(&mut self, cidr: Ipv4Cidr) {
        self.cidrs[cidr.prefix_len() as usize].remove(&u32::from(cidr.min()));
    }

    fn find_exact_cidr(&self, addr: Ipv4Addr) -> Option<Ipv4Cidr> {
        let mut bit_mask = u32::MAX;
        let mut addr_bits = u32::from(addr);

        for len in (0..=32).rev() {
            addr_bits &= bit_mask;
            bit_mask <<= 1;

            if self.cidrs[len as usize].contains(&addr_bits) {
                let cidr = Ipv4Cidr::from_bits(addr_bits, len)
                    .expect("Len and Ipv4Addr should always be valid.");

                return Some(cidr);
            }
        }

        None
    }

    fn size(&self) -> usize {
        self.cidrs.iter().map(|s| s.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::HashRoutingTable;
    use crate::routing_table::tests::{complex_test, empty_test, simple_test, one_global_cidr};

    #[test]
    fn test_hash_empty_case() {
        empty_test(Box::new(HashRoutingTable::new()));
    }

    #[test]
    fn test_one_global_cidr() {
        one_global_cidr(Box::new(HashRoutingTable::new()));
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
