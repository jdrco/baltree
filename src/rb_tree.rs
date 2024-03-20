use crate::bs_tree::{BinarySearchTree, GenericTree, Node, NodeColor, Tree};
use colored::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct RedBlack {
    pub tree: BinarySearchTree,
}

impl RedBlack {
    pub fn new() -> Self {
        RedBlack {
            tree: BinarySearchTree::new(),
        }
    }

    pub fn insert(&mut self, key: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            key,
            left: None,
            right: None,
            parent: None,
            height: 1,
            color: Some(NodeColor::Red), // New nodes are always red in Red-Black Tree
        }));
    
        let mut p_inserted = None; // This will be the parent of the inserted node
        let mut root = self.tree.root.clone(); // Start from the root of the tree
    
        while let Some(current) = root {
            p_inserted = Some(Rc::clone(&current)); // Keep track of the potential parent
            // Scope to limit the duration of borrow
            let next = {
                if key < current.borrow().key {
                    current.borrow().left.clone()
                } else {
                    current.borrow().right.clone()
                }
            };
            root = next;
        }
    
        // Set the parent of the new node
        new_node.borrow_mut().parent = p_inserted.clone();
    
        // Insert the new node into the tree
        match p_inserted {
            None => {
                // Tree was empty, this node becomes root
                new_node.borrow_mut().color = Some(NodeColor::Black); // Make root black
                self.tree.root = Some(new_node);
            },
            Some(parent) => {
                if key < parent.borrow().key {
                    parent.borrow_mut().left = Some(new_node.clone());
                } else {
                    parent.borrow_mut().right = Some(new_node.clone());
                }
                self.tree.root = self.insert_fixup(new_node.clone());
            },
        }
        self.ensure_black_root();
    }
    
    fn insert_fixup(&mut self, curr_node: Tree) -> GenericTree {
        let mut node = curr_node.clone();
        while curr_node.borrow().parent.clone().is_some() {
            let mut parent = curr_node.borrow().parent.as_ref().unwrap().clone();
            if parent.borrow().color == Some(NodeColor::Black) {
                break;
            }

            let grandparent = parent.borrow().parent.clone();
            let is_parent_left = if let Some(grandparent_ref) = grandparent.as_ref() {
                if let Some(left_child_ref) = grandparent_ref.borrow().left.as_ref() {
                    left_child_ref.borrow().key == parent.borrow().key
                } else {
                    false // If the left child does not exist, the parent cannot be the left child
                }
            } else {
                false // If the grandparent does not exist, the parent cannot be the left child
            };
            
            let uncle = if let Some(grandparent_ref) = grandparent.as_ref() {
                if is_parent_left {
                    grandparent_ref.borrow().right.clone()
                } else {
                    grandparent_ref.borrow().left.clone()
                }
            } else {
                None
            };
            
            match uncle {
                Some(uncle_node) if uncle_node.borrow().color == Some(NodeColor::Red) => {
                    parent.borrow_mut().color = Some(NodeColor::Black);
                    uncle_node.borrow_mut().color = Some(NodeColor::Black);
                    grandparent.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);
                    node = grandparent.unwrap();
                },
                _ => {
                    if is_parent_left {
                        if parent.borrow().clone().key < node.borrow().clone().key {
                            let parent_tmp = node.borrow().parent.as_ref().unwrap().clone();
                            node = parent_tmp;
                            Self::rotate_left(node.clone());
                            parent = node.borrow().parent.as_ref().unwrap().clone();
                        }
                        parent.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);
                        parent.borrow_mut().color = Some(NodeColor::Black);
                        let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                        Self::rotate_right(grandparent);
                    } else {
                        if parent.borrow().clone().key > node.borrow().clone().key {
                            // TODO: Refactor
                            let parent_tmp = node.borrow().parent.as_ref().unwrap().clone();
                            node = parent_tmp;
                            Self::rotate_right(node.clone());
                            parent = node.borrow().parent.as_ref().unwrap().clone();
                        }
                        parent.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);
                        parent.borrow_mut().color = Some(NodeColor::Black);
                        let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                        Self::rotate_left(grandparent);
                    }
                    break;
                }
            }
        };
        self.find_root(node)
    }

    fn find_root(&self, node: Tree) -> GenericTree {
        match node.borrow().parent {
            Some(ref parent) => self.find_root(parent.clone()),
            None => Some(node.clone()),
        }
    }

    fn ensure_black_root(&mut self) {
        if let Some(ref root) = self.tree.root {
            root.borrow_mut().color = Some(NodeColor::Black);
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
            self.print_helper(&Some(right.clone()), space, "R: ");
        }

        for _ in 10..space {
            print!(" ");
        }
        // Modify this line to include the color of the node
        let node_ref = node.as_ref().unwrap().borrow();
        match node_ref.color {
            Some(NodeColor::Red) => {
                println!("{}{}", prefix.red(), node_ref.key.to_string().red())
            }
            Some(NodeColor::Black) => {
                println!("{}{}", prefix.black(), node_ref.key.to_string().black())
            }
            None => {} // Handles the case where the color is None
        };

        if let Some(ref left) = node_ref.left {
            self.print_helper(&Some(left.clone()), space, "L: ");
        }
    }

    fn rotate_left(parent: Tree) {
        let right_child = parent.borrow().right.clone();

        match right_child {
            Some(ref right_child) => {
                parent.borrow_mut().right = right_child.borrow().left.clone();
                right_child.borrow_mut().parent = parent.borrow().clone().parent;
                if right_child.borrow().left.is_some() {
                    // make right_child's left child's parent the current parent
                    let right_left_child = right_child.borrow().left.clone();
                    right_left_child.unwrap().borrow_mut().parent = Some(parent.clone());
                }
            },
            None => {
                parent.borrow_mut().right = None;
            }
        }

        let grandparent = parent.borrow().clone().parent;

        match grandparent {
            Some(ref grandparent) => {
                if grandparent.borrow().clone().key < parent.borrow().clone().key {
                    grandparent.borrow_mut().right = right_child.clone();
                } else {
                    grandparent.borrow_mut().left = right_child.clone();
                }
            },
            None => {},
        }

        right_child.as_ref().unwrap().borrow_mut().left = Some(parent.clone());
        parent.borrow_mut().parent = right_child.clone();

    }

    fn rotate_right(parent: Tree) {
        let left_child = parent.borrow().left.clone();

        match left_child {
            Some(ref left_child) => {
                parent.borrow_mut().left = left_child.borrow().right.clone();
                left_child.borrow_mut().parent = parent.borrow().clone().parent;
                if left_child.borrow().right.is_some() {
                    let right_left_child = left_child.borrow().right.clone();
                    right_left_child.unwrap().borrow_mut().parent = Some(parent.clone());
                }
            },
            None => {
                parent.borrow_mut().left = None;
            }
        }

        let grandparent = parent.borrow().clone().parent;

        match grandparent {
            Some(grandparent) => {
                if grandparent.borrow().clone().key < parent.borrow().clone().key {
                    grandparent.borrow_mut().right = left_child.clone();
                } else {
                    grandparent.borrow_mut().left = left_child.clone();
                }
            },
            None => {},
        }
        
        left_child.as_ref().unwrap().borrow_mut().right = Some(parent.clone());
        parent.borrow_mut().parent = left_child.clone();
    }
}