//#![feature(std_misc)] // For Duration
#![feature(test)]
#![feature(scoped)]
#![feature(collections)]

extern crate test;
extern crate criterion;
extern crate argparse;
extern crate forkjoin;
extern crate time;

mod fib;
mod quicksort;
mod nqueens;
mod sumtree;
mod spawnpool;

use criterion::{Criterion,Fun};

use argparse::{ArgumentParser,Store,List};
use std::convert::AsRef;

use fib::{seqfib, parfib, seqfib_spam, parfib_once};
use quicksort::{create_vec_rnd, verify_sorted, seq_qsort, par_qsort, par_qsort_once};
use nqueens::{seq_nqueens_summa, par_nqueens_summa, par_nqueens_summa_once};
use spawnpool::{spawn, spawn_drop, spawn_schedule_drop};
use sumtree::{gen_unbalanced_tree, seq_sumtree, seq_sumtree_iter, par_sumtree, par_sumtree_once};


fn main() {
    let mut samples: usize = 25;
    let mut threads: Vec<usize> = vec![1,2,4];
    let mut fib_args: Vec<usize> = vec![31];
    let mut qsort_args: Vec<usize> = vec![0, 20000];
    let mut nqueens_args: Vec<usize> = vec![8];
    let mut sumtree_args: Vec<usize> = vec![12];

    let mut functions: Vec<String> = vec![];

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Measure performance of ForkJoin(https://github.com/faern/forkjoin)");

        ap.refer(&mut samples).add_option(&["-s", "--samples"], Store, "Number of samples to collect for each benchmark");
        ap.refer(&mut threads).add_option(&["-t", "--threads"], List, "Number of threads to run on");
        ap.refer(&mut fib_args).add_option(&["--fib"], List, "Arguments to fib");
        ap.refer(&mut qsort_args).add_option(&["--qsort"], List, "Size of lists to sort by quicksort");
        ap.refer(&mut nqueens_args).add_option(&["--nqueens"], List, "Size of chessboard");
        ap.refer(&mut sumtree_args).add_option(&["--sumtree"], List, "Depth of tree in sumtree");
        ap.refer(&mut functions).add_argument("functions", List, "List of functions to benchmark").required();

        ap.parse_args_or_exit();
    }
    println!("==================================");
    println!("Number of samples: {}", samples);
    println!("Threads: {:?}", threads);
    println!("Fib arguments: {:?}", fib_args);
    println!("Quicksort arguments: {:?}", qsort_args);
    println!("Nqueens arguments: {:?}", nqueens_args);
    println!("Sumtree depths: {:?}", sumtree_args);
    println!("Benchmarked functions: {:?}", functions);
    println!("==================================");

    let mut criterion = Criterion::default();
    criterion.sample_size(samples);

    for function in functions {
        match function.as_ref() {
            "spawn" => bench_spawn(&mut criterion, &threads),
            "spawn_drop" => bench_spawn_drop(&mut criterion, &threads),
            "spawn_schedule_drop" => bench_spawn_schedule_drop(&mut criterion, &threads),
            "fib" => bench_fib(&mut criterion, &fib_args, &threads),
            "seqfib_spam" => bench_seqfib_spam(&mut criterion, &fib_args, &threads),
            "qsort" => bench_qsort(&mut criterion, &qsort_args, &threads),
            "nqueens_summa" => bench_nqueens_summa(&mut criterion, &nqueens_args, &threads),
            "sumtree" => bench_sumtree(&mut criterion, &sumtree_args, &threads),
            "fib_once" => fib_once(&fib_args, &threads),
            "qsort_once" => qsort_once(&qsort_args, &threads),
            "nqueens_summa_once" => nqueens_summa_once(&nqueens_args, &threads),
            "sumtree_once" => sumtree_once(&sumtree_args, &threads),
            other => panic!("Invalid function to benchmark: {}", other),
        }
    }
}

fn bench_spawn(criterion: &mut Criterion, threads: &[usize]) {
    let mut funs: Vec<Fun<usize>> = Vec::new();
    for &t in threads.iter() {
        funs.push(Fun::new(&format!("T{}", t), move |b,_| spawn(b, t)));
    }

    criterion.bench_compare_implementations("spawn", funs, &0);
}

fn bench_spawn_drop(criterion: &mut Criterion, threads: &[usize]) {
    let mut funs: Vec<Fun<usize>> = Vec::new();
    for &t in threads.iter() {
        funs.push(Fun::new(&format!("T{}", t), move |b,_| spawn_drop(b, t)));
    }

    criterion.bench_compare_implementations("spawn_drop", funs, &0);
}

fn bench_spawn_schedule_drop(criterion: &mut Criterion, threads: &[usize]) {
    let mut funs: Vec<Fun<usize>> = Vec::new();
    for &t in threads.iter() {
        funs.push(Fun::new(&format!("T{}", t), move |b,_| spawn_schedule_drop(b, t)));
    }

    criterion.bench_compare_implementations("spawn_schedule_drop", funs, &0);
}

fn bench_fib(criterion: &mut Criterion, args: &[usize], threads: &[usize]) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        funs.push(Fun::new("seq", |b,i| seqfib(b, i)));
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| parfib(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("fib_{}", arg), funs, arg);
    }
}

fn bench_seqfib_spam(criterion: &mut Criterion, args: &[usize], threads: &[usize]) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| seqfib_spam(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("seqfib_spam_{}", arg), funs, arg);
    }
}

fn bench_qsort(criterion: &mut Criterion, args: &[usize], threads: &[usize]) {
    let seed = 893475343;
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        funs.push(Fun::new("seq", move |b,i| seq_qsort(b, *i, move |d| create_vec_rnd(seed, d))));
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| par_qsort(b, t, *i, move |d| create_vec_rnd(seed, d))));
        }

        criterion.bench_compare_implementations(&format!("qsort_{}", arg), funs, arg);
    }
}

fn bench_nqueens_summa(criterion: &mut Criterion, args: &[usize], threads: &[usize]) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        funs.push(Fun::new("seq", move |b,i| seq_nqueens_summa(b, i)));
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| par_nqueens_summa(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("nqueens_summa_{}", arg), funs, arg);
    }
}

fn bench_sumtree(criterion: &mut Criterion, args: &[usize], threads: &[usize]) {
    for arg in args {
        let tree = gen_unbalanced_tree(*arg);
        let tree2 = tree.clone();

        let mut funs: Vec<Fun<usize>> = Vec::new();
        funs.push(Fun::new("seqiter", move |b,_| seq_sumtree_iter(b, &tree2)));
        funs.push(Fun::new("seq", move |b,_| seq_sumtree(b, &tree2)));
        for &t in threads.iter() {
            let tree_clone = tree.clone();
            funs.push(Fun::new(&format!("T{}", t), move |b,_| par_sumtree(b, t, &tree_clone)));
        }

        criterion.bench_compare_implementations(&format!("sumtree_{}", arg), funs, arg);
    }
}

fn time_once<F: FnMut()>(mut f: F) {
    let start = time::precise_time_ns();
    f();
    let end = time::precise_time_ns();
    let elapsed = end - start;
    println!("Timing: {}", format(elapsed));
}

fn format(ns: u64) -> String {
    let prefix = ["ns", "us", "ms", "s"];
    let mut prefix_i = 0;
    let mut t: f64 = ns as f64;
    while t > 1000.0 && prefix_i < prefix.len() {
        t /= 1000.0;
        prefix_i += 1;
    }
    format!("{:.2} {}", t, prefix[prefix_i])
}

fn fib_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        for &t in threads {
            println!("Running fib({})/T{}", arg, t);
            time_once(|| {parfib_once(t, arg);});
        }
        println!("");
    }
    println!("");
}

fn qsort_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        let mut data: Vec<usize> = (0..arg).collect();
        for &t in threads {
            create_vec_rnd(893475343, &mut data[..]);

            println!("Running qsort({})/T{}", arg, t);
            time_once(|| par_qsort_once(t, &mut data[..]));
            verify_sorted(&data[..]);
        }
        println!("");
    }
    println!("");
}

fn nqueens_summa_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        for &t in threads {
            println!("Running nqueens_summa({})/T{}", arg, t);
            time_once(|| par_nqueens_summa_once(t, arg));
        }
        println!("");
    }
    println!("");
}

fn sumtree_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        let tree = gen_unbalanced_tree(arg);
        for &t in threads {
            println!("Running sumtree({})/T{}", arg, t);
            time_once(|| drop(par_sumtree_once(t, &tree)));
        }
        println!("");
    }
    println!("");
}
