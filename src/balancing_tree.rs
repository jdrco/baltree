use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

pub type Tree = Rc<RefCell<Node>>;
pub type GenericTree = Option<Tree>;

pub struct BalancingTree {
    pub root: GenericTree,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub key: i32,
    pub left: GenericTree,
    pub right: GenericTree,
    pub parent: GenericTree,
    pub height: i32, // For AVL. RBT can ignore or use for extra calculations if needed.
                     // color: Color, // For RBT
}

impl BalancingTree {
    pub fn new() -> Self {
        BalancingTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn print_inorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder_traversal(&self.root, &mut result);
        result
    }

    fn inorder_traversal(node: &GenericTree, result: &mut Vec<i32>) {
        if let Some(ref node) = node {
            Self::inorder_traversal(&node.borrow().left, result);
            result.push(node.borrow().key);
            Self::inorder_traversal(&node.borrow().right, result);
        }
    }

    pub fn rotate_left(node: Tree) -> Tree {
        let right_node = node
            .borrow_mut()
            .right
            .take()
            .expect("Right node must exist for rotation");
        let right_left = right_node.borrow_mut().left.take();

        let node_parent = node.borrow().parent.clone();

        node.borrow_mut().right = right_left.clone();

        if let Some(right_left) = right_left {
            right_left.borrow_mut().parent = Some(node.clone());
        }

        right_node.borrow_mut().left = Some(node.clone());
        right_node.borrow_mut().parent = node_parent.clone();

        // Parent's pointers
        if let Some(parent) = node_parent {
            let mut parent_borrow_mut = parent.borrow_mut();
            if let Some(ref parent_right) = parent_borrow_mut.right {
                if Rc::ptr_eq(&node, parent_right) {
                    parent_borrow_mut.right = Some(right_node.clone());
                }
            } else {
                parent_borrow_mut.left = Some(right_node.clone());
            }
        }

        Self::update_height(&node);
        Self::update_height(&right_node);

        right_node
    }

    pub fn rotate_right(node: Tree) -> Tree {
        let left_node = node
            .borrow_mut()
            .left
            .take()
            .expect("Left node must exist for rotation");
        let left_right = left_node.borrow_mut().right.take();

        let node_parent = node.borrow().parent.clone();

        node.borrow_mut().left = left_right.clone();

        if let Some(left_right) = left_right {
            left_right.borrow_mut().parent = Some(node.clone());
        }

        left_node.borrow_mut().right = Some(node.clone());
        left_node.borrow_mut().parent = node_parent.clone();

        // Parent's pointers
        if let Some(parent) = node_parent {
            let mut parent_borrow_mut = parent.borrow_mut();
            if let Some(ref parent_left) = parent_borrow_mut.left {
                if Rc::ptr_eq(&node, parent_left) {
                    parent_borrow_mut.left = Some(left_node.clone());
                }
            } else {
                parent_borrow_mut.right = Some(left_node.clone());
            }
        }

        Self::update_height(&node);
        Self::update_height(&left_node);

        left_node
    }

    pub fn get_height(&self) -> i32 {
        self.height_helper(&self.root)
    }

    fn height_helper(&self, node: &GenericTree) -> i32 {
        match node {
            Some(node) => {
                1 + max(
                    self.height_helper(&node.borrow().left),
                    self.height_helper(&node.borrow().right),
                )
            }
            None => 0,
        }
    }

    pub fn update_height(node: &Tree) {
        let left_height = node.borrow().left.as_ref().map_or(0, |n| n.borrow().height);
        let right_height = node
            .borrow()
            .right
            .as_ref()
            .map_or(0, |n| n.borrow().height);
        node.borrow_mut().height = 1 + max(left_height, right_height);
    }

    pub fn get_balance(node: &Tree) -> i32 {
        let left_height = node.borrow().left.as_ref().map_or(0, |n| n.borrow().height);
        let right_height = node
            .borrow()
            .right
            .as_ref()
            .map_or(0, |n| n.borrow().height);
        left_height - right_height
    }

    // Count Function
    pub fn count_leaves(&self) -> i32 {
        Self::count_leaves_recursive(&self.root)
    }

    // Helper function to recursively count the leaves
    fn count_leaves_recursive(node: &GenericTree) -> i32 {
        match node {
            Some(node) => {
                let node_borrowed = node.borrow();
                if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                    1 // This node is a leaf
                } else {
                    // Recursively count the leaves in the left and right subtrees and sum them up
                    Self::count_leaves_recursive(&node_borrowed.left)
                        + Self::count_leaves_recursive(&node_borrowed.right)
                }
            }
            None => 0, // If the node is None, it's not a leaf
        }
    }
}