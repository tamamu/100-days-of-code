#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut result = Vec::new();
        let mut q: VecDeque<(usize, Rc<RefCell<TreeNode>>)> = VecDeque::new();
        if let Some(node) = root {
            q.push_front((0, node));
        }
        while let Some((l, node)) = q.pop_front() {
            if result.len() == l {
                result.push(Vec::new());
            }
            let borrowed = node.borrow();
            if let Some(ref left) = borrowed.left {
                q.push_back((l + 1, left.clone()));
            }
            if let Some(ref right) = borrowed.right {
                q.push_back((l + 1, right.clone()));
            }
            result[l].push(borrowed.val);
        }
        result
    }
}
fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    println!("{:?}", Solution::level_order(tree));
}
