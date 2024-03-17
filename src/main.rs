mod avl_tree;
mod balancing_tree;
mod red_black_tree;

use crate::avl_tree::AVL;

use crate::red_black_tree::RedBlackTree;

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
    // avl.insert(13);
    // avl.insert(1);
    // avl.insert(2);
    // avl.insert(3);
    // avl.insert(6);
    // avl.insert(7);
    // avl.insert(9);
    // avl.insert(10);
    // avl.insert(14);
    // avl.insert(15);

    println!("-------------------AVL Tree: Begin-------------------");
    println!("Print In-Order: {:?}", avl.tree.print_inorder());
    println!("Count Leaves: {:?}", avl.tree.count_leaves());
    println!("Height: {}", avl.tree.get_height());
    println!("Is Empty: {}", avl.tree.is_empty());
    println!("Print Tree Structure:");
    avl.print_structure();
    avl.delete(5);
    println!("Print Tree Structure (After Delete 5):");
    avl.print_structure();
    println!("--------------------AVL Tree: End--------------------");
}
