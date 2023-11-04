use crate::Ipv4Cidr;
pub use hash_routing_table::HashRoutingTable;
pub use list_routing_table::ListRoutingTable;
use std::net::Ipv4Addr;
pub use trie_routing_table::TrieRoutingTable;

mod hash_routing_table;
mod list_routing_table;
mod trie_routing_table;

pub trait RoutingTable {
    fn add_cidr(&mut self, cidr: Ipv4Cidr);

    fn remove_cidr(&mut self, cidr: Ipv4Cidr);

    fn find_exact_cidr(&self, addr: Ipv4Addr) -> Option<Ipv4Cidr>;

    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::RoutingTable;
    use crate::Ipv4Cidr;
    use std::net::Ipv4Addr;

    pub fn empty_test(routing_table: Box<dyn RoutingTable>) {
        let test_cases = [
            Ipv4Addr::new(0, 0, 0, 0),
            Ipv4Addr::new(0, 0, 0, 0),
            Ipv4Addr::new(0, 0, 0, 0),
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 200, 4),
            Ipv4Addr::new(192, 168, 200, 8),
            Ipv4Addr::new(169, 254, 0, 0),
            Ipv4Addr::new(127, 0, 0, 0),
            Ipv4Addr::new(100, 64, 0, 0),
        ];

        for addr in test_cases {
            let result = routing_table.find_exact_cidr(addr);

            assert_eq!(result, None, "we expect no cidr is found");
        }

        assert_eq!(0, routing_table.size());
    }

    pub fn one_global_cidr(mut routing_table: Box<dyn RoutingTable>) {
        let cidr = Ipv4Cidr::new(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap();
        routing_table.add_cidr(cidr);

        let test_cases = [
            Ipv4Addr::new(0, 0, 0, 0),
            Ipv4Addr::new(0, 0, 0, 0),
            Ipv4Addr::new(0, 0, 0, 0),
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(192, 168, 200, 4),
            Ipv4Addr::new(192, 168, 200, 8),
            Ipv4Addr::new(169, 254, 0, 0),
            Ipv4Addr::new(127, 0, 0, 0),
            Ipv4Addr::new(100, 64, 0, 0),
        ];

        for addr in test_cases {
            let result = routing_table.find_exact_cidr(addr);

            assert_eq!(
                result,
                Some(cidr),
                "we expect global cidr to be always resolvable"
            );
        }

        assert_eq!(1, routing_table.size());
    }

    pub fn simple_test(mut routing_table: Box<dyn RoutingTable>) {
        routing_table.add_cidr(Ipv4Cidr::new_host(Ipv4Addr::new(127, 0, 0, 1)));
        routing_table.add_cidr(Ipv4Cidr::new_host(Ipv4Addr::new(127, 0, 0, 1)));
        routing_table.add_cidr(Ipv4Cidr::new_host(Ipv4Addr::new(192, 168, 0, 1)));

        routing_table.remove_cidr(Ipv4Cidr::new_host(Ipv4Addr::new(127, 0, 0, 1)));

        let test_cases = [
            (Ipv4Addr::new(0, 0, 0, 0), None),
            (Ipv4Addr::new(0, 0, 0, 0), None),
            (Ipv4Addr::new(0, 0, 0, 0), None),
            (Ipv4Addr::new(192, 168, 0, 0), None),
            (Ipv4Addr::new(192, 168, 0, 0), None),
            (Ipv4Addr::new(192, 168, 200, 4), None),
            (Ipv4Addr::new(192, 168, 200, 8), None),
            (Ipv4Addr::new(169, 254, 0, 0), None),
            (Ipv4Addr::new(127, 0, 0, 0), None),
            (Ipv4Addr::new(127, 0, 0, 1), None),
            (Ipv4Addr::new(100, 64, 0, 0), None),
            (
                Ipv4Addr::new(192, 168, 0, 1),
                Some(Ipv4Cidr::new_host(Ipv4Addr::new(192, 168, 0, 1))),
            ),
        ];

        for (addr, expected) in test_cases {
            let result = routing_table.find_exact_cidr(addr);

            assert_eq!(
                result, expected,
                "we find {addr} inside manager and expect result to be {result:?}"
            );
        }
    }

    pub fn complex_test(mut routing_table: Box<dyn RoutingTable>) {
        let cidrs = [
            Ipv4Cidr::new(Ipv4Addr::new(0, 0, 0, 0), 8).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(0, 0, 0, 0), 32).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(192, 168, 0, 0), 16).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(192, 168, 0, 0), 13).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(192, 168, 200, 4), 30).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(192, 168, 200, 8), 30).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(169, 254, 0, 0), 16).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(127, 0, 0, 0), 8).unwrap(),
            Ipv4Cidr::new(Ipv4Addr::new(100, 64, 0, 0), 10).unwrap(),
        ];

        for cidr in cidrs {
            routing_table.add_cidr(cidr);
        }

        let test_cases = [
            (Ipv4Addr::new(0, 0, 0, 0), Some(cidrs[1])),
            (Ipv4Addr::new(0, 0, 0, 1), Some(cidrs[0])),
            (Ipv4Addr::new(1, 0, 0, 0), None),
            (Ipv4Addr::new(192, 168, 200, 4), Some(cidrs[4])),
            (Ipv4Addr::new(192, 168, 200, 5), Some(cidrs[4])),
            (Ipv4Addr::new(192, 168, 200, 6), Some(cidrs[4])),
            (Ipv4Addr::new(192, 168, 200, 7), Some(cidrs[4])),
        ];

        for (addr, expected) in test_cases {
            let result = routing_table.find_exact_cidr(addr);

            assert_eq!(
                result, expected,
                "we find {addr} inside manager and expect result to be {expected:?}"
            );
        }

        assert_eq!(cidrs.len(), routing_table.size());
    }
}
