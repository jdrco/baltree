use crate::bs_tree::{BinarySearchTree, GenericTree, Node, NodeColor, Tree};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AVL {
    pub tree: BinarySearchTree,
}

impl AVL {
    pub fn new() -> Self {
        AVL {
            tree: BinarySearchTree::new(),
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
        self.search_node(&self.tree.root, &dummy)
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

    pub fn insert(&mut self, key: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            key,
            left: None,
            right: None,
            parent: None,
            height: 1,
            color: Some(NodeColor::Black), // Default color for AVL nodes
        }));
        self.tree.root = Some(AVL::insert_node(self.tree.root.clone(), new_node));
    }

    fn insert_node(root: GenericTree, new_node: Tree) -> Tree {
        match root {
            Some(node) => {
                {
                    let temp_left = node.borrow().left.clone();
                    let temp_right = node.borrow().right.clone();

                    if new_node.borrow().key < node.borrow().key {
                        let left_tree = AVL::insert_node(temp_left, new_node.clone());
                        node.borrow_mut().left = Some(left_tree);
                    } else {
                        let right_tree = AVL::insert_node(temp_right, new_node.clone());
                        node.borrow_mut().right = Some(right_tree);
                    }
                }
                AVL::balance(node)
            }
            None => new_node,
        }
    }

    fn balance(node: Tree) -> Tree {
        BinarySearchTree::update_height(&node);
        let diff = BinarySearchTree::get_balance(&node);
        if diff > 1 {
            if BinarySearchTree::get_balance(&node.borrow().left.as_ref().unwrap()) < 0 {
                let left = node.borrow_mut().left.take().unwrap();
                node.borrow_mut().left = Some(AVL::rotate_left(left));
            }
            return AVL::rotate_right(node);
        } else if diff < -1 {
            if BinarySearchTree::get_balance(&node.borrow().right.as_ref().unwrap()) > 0 {
                let right = node.borrow_mut().right.take().unwrap();
                node.borrow_mut().right = Some(AVL::rotate_right(right));
            }
            return AVL::rotate_left(node);
        }
        node
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

        BinarySearchTree::update_height(&node);
        BinarySearchTree::update_height(&right_node);

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

        BinarySearchTree::update_height(&node);
        BinarySearchTree::update_height(&left_node);

        left_node
    }

    pub fn delete(&mut self, key: i32) {
        self.tree.root = Self::delete_recursive(self.tree.root.take(), key);
    }

    fn delete_recursive(node: GenericTree, key: i32) -> GenericTree {
        if let Some(current_node) = node.clone() {
            let mut current = current_node.borrow_mut();
            if key < current.key {
                current.left = Self::delete_recursive(current.left.take(), key);
            } else if key > current.key {
                current.right = Self::delete_recursive(current.right.take(), key);
            } else {
                // Node with only one child or no child
                if current.left.is_none() {
                    return current.right.take();
                } else if current.right.is_none() {
                    return current.left.take();
                }

                // Node with two children: Get the inorder successor (smallest in the right subtree)
                let temp = Self::min_value_node(current.right.as_ref().unwrap());
                current.key = temp.borrow().key;
                current.right = Self::delete_recursive(current.right.take(), current.key);
            }

            // Existing balance function is called here
            //Some(Self::balance(current_node))
        } else {
            return None;
        }
        Some(Self::balance(node?))
    }

    fn min_value_node(node: &Tree) -> Tree {
        match &node.borrow().left {
            Some(left) => Self::min_value_node(left),
            None => node.clone(),
        }
    }

    pub fn print_structure(&self) {
        self.print_helper(&self.tree.root, 0, "Root: ");
    }

    fn print_helper(&self, node: &GenericTree, space: usize, prefix: &str) {
        if node.is_none() {
            return;
        }
        let space = space + 10;

        if let Some(ref right) = node.as_ref().unwrap().borrow().right {
            self.print_helper(&Some(right.clone()), space, "Right: ");
        }

        for _ in 10..space {
            print!(" ");
        }
        println!("{}{}", prefix, node.as_ref().unwrap().borrow().key);

        if let Some(ref left) = node.as_ref().unwrap().borrow().left {
            self.print_helper(&Some(left.clone()), space, "Left: ");
        }
    }
}
