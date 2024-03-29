use crate::avl_tree::AVLTree;
use crate::rb_tree::RedBlackTree;
use std::io;

fn parse_keys(input: &str) -> (Vec<i32>, bool) {
    let mut invalid_input_encountered = false;
    let keys = input
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter_map(|s| match s.trim().parse::<i32>() {
            Ok(num) => Some(num),
            Err(_) => {
                if !s.trim().is_empty() {
                    invalid_input_encountered = true;
                }
                None
            }
        })
        .collect::<Vec<i32>>();

    (keys, invalid_input_encountered)
}

fn avl_interface() {
    println!("AVL Tree Created!");
    let mut avl = AVLTree::new();
    loop {
        println!("+------------------------+");
        println!("| Enter AVL Tree Command |");
        println!("+------------------------+");
        println!("1: Add Keys to AVL\n2: Delete Key from AVL\n3: Find the number of leaves\n4: Find the height of tree\n5: Print In-Order Tree\n6: Print Pre-Order Tree\n7: Print Post-Order Tree\n8: Check if Tree is empty\n9: Print Tree Structure\n10: Exit to Main Menu");

        let mut input = String::new();
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => {
                println!("Enter Keys to Insert (comma or space-separated): ");
                input.clear();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let (keys, invalid_input_encountered) = parse_keys(&input);
                if keys.is_empty() {
                    if invalid_input_encountered {
                        println!("No valid numbers found. Please insert a stream of numbers (comma or space-separated).");
                    } else {
                        println!("No input provided. Please enter some numbers.");
                    }
                } else {
                    for k in keys {
                        if avl.tree.search(k).is_some() {
                            println!("Key {} already exists", k);
                        } else {
                            avl.insert(k);
                            println!("Key {} inserted.", k);
                        }
                    }
                    if invalid_input_encountered {
                        println!("Some inputs were not valid numbers and were ignored.");
                    }
                }
            }
            "2" => {
                println!("Enter Key to Delete: ");
                input.clear(); // Clear the input buffer before reading a new value
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let key = input.trim().parse::<i32>(); // Attempt to parse the input as an integer

                match key {
                    Ok(k) => {
                        if avl.tree.search(k).is_some() {
                            avl.delete(k);
                            println!("Key {} deleted.", k);
                        } else {
                            println!("Key does not exist");
                        }
                    }
                    Err(_) => println!("Please enter a valid integer."),
                }
            }
            "3" => {
                println!("The number of leaves is: {}", avl.tree.count_leaves());
            }
            "4" => {
                println!("The height of the tree is: {}", avl.tree.get_height());
            }
            "5" => {
                println!("The tree when in-order is: {:?}", avl.tree.print_inorder());
            }
            "6" => {
                println!(
                    "The tree when pre-order is: {:?}",
                    avl.tree.print_preorder()
                );
            }
            "7" => {
                println!(
                    "The tree when post-order is: {:?}",
                    avl.tree.print_postorder()
                );
            }
            "8" => {
                println!("Checking if tree is empty: {}", avl.tree.is_empty());
            }
            "9" => {
                println!("Printing Tree:");
                avl.tree.print_structure()
            }
            "10" => {
                println!("Returning to Main Menu.");
                break;
            }
            _ => println!("Invalid input, try again!"),
        }
    }
}

fn rb_interface() {
    println!("Red Black Tree Created!");
    let mut rbt = RedBlackTree::new();
    loop {
        println!("+------------------------------+");
        println!("| Enter Red Black Tree Command |");
        println!("+------------------------------+");
        println!("1: Add Keys to RBT\n2: Delete Key from RBT\n3: Find the number of leaves\n4: Find the height of tree\n5: Print In-Order Tree\n6: Print Pre-Order Tree\n7: Print Post-Order Tree\n8: Check if Tree is empty\n9: Print Tree Structure\n10: Exit to Main Menu");

        let mut input = String::new();
        input.clear(); // Clear the input buffer before reading a new value
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => {
                println!("Enter Keys to Insert (comma or space-separated): ");
                input.clear();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let (keys, invalid_input_encountered) = parse_keys(&input);
                if keys.is_empty() {
                    if invalid_input_encountered {
                        println!("No valid numbers found. Please insert a stream of numbers (comma or space-separated).");
                    } else {
                        println!("No input provided. Please enter some numbers.");
                    }
                } else {
                    for k in keys {
                        if rbt.tree.search(k).is_some() {
                            println!("Key {} already exists", k);
                        } else {
                            rbt.insert(k);
                            println!("Key {} inserted.", k);
                        }
                    }
                    if invalid_input_encountered {
                        println!("Some inputs were not valid numbers and were ignored.");
                    }
                }
            }
            "2" => {
                println!("Enter Key to Delete: ");
                input.clear(); // Clear the input buffer before reading a new value
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let key = input.trim().parse::<i32>(); // Attempt to parse the input as an integer

                match key {
                    Ok(k) => {
                        if rbt.tree.search(k).is_some() {
                            // TODO: handle delete
                            rbt.delete(k);
                            println!("Key {} deleted.", k);
                        } else {
                            println!("Key does not exist");
                        }
                    }
                    Err(_) => println!("Please enter a valid integer."),
                }
            }
            "3" => {
                println!("The number of leaves is: {}", rbt.tree.count_leaves());
            }
            "4" => {
                println!("The height of the tree is: {}", rbt.tree.get_height());
            }
            "5" => {
                println!("The tree when in-order is: {:?}", rbt.tree.print_inorder());
            }
            "6" => {
                println!(
                    "The tree when pre-order is: {:?}",
                    rbt.tree.print_preorder()
                );
            }
            "7" => {
                println!(
                    "The tree when post-order is: {:?}",
                    rbt.tree.print_postorder()
                );
            }
            "8" => {
                println!("Checking if tree is empty: {}", rbt.tree.is_empty());
            }
            "9" => {
                println!("Printing Tree:");
                rbt.tree.print_structure()
            }
            "10" => {
                println!("Returning to Main Menu.");
                break;
            }
            _ => println!("Invalid input, try again!"),
        }
    }
}

pub fn user_input_display() {
    println!();
    println!("Welcome to Baltree! An implementation of self balancing trees.");
    println!();
    loop {
        println!("+---------------+");
        println!("| Enter Command |");
        println!("+---------------+");
        println!("1: Create AVL tree\n2: Create Red Black tree\n3: Quit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => avl_interface(),
            "2" => rb_interface(),
            "3" => {
                println!("Quit");
                break;
            }
            _ => println!("Invalid input, try again!"),
        }
    }
}
