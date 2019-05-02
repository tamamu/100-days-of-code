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
        Solution::helper(&preorder, &inorder)
    }
    pub fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        let root = preorder[0];
        let mut node = TreeNode::new(root);
        if let Some(inorder_idx) = inorder.iter().position(|&x| x == root) {
            let (_, preorder) = preorder.split_at(1);
            let (pre_left, pre_right) = preorder.split_at(inorder_idx);
            let (in_left, in_right) = inorder.split_at(inorder_idx);
            let (_, in_right) = in_right.split_at(1);
            node.left = Solution::helper(pre_left, in_left);
            node.right = Solution::helper(pre_right, in_right);
        }
        Some(Rc::new(RefCell::new(node)))
    }
}

fn main() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    println!("{:?}", Solution::build_tree(preorder, inorder));
}
