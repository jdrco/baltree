use baltree::avl_tree::AVLTree;
use baltree::rb_tree::RedBlackTree;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_avl_tree(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];

    for &size in &tree_sizes {
        let mut group = c.benchmark_group(format!("AVL Tree Size {}", size));

        group.bench_function("AVL Insert", |b| {
            b.iter(|| {
                let mut tree = AVLTree::new();
                for value in 1..=size {
                    tree.insert(value);
                }
            });
        });

        group.bench_with_input("AVL Search", &size, |b, &size| {
            b.iter_batched_ref(
                || {
                    let mut tree = AVLTree::new();
                    for value in 1..=size {
                        tree.insert(value);
                    }
                    tree
                },
                |tree| {
                    for value in 1..=(size / 10) {
                        tree.tree.search(value);
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

fn benchmark_rb_tree(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];

    for &size in &tree_sizes {
        let mut group = c.benchmark_group(format!("RB Tree Size {}", size));

        group.bench_function("RB Insert", |b| {
            b.iter(|| {
                let mut tree = RedBlackTree::new();
                for value in 1..=size {
                    tree.insert(value);
                }
            });
        });

        group.bench_with_input("RB Search", &size, |b, &size| {
            b.iter_batched_ref(
                || {
                    let mut tree = RedBlackTree::new();
                    for value in 1..=size {
                        tree.insert(value);
                    }
                    tree
                },
                |tree| {
                    for value in 1..=(size / 10) {
                        tree.tree.search(value);
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

criterion_group!(benches, benchmark_avl_tree, benchmark_rb_tree);
criterion_main!(benches);
