use crate::balancing_tree::{BalancingTree, GenericTree, Node, NodeColor, Tree};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AVL {
    pub tree: BalancingTree,
}

impl AVL {
    pub fn new() -> Self {
        AVL {
            tree: BalancingTree::new(),
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
        BalancingTree::update_height(&node);
        let diff = BalancingTree::get_balance(&node);
        if diff > 1 {
            if BalancingTree::get_balance(&node.borrow().left.as_ref().unwrap()) < 0 {
                let left = node.borrow_mut().left.take().unwrap();
                node.borrow_mut().left = Some(BalancingTree::rotate_left(left));
            }
            return BalancingTree::rotate_right(node);
        } else if diff < -1 {
            if BalancingTree::get_balance(&node.borrow().right.as_ref().unwrap()) > 0 {
                let right = node.borrow_mut().right.take().unwrap();
                node.borrow_mut().right = Some(BalancingTree::rotate_right(right));
            }
            return BalancingTree::rotate_left(node);
        }
        node
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
