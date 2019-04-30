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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            None
        } else {
            let mut postorder = postorder;
            let val = postorder.pop().unwrap();
            let mut root = TreeNode::new(val);
            if let Some(inorder_idx) = inorder.iter().position(|&x| x == root.val) {
                root.right = Solution::build_tree(
                    inorder.clone()[inorder_idx + 1..].to_vec(),
                    postorder.clone()[inorder_idx..].to_vec(),
                );
                root.left = Solution::build_tree(
                    inorder.clone()[..inorder_idx].to_vec(),
                    postorder.clone()[..inorder_idx].to_vec(),
                );
            }
            Some(Rc::new(RefCell::new(root)))
        }
    }
}

fn main() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    println!("{:?}", Solution::build_tree(inorder, postorder));
}
