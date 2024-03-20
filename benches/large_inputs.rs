use criterion::{black_box, criterion_group, criterion_main, Criterion};
use baltree::red_black_tree::RedBlack;

fn insert_and_search_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert_and_Search");

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

criterion_group!(benches, insert_and_search_benchmark);
criterion_main!(benches);
