//#![feature(std_misc)] // For Duration

extern crate criterion;
extern crate forkjoin;

mod fib;
mod quicksort;

//use std::time::duration::Duration;

use criterion::Criterion;

use fib::{parfib_t1, parfib_t4, parfib_i30, seqfib};
use quicksort::{seq_qsort_sorted, par_qsort_t1_sorted, par_qsort_t4_sorted};
use quicksort::{seq_qsort_rnd, par_qsort_t1_rnd, par_qsort_t4_rnd};

fn main() {
    let threads = vec![1,2,3,4,5];
    let fibdata = vec![20, 25, 30, 35];
    let qsortdata = vec![1, 1000, 5000, 10000];


    Criterion::default().sample_size(20)
        .bench_with_inputs("seqfib", seqfib, fibdata.clone())
        .bench_with_inputs("parfib_t1", parfib_t1, fibdata.clone())
        .bench_with_inputs("parfib_t4", parfib_t4, fibdata.clone())
        .bench_with_inputs("parfib_i30", parfib_i30, threads.clone())

        .bench_with_inputs("seq_qsort_sorted", seq_qsort_sorted, qsortdata.clone())
        .bench_with_inputs("par_qsort_t1_sorted", par_qsort_t1_sorted, qsortdata.clone())
        .bench_with_inputs("par_qsort_t4_sorted", par_qsort_t4_sorted, qsortdata.clone())

        .bench_with_inputs("seq_qsort_rnd", seq_qsort_rnd, qsortdata.clone())
        .bench_with_inputs("par_qsort_t1_rnd", par_qsort_t1_rnd, qsortdata.clone())
        .bench_with_inputs("par_qsort_t4_rnd", par_qsort_t4_rnd, qsortdata.clone())
        ;
}
