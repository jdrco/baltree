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
        if self.tree.root.is_none() {
            new_node.borrow_mut().color = Some(NodeColor::Black);
            self.tree.root = Some(new_node);
        } else {
            self.tree.root = Some(Self::insert_node(self.tree.root.clone(), new_node.clone()));
            self.ensure_black_root();
        }
    }

    fn insert_node(root: GenericTree, new_node: Tree) -> Tree {
        match root {
            Some(node) => {
                // BST insert
                {
                    let temp_left = node.borrow().left.clone();
                    let temp_right = node.borrow().right.clone();

                    if new_node.borrow().key < node.borrow().key {
                        let left_tree = RedBlack::insert_node(temp_left, new_node.clone());
                        node.borrow_mut().left = Some(left_tree);
                    } else {
                        let right_tree = RedBlack::insert_node(temp_right, new_node.clone());
                        node.borrow_mut().right = Some(right_tree);
                    }
                }
                // Fix violations
                RedBlack::insert_balance(node)
            }
            None => new_node,
        }
    }

    fn insert_balance(mut node: Tree) -> Tree {
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
    
                    // node = grandparent.unwrap(); // Continue with the grandparent
                },
                _ => { // Uncle is black or null, need to perform rotations
                    if is_parent_left.is_some() {
                        if Rc::ptr_eq(&node, &parent.borrow().right.as_ref().unwrap()) {
                            // Left-Right Case
                            // Placeholder for LEFT-ROTATE(parent)
                            // node = parent.clone(); // Update `node` to be the parent for the next rotation
                        }
                        // Left-Left Case
                        // Placeholder for setting colors and RIGHT-ROTATE(grandparent)
                    } else {
                        if Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap()) {
                            // Right-Left Case
                            // Placeholder for RIGHT-ROTATE(parent)
                            // node = parent.clone(); // Update `node` to be the parent for the next rotation
                        }
                        // Right-Right Case
                        // Placeholder for setting colors and LEFT-ROTATE(grandparent)
                    }
    
                    // After rotation, set colors
                    parent.borrow_mut().color = Some(NodeColor::Black);
                    grandparent.unwrap().borrow_mut().color = Some(NodeColor::Red);
                    
                    // Placeholder for final rotation based on the case
    
                    break; // Exit loop after handling the imbalance
                }
            }
        }
    
        node // Return the potentially modified tree
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
