use crate::balancing_tree::{BalancingTree, GenericTree, Node, NodeColor, Tree};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RedBlack {
    pub tree: BalancingTree,
}

impl RedBlack {
    pub fn new() -> Self {
        RedBlack {
            tree: BalancingTree::new(),
        }
    }

    pub fn insert(&mut self, key: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            key,
            left: None,
            right: None,
            parent: None,
            height: 0,
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
            },
        }

        // Ensure the root remains black (may not be needed depending on your color-fixing logic)
        self.ensure_black_root();
    }
    

    fn insert_balance(node: Tree) -> Tree {
        let mut result_node = node.clone();
        while let Some(parent) = node.borrow().parent.clone() {
            if parent.borrow().color == Some(NodeColor::Black) {
                break; // The tree is already balanced if the parent is black.
            }
    
            let grandparent = parent.borrow().parent.clone();
            let is_parent_left = grandparent.as_ref().and_then(|gp| {
                gp.borrow().left.as_ref().map_or(Some(false), |left| Some(Rc::ptr_eq(left, &parent)))
            });
    
            let uncle = grandparent.as_ref().and_then(|gp| {
                if is_parent_left.is_some() {
                    gp.borrow().right.clone()
                } else {
                    gp.borrow().left.clone()
                }
            });
    
            match uncle {
                Some(ref uncle_node) if uncle_node.borrow().color == Some(NodeColor::Red) => {
                    // Case when uncle is red: recolor parent, uncle, and grandparent
                    parent.borrow_mut().color = Some(NodeColor::Black);
                    uncle_node.borrow_mut().color = Some(NodeColor::Black);
                    grandparent.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);
    
                    result_node = grandparent.unwrap(); // Continue with the grandparent
                },
                _ => { // Uncle is black or null, need to perform rotations
                    if is_parent_left.is_some() {
                        if Rc::ptr_eq(&node, &parent.borrow().right.as_ref().unwrap()) {
                            // Left-Right Case
                            result_node = parent.clone(); // Update `node` to be the parent for the next rotation
                            // Placeholder for LEFT-ROTATE(parent)
                            result_node = BalancingTree::rotate_left(result_node);
                        } else {
                            // Left-Left Case
                            // Placeholder for setting colors and RIGHT-ROTATE(grandparent)
                            result_node = BalancingTree::rotate_right(grandparent.clone().unwrap());
                        }
                    } else {
                        if Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap()) {
                            // Right-Left Case
                            result_node = parent.clone(); // Update `node` to be the parent for the next rotation
                            // Placeholder for RIGHT-ROTATE(parent)
                            result_node = BalancingTree::rotate_right(result_node);
                        } else {
                            // Right-Right Case
                            // Placeholder for setting colors and LEFT-ROTATE(grandparent)
                            result_node = BalancingTree::rotate_left(grandparent.clone().unwrap());
                        }
                    }
    
                    // After rotation, set colors
                    parent.borrow_mut().color = Some(NodeColor::Black);
                    grandparent.unwrap().borrow_mut().color = Some(NodeColor::Red);
                    break; // Exit loop after handling the imbalance
                }
            }
        }
    
        result_node // Return the potentially modified tree
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
            self.print_helper(&Some(right.clone()), space, "Right: ");
        }

        for _ in 10..space {
            print!(" ");
        }
        // Modify this line to include the color of the node
        let node_ref = node.as_ref().unwrap().borrow();
        let color = match node_ref.color {
            Some(NodeColor::Red) => "Red",
            Some(NodeColor::Black) => "Black",
            None => "None",
        };
        println!("{}{} ({})", prefix, node_ref.key, color);

        if let Some(ref left) = node_ref.left {
            self.print_helper(&Some(left.clone()), space, "Left: ");
        }
    }
}
