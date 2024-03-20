use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

pub type Tree = Rc<RefCell<Node>>;
pub type GenericTree = Option<Tree>;

pub struct BinarySearchTree {
    pub root: GenericTree,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub key: i32,
    pub left: GenericTree,
    pub right: GenericTree,
    pub parent: GenericTree,
    pub height: i32,
    pub color: Option<NodeColor>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
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

    pub fn print_postorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::postorder_traversal(&self.root, &mut result);
        result
    }

    fn postorder_traversal(node: &GenericTree, result: &mut Vec<i32>) {
        if let Some(ref node) = node {
            Self::postorder_traversal(&node.borrow().left, result);
            Self::postorder_traversal(&node.borrow().right, result);
            result.push(node.borrow().key);
        }
    }

    pub fn print_preorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::preorder_traversal(&self.root, &mut result);
        result
    }

    fn preorder_traversal(node: &GenericTree, result: &mut Vec<i32>) {
        if let Some(ref node) = node {
            result.push(node.borrow().key);
            Self::preorder_traversal(&node.borrow().left, result);
            Self::preorder_traversal(&node.borrow().right, result);
        }
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

    pub fn search(&self, key: i32) -> Option<Tree> {
        let dummy = Node {
            key,
            right: None,
            left: None,
            parent: None,
            height: 1,
            color: Some(NodeColor::Red),
        };
        self.search_node(&self.root, &dummy)
    }

    fn search_node(&self, tree_node: &Option<Tree>, node: &Node) -> Option<Tree> {
        match tree_node {
            Some(sub_tree) => {
                let sub_tree_clone = sub_tree.borrow().clone();
                if sub_tree_clone.key == node.key {
                    Some(sub_tree.clone())
                } else {
                    if sub_tree_clone.key > node.key {
                        self.search_node(&sub_tree_clone.left, node)
                    } else {
                        self.search_node(&sub_tree_clone.right, node)
                    }
                }
            }
            None => None,
        }
    }
}