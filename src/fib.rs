use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};

pub fn parfib_t1(b: &mut Bencher, &i: &usize) {
    b.iter_with_setup(|| {
        ForkPool::with_threads(1)
    }, |forkpool| {
        let fibpool = forkpool.init_algorithm(FIB);
        let job = fibpool.schedule(i);
        job.recv().unwrap()
    })
}

pub fn parfib_t4(b: &mut Bencher, &i: &usize) {
    b.iter_with_setup(|| {
        ForkPool::with_threads(4)
    }, |forkpool| {
        let fibpool = forkpool.init_algorithm(FIB);
        let job = fibpool.schedule(i);
        job.recv().unwrap()
    })
}


pub fn parfib_i30(b: &mut Bencher, &t: &usize) {
    b.iter_with_setup(|| {
        ForkPool::with_threads(t)
    }, |forkpool| {
        let fibpool = forkpool.init_algorithm(FIB);
        let job = fibpool.schedule(30);
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
