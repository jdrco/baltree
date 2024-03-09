use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode>>; // Represents a pointer to the root node of a subtree
type RedBlackTree = Option<Tree>; // Can either be None or a Tree type

/*
What components do the Red-black tree and AVL tree have in common? Don’t Repeat Yourself! Never, ever repeat yourself – a fundamental idea in programming.
*/

#[derive(Clone, Debug, PartialEq)]
struct TreeNode {
    pub color: NodeColor, //Red or Black, Red is always assigned to newly added nodes
    pub key: u32, // Could be a numerical value where right side is >= to root and left is < than root.
    pub parent: RedBlackTree, // The node directly above the current node in the tree structure
    left: RedBlackTree, // The left child of the Node
    right: RedBlackTree, // The right child of the Node
}

impl TreeNode {
    pub fn new(data: u32) -> RedBlackTree {
        let node: TreeNode = TreeNode {
            color: NodeColor::Red,
            key: data,
            parent: None,
            left: None,
            right: None,
        };

        Some(Rc::new(RefCell::new(node)))
    }

    pub fn insert(node: &mut RedBlackTree, data: u32) {
        match node {
            Some(node) => {
                let root_key = (*node).borrow_mut().key;
                if data >= root_key {
                    let ref mut right = (*node).borrow_mut().right;
                    if right.is_none() {
                        if let Some(ref mut new_node) = TreeNode::new(data) {
                            (*new_node).borrow_mut().parent = Some(Rc::clone(node));
                            *right = Some(Rc::clone(new_node));
                        }
                       // TreeNode::fix()
                       return;
                    } 
                    TreeNode::insert(right, data);
                } else {
                    let ref mut left = (*node).borrow_mut().left;
                    if left.is_none() {
                        if let Some(ref mut new_node) = TreeNode::new(data) {
                            (*new_node).borrow_mut().parent = Some(Rc::clone(node));
                            *left = Some(Rc::clone(new_node));
                        }
                        // TreeNode::fix()
                        return;
                    }
                    TreeNode::insert(left, data);
                }
            },
            None => {}
        }
    }

    pub fn disp(node: RedBlackTree) {
        match node {
            Some(node) => {
                TreeNode::disp((*node).borrow().left.clone());
                println!("This is node: {}", (*node).borrow().key);
              
                if let Some(parent) = (*node).borrow().parent.clone() {
                    println!("My parent is node: {}", (*parent).borrow().key);
                } else {
                    println!("My parent is None.")
                }

                if let Some(left) = (*node).borrow().left.clone() {
                    println!("My left child is node: {}", (*left).borrow().key);
                } else {
                    println!("My left child is None.")
                }
                   
                if let Some(right) = (*node).borrow().right.clone() {
                    println!("My right child is node: {}", (*right).borrow().key);
                } else {
                    println!("My right child is None.")
                }
                
                TreeNode::disp((*node).borrow().right.clone());
            },
            None => {}
        }
    }
}


fn main() {
    let ref mut node = TreeNode::new(8);
    TreeNode::insert(node, 4);
    TreeNode::insert(node, 3);
    TreeNode::insert(node, 2);
    TreeNode::insert(node, 7);
    TreeNode::insert(node, 9);

    TreeNode::disp((*node).clone());
}
