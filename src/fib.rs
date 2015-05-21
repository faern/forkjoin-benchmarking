use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,ReduceStyle,Algorithm};
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

pub fn parfib_no_threshold(b: &mut Bencher, threads: usize, &i: &usize) {
    let forkpool = ForkPool::with_threads(threads);
    let fibpool = forkpool.init_algorithm(FIB_NO_THRESHOLD);

    b.iter_with_large_drop(|| {
        let job = fibpool.schedule(test::black_box(i));
        job.recv().unwrap()
    })
}

pub fn parfib_once(threads: usize, i: usize) -> usize {
    let forkpool = ForkPool::with_threads(threads);
    let fibpool = forkpool.init_algorithm(FIB);

    let job = fibpool.schedule(test::black_box(i));
    job.recv().unwrap()
}

pub fn parfib_no_threshold_once(threads: usize, i: usize) -> usize {
    let forkpool = ForkPool::with_threads(threads);
    let fibpool = forkpool.init_algorithm(FIB_NO_THRESHOLD);

    let job = fibpool.schedule(test::black_box(i));
    job.recv().unwrap()
}

const FIB: Algorithm<usize, usize> = Algorithm {
    fun: fib_task,
    style: AlgoStyle::Reduce(ReduceStyle::NoArg(fib_join)),
};

fn fib_task(n: usize, _: usize) -> TaskResult<usize, usize> {
    if n <= 20 {
        TaskResult::Done(fib(n))
    } else {
        TaskResult::Fork(vec![n-2,n-1], None)
    }
}

const FIB_NO_THRESHOLD: Algorithm<usize, usize> = Algorithm {
    fun: fib_task_no_threshold,
    style: AlgoStyle::Reduce(ReduceStyle::NoArg(fib_join)),
};

fn fib_task_no_threshold(n: usize, _: usize) -> TaskResult<usize, usize> {
    if n < 2 {
        TaskResult::Done(1)
    } else {
        TaskResult::Fork(vec![n-2,n-1], None)
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
