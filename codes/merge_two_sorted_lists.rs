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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1 == None {
            return l2;
        }
        if l2 == None {
            return l1;
        }
        let mut n1 = l1.unwrap();
        let mut n2 = l2.unwrap();
        if n1.val < n2.val {
            let next = n1.next.take();
            n1.next = Solution::merge_two_lists(Some(n2), next);
            return Some(n1);
        } else {
            let next = n2.next.take();
            n2.next = Solution::merge_two_lists(Some(n1), next);
            return Some(n2);
        }
    }
}

fn main() {
    let a = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let b = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    println!("{:?}", Solution::merge_two_lists(a, b));
}
