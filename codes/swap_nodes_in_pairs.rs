// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(node1) = head {
            if let Some(node2) = node1.next.clone() {
                let mut node1 = node1;
                let mut node2 = node2;
                let node3 = node2.next.clone();
                node1.next = Solution::swap_pairs(node3);
                node2.next = Some(node1);
                Some(node2)
            } else {
                Some(node1)
            }
        } else {
            None
        }
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.val)?;
        if let Some(ref node) = self.next {
            write!(f, " -> {}", node);
        }
        Ok(())
    }
}

fn main() {
    let list = Some(Box::new(ListNode {
        next: Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 4 })),
                val: 3,
            })),
            val: 2,
        })),
        val: 1,
    }));
    println!("{}", Solution::swap_pairs(list).unwrap());
}
