use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};
use test;

use std::thread::{self,JoinGuard};

pub fn seqfib_spam(b: &mut Bencher, threads: usize, &i: &usize) {
    let expected_result = fib(i);

    b.iter_with_setup_and_verify(|| {}, |()| {
        let joins = (0..threads).map(|_| {
            thread::scoped(move || {
                fib(test::black_box(i))
            })
        }).collect::<Vec<JoinGuard<usize>>>();

        joins.into_iter().map(|join| {
            join.join()
        }).collect::<Vec<usize>>()
    }, |results| {
        assert_eq!(threads, results.len());
        for result in results {
            assert_eq!(expected_result, result);
        }
    });
}

pub fn seqfib(b: &mut Bencher, &i: &usize) {
    b.iter_with_large_drop(|| {
        fib(test::black_box(i))
    })
}

pub fn parfib(b: &mut Bencher, threads: usize, &i: &usize) {
    let forkpool = ForkPool::with_threads(threads);
    let fibpool = forkpool.init_algorithm(FIB);

    b.iter_with_large_drop(|| {
        let job = fibpool.schedule(test::black_box(i));
        job.recv().unwrap()
    })
}

const FIB: Algorithm<usize, usize> = Algorithm {
    fun: fib_task,
    style: AlgoStyle::Summa(SummaStyle::NoArg(fib_join)),
};

fn fib_task(n: usize) -> TaskResult<usize, usize> {
    if n <= 20 {
        TaskResult::Done(fib(n))
    } else {
        TaskResult::Fork(vec![n-1,n-2], None)
    }
}

fn fib_join(values: &[usize]) -> usize {
    values.iter().fold(0, |acc, &v| acc + v)
}

fn fib(n: usize) -> usize {
    if n < 2 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}
