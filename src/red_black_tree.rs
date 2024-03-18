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
        self.tree.root = Some(RedBlack::insert_node(self.tree.root.clone(), new_node.clone()));
    }

    fn insert_node(root: GenericTree, new_node: Tree) -> Tree {
        match root {
            Some(node) => {
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
                // RedBlack::balance(node)
                node
            }
            None => new_node,
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
