use criterion::Bencher;
use forkjoin::{FJData,TaskResult,ForkPool,AlgoStyle,ReduceStyle,Algorithm};
use test;

pub fn spawn(b: &mut Bencher, threads: usize) {
    b.iter_with_setup_and_verify(|| {}, |()| {
        let forkpool: ForkPool<usize, ()> = ForkPool::with_threads(test::black_box(threads));
        forkpool
    }, |forkpool| {
        drop(forkpool);
    });
}

pub fn spawn_drop(b: &mut Bencher, threads: usize) {
    b.iter(|| {
        let forkpool: ForkPool<usize, ()> = ForkPool::with_threads(test::black_box(threads));
        drop(test::black_box(forkpool));
    });
}

pub fn spawn_schedule_drop(b: &mut Bencher, threads: usize) {
    b.iter(|| {
        let forkpool: ForkPool<usize, ()> = ForkPool::with_threads(test::black_box(threads));
        let voidpool = forkpool.init_algorithm(Algorithm {
            fun: void_task,
            style: AlgoStyle::Reduce(ReduceStyle::NoArg(void_join)),
        });

        let job = voidpool.schedule(0);
        job.recv().unwrap()
    });
}

fn void_task(_: usize, _: FJData) -> TaskResult<usize, ()> {
    TaskResult::Done(())
}

fn void_join(_: &[()]) -> () {}
