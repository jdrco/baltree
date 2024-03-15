use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
enum InsertedSide {
    Right,
    Left,
    Root,
}

type Tree = Rc<RefCell<TreeNode>>;
type RedBlackTree = Option<Tree>;

#[derive(Clone, Debug, PartialEq)]
struct TreeNode {
    pub color: NodeColor,
    pub key: u32,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
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
        let mut inserted_side: InsertedSide = InsertedSide::Root;
        match node {
            Some(node) => {
                let root_key = (*node).borrow().key;
                if data >= root_key {
                    let right: &mut Option<Rc<RefCell<TreeNode>>> = &mut (*node).borrow_mut().right;
                    if right.is_none() {
                        if let Some(ref mut new_node) = &mut TreeNode::new(data) {
                            (*new_node).borrow_mut().parent = Some(Rc::clone(node));
                            *right = Some(new_node.clone());
                            inserted_side = InsertedSide::Right;
                        }
                    } else {
                        TreeNode::insert(right, data);
                    }   
                } else {
                    let left: &mut Option<Rc<RefCell<TreeNode>>> = &mut (*node).borrow_mut().left;
                    if left.is_none() {
                        if let Some(new_node) = TreeNode::new(data) {
                            new_node.borrow_mut().parent = Some(Rc::clone(node));
                            *left = Some(new_node);
                            inserted_side = InsertedSide::Left;
                        }
                    } else {
                        TreeNode::insert(left, data);   
                    }  
                }
                TreeNode::fix(Rc::clone(node), inserted_side);
            }
            None => {}
        }
    }

    pub fn fix(parent: Tree, side: InsertedSide) {
        let parent = parent.borrow().parent;
        if parent.is_none() {
            node.borrow_mut().color = NodeColor::Black;
        } else {
            println!("Hello World");
        }
  
    }

    pub fn disp(node: RedBlackTree) {
        match node {
            Some(node) => {
                TreeNode::disp(node.borrow().left.clone());
                println!("This is node: {}. The rc count is {}", node.borrow().key, Rc::strong_count(&node));

                if let Some(parent) = node.borrow().parent.clone() {
                    println!("My parent is node: {}", parent.borrow().key);
                } else {
                    println!("My parent is None.");
                }

                if let Some(left) = node.borrow().left.clone() {
                    println!("My left child is node: {}", left.borrow().key);
                } else {
                    println!("My left child is None.");
                }

                if let Some(right) = node.borrow().right.clone() {
                    println!("My right child is node: {}", right.borrow().key);
                } else {
                    println!("My right child is None.");
                }

                TreeNode::disp(node.borrow().right.clone());
            }
            None => {}
        }
    }
}

fn main() {
    let mut node = TreeNode::new(8);
    TreeNode::insert(&mut node, 4);
    TreeNode::insert(&mut node, 3);
    TreeNode::insert(&mut node, 2);
    TreeNode::insert(&mut node, 7);
    TreeNode::insert(&mut node, 9);

    TreeNode::disp(node.clone());
}
