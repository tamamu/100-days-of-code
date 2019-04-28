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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let borrowed = node.borrow();
            let mut left = Solution::postorder_traversal(borrowed.left.clone());
            let mut right = Solution::postorder_traversal(borrowed.right.clone());
            for v in left.drain(0..) {
                result.push(v);
            }
            for v in right.drain(0..) {
                result.push(v);
            }
            result.push(borrowed.val);
        }
        result
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: None,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));
    println!("{:?}", Solution::postorder_traversal(tree));
}
