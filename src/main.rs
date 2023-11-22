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

fn in_order_traversal(node:&Option<Rc<RefCell<TreeNode>>>){
    if let Some(node) =node{
        in_order_traversal(&node.borrow().left);
        println!("{}",node.borrow().value);
        in_order_traversal(&node.borrow().right);
    }
}

fn insert_into_bst(root: &Option<Rc<RefCell<TreeNode>>>, value: i32) -> Option<Rc<RefCell<TreeNode>>> {
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

fn main() {
    let root = TreeNode::new(1);
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_tree_creation(){

    }

    #[test]
    fn test_tree_modification(){

    }

    #[test]
    fn test_in_order_traversal(){

    }
    
}