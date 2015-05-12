//#![feature(std_misc)] // For Duration
#![feature(test)]
#![feature(scoped)]

extern crate test;
extern crate criterion;
extern crate argparse;
extern crate forkjoin;

mod fib;
// mod quicksort;
// mod sumtree;

//use std::time::duration::Duration;

use criterion::{Criterion,Fun};

use argparse::{ArgumentParser,Store,List};
use std::convert::AsRef;

use fib::{seqfib, parfib, seqfib_spam};
// use quicksort::{seq_qsort_sorted, par_qsort_t1_sorted, par_qsort_t4_sorted};
// use quicksort::{seq_qsort_rnd, par_qsort_t1_rnd, par_qsort_t4_rnd};
// use sumtree::{seq_sumtree_balanced, par_sumtree_balanced_t1, par_sumtree_balanced_t4};
// use sumtree::{seq_sumtree_unbalanced, par_sumtree_unbalanced_t1, par_sumtree_unbalanced_t4};

fn main() {
    let mut samples: usize = 25;
    let mut threads: Vec<usize> = vec![1,2,4];
    let mut fibargs: Vec<usize> = vec![31];
    let mut functions: Vec<String> = vec![];

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Measure performance of ForkJoin(https://github.com/faern/forkjoin)");

        ap.refer(&mut samples).add_option(&["-s", "--samples"], Store, "Number of samples to collect for each benchmark");
        ap.refer(&mut threads).add_option(&["-t", "--threads"], List, "Number of threads to run on");
        ap.refer(&mut fibargs).add_option(&["--fib"], List, "Arguments to fib");
        ap.refer(&mut functions).add_argument("functions", List, "List of functions to benchmark").required();

        ap.parse_args_or_exit();
    }
    println!("==================================");
    println!("Number of samples: {}", samples);
    println!("Threads: {:?}", threads);
    println!("Fib arguments: {:?}", fibargs);
    println!("Benchmarked functions: {:?}", functions);
    println!("==================================");

    // let sumtree_data = vec![0, 10, 14, 18];
    // let qsortdata = vec![0, 1000, 5000, 10000];

    let mut criterion = Criterion::default();
    criterion.sample_size(samples);

    for function in functions {
        match function.as_ref() {
            "fib" => bench_fib(&mut criterion, &fibargs, &threads),
            "seqfib_spam" => bench_seqfib_spam(&mut criterion, &fibargs, &threads),
            other => panic!("Invalid function to benchmark: {}", other),
        }
    }
}

fn bench_fib(criterion: &mut Criterion, fibargs: &[usize], threads: &[usize]) {
    for fibarg in fibargs {
        let mut fibfuns: Vec<Fun<usize>> = Vec::new();
        fibfuns.push(Fun::new("seq", |b,i| seqfib(b, i)));
        for &t in threads.iter() {
            fibfuns.push(Fun::new(&format!("T{}", t), move |b,i| parfib(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("fib_{}", fibarg), fibfuns, fibarg);
    }
}

fn bench_seqfib_spam(criterion: &mut Criterion, fibargs: &[usize], threads: &[usize]) {
    for fibarg in fibargs {
        let mut fibfuns: Vec<Fun<usize>> = Vec::new();
        for &t in threads.iter() {
            fibfuns.push(Fun::new(&format!("seqfib_on_{}_threads", t), move |b,i| seqfib_spam(b, t, i)));
        }

        criterion.bench_compare_implementations(&format!("seqfib_spam_{}", fibarg), fibfuns, fibarg);
    }
}

        // .bench_with_inputs("seqfib", seqfib, fibdata.clone())
        // .bench_with_inputs("parfib_t1", parfib_t1, fibdata.clone())
        // .bench_with_inputs("parfib_t4", parfib_t4, fibdata.clone())
        // .bench_with_inputs("parfib_i30", parfib_i30, threads.clone())

        // .bench_with_inputs("seq_sumtree_unbalanced", seq_sumtree_unbalanced, sumtree_data.clone())
        // .bench_with_inputs("par_sumtree_unbalanced_t1", par_sumtree_unbalanced_t1, sumtree_data.clone())
        // .bench_with_inputs("par_sumtree_unbalanced_t4", par_sumtree_unbalanced_t4, sumtree_data.clone())

        // .bench_with_inputs("seq_sumtree_balanced", seq_sumtree_balanced, sumtree_data.clone())
        // .bench_with_inputs("par_sumtree_balanced_t1", par_sumtree_balanced_t1, sumtree_data.clone())
        // .bench_with_inputs("par_sumtree_balanced_t4", par_sumtree_balanced_t4, sumtree_data.clone())

        // .bench_with_inputs("seq_qsort_sorted", seq_qsort_sorted, qsortdata.clone())
        // .bench_with_inputs("par_qsort_t1_sorted", par_qsort_t1_sorted, qsortdata.clone())
        // .bench_with_inputs("par_qsort_t4_sorted", par_qsort_t4_sorted, qsortdata.clone())
        //
        // .bench_with_inputs("seq_qsort_rnd", seq_qsort_rnd, qsortdata.clone())
        // .bench_with_inputs("par_qsort_t1_rnd", par_qsort_t1_rnd, qsortdata.clone())
        // .bench_with_inputs("par_qsort_t4_rnd", par_qsort_t4_rnd, qsortdata.clone())
