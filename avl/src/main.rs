use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

type Tree = Rc<RefCell<Node>>;
type AVLTree = Option<Tree>;

struct AVL {
    root: AVLTree,
}

#[derive(Clone, Debug, PartialEq)]
struct Node {
    key: i32,
    left: AVLTree,
    right: AVLTree,
    parent: AVLTree,
    height: i32,
}

impl AVL {
    pub fn new() -> Self {
        AVL { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn print_inorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder_traversal(&self.root, &mut result);
        result
    }

    fn inorder_traversal(node: &AVLTree, result: &mut Vec<i32>) {
        if let Some(ref node) = node {
            Self::inorder_traversal(&node.borrow().left, result);
            result.push(node.borrow().key);
            Self::inorder_traversal(&node.borrow().right, result);
        }
    }

    pub fn search(&self, key: i32) -> bool {
        Self::search_recursive(&self.root, key)
    }

    fn search_recursive(node: &AVLTree, key: i32) -> bool {
        match node {
            Some(node) => {
                let node_borrowed = node.borrow();
                if key == node_borrowed.key {
                    true
                } else if key < node_borrowed.key {
                    Self::search_recursive(&node_borrowed.left, key)
                } else {
                    Self::search_recursive(&node_borrowed.right, key)
                }
            }
            None => false,
        }
    }

    pub fn insert(&mut self, key: i32) {
        if Self::search(&self, key) {
            println!("Key already exists");
            return;
        }

        let new_node = Rc::new(RefCell::new(Node {
            key,
            left: None,
            right: None,
            parent: None,
            height: 1,
        }));
        self.root = Some(Self::insert_node(self.root.clone(), new_node));
    }

    fn insert_node(root: AVLTree, new_node: Tree) -> Tree {
        match root {
            Some(node) => {
                {
                    let temp_left = node.borrow().left.clone();
                    let temp_right = node.borrow().right.clone();

                    if new_node.borrow().key < node.borrow().key {
                        let left_tree = Self::insert_node(temp_left, new_node.clone());
                        node.borrow_mut().left = Some(left_tree.clone());
                        left_tree.borrow_mut().parent = Some(node.clone());
                    } else {
                        let right_tree = Self::insert_node(temp_right, new_node.clone());
                        node.borrow_mut().right = Some(right_tree.clone());
                        right_tree.borrow_mut().parent = Some(node.clone());
                    }
                }

                Self::balance(node)
            }
            None => new_node,
        }
    }

    fn balance(node: Tree) -> Tree {
        Self::update_height(&node);

        let diff = Self::get_balance(&node);

        if diff > 1 {
            if Self::get_balance(&node.borrow().left.as_ref().unwrap()) < 0 {
                let left = node.borrow_mut().left.take().unwrap();
                node.borrow_mut().left = Some(Self::rotate_left(left));
            }
            Self::rotate_right(node)
        } else if diff < -1 {
            if Self::get_balance(&node.borrow().right.as_ref().unwrap()) > 0 {
                let right = node.borrow_mut().right.take().unwrap();
                node.borrow_mut().right = Some(Self::rotate_right(right));
            }
            Self::rotate_left(node)
        } else {
            node
        }
    }

    fn rotate_left(node: Tree) -> Tree {
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

        Self::update_height(&node);
        Self::update_height(&right_node);

        right_node
    }

    fn rotate_right(node: Tree) -> Tree {
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

        Self::update_height(&node);
        Self::update_height(&left_node);

        left_node
    }

    pub fn get_height(&self) -> i32 {
        self.height_helper(&self.root)
    }

    fn height_helper(&self, node: &AVLTree) -> i32 {
        match node {
            Some(node) => {
                1 + max(
                    self.height_helper(&node.borrow().left),
                    self.height_helper(&node.borrow().right),
                )
            }
            None => 0,
        }
    }

    fn update_height(node: &Tree) {
        let left_height = node.borrow().left.as_ref().map_or(0, |n| n.borrow().height);
        let right_height = node
            .borrow()
            .right
            .as_ref()
            .map_or(0, |n| n.borrow().height);
        node.borrow_mut().height = 1 + max(left_height, right_height);
    }

    fn get_balance(node: &Tree) -> i32 {
        let left_height = node.borrow().left.as_ref().map_or(0, |n| n.borrow().height);
        let right_height = node
            .borrow()
            .right
            .as_ref()
            .map_or(0, |n| n.borrow().height);
        left_height - right_height
    }

    pub fn pretty_print(&self) {
        self.print_helper(&self.root, 0, "Root: ");
    }

    fn print_helper(&self, node: &AVLTree, space: usize, prefix: &str) {
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
        println!("{}{}", prefix, node.as_ref().unwrap().borrow().key);

        if let Some(ref left) = node.as_ref().unwrap().borrow().left {
            self.print_helper(&Some(left.clone()), space, "L: ");
        }
    }

    // Count Function
    pub fn count_leaves(&self) -> i32 {
        Self::count_leaves_recursive(&self.root)
    }

    // Helper function to recursively count the leaves
    fn count_leaves_recursive(node: &AVLTree) -> i32 {
        match node {
            Some(node) => {
                let node_borrowed = node.borrow();
                if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                    1 // This node is a leaf
                } else {
                    // Recursively count the leaves in the left and right subtrees and sum them up
                    Self::count_leaves_recursive(&node_borrowed.left)
                        + Self::count_leaves_recursive(&node_borrowed.right)
                }
            }
            None => 0, // If the node is None, it's not a leaf
        }
    }

    pub fn delete(&mut self, key: i32) {
        self.root = Self::delete_recursive(self.root.take(), key);
    }

    fn delete_recursive(node: AVLTree, key: i32) -> AVLTree {
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

            // Your existing balance function is called here
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
}

fn main() {
    let mut avl = AVL::new();
    avl.insert(10);
    avl.insert(20);
    avl.insert(30);
    avl.insert(40);
    avl.insert(50);
    avl.insert(25);
    avl.insert(29);
    avl.insert(27);
    avl.insert(27);
    avl.delete(19);

    println!("{:?}", avl.print_inorder());
    //avl.pretty_print();
    println!("Height: {}", avl.get_height());
    avl.pretty_print();
    println!("{:?}", avl.count_leaves());
}
