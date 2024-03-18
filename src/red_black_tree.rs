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
        while let Some(parent_node) = node.borrow().parent.clone() {
            let parent_color = &parent_node.borrow().color;
    
            match parent_color {
                Some(NodeColor::Black) => {
                    // Case 1: Parent is black, so tree is still balanced. Nothing to do here.
                    break;
                },
                Some(NodeColor::Red) => {
                    // Case 2 handling: Parent of new node is red
                        // a. If color is black or null then do suitable rotation and recolor
                        // b. if color is red then recolor and also check if grandparent of new node is not root node then recolor it and recheck
                    
                    break;
                },
                None => {
                    // Should technically never happen in a well-formed Red-Black Tree
                    break;
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
