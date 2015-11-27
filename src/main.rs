//#![feature(std_misc)] // For Duration
#![feature(test)]
#![feature(unique)] // For mergesort
#![feature(vec_push_all)]
#![allow(mutable_transmutes)]

extern crate test;
extern crate criterion;
extern crate argparse;
extern crate forkjoin;
extern crate time;

mod fib;
mod quicksort;
mod mergesort;
mod nqueens;
mod sumtree;
mod spawnpool;
mod sortutils;

use criterion::{Criterion,Fun};

use argparse::{ArgumentParser,Store,List,StoreFalse};
use std::convert::AsRef;

use sortutils::{verify_sorted, create_vec_rnd};
use fib::{seqfib, parfib, parfib_no_threshold, seqfib_spam, parfib_once, parfib_no_threshold_once};
use quicksort::{seq_qsort, par_qsort, par_qsort_once};
use mergesort::{seq_mergesort, par_mergesort, par_mergesort_once};
use nqueens::{seq_nqueens_reduce, seq_nqueens_search, par_nqueens_reduce, par_nqueens_search, par_nqueens_search_first, par_nqueens_reduce_once};
use spawnpool::{spawn, spawn_drop, spawn_schedule_drop};
use sumtree::{gen_unbalanced_tree, gen_list_tree, gen_balanced_tree, seq_sumtree, par_sumtree, par_sumtree_once};


fn main() {
    let mut samples: usize = 25;
    let mut threads: Vec<usize> = vec![1,2,4];
    let mut fib_args: Vec<usize> = vec![31];
    let mut sort_args: Vec<usize> = vec![0, 20000];
    let mut nqueens_args: Vec<usize> = vec![8];
    let mut sumtree_args: Vec<usize> = vec![12];
    let mut seq: bool = true;

    let mut functions: Vec<String> = vec![];

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Measure performance of ForkJoin(https://github.com/faern/forkjoin)");

        ap.refer(&mut samples).add_option(&["-s", "--samples"], Store, "Number of samples to collect for each benchmark");
        ap.refer(&mut threads).add_option(&["-t", "--threads"], List, "Number of threads to run on");
        ap.refer(&mut fib_args).add_option(&["--fib"], List, "Arguments to fib");
        ap.refer(&mut sort_args).add_option(&["--sort"], List, "Size of lists to sort by quicksort");
        ap.refer(&mut nqueens_args).add_option(&["--nqueens"], List, "Size of chessboard");
        ap.refer(&mut sumtree_args).add_option(&["--sumtree"], List, "Depth of tree in sumtree");
        ap.refer(&mut seq).add_option(&["--noseq"], StoreFalse, "Disable running of sequential algorithms");
        ap.refer(&mut functions).add_argument("functions", List, "List of functions to benchmark").required();

        ap.parse_args_or_exit();
    }
    println!("==================================");
    println!("Number of samples: {}", samples);
    println!("Threads: {:?}", threads);
    println!("Fib arguments: {:?}", fib_args);
    println!("Sorting vector sizes: {:?}", sort_args);
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
            "fib" => bench_fib(&mut criterion, &fib_args, &threads, seq),
            "fib_no_threshold" => bench_fib_no_threshold(&mut criterion, &fib_args, &threads, seq),
            "seqfib_spam" => bench_seqfib_spam(&mut criterion, &fib_args, &threads),
            "qsort" => bench_qsort(&mut criterion, &sort_args, &threads, seq),
            "mergesort" => bench_mergesort(&mut criterion, &sort_args, &threads, seq),
            "nqueens_reduce" => bench_nqueens_reduce(&mut criterion, &nqueens_args, &threads, seq),
            "nqueens_search" => bench_nqueens_search(&mut criterion, &nqueens_args, &threads, seq),
            "nqueens_search_first" => bench_nqueens_search_first(&mut criterion, &nqueens_args, &threads, seq),
            "sumtree_unbalanced" => bench_sumtree_unbalanced(&mut criterion, &sumtree_args, &threads, seq),
            "sumtree_list" => bench_sumtree_listtree(&mut criterion, &sumtree_args, &threads, seq),
            "sumtree_balanced" => bench_sumtree_balanced(&mut criterion, &sumtree_args, &threads, seq),
            "fib_once" => fib_once(&fib_args, &threads),
            "fib_no_threshold_once" => fib_no_threshold_once(&fib_args, &threads),
            "qsort_once" => qsort_once(&sort_args, &threads),
            "mergesort_once" => mergesort_once(&sort_args, &threads),
            "nqueens_reduce_once" => nqueens_reduce_once(&nqueens_args, &threads),
            "sumtree_unbalanced_once" => sumtree_unbalanced_once(&sumtree_args, &threads),
            "sumtree_list_once" => sumtree_listtree_once(&sumtree_args, &threads),
            "sumtree_balanced_once" => sumtree_balanced_once(&sumtree_args, &threads),
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

fn bench_fib(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", |b,i| seqfib(b, i)));}
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| parfib(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("fib_{}", arg), funs, arg);
    }
}

