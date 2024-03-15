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

    pub fn insert(&mut self, key: i32) {
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
                // Separate scope to ensure the mutable borrow ends before balance is called
                {
                    // Temporarily take the left or right child to avoid multiple mutable borrows
                    let temp_left = node.borrow().left.clone();
                    let temp_right = node.borrow().right.clone();

                    if new_node.borrow().key < node.borrow().key {
                        let left_tree = Self::insert_node(temp_left, new_node.clone());
                        node.borrow_mut().left = Some(left_tree);
                    } else {
                        let right_tree = Self::insert_node(temp_right, new_node.clone());
                        node.borrow_mut().right = Some(right_tree);
                    }
                }

                // Now that mutable borrows from above have ended, we can balance the tree
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
        let right_node = node.borrow_mut().right.take().unwrap();
        let right_left = right_node.borrow_mut().left.take();

        node.borrow_mut().right = right_left;

        // If the right node has a left child, we need to set its parent pointer
        if let Some(ref right_left) = node.borrow().right {
            right_left.borrow_mut().parent = Some(node.clone());
        }

        right_node.borrow_mut().left = Some(node.clone());
        right_node.borrow_mut().parent = node.borrow().parent.clone();

        // Adjust parent's child pointer
        if let Some(ref parent) = node.borrow().parent {
            if Rc::ptr_eq(&node, &parent.borrow().left.as_ref().unwrap()) {
                parent.borrow_mut().left = Some(right_node.clone());
            } else {
                parent.borrow_mut().right = Some(right_node.clone());
            }
        }

        Self::update_height(&node);
        Self::update_height(&right_node);

        right_node
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

    // Rotate right
    fn rotate_right(node: Tree) -> Tree {
        let left_node = node.borrow_mut().left.take().unwrap();
        let left_right = left_node.borrow_mut().right.take();

        node.borrow_mut().left = left_right;

        // If the left node has a right child, we need to set its parent pointer
        if let Some(ref left_right) = node.borrow().left {
            left_right.borrow_mut().parent = Some(node.clone());
        }

        left_node.borrow_mut().right = Some(node.clone());
        left_node.borrow_mut().parent = node.borrow().parent.clone();

        // Adjust parent's child pointer
        if let Some(ref parent) = node.borrow().parent {
            if Rc::ptr_eq(&node, &parent.borrow().right.as_ref().unwrap()) {
                parent.borrow_mut().right = Some(left_node.clone());
            } else {
                parent.borrow_mut().left = Some(left_node.clone());
            }
        }

        Self::update_height(&node);
        Self::update_height(&left_node);

        left_node
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

fn main() {
    let mut avl = AVL::new();
    avl.insert(4);
    avl.insert(5);
    avl.insert(8);
    avl.insert(11);
    avl.insert(12);
    avl.insert(18);
    avl.insert(17);
    avl.insert(19);

    println!("{:?}", avl.print_inorder());
    avl.pretty_print();
    println!("Height: {}", avl.get_height());
}
