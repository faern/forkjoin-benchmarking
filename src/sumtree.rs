use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};
use std::mem;

pub fn seq_sumtree_balanced(b: &mut Bencher, &i: &usize) {
    seq_sumtree(b, || gen_balanced_tree(i));
}

pub fn par_sumtree_balanced_t1(b: &mut Bencher, &i: &usize) {
    par_sumtree(b, 1, || gen_balanced_tree(i));
}

pub fn par_sumtree_balanced_t4(b: &mut Bencher, &i: &usize) {
    par_sumtree(b, 4, || gen_balanced_tree(i));
}

pub fn seq_sumtree_unbalanced(b: &mut Bencher, &i: &usize) {
    seq_sumtree(b, || gen_unbalanced_tree(i));
}

pub fn par_sumtree_unbalanced_t1(b: &mut Bencher, &i: &usize) {
    par_sumtree(b, 1, || gen_unbalanced_tree(i));
}

pub fn par_sumtree_unbalanced_t4(b: &mut Bencher, &i: &usize) {
    par_sumtree(b, 4, || gen_unbalanced_tree(i));
}

#[inline]
fn seq_sumtree<F>(b: &mut Bencher, treegen: F) where
    F: Fn() -> Tree
{
    // let tree = treegen();
    // let expected_sum = sum_tree_seq(&tree);
    b.iter_with_setup(|| {
        treegen()
    }, |tree| {
        sum_tree_seq(&tree)
    });
}

#[inline]
fn par_sumtree<F>(b: &mut Bencher, threads: usize, treegen: F) where
    F: Fn() -> Tree
{
    let tree = treegen();
    let expected_sum = sum_tree_seq(&tree);

    let forkpool = ForkPool::with_threads(threads);
    let sumpool = forkpool.init_algorithm(Algorithm {
        fun: sum_tree_task,
        style: AlgoStyle::Summa(SummaStyle::Arg(sum_tree_join)),
    });

    b.iter(|| {
        let job = sumpool.schedule(&tree);
        job.recv().unwrap()
    });
}

#[derive(Debug)]
struct Tree {
    value: usize,
    children: Vec<Tree>,
}

#[inline]
fn sum_tree_seq(t: &Tree) -> usize {
    t.value + t.children.iter().fold(0, |acc, t2| acc + sum_tree_seq(t2))
}

#[inline]
fn sum_tree_task(t: &Tree) -> TaskResult<&Tree, usize> {
    let val = t.value;

    if t.children.is_empty() {
        TaskResult::Done(val)
    } else {
        let mut fork_args: Vec<&Tree> = vec![];
        for c in t.children.iter() {
            fork_args.push(c);
        }
        TaskResult::Fork(fork_args, Some(val))
    }
}

#[inline]
fn sum_tree_join(value: &usize, values: &[usize]) -> usize {
    *value + values.iter().fold(0, |acc, &v| acc + v)
}

/// Generate a very unbalanced tree
#[inline]
fn gen_unbalanced_tree(depth: usize) -> Tree {
    let mut children = vec![];
    for i in 0..depth {
        children.push(gen_unbalanced_tree(i));
    }
    Tree {
        value: depth + 1000,
        children: children,
    }
}

#[inline]
fn gen_balanced_tree(depth: usize) -> Tree {
    let mut children = vec![];
    if depth > 0 {
        for _ in 0..2 {
            children.push(gen_balanced_tree(depth-1));
        }
    }
    Tree {
        value: depth + 1000,
        children: children,
    }
}
