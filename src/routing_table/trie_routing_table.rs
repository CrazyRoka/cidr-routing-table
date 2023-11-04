use std::{net::Ipv4Addr, ptr};

use crate::{utils::MAX_LENGTH, Ipv4Cidr, RoutingTable};

#[derive(Clone)]
struct TrieNode {
    children: [*mut TrieNode; 2],
    is_leaf: bool,
}

impl TrieNode {
    fn new(is_leaf: bool) -> Self {
        Self {
            children: [ptr::null_mut(), ptr::null_mut()],
            is_leaf,
        }
    }

    #[inline]
    fn get(&self, idx: usize) -> *mut TrieNode {
        self.children[idx]
    }

    #[inline]
    fn get_or_add(&mut self, idx: usize) -> *mut TrieNode {
        if self.children[idx].is_null() {
            self.children[idx] = Box::into_raw(Box::new(TrieNode::new(false)));
        }

        self.children[idx]
    }

    fn mark_leaf(&mut self) {
        self.is_leaf = true;
    }

    fn unmark_leaf(&mut self) {
        self.is_leaf = false;
    }
}

impl Drop for TrieNode {
    fn drop(&mut self) {
        for child in self.children {
            if !child.is_null() {
                unsafe { drop(Box::from_raw(child)) }
            }
        }
    }
}

pub struct TrieRoutingTable {
    root: TrieNode,
    size: usize,
}

impl TrieRoutingTable {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(false),
            size: 0,
        }
    }

    #[inline]
    fn take_bit(&self, bit_addr: u32, r_idx: u8) -> u32 {
        (bit_addr >> (MAX_LENGTH - r_idx)) & 1
    }
}

impl RoutingTable for TrieRoutingTable {
    fn add_cidr(&mut self, cidr: Ipv4Cidr) {
        let bit_addr = u32::from(cidr.min());
        let mut node: *mut TrieNode = &mut self.root;

        for len in 1..=cidr.prefix_len() {
            let bit = self.take_bit(bit_addr, len);
            node = unsafe { (*node).get_or_add(bit as usize) };
        }

        self.size += 1;
        unsafe { (*node).mark_leaf() };
    }

    fn remove_cidr(&mut self, cidr: Ipv4Cidr) {
        let bit_addr = u32::from(cidr.min());
        let mut node: *mut TrieNode = &mut self.root;

        for len in 1..=cidr.prefix_len() {
            let bit = self.take_bit(bit_addr, len);
            node = unsafe { (*node).get(bit as usize) };

            if node.is_null() {
                return;
            }
        }

        self.size -= 1;
        unsafe { (*node).unmark_leaf() };
    }

    fn find_exact_cidr(&self, addr: std::net::Ipv4Addr) -> Option<Ipv4Cidr> {
        let bit_addr = u32::from(addr);
        let mut best_len = if self.root.is_leaf { 0 } else { u8::MAX };
        let mut node: *const TrieNode = &self.root;

        for len in 1..=MAX_LENGTH {
            let bit = self.take_bit(bit_addr, len);

            node = unsafe { (*node).get(bit as usize) };
            if node.is_null() {
                break;
            }

            if unsafe { (*node).is_leaf } {
                best_len = len;
            }
        }

        if best_len == u8::MAX {
            Option::None
        } else {
            let truncated_addr = if best_len == 0 {
                0
            } else {
                bit_addr & !((1 << (MAX_LENGTH - best_len)) - 1)
            };
            Option::Some(Ipv4Cidr::new(Ipv4Addr::from(truncated_addr), best_len).unwrap())
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::TrieRoutingTable;
    use crate::routing_table::tests::{complex_test, empty_test, one_global_cidr, simple_test};

    #[test]
    fn test_hash_empty_case() {
        empty_test(Box::new(TrieRoutingTable::new()));
    }

    #[test]
    fn test_one_global_cidr() {
        one_global_cidr(Box::new(TrieRoutingTable::new()));
    }

    #[test]
    fn test_hash_simple() {
        simple_test(Box::new(TrieRoutingTable::new()));
    }

    #[test]
    fn test_hash_complex() {
        complex_test(Box::new(TrieRoutingTable::new()))
    }
}
