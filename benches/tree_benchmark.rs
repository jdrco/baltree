use criterion::{black_box, criterion_group, criterion_main, Criterion};
use baltree::rb_tree::RedBlack;
use baltree::avl_tree::AVL;
use std::time::Instant;

fn benchmark_avl_tree(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];
    for &size in &tree_sizes {
        let values: Vec<i32> = (1..=size).collect();
        let search_values: Vec<i32> = (1..=size / 10).collect();

        let mut avl = AVL::new();

        let start_time = Instant::now();
        for value in &values {
            avl.insert(*value);
        }
        let avl_insert_time = start_time.elapsed();

        let start_time = Instant::now();
        for value in &search_values {
            avl.search(*value);
        }
        let avl_search_time = start_time.elapsed();

        // Record AVL benchmark results...
        c.benchmark_group(format!("Tree Size {}", size))
            .bench_function("AVL Insert", |b| b.iter(|| black_box(avl_insert_time)));
        
        c.benchmark_group(format!("Tree Size {}", size))
            .bench_function("AVL Search", |b| b.iter(|| black_box(avl_search_time)));
    }
}

fn benchmark_rb_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("RB_Insert_and_Search");

    let tree_sizes = vec![10_000, 40_000, 70_000, 100_000, 130_000];
    for size in tree_sizes {
        let mut red_black_tree = RedBlack::new();

        // Insert elements into the tree
        for i in 1..=size {
            red_black_tree.insert(i);
        }

        group.bench_function(format!("Size: {}", size), |b| {
            b.iter(|| {
                // Search for a portion of the inserted elements
                let search_count = size / 10; // Search for 1/10th of the inserted elements
                for i in 1..=search_count {
                    let key_to_search = i;
                    let _ = red_black_tree.search(key_to_search);
                }
            })
        });
    }

    group.finish();
}

// fn benchmark_red_black_tree(c: &mut Criterion) {
//     let tree_sizes = [10000, 40000, 70000, 100000, 130000];
//     for &size in &tree_sizes {
//         let values: Vec<u32> = (1..=size).collect();
//         let search_values: Vec<u32> = (1..=size / 10).collect();

//         let mut rb_tree = RedBlackTree::new(); // Directly using RedBlackTree

//         let start_time = Instant::now();
//         for value in &values {
//             rb_tree.insert(*value);
//         }
//         let rb_insert_time = start_time.elapsed();

//         let start_time = Instant::now();
//         for value in &search_values {
//             rb_tree.search(*value);
//         }
//         let rb_search_time = start_time.elapsed();

//         // Record Red-Black Tree benchmark results...
//         c.benchmark_group(format!("Tree Size {}", size))
//         .bench_function("AVL Insert", |b| b.iter(|| black_box(avl_insert_time)));

//         c.benchmark_group(format!("Tree Size {}", size))
//         .bench_function("AVL Search", |b| b.iter(|| black_box(avl_search_time)));
//         }
//}

criterion_group!(benches, benchmark_avl_tree, benchmark_rb_tree);//, benchmark_red_black_tree);
criterion_main!(benches);