mod avl_tree;
mod balancing_tree;
mod red_black_tree;

use crate::avl_tree::AVL;
use crate::red_black_tree::RedBlackTree;
use std::io;

fn avl_interface() {
    println!("AVL Tree Created!");
    let mut avl = AVL::new();
    loop {
        println!("Enter command:\n1: Add Key to AVL\n2: Delete Key from AVL\n3: Find the number of leaves\n4: Find the height of tree\n5: Print In-Order Tree\n6: Check if Tree is empty\n7: Print Tree Structure\n8: Exit to Main Menu");

        let mut input = String::new();
        input.clear(); // Clear the input buffer before reading a new value
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                println!("Enter Key to Insert: ");
                input.clear(); // Clear the input buffer before reading a new value
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let key = input.trim().parse::<i32>(); // Attempt to parse the input as an integer

                match key {
                    Ok(k) => {
                        if avl.search(k) {
                            println!("Key already exists")
                        } else {
                            avl.insert(k);
                            println!("Key {} inserted.", k);
                        }
                    },
                    Err(_) => println!("Please enter a valid integer."),
                }
            },
            "2" => {
                println!("Enter Key to Delete: ");
                input.clear(); // Clear the input buffer before reading a new value
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let key = input.trim().parse::<i32>(); // Attempt to parse the input as an integer
                
                match key {
                    Ok(k) => {
                        if avl.search(k) {
                            avl.delete(k);
                            println!("Key {} deleted.", k);
                        } else {
                            println!("Key does not exist");
                        }
                    },
                    Err(_) => println!("Please enter a valid integer."),
                }
            }
            "3" => {
                println!("The number of leaves is: {}", avl.tree.count_leaves());
            },
            "4" => {
                println!("The height of the tree is: {}", avl.tree.get_height());
            },
            "5" => {
                println!("The tree when in-order is:\n"); 
                avl.print_structure();
            },
            "6" => {
                println!("Checking if tree is empty: {}", avl.tree.is_empty());
            },
            "7" => {
                println!("Printing Tree:\n");
                avl.print_structure()
            },
            // Implement other cases similarly...
            "8" => {
                println!("Returning to Main Menu.");
                break;
            },
            _ => println!("Invalid input, try again!"),
        }
    }
}

fn rbt_interface() {
    println!("Red Black Tree Created!");
    //let mut rbt = RBT::new();
    // loop {
    //     println!("Enter command:\n1: Add Key to RBT\n2: Delete Key from RBT\n3: Find the number of leaves\n4: Find the height of tree\n5: Print In-Order Tree\n6: Check if Tree is empty\n7: Print Tree Structure\n8: Exit to Main Menu");

    //     let mut input = String::new();
    //     input.clear(); // Clear the input buffer before reading a new value
    //     io::stdin().read_line(&mut input).expect("Failed to read line");

    //     match input.trim() {
    //         "1" => {
    //             println!("Enter Key to Insert: ");
    //             input.clear(); // Clear the input buffer before reading a new value
    //             io::stdin().read_line(&mut input).expect("Failed to read line");
    //             let key = input.trim().parse::<i32>(); // Attempt to parse the input as an integer

    //             match key {
    //                 Ok(k) => {
    //                     if rbt.search(k) {
    //                         println!("Key already exists")
    //                     } else {
    //                         rbt.insert(k);
    //                         println!("Key {} inserted.", k);
    //                     }
    //                 },
    //                 Err(_) => println!("Please enter a valid integer."),
    //             }
    //         },
    //         "2" => {
    //             println!("Enter Key to Delete: ");
    //             input.clear(); // Clear the input buffer before reading a new value
    //             io::stdin().read_line(&mut input).expect("Failed to read line");
    //             let key = input.trim().parse::<i32>(); // Attempt to parse the input as an integer
                
    //             match key {
    //                 Ok(k) => {
    //                     if rbt.search(k) {
    //                         rbt.delete(k);
    //                         println!("Key {} deleted.", k);
    //                     } else {
    //                         println!("Key does not exist");
    //                     }
    //                 },
    //                 Err(_) => println!("Please enter a valid integer."),
    //             }
    //         }
    //         "3" => {
    //             println!("The number of leaves is: {}", rbt.tree.count_leaves());
    //         },
    //         "4" => {
    //             println!("The height of the tree is: {}", rbt.tree.get_height());
    //         },
    //         "5" => {
    //             println!("The tree when in-order is:\n"); 
    //             rbt.print_structure();
    //         },
    //         "6" => {
    //             println!("Checking if tree is empty: {}", rbt.tree.is_empty());
    //         },
    //         "7" => {
    //             println!("Printing Tree:\n");
    //             rbt.print_structure()
    //         },
    //         // Implement other cases similarly...
    //         "8" => {
    //             println!("Returning to Main Menu.");
    //             break;
    //         },
    //         _ => println!("Invalid input, try again!"),
    //     }
    //}
}

fn main() {
    // let mut avl = AVL::new();

    // avl.insert(4);
    // println!("{:?}", avl.search(4));
    // avl.insert(5);
    // avl.insert(8);
    // avl.insert(11);
    // avl.insert(12);
    // avl.insert(18);
    // avl.insert(17);
    // avl.insert(19);
    // // avl.insert(13);
    // // avl.insert(1);
    // // avl.insert(2);
    // // avl.insert(3);
    // // avl.insert(6);
    // // avl.insert(7);
    // // avl.insert(9);
    // // avl.insert(10);
    // // avl.insert(14);
    // // avl.insert(15);

    // println!("-------------------AVL Tree: Begin-------------------");
    // println!("Print In-Order: {:?}", avl.tree.print_inorder());
    // println!("Count Leaves: {:?}", avl.tree.count_leaves());
    // println!("Height: {}", avl.tree.get_height());
    // println!("Is Empty: {}", avl.tree.is_empty());
    // println!("Print Tree Structure:");
    // avl.print_structure();
    // avl.delete(5);
    // println!("Print Tree Structure (After Delete 5):");
    //avl.print_structure();
    // println!("--------------------AVL Tree: End--------------------");

    loop {
        println!("Enter command:\n1: Create AVL tree\n2: Create Red Black tree\n3: Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                avl_interface()
            }
            "2" => {
                println!("Red Black")
            }
            "3" => {
                println!("Quit");
                break;
            }
            _=> println!("Invalid input, try again!"),
        }
    }
}
