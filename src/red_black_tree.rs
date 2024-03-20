pub mod red_black_tree {
    use crate::balancing_tree::{BinarySearchTree, GenericTree, Node, NodeColor, Tree};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct RedBlack {
        pub tree: BinarySearchTree,
        pub count: i32,
    }
    
    impl RedBlack {
        pub fn new() -> Self {
            RedBlack {
                tree: BinarySearchTree::new(),
                count: 0,
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
            // Ensure the root remains black (may not be needed depending on color-fixing logic)
            self.ensure_black_root();
        }
        
        fn insert_fixup(&mut self, tree_node: Tree) -> GenericTree {
            let mut node = tree_node.clone();
            while tree_node.borrow().parent.clone().is_some() {
                let mut parent = tree_node.borrow().parent.as_ref().unwrap().clone();
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
                    None // If the grandparent does not exist, uncle cannot be determined
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
                                self.rotate_left(node.clone());
                                parent = node.borrow().parent.as_ref().unwrap().clone();
                            }
                            parent.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);
                            parent.borrow_mut().color = Some(NodeColor::Black);
                            let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                            self.rotate_right(grandparent);
                        } else {
                            if parent.borrow().clone().key > node.borrow().clone().key {
                                let parent_tmp = node.borrow().parent.as_ref().unwrap().clone();
                                node = parent_tmp;
                                self.rotate_right(node.clone());
                                parent = node.borrow().parent.as_ref().unwrap().clone();
                            }
                            parent.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);
                            parent.borrow_mut().color = Some(NodeColor::Black);
                            let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                            self.rotate_left(grandparent);
                        }
                        break;
                    }
                }
            };
    
            self.find_root(node)
    
        }
    
        fn find_root(&self, node: Tree) -> GenericTree {
            match node.borrow().parent {
                Some(ref parent) => self.find_root(parent.clone()), // Continue climbing if there's a parent
                None => Some(node.clone()), // Return the node if it has no parent (i.e., it's the root)
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
    
        fn rotate_left(& mut self, parent: Tree) {
            let right_child = parent.borrow().right.clone();
    
            match right_child {
                Some(ref right_child) => {
                    parent.borrow_mut().right = right_child.borrow().left.clone();
                    if right_child.borrow().left.is_some() {
                        // make right_child's left child's parent the current parent
                        let right_left_child = right_child.borrow().left.clone();
                        right_left_child.unwrap().borrow_mut().parent = Some(parent.clone());
                    }
                    // Set parent to grandparent, could be None
                    right_child.borrow_mut().parent = parent.borrow().clone().parent;
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
                None => {
                    self.tree.root = right_child.clone();
                },
            }
    
            // set the right_child's left child to parent
            right_child.as_ref().unwrap().borrow_mut().left = Some(parent.clone());
            parent.borrow_mut().parent = right_child.clone();
    
        }
    
        fn rotate_right(&mut self, parent: Tree) {
            let left_child = parent.borrow().left.clone();
    
            match left_child {
                Some(ref left_child) => {
                    parent.borrow_mut().left = left_child.borrow().right.clone();
                    if left_child.borrow().right.is_some() {
                        // make left_child's right child's parent the current parent
                        let right_left_child = left_child.borrow().right.clone();
                        right_left_child.unwrap().borrow_mut().parent = Some(parent.clone());
                    }
    
                    // Set parent to grandparent, could be None
                    left_child.borrow_mut().parent = parent.borrow().clone().parent;
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
                None => {
                    self.tree.root = left_child.clone();
                },
            }
            
            // set the left_child's right child to parent
            left_child.as_ref().unwrap().borrow_mut().right = Some(parent.clone());
            parent.borrow_mut().parent = left_child.clone();
            print!("Hello")
    
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
                },
                None => {None}
            }
        }
    
    
        // 2- delete a node from the red-black tree
        pub fn delete(&mut self, key: i32) {
            let node_to_be_deleted: Option<Rc<RefCell<Node>>> = self.search(key);
            if let Some(node_to_be_deleted) = node_to_be_deleted {
                
                // Track the parent and children of the node
                let mut parent: Option<Rc<RefCell<Node>>> = node_to_be_deleted.borrow().parent.clone();
                let left_child: Option<Rc<RefCell<Node>>> = node_to_be_deleted.borrow().left.clone();
                let right_child: Option<Rc<RefCell<Node>>> = node_to_be_deleted.borrow().right.clone();
    
                // Save the color of nodeToBeDeleted
                let mut u_original_color: Option<NodeColor> = node_to_be_deleted.borrow().color.clone();
    
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
                    u_original_color = y.as_ref().unwrap().borrow().color.clone();
                    x = y.as_ref().unwrap().borrow().right.clone();
    
                    if y.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key == node_to_be_deleted.borrow().key {
                        if let Some(x) = &x {
                            x.borrow_mut().parent = y.clone();
                        } else {
                            parent = y.clone();
                        }
                    } else {
                        self.transplant(y.clone(), y.as_ref().unwrap().borrow().right.clone());
                        y.as_ref().unwrap().borrow_mut().right = node_to_be_deleted.borrow().right.clone();
                        y.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().parent = y.clone();
                    }
                    self.transplant(Some(node_to_be_deleted.clone()), y.clone());
                    y.as_ref().unwrap().borrow_mut().left = left_child.clone();
                    left_child.as_ref().unwrap().borrow_mut().parent = y.clone();
                    y.as_ref().unwrap().borrow_mut().color = node_to_be_deleted.borrow().color.clone();
                }
                if u_original_color == Some(NodeColor::Black) {
                    self.delete_fix(x.clone(), parent.clone());
                }
            } else {
                println!("Key {} not found", key);
            }
            self.count -= 1;
        }
    
        fn delete_fix(&mut self, x: Option<Tree>, parent: Option<Tree>) {
            // Track the current parent to make it easier to access its data
            let mut cur_p: Option<Rc<RefCell<Node>>> = parent.clone();

            // Track the current x as it changes after rotations
            let mut cur_x: Option<Rc<RefCell<Node>>> = x.clone();
            
            // Track whether x has become the root
            let mut x_is_root = cur_p.is_none();

            // Track when x is black
            let mut x_is_black = self.check_color(cur_x.clone());
            
            // While x is not the root and it is black
            while !x_is_root && x_is_black {
                // Variable to hold sibling of x
                let mut s: Option<Rc<RefCell<Node>>>;

                // Variable to check whether x is left or right child
                let child_is_left:bool;

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
                            self.rotate_left(cur_p.as_ref().unwrap().clone());
                            s = cur_p.as_ref().unwrap().borrow().right.clone();
                        } else {
                            self.rotate_right(cur_p.as_ref().unwrap().clone());
                            s = cur_p.as_ref().unwrap().borrow().left.clone();
                        }
                        
                    }

                    // Children of sibling of x
                    let mut s_left: Option<Rc<RefCell<Node>>> = s.as_ref().unwrap().borrow().clone().left.clone();
                    let mut s_right: Option<Rc<RefCell<Node>>> = s.as_ref().unwrap().borrow().clone().right.clone();

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
                        let grandparent: Option<Rc<RefCell<Node>>> = cur_p.as_ref().unwrap().borrow().clone().parent.clone();
                        cur_p = grandparent.clone();

                        // Update tracking variable after x is reassigned
                        x_is_black = self.check_color(cur_x.clone());
                        x_is_root = cur_p.is_none();

                    } else {
                        if child_is_left {
                            if s_right_is_black {
                                // Set the left sibling to be black as well
                                if s_left.is_some() {
                                    s_left.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);
                                }

                                // Set s to be red
                                s.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);
                                
                                // Rotate s to the right
                                self.rotate_right(s.as_ref().unwrap().clone());
                                
                                // Reassign s to the new right child of x's parent
                                s = cur_p.as_ref().unwrap().borrow().right.clone();
                                s_right = s.as_ref().unwrap().borrow().right.clone();
                                s_left = s.as_ref().unwrap().borrow().left.clone();
                            }
                        } else {
                            if s_left_is_black{
                                // Set right child to black
                                if s_right.is_some() {
                                    s_right.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);
                                }
                                
                                // Set color of s to red
                                s.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Red);

                                // Rotate s to the left and then reassign sibling
                                self.rotate_left(s.as_ref().unwrap().clone());
                                s = cur_p.as_ref().unwrap().borrow().left.clone();
                                s_right = s.as_ref().unwrap().borrow().right.clone();
                                s_left = s.as_ref().unwrap().borrow().left.clone();
                            }
                        }

                        // Set the color of s to the color of x's parent
                        s.as_ref().unwrap().borrow_mut().color = cur_p.as_ref().unwrap().borrow().color.clone();

                        // Set x's parent color to black
                        cur_p.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);

                        if child_is_left {
                            // Set the right child of s to black
                            if s_right.is_some() {
                                s_right.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);
                            }

                            // Rotate the parent left
                            self.rotate_left(cur_p.as_ref().unwrap().clone());
                        } else {
                            // Set the left child of sibling to black
                            if s_left.is_some() {
                                s_left.as_ref().unwrap().borrow_mut().color = Some(NodeColor::Black);
                            }

                            // Rotate the parent to the right
                            self.rotate_right(cur_p.as_ref().unwrap().clone());
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
            // transplant is responsible for deleting the node and replacing it with child
            let u = node_to_be_deleted.unwrap().clone();
            let u_p = u.borrow().parent.clone();
            if u_p.is_none() {
                // deleting root node
                self.tree.root = child.clone();
            } else {
                if u_p.as_ref().unwrap().borrow().clone().key > u.borrow().clone().key {
                    // node_to_be_deleted is on the left of parent
                    u_p.as_ref().unwrap().borrow_mut().left = child.clone();
                } else {
                    // node_to_be_deleted is on the right of parent
                    u_p.as_ref().unwrap().borrow_mut().right = child.clone();
                }
            }
            if child.is_some() {
                // replacement node exists
                child.as_ref().unwrap().borrow_mut().parent = u_p.clone();
            }
        }
    
        fn find_min(&self, tree: Option<Tree>) -> Option<Tree> {
            match tree {
                Some(sub_tree) => {
                    let mut left = Some(sub_tree.clone());
                    while left.as_ref().unwrap().borrow().left.clone().is_some() {
                        left = left.unwrap().borrow().left.clone();
                    }
                    left
                },
                None => {
                    tree
                }
            }
        }
    
        fn find_max(&self, tree: Option<Tree>) -> Option<Tree> {
            match tree {
                Some(sub_tree) => {
                    let mut right = Some(sub_tree.clone());
                    while right.as_ref().unwrap().borrow().right.clone().is_some() {
                        right = right.unwrap().borrow().right.clone();
                    }
                    right
                },
                None => {
                    tree
                }
            }
        }

        fn check_color(&self, tree: Option<Tree>) -> bool {
            tree.as_ref().map_or(true,|node| node.borrow().color == Some(NodeColor::Black))
        }
    }

}

pub use red_black_tree::*;