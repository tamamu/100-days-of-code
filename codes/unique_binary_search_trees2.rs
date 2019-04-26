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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            vec![]
        } else {
            Solution::generate(1, n)
        }
    }
    pub fn generate(s: i32, e: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if s > e {
            vec![None]
        } else {
            let mut res = Vec::new();
            for root in s..e + 1 {
                for left in Solution::generate(s, root - 1).iter() {
                    for right in Solution::generate(root + 1, e).iter() {
                        let new_tree = Some(Rc::new(RefCell::new(TreeNode {
                            val: root,
                            left: left.clone(),
                            right: right.clone(),
                        })));
                        res.push(new_tree);
                    }
                }
            }
            res
        }
    }
}

fn main() {
    println!("{:?}", Solution::generate_trees(3));
}
