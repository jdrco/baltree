use criterion::{black_box, criterion_group, criterion_main, Criterion};
use baltree::red_black_tree::RedBlackTree;
use baltree::avl_tree::AVL;
use std::time::Instant;

fn benchmark_avl_tree(c: &mut Criterion) {
    let tree_sizes = [10000, 40000, 70000, 100000, 130000];
    for &size in &tree_sizes {
        let values: Vec<i32> = (1..=size).collect();
        let search_values: Vec<i32> = (1..=size / 10).collect();

        let mut avl = AVL::new(); // Directly using AVL

        let start_time = Instant::now();
        for value in &values {
            avl.insert(*value);
        }
        let avl_insert_time = start_time.elapsed();

        // let start_time = Instant::now();
        // for value in &search_values {
        //     avl.search(*value);
        // }
        // let avl_search_time = start_time.elapsed();

        // Record AVL benchmark results...
        c.benchmark_group(format!("Tree Size {}", size))
            .bench_function("AVL Insert", |b| b.iter(|| black_box(avl_insert_time)));
        
        // c.benchmark_group(format!("Tree Size {}", size))
        //     .bench_function("AVL Search", |b| b.iter(|| black_box(avl_search_time)));
    }
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

criterion_group!(benches, benchmark_avl_tree);//, benchmark_red_black_tree);
criterion_main!(benches);
