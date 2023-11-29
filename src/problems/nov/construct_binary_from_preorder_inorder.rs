//Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
    let mid = inorder.iter().position(|&x| x == preorder[0]).unwrap();
    let left = build_tree(preorder[1..mid + 1].to_vec(), inorder[0..mid].to_vec());
    let right = build_tree(preorder[mid + 1..].to_vec(), inorder[mid + 1..].to_vec());
    root.borrow_mut().left = left;
    root.borrow_mut().right = right;
    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_tree_from_traversals() {
        // Arrange
        let preorder = vec![1, 2, 4, 5, 3, 6];
        let inorder = vec![4, 2, 5, 1, 6, 3];

        // Act
        let result = build_tree(preorder, inorder);

        // Assert
        assert_eq!(result.unwrap().borrow().val, 1);
    }
    #[test]
    fn test_return_root_node() {
        // Arrange
        let preorder = vec![1, 2, 3];
        let inorder = vec![2, 1, 3];

        // Act
        let result = build_tree(preorder, inorder);

        // Assert
        assert_eq!(result.unwrap().borrow().val, 1);
    }
}
