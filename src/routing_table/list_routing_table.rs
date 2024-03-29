use crate::{Ipv4Cidr, RoutingTable};
use std::net::Ipv4Addr;

#[derive(Default)]
pub struct ListRoutingTable {
    cidrs: Vec<Ipv4Cidr>,
}

impl ListRoutingTable {
    pub fn new() -> Self {
        Self { cidrs: Vec::new() }
    }
}

impl RoutingTable for ListRoutingTable {
    fn add_cidr(&mut self, cidr: Ipv4Cidr) {
        self.cidrs.push(cidr);
    }

    fn remove_cidr(&mut self, cidr: Ipv4Cidr) {
        self.cidrs.retain(|cur| cur != &cidr);
    }

    fn find_exact_cidr(&self, addr: Ipv4Addr) -> Option<Ipv4Cidr> {
        self.cidrs.iter().fold(None, |acc, cidr| {
            if cidr.contains(addr) {
                match acc {
                    None => Some(*cidr),
                    Some(other) if other.prefix_len() < cidr.prefix_len() => Some(*cidr),
                    Some(_) => acc,
                }
            } else {
                acc
            }
        })
    }

    fn size(&self) -> usize {
        self.cidrs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::ListRoutingTable;
    use crate::routing_table::tests::{complex_test, empty_test, simple_test, one_global_cidr};

    #[test]
    fn test_list_empty_case() {
        empty_test(Box::new(ListRoutingTable::new()));
    }

    #[test]
    fn test_one_global_cidr() {
        one_global_cidr(Box::new(ListRoutingTable::new()));
    }

    #[test]
    fn test_list_simple() {
        simple_test(Box::new(ListRoutingTable::new()));
    }

    #[test]
    fn test_list_complex() {
        complex_test(Box::new(ListRoutingTable::new()))
    }
}
