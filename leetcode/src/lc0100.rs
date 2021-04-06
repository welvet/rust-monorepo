use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;

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

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (q, p) {
            (Some(q), Some(p)) => {
                let p_node = p.borrow();
                let q_node = q.borrow();
                p_node.val == q_node.val
                    && Solution::is_same_tree(p_node.left.clone(), q_node.left.clone())
                    && Solution::is_same_tree(p_node.right.clone(), q_node.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

fn create_tree(root_val: i32, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(root_val);
    root.left = Some(Rc::from(RefCell::new(TreeNode::new(left))));
    root.right = Some(Rc::from(RefCell::new(TreeNode::new(right))));
    return Some(Rc::from(RefCell::new(root)));
}

#[test]
fn test() {
    assert_eq!(
        true,
        Solution::is_same_tree(create_tree(1, 2, 3), create_tree(1, 2, 3))
    );
    assert_eq!(
        false,
        Solution::is_same_tree(create_tree(1, 2, 4), create_tree(1, 2, 3))
    );
}