fn bench_fib_no_threshold(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", |b,i| seqfib(b, i)));}
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| parfib_no_threshold(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("fib_no_threshold_{}", arg), funs, arg);
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

fn bench_qsort(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    let seed = 893475343;
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,i| seq_qsort(b, *i, move |d| create_vec_rnd(seed, d))));}
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| par_qsort(b, t, *i, move |d| create_vec_rnd(seed, d))));
        }

        criterion.bench_compare_implementations(&format!("qsort_{}", arg), funs, arg);
    }
}

fn bench_mergesort(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    let seed = 893475343;
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,i| seq_mergesort(b, *i, move |d| create_vec_rnd(seed, d))));}
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| par_mergesort(b, t, *i, move |d| create_vec_rnd(seed, d))));
        }

        criterion.bench_compare_implementations(&format!("mergesort_{}", arg), funs, arg);
    }
}

fn bench_nqueens_reduce(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,i| seq_nqueens_reduce(b, i)));}
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| par_nqueens_reduce(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("nqueens_reduce_{}", arg), funs, arg);
    }
}

fn bench_nqueens_search(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,i| seq_nqueens_reduce(b, i)));} // Sequential only exists in reduce style for now
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| par_nqueens_search(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("nqueens_search_{}", arg), funs, arg);
    }
}

fn bench_nqueens_search_first(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,i| seq_nqueens_search(b, i)));} // Sequential only exists in reduce style for now
        for &t in threads.iter() {
            funs.push(Fun::new(&format!("T{}", t), move |b,i| par_nqueens_search_first(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("nqueens_search_first_{}", arg), funs, arg);
    }
}

fn bench_sumtree_unbalanced(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let tree = gen_unbalanced_tree(*arg);
        let tree2 = tree.clone();

        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,_| seq_sumtree(b, &tree2)));}
        for &t in threads.iter() {
            let tree_clone = tree.clone();
            funs.push(Fun::new(&format!("T{}", t), move |b,_| par_sumtree(b, t, &tree_clone)));
        }

        criterion.bench_compare_implementations(&format!("sumtree_unbalanced_{}", arg), funs, arg);
    }
}

fn bench_sumtree_listtree(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let tree = gen_list_tree(*arg);
        let tree2 = tree.clone();

        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,_| seq_sumtree(b, &tree2)));}
        for &t in threads.iter() {
            let tree_clone = tree.clone();
            funs.push(Fun::new(&format!("T{}", t), move |b,_| par_sumtree(b, t, &tree_clone)));
        }

        criterion.bench_compare_implementations(&format!("sumtree_listtree_{}", arg), funs, arg);
    }
}

fn bench_sumtree_balanced(criterion: &mut Criterion, args: &[usize], threads: &[usize], seq: bool) {
    for arg in args {
        let tree = gen_balanced_tree(*arg);
        let tree2 = tree.clone();

        let mut funs: Vec<Fun<usize>> = Vec::new();
        if seq {funs.push(Fun::new("seq", move |b,_| seq_sumtree(b, &tree2)));}
        for &t in threads.iter() {
            let tree_clone = tree.clone();
            funs.push(Fun::new(&format!("T{}", t), move |b,_| par_sumtree(b, t, &tree_clone)));
        }

        criterion.bench_compare_implementations(&format!("sumtree_balanced_{}", arg), funs, arg);
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

fn fib_no_threshold_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        for &t in threads {
            println!("Running fib_no_threshold({})/T{}", arg, t);
            time_once(|| {parfib_no_threshold_once(t, arg);});
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

fn mergesort_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        let mut data: Vec<usize> = (0..arg).collect();
        for &t in threads {
            create_vec_rnd(893475343, &mut data[..]);

            println!("Running mergesort({})/T{}", arg, t);
            time_once(|| par_mergesort_once(t, &mut data[..]));
            verify_sorted(&data[..]);
        }
        println!("");
    }
    println!("");
}

fn nqueens_reduce_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        for &t in threads {
            println!("Running nqueens_reduce({})/T{}", arg, t);
            time_once(|| par_nqueens_reduce_once(t, arg));
        }
        println!("");
    }
    println!("");
}

fn sumtree_unbalanced_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        let tree = gen_unbalanced_tree(arg);
        for &t in threads {
            println!("Running sumtree_balanced({})/T{}", arg, t);
            time_once(|| drop(par_sumtree_once(t, &tree)));
        }
        println!("");
    }
    println!("");
}

fn sumtree_listtree_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        let tree = gen_list_tree(arg);
        for &t in threads {
            println!("Running sumtree_list({})/T{}", arg, t);
            time_once(|| drop(par_sumtree_once(t, &tree)));
        }
        println!("");
    }
    println!("");
}

fn sumtree_balanced_once(args: &[usize], threads: &[usize]) {
    for &arg in args {
        let tree = gen_balanced_tree(arg);
        for &t in threads {
            println!("Running sumtree_balanced({})/T{}", arg, t);
            time_once(|| drop(par_sumtree_once(t, &tree)));
        }
        println!("");
    }
    println!("");
}
