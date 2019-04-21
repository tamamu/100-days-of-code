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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return None;
        }
        let h = head.unwrap();
        Solution::f(
            Some(Box::new(ListNode {
                val: h.val,
                next: None,
            })),
            h.next,
        )
    }
    pub fn f(h: Option<Box<ListNode>>, t: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if t == None {
            return h;
        }
        let t = t.unwrap();
        if t.next == None {
            Some(Box::new(ListNode {
                val: t.val,
                next: h,
            }))
        } else {
            Solution::f(
                Some(Box::new(ListNode {
                    val: t.val,
                    next: h,
                })),
                t.next,
            )
        }
    }
    // pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut prev = None;
    //     let mut curr = head;

    //     while let Some(mut boxed_node) = curr.take() {
    //         let next = boxed_node.next.take();
    //         boxed_node.next = prev.take();
    //         prev = Some(boxed_node);
    //         curr = next;
    //     }

    //     prev
    // }
}

fn main() {
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    println!("{:?}", Solution::reverse_list(list));
}
