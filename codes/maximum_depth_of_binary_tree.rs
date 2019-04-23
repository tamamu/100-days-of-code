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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = Solution::max_depth(node.borrow().left.clone());
            let right = Solution::max_depth(node.borrow().right.clone());
            (1 + left).max(1 + right)
        } else {
            0
        }
    }
}

macro_rules! node {
    ($t:tt) => {
        Some(Rc::new(RefCell::new(TreeNode $t)))
    };
}

fn main() {
    let tree = node! ({
        val:3,
        left: node! ({
            val:9,
            left: None,
            right: None
        }),
        right: node!({
            val:20,
            left: node!({
                val:15,
                left: None,
                right: None
            }),
            right: node!({
                val: 7,
                left: None,
                right: None
            })
        })
    });
    println!("{}", Solution::max_depth(tree));
}
