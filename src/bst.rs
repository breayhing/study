use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }
}

/// ## helper function to collect values from tree to a vector
fn in_order_traversal(node: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(node) = node {
        in_order_traversal(&node.borrow().left, vec);
        vec.push(node.borrow().value);
        in_order_traversal(&node.borrow().right, vec);
    }
}

/// ## insert a value into a bst tree
fn insert_into_bst(
    root: &Option<Rc<RefCell<TreeNode>>>,
    value: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root) = root {
        let mut root_borrowed = root.borrow_mut();
        if value < root_borrowed.value {
            root_borrowed.left = insert_into_bst(&root_borrowed.left, value);
        } else if value > root_borrowed.value {
            root_borrowed.right = insert_into_bst(&root_borrowed.right, value);
        }
        Some(root.clone())
    } else {
        // create new root and return if no tree exists
        Some(TreeNode::new(value))
    }
}

// Helper function to find the minimum value node in the right subtree
fn min_value_node(node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let mut current = node;
    loop {
        let next_node = { current.borrow().left.clone() };
        if let Some(next) = next_node {
            current = next;
        } else {
            break;
        }
    }
    current
}

fn delete_from_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    value: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match root {
        None => return None,
        Some(root) => root,
    };

    let mut root_borrowed = root.borrow_mut();

    if value < root_borrowed.value {
        root_borrowed.left = delete_from_bst(root_borrowed.left.clone(), value);
    } else if value > root_borrowed.value {
        root_borrowed.right = delete_from_bst(root_borrowed.right.clone(), value);
    } else {
        if root_borrowed.left.is_none() {
            return root_borrowed.right.clone();
        } else if root_borrowed.right.is_none() {
            return root_borrowed.left.clone();
        }

        let min_node = min_value_node(root_borrowed.right.clone().unwrap());
        root_borrowed.value = min_node.borrow().value;
        root_borrowed.right = delete_from_bst(root_borrowed.right.clone(), root_borrowed.value);
    }

    Some(Rc::clone(&root))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_creation() {
        let mut root = None;
        let values = [3, 2, 1, 4, 5, 6];
        for &value in &values {
            root = insert_into_bst(&root, value);
        }
        let mut collected_values = Vec::new();
        in_order_traversal(&root, &mut collected_values);
        assert_eq!(collected_values, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_tree_modification() {
        let mut root = None;
        root = insert_into_bst(&root, 4);
        root = insert_into_bst(&root, 2);
        root = insert_into_bst(&root, 6);
        root = insert_into_bst(&root, 1);
        root = insert_into_bst(&root, 3);
        root = insert_into_bst(&root, 5);

        root = delete_from_bst(root, 4); // Delete the node with value 4
        let mut collected_values = Vec::new();
        in_order_traversal(&root, &mut collected_values);
        assert_eq!(collected_values, vec![1, 2, 3, 5, 6]);
        root = delete_from_bst(root, 3); // Delete the node with value 3
        collected_values.clear();
        in_order_traversal(&root, &mut collected_values);
        assert_eq!(collected_values, vec![1, 2, 5, 6]);
    }

    #[test]
    fn test_in_order_traversal() {
        let mut root = None;
        let values = [3, 2, 1, 4, 5, 6];
        for &value in &values {
            root = insert_into_bst(&root, value);
        }
        let mut collected_values = Vec::new();
        in_order_traversal(&root, &mut collected_values);
        assert_eq!(collected_values, vec![1, 2, 3, 4, 5, 6]);
    }
}
