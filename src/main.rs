extern crate criterion;
extern crate forkjoin;

mod fib;
mod quicksort;

use criterion::Criterion;
use fib::{parfib, seqfib};
use quicksort::{parqsort_inv,parqsort,seqqsort_inv,seqqsort};

fn main() {
    let fibdata = vec![25, 32, 40];
    let qsortdata = vec![200, 400, 800];
    Criterion::default().with_plots()
        .bench_with_inputs("parfib", parfib, fibdata.clone())
        .bench_with_inputs("seqfib", seqfib, fibdata.clone())
        .bench_with_inputs("parqsort_inv", parqsort_inv, qsortdata.clone())
        .bench_with_inputs("parqsort", parqsort, qsortdata.clone())
        .bench_with_inputs("seqqsort_inv", seqqsort_inv, qsortdata.clone())
        .bench_with_inputs("seqqsort", seqqsort, qsortdata.clone());
}
