use crate::bs_tree::{BinarySearchTree, GenericTree, Node, NodeColor, Tree};
use colored::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct RedBlackTree {
    pub tree: BinarySearchTree,
}

impl RedBlackTree {
    pub fn new() -> Self {
        RedBlackTree {
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
            }
            Some(parent) => {
                if key < parent.borrow().key {
                    parent.borrow_mut().left = Some(new_node.clone());
                } else {
                    parent.borrow_mut().right = Some(new_node.clone());
                }
                self.tree.root = self.insert_fixup(new_node.clone());
            }
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
                }
                _ => {
                    if is_parent_left {
                        if parent.borrow().clone().key < node.borrow().clone().key {
                            let parent_node_clone = node.borrow().parent.as_ref().unwrap().clone();
                            node = parent_node_clone;
                            Self::rotate_left(node.clone());
                            let new_parent_node_clone =
                                node.borrow().parent.as_ref().unwrap().clone();
                            parent = new_parent_node_clone;
                        }
                        parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                            Some(NodeColor::Red);
                        parent.borrow_mut().color = Some(NodeColor::Black);
                        let grandparent = node
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .clone();
                        Self::rotate_right(grandparent);
                    } else {
                        if parent.borrow().clone().key > node.borrow().clone().key {
                            let parent_node_clone = node.borrow().parent.as_ref().unwrap().clone();
                            node = parent_node_clone;
                            Self::rotate_right(node.clone());
                            let new_parent_node_clone =
                                node.borrow().parent.as_ref().unwrap().clone();
                            parent = new_parent_node_clone;
                        }
                        parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                            Some(NodeColor::Red);
                        parent.borrow_mut().color = Some(NodeColor::Black);
                        let grandparent = node
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .clone();
                        Self::rotate_left(grandparent);
                    }
                    break;
                }
            }
        }
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

    fn rotate_left(node: Tree) {
        let right_child = node.borrow().right.clone();

        match right_child {
            Some(ref right_child) => {
                node.borrow_mut().right = right_child.borrow().left.clone();
                right_child.borrow_mut().parent = node.borrow().clone().parent;
                if right_child.borrow().left.is_some() {
                    let right_left_child = right_child.borrow().left.clone();
                    right_left_child.unwrap().borrow_mut().parent = Some(node.clone());
                }
            }
            None => {
                node.borrow_mut().right = None;
            }
        }

        let grandparent = node.borrow().clone().parent;

        match grandparent {
            Some(ref grandparent) => {
                if grandparent.borrow().clone().key < node.borrow().clone().key {
                    grandparent.borrow_mut().right = right_child.clone();
                } else {
                    grandparent.borrow_mut().left = right_child.clone();
                }
            }
            None => {}
        }

        right_child.as_ref().unwrap().borrow_mut().left = Some(node.clone());
        node.borrow_mut().parent = right_child.clone();
    }

    fn rotate_right(node: Tree) {
        let left_child = node.borrow().left.clone();

        match left_child {
            Some(ref left_child) => {
                node.borrow_mut().left = left_child.borrow().right.clone();
                left_child.borrow_mut().parent = node.borrow().clone().parent;
                if left_child.borrow().right.is_some() {
                    let right_left_child = left_child.borrow().right.clone();
                    right_left_child.unwrap().borrow_mut().parent = Some(node.clone());
                }
            }
            None => {
                node.borrow_mut().left = None;
            }
        }

        let grandparent = node.borrow().clone().parent;

        match grandparent {
            Some(grandparent) => {
                if grandparent.borrow().clone().key < node.borrow().clone().key {
                    grandparent.borrow_mut().right = left_child.clone();
                } else {
                    grandparent.borrow_mut().left = left_child.clone();
                }
            }
            None => {}
        }

        left_child.as_ref().unwrap().borrow_mut().right = Some(node.clone());
        node.borrow_mut().parent = left_child.clone();
    }

    pub fn delete(&mut self, key: i32) {
        let node_to_be_deleted: Option<Rc<RefCell<Node>>> = self.tree.search(key);
        if let Some(node_to_be_deleted) = node_to_be_deleted {
            // Track the parent and children of the node
            let mut parent: Option<Rc<RefCell<Node>>> = node_to_be_deleted.borrow().parent.clone();
            let left_child: Option<Rc<RefCell<Node>>> = node_to_be_deleted.borrow().left.clone();
            let right_child: Option<Rc<RefCell<Node>>> = node_to_be_deleted.borrow().right.clone();

            // Save the color of nodeToBeDeleted
            let mut u_og_color: Option<NodeColor> = node_to_be_deleted.borrow().color.clone();

            // If left child is None assign it to x
            let x: Option<Rc<RefCell<Node>>>;

            if left_child.is_none() {
                x = right_child.clone();
                self.transplant(Some(node_to_be_deleted.clone()), right_child.clone());
            } else if right_child.is_none() {
                x = left_child.clone();
                self.transplant(Some(node_to_be_deleted.clone()), left_child.clone());
            } else {
                let y = self.find_min(right_child.clone());
                u_og_color = y.as_ref().unwrap().borrow().color.clone();
                x = y.as_ref().unwrap().borrow().right.clone();

                if y.as_ref()
                    .unwrap()
                    .borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .key
                    == node_to_be_deleted.borrow().key
                {
                    if let Some(x) = &x {
                        x.borrow_mut().parent = y.clone();
                    } else {
                        parent = y.clone();
                    }
                } else {
                    self.transplant(y.clone(), y.as_ref().unwrap().borrow().right.clone());
                    y.as_ref().unwrap().borrow_mut().right =
                        node_to_be_deleted.borrow().right.clone();
                    y.as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .parent = y.clone();
                }
                self.transplant(Some(node_to_be_deleted.clone()), y.clone());
                y.as_ref().unwrap().borrow_mut().left = left_child.clone();
                left_child.as_ref().unwrap().borrow_mut().parent = y.clone();
                y.as_ref().unwrap().borrow_mut().color = node_to_be_deleted.borrow().color.clone();
            }
            if u_og_color == Some(NodeColor::Black) {
                self.delete_fix(x.clone(), parent.clone());
            }
        } else {
            println!("Key {} not found", key);
        }
    }

    fn delete_fix(&mut self, x: Option<Tree>, parent: Option<Tree>) {
        // Track the current parent to make it easier to access its data
        let mut cur_p: GenericTree = parent.clone();

        // Track the current x as it changes after rotations
        let mut cur_x: GenericTree = x.clone();

        // Track whether x has become the root
        let mut x_is_root = cur_p.is_none();

        // Track when x is black
        let mut x_is_black = self.check_color(cur_x.clone());

        // While x is not the root and it is black
        while !x_is_root && x_is_black {
            // Variable to hold sibling of x
            let mut s: Option<Rc<RefCell<Node>>>;

            // Variable to check whether x is left or right child
            let child_is_left: bool;

            // Set it to true when we are left child and false for right
            if cur_x == cur_p.as_ref().unwrap().borrow().left {
                child_is_left = true;
            } else {
                child_is_left = false;
            }

            if child_is_left {
                // Sibling assigned as right child of parent of x
                s = cur_p.as_ref().unwrap().borrow().right.clone();
            } else {
                // Sibling assigned as left child of parent of x
                s = cur_p.as_ref().unwrap().borrow().left.clone();
            }

            // Need to check so that we do not get an error
            if s.is_some() {
                // Check if the sibling is red
                let s_is_black = self.check_color(s.clone());
                if !s_is_black {
                    // If yes then set it to black and change the parent to red
                    s.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);
                    cur_p.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);

                    // Rotate the parent left or right depending on whether
                    // x is the left or right child. Then reassign sibling
                    // to new position.
                    if child_is_left {
                        Self::rotate_left(cur_p.as_ref().unwrap().clone());
                        s = cur_p.as_ref().unwrap().borrow().right.clone();
                    } else {
                        Self::rotate_right(cur_p.as_ref().unwrap().clone());
                        s = cur_p.as_ref().unwrap().borrow().left.clone();
                    }
                }

                // Children of sibling of x
                let mut s_left: GenericTree = s.as_ref().unwrap().borrow().clone().left.clone();
                let mut s_right: GenericTree = s.as_ref().unwrap().borrow().clone().right.clone();

                // Check the color of both children of sibling
                let s_left_is_black = self.check_color(s_left.clone());
                let s_right_is_black = self.check_color(s_right.clone());

                // If both are black we are either at a leaf or the root
                if s_left_is_black && s_right_is_black {
                    // Set the sibling to Red since both children are black
                    s.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);

                    // Store x's parent in x
                    cur_x = cur_p.clone();

                    // Need to track x's grandparent as its parent after reassignment
                    let grandparent: GenericTree =
                        cur_p.as_ref().unwrap().borrow().clone().parent.clone();
                    cur_p = grandparent.clone();

                    // Update tracking variable after x is reassigned
                    x_is_black = self.check_color(cur_x.clone());
                    x_is_root = cur_p.is_none();
                } else {
                    if child_is_left {
                        if s_right_is_black {
                            // Set the left sibling to be black as well
                            if s_left.is_some() {
                                s_left.as_ref().unwrap().borrow_mut().color =
                                    Some(NodeColor::Black);
                            }

                            // Set s to be red
                            s.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);

                            // Rotate s to the right
                            Self::rotate_right(s.as_ref().unwrap().clone());

                            // Reassign s to the new right child of x's parent
                            s = cur_p.as_ref().unwrap().borrow().right.clone();
                            s_right = s.as_ref().unwrap().borrow().right.clone();
                            s_left = s.as_ref().unwrap().borrow().left.clone();
                        }
                    } else {
                        if s_left_is_black {
                            // Set right child to black
                            if s_right.is_some() {
                                s_right.as_ref().unwrap().borrow_mut().color =
                                    Some(NodeColor::Black);
                            }

                            // Set color of s to red
                            s.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);

                            // Rotate s to the left and then reassign sibling
                            Self::rotate_left(s.as_ref().unwrap().clone());
                            s = cur_p.as_ref().unwrap().borrow().left.clone();
                            s_right = s.as_ref().unwrap().borrow().right.clone();
                            s_left = s.as_ref().unwrap().borrow().left.clone();
                        }
                    }

                    // Set the color of s to the color of x's parent
                    s.as_ref().unwrap().borrow_mut().color =
                        cur_p.as_ref().unwrap().borrow().color.clone();

                    // Set x's parent color to black
                    cur_p.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);

                    if child_is_left {
                        // Set the right child of s to black
                        if s_right.is_some() {
                            s_right.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);
                        }

                        // Rotate the parent left
                        Self::rotate_left(cur_p.as_ref().unwrap().clone());
                    } else {
                        // Set the left child of sibling to black
                        if s_left.is_some() {
                            s_left.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);
                        }

                        // Rotate the parent to the right
                        Self::rotate_right(cur_p.as_ref().unwrap().clone());
                    }

                    // We are at the root
                    x_is_root = true;
                }
            }
        }

        // Set x to black once loop breaks in case it is root
        if let Some(cur_x) = x {
            cur_x.borrow_mut().color = Some(NodeColor::Black);
        }
    }

    fn transplant(&mut self, node_to_be_deleted: Option<Tree>, child: Option<Tree>) {
        let node = match node_to_be_deleted {
            Some(node) => node.clone(),
            None => return,
        };
        let parent_node = node.borrow().parent.clone();

        match parent_node {
            Some(ref parent) => {
                if parent.borrow().key > node.borrow().key {
                    parent.borrow_mut().left = child.clone();
                } else {
                    parent.borrow_mut().right = child.clone();
                }
            }
            None => self.tree.root = child.clone(),
        }

        if let Some(ref child_node) = child {
            child_node.borrow_mut().parent = parent_node;
        }
    }

    fn find_min(&self, tree: Option<Tree>) -> Option<Tree> {
        let mut current = tree;
        while current
            .as_ref()
            .map_or(false, |n| n.borrow().left.is_some())
        {
            current = current.unwrap().borrow().left.clone();
        }
        current
    }

    fn check_color(&self, tree: Option<Tree>) -> bool {
        tree.as_ref()
            .map_or(true, |node| node.borrow().color == Some(NodeColor::Black))
    }
}
