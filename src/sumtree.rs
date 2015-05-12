use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};
use test;

pub fn seq_sumtree(b: &mut Bencher, tree: &Tree) {
    b.iter(|| {
        sum_tree_seq(test::black_box(tree))
    });
}

pub fn par_sumtree(b: &mut Bencher, threads: usize, tree: &Tree) {
    let forkpool = ForkPool::with_threads(threads);
    let sumpool = forkpool.init_algorithm(Algorithm {
        fun: sum_tree_task,
        style: AlgoStyle::Summa(SummaStyle::Arg(sum_tree_join)),
    });

    b.iter(|| {
        let job = sumpool.schedule(test::black_box(tree));
        job.recv().unwrap()
    });
}

#[derive(Debug)]
pub struct Tree {
    value: usize,
    children: Vec<Tree>,
}
impl Clone for Tree {
    fn clone(&self) -> Tree {
        Tree {
            value: self.value,
            children: self.children.iter().map(|t| t.clone()).collect(),
        }
    }
}

fn sum_tree_seq(t: &Tree) -> usize {
    t.value + t.children.iter().fold(0, |acc, t2| acc + sum_tree_seq(t2))
}

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

fn sum_tree_join(value: &usize, values: &[usize]) -> usize {
    *value + values.iter().fold(0, |acc, &v| acc + v)
}

/// Generate a very unbalanced tree
pub fn gen_unbalanced_tree(depth: usize) -> Tree {
    let mut children = vec![];
    for i in 0..depth {
        children.push(gen_unbalanced_tree(i));
    }
    Tree {
        value: depth + 1000,
        children: children,
    }
}

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
