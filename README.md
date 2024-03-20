# baltree

This project aims to implement and analyze the following self-balancing trees in the Rust programming language:

- Red Black Tree
- AVL Tree

# Demo


# Crates Used:
`rand`: This crate was used for generating random numbers to be added to the Trees

`criterion`: This crate is a benchmarking library used to measure and report the performance of the code.

`colored`: This crate is used to color the output of the Red Black Tree structure.

# Project Setup:

The project setup is found in the `src/` directory in the root of the project.

- `main.rs`: Serves as the main entry point of the application. Creates and inserts values into a Red Black and AVL Tree, also prints and showcases other functions such as search and delete.

- `avl_tree.rs`: Contains all the functionality regarding the creation of the AVL Trees. Serves as the main module for the AVL Tree.

- `rb_tree.rs`: Contains all the functionality regarding the creation of the Red Black Trees. Serves as the main module for the Red Black Tree.

- `bs_tree.rs`: Contains the logic responsible for a common/shared methods among both trees as they are both technically Binary Search Trees.

- `lib.rs`: Contains the imports that help centralize the connection of all modules.

- `cli.rs`: Contains the state logic for user input through the terminal.

# Usage Instructions:

## Steps to run locally:

1. Download the zip file or use `git clone https://github.com/jdrco/baltree` on your terminal and `cd` into that directory.
2. Run 'cargo build' and '`cargo run` on the root directory to compile and run the rust code to print an AVL tree.

## Steps to run benchmark tests:

1. Download the zip file or clone the repo and build the code using `cargo build`.
2. Run `cargo bench` to run and view the benchmark tests for each of the trees.
