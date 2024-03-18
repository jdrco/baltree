use crate::balancing_tree::{BalancingTree, GenericTree, Node, Tree, NodeColor};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RedBlackTree {
    pub tree: BalancingTree,
}

impl RedBlackTree {
    // RBT-specific methods
    pub fn new() -> Self {
        RedBlackTree {
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
            color: NodeColor::Red,
        }));
        self.tree.root = Some(RedBlackTree::insert_node(self.tree.root.clone(), new_node));
    }

    fn insert_node(root: GenericTree, new_node: Tree) -> Tree {
        match root {
            Some(node) => {
                {
                    let temp_left = node.borrow().left.clone();
                    let temp_right = node.borrow().right.clone();

                    if new_node.borrow().key < node.borrow().key {
                        let left_tree = RedBlackTree::insert_node(temp_left, new_node.clone());
                        node.borrow_mut().left = Some(left_tree.clone());
                        left_tree.borrow_mut().parent = Some(node.clone());
                    } else {
                        let right_tree = RedBlackTree::insert_node(temp_right, new_node.clone());
                        node.borrow_mut().right = Some(right_tree.clone());
                        right_tree.borrow_mut().parent = Some(node.clone());
                    }
                }
                RedBlackTree::recolor(node, new_node.borrow().key)
            }
            None => new_node,
        }
    }

    fn recolor(p_inserted_node: Tree, inserted_val: i32) -> Tree {
        {
            // If x is the root, change its color to black
            {
                if p_inserted_node.borrow_mut().parent.is_none() {
                    p_inserted_node.borrow_mut().color = NodeColor::Black;
                }
            }
    
            // If x's parent is not black and x is not the root
            if let Some(parent_rc) = p_inserted_node.borrow_mut().parent.clone() {
                let mut borrowed_parent = parent_rc.borrow_mut();

                if borrowed_parent.color != NodeColor::Black {
                    // Find the grandparent
                    let grandparent_rc: Rc<RefCell<Node>> = borrowed_parent.parent.clone().expect("Grandparent should exist");
    
                    // Separate scope for mutable borrow of grandparent
                    {
                        let mut borrowed_grandparent = grandparent_rc.borrow_mut();
    
                        // Find the uncle
                        let uncle_rc = if let Some(grandparent) = borrowed_grandparent.parent.clone() {
                            if grandparent.borrow().left.as_ref() == Some(&parent_rc) {
                                grandparent.borrow().right.clone()
                            } else {
                                grandparent.borrow().left.clone()
                            }
                        } else {
                            None
                        };
    
                        if let Some(uncle) = uncle_rc.clone() {
                            // If uncle is red
                            if uncle.borrow().color == NodeColor::Red {
                                // (i) Change the color of parent and uncle as black
                                borrowed_parent.color = NodeColor::Black;
                                uncle.borrow_mut().color = NodeColor::Black;
    
                                // (ii) Color of grandparent as red
                                borrowed_grandparent.color = NodeColor::Red;
    
                                // (iii) Change x = x's grandparent, repeat steps 2 and 3 for new x
                                return RedBlackTree::recolor(grandparent_rc.clone(), inserted_val);
                            } else {
                                Self::rotate_node_case(p_inserted_node.clone());
                            }
                        }
                    }
                }
            }
        }
    
        // No further modifications needed, return the unchanged node
        p_inserted_node
    }
    
    // Right rotation of grandparent and swap colors of gp and parent
    fn ll_case(gp_node: Tree) -> Tree {
        let gp_color: NodeColor = gp_node.borrow().color.clone();
        let p_color: NodeColor = gp_node.borrow().left.as_ref().expect("Left parent node should exist").borrow().color.clone();

        {
            gp_node.borrow_mut().color = p_color;
        }

        {
            gp_node.borrow_mut().left.as_mut().expect("Left parent node should exist").borrow_mut().color = gp_color;
        }

        BalancingTree::rotate_right(gp_node.clone())
        
    }
    
    // Left rotation of parent node. Then right rotation of grandparent, swapping colours between its left node and itself.
    fn lr_case(gp_node: Tree) -> Tree {
        {
            if let Some(p_node) = gp_node.borrow_mut().left.clone() {
                BalancingTree::rotate_left(p_node);
            }
          
        }

        RedBlackTree::ll_case(gp_node.clone())

    }

    // Left rotation of grandparent then swap colors of parent and gp.
    fn rr_case(gp_node: Tree) -> Tree {
        let gp_color: NodeColor = gp_node.borrow().color.clone();
        let p_color: NodeColor = gp_node.borrow().right.as_ref().expect("Right parent node should exist").borrow().color.clone();

        {
            gp_node.borrow_mut().color = p_color;
        }

        {
            gp_node.borrow_mut().right.as_mut().expect("Right parent node should exist").borrow_mut().color = gp_color;
        }

        BalancingTree::rotate_left(gp_node.clone())

    }

    fn rl_case(gp_node: Tree) -> Tree {
        {
            if let Some(p_node) = gp_node.borrow_mut().right.clone() {
                BalancingTree::rotate_left(p_node);
            }
        }

        RedBlackTree::rr_case(gp_node.clone())
    }

    fn rotate_node_case(x: Tree) {
        if let Some(parent_rc) = x.borrow().parent.clone() {
            if let Some(grandparent_rc) = parent_rc.borrow().parent.clone() {
                // Check the orientation of x, p, and g
                if let Some(parent_left) = grandparent_rc.borrow().left.clone() {
                    if let Some(x_left) = parent_left.borrow().left.clone() {
                        if Rc::ptr_eq(&x, &x_left) {
                            // LL Case
                            RedBlackTree::ll_case(grandparent_rc.clone());
                        } else {
                            // LR Case
                            RedBlackTree::lr_case(grandparent_rc.clone());
                        }
                    } else {
                        // LR Case
                        RedBlackTree::lr_case(grandparent_rc.clone());
                    }
                } else {
                    if let Some(parent_right) = grandparent_rc.borrow().right.clone() {
                        if let Some(x_right) = parent_right.borrow().right.clone() {
                            if Rc::ptr_eq(&x, &x_right) {
                                // RR Case
                                RedBlackTree::rr_case(grandparent_rc.clone());
                            } else {
                                // RL Case
                                RedBlackTree::rl_case(grandparent_rc.clone());
                            }
                        } else {
                            // RL Case
                            RedBlackTree::rl_case(grandparent_rc.clone());
                        }
                    } else {
                        panic!("Invalid tree structure: No parent's left or right child.");
                    }
                };
            }
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
            NodeColor::Red => "Red",
            NodeColor::Black => "Black",
            NodeColor::NoColor => "None",
        };
        println!("{}{} ({})", prefix, node_ref.key, color);

        if let Some(ref left) = node_ref.left {
            self.print_helper(&Some(left.clone()), space, "Left: ");
        }
    }

}
