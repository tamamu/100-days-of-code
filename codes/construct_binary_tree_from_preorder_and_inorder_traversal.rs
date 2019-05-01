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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        let mut preorder = preorder;
        let val = preorder.remove(0);
        let mut root = TreeNode::new(val);
        if let Some(inorder_index) = inorder.iter().position(|&x| x == root.val) {
            root.left = Solution::build_tree(
                preorder.clone()[..inorder_index].to_vec(),
                inorder.clone()[..inorder_index].to_vec(),
            );
            root.right = Solution::build_tree(
                preorder.clone()[inorder_index..].to_vec(),
                inorder.clone()[inorder_index + 1..].to_vec(),
            );
        }
        Some(Rc::new(RefCell::new(root)))
    }
}

fn main() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    println!("{:?}", Solution::build_tree(preorder, inorder));
}
