
use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};

pub fn parfib(b: &mut Bencher, &size: &usize) {
    let forkpool = ForkPool::with_threads(4);
    let fibpool = forkpool.init_algorithm(FIB);

    b.iter(|| {
        let job = fibpool.schedule(size);
        job.recv().unwrap()
    })
}

pub fn seqfib(b: &mut Bencher, &size: &usize) {
    b.iter(|| {
        fib(size)
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
