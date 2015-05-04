extern crate criterion;
extern crate forkjoin;

mod fib;
// mod quicksort;

use criterion::Criterion;
use fib::{parfib, seqfib, FibData};
// use quicksort::{parqsort_inv,parqsort,seqqsort_inv,seqqsort};

fn main() {
    let threads = vec![1,4];
    let fibdata = vec![20, 25, 30];
    let mut parfibdata = vec![];
    for t in threads.iter() {
        for i in fibdata.iter() {
            parfibdata.push(FibData(*t, *i));
        }
    }

    // let qsortdata = vec![200, 400, 800];
    Criterion::default()
        .bench_with_inputs("seqfib", seqfib, fibdata.clone())
        .bench_with_inputs("parfib", parfib, parfibdata)
        // .bench_with_inputs("parqsort_inv", parqsort_inv, qsortdata.clone())
        // .bench_with_inputs("parqsort", parqsort, qsortdata.clone())
        // .bench_with_inputs("seqqsort_inv", seqqsort_inv, qsortdata.clone())
        // .bench_with_inputs("seqqsort", seqqsort, qsortdata.clone());
        ;
}
