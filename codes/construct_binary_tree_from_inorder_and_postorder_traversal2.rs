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
use std::ops::Range;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&inorder, 0..inorder.len(), &postorder, 0..postorder.len())
    }
    pub fn helper(
        inorder: &Vec<i32>,
        in_range: Range<usize>,
        postorder: &Vec<i32>,
        post_range: Range<usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_range.start >= in_range.end || post_range.start >= post_range.end {
            return None;
        }
        let root = postorder[post_range.end - 1];
        let mut node = TreeNode::new(root);
        if let Some(inorder_idx) = inorder[in_range.clone()].iter().position(|&x| x == root) {
            let pad = inorder_idx + in_range.start;
            node.left = Solution::helper(
                &inorder,
                in_range.start..in_range.start + inorder_idx,
                &postorder,
                post_range.start..post_range.start + inorder_idx,
            );
            node.right = Solution::helper(
                &inorder,
                pad + 1..in_range.end,
                &postorder,
                post_range.start + inorder_idx..post_range.end - 1,
            );
        }
        Some(Rc::new(RefCell::new(node)))
    }
}

fn main() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    // let inorder = vec![1, 2];
    // let postorder = vec![2, 1];
    println!("{:?}", Solution::build_tree(inorder, postorder));
}
