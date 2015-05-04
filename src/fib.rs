use std::fmt;
use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};

pub struct FibData(pub usize, pub usize);

impl fmt::Display for FibData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "t{}-i{}", self.0, self.1)
    }
}


pub fn parfib(b: &mut Bencher, d: &FibData) {
    b.iter_with_setup(|| {
        ForkPool::with_threads(d.0)
    }, |forkpool| {
        let fibpool = forkpool.init_algorithm(FIB);
        let job = fibpool.schedule(d.1);
        job.recv().unwrap()
    })
}

pub fn seqfib(b: &mut Bencher, &i: &usize) {
    b.iter_with_large_drop(|| {
        fib(i)
    })
}

const FIB: Algorithm<usize, usize> = Algorithm {
    fun: fib_task,
    style: AlgoStyle::Summa(SummaStyle::NoArg(fib_join)),
};

fn fib_task(n: usize) -> TaskResult<usize, usize> {
    if n < 10 {
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
