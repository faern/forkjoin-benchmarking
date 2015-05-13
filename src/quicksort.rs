use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};
use std::mem;

pub fn par_qsort<F>(b: &mut Bencher, threads: usize, size: usize, datafun: F) where
    F: Fn(&mut [usize])
{
    let mut data: Vec<usize> = (0..size).collect();
    let mut data_bench: Vec<usize> = unsafe { Vec::from_raw_parts(data.as_mut_ptr(), data.len(), data.capacity()) };
    let mut data_verify: Vec<usize> = unsafe { Vec::from_raw_parts(data.as_mut_ptr(), data.len(), data.capacity()) };

    b.iter_with_setup_and_verify(|| {
        datafun(&mut data[..]);
    }, |()| {
        let forkpool = ForkPool::with_threads(threads);
        let sortpool = forkpool.init_algorithm(Algorithm {
            fun: quicksort_task,
            style: AlgoStyle::Summa(SummaStyle::NoArg(quicksort_join)),
        });
        let job = sortpool.schedule(&mut data_bench[..]);
        job.recv().unwrap()
    }, |()| {
        verify_sorted(&mut data_verify[..]);
    });

    mem::forget(data_bench);
    mem::forget(data_verify);
}

pub fn seq_qsort<F>(b: &mut Bencher, size: usize, datafun: F) where
    F: Fn(&mut [usize])
{
    let mut data: Vec<usize> = (0..size).collect();
    let mut data_bench: Vec<usize> = unsafe { Vec::from_raw_parts(data.as_mut_ptr(), data.len(), data.capacity()) };
    let mut data_verify: Vec<usize> = unsafe { Vec::from_raw_parts(data.as_mut_ptr(), data.len(), data.capacity()) };

    b.iter_with_setup_and_verify(|| {
        datafun(&mut data[..]);
    }, |()| {
        quicksort_seq(&mut data_bench[..]);
    }, |()| {
        verify_sorted(&mut data_verify[..]);
    });

    mem::forget(data_bench);
    mem::forget(data_verify);
}

pub fn par_qsort_once(threads: usize, data: &mut [usize]) {
    let forkpool = ForkPool::with_threads(threads);
    let sortpool = forkpool.init_algorithm(Algorithm {
        fun: quicksort_task,
        style: AlgoStyle::Summa(SummaStyle::NoArg(quicksort_join)),
    });
    let job = sortpool.schedule(data);
    job.recv().unwrap();
}

pub fn verify_sorted(data: &[usize]) {
    if data.len() > 0 {
        let mut last = data[0];
        for d in data {
            assert!(last <= *d);
            last = *d;
        }
    }
}

pub fn create_vec_rnd(mut x: usize, d: &mut [usize]) {
    let mut i = 0;
    let n = d.len();
    while i < n {
        let num = (i * n ^ x) % n;
        d[i] = num;
        x ^= i*num;
        i += 1;
    }
}

fn quicksort_task(d: &mut [usize]) -> TaskResult<&mut [usize], ()> {
    let len = d.len();
    if len <= 1000 {
        quicksort_seq(d);
        TaskResult::Done(())
    } else {
        let pivot = partition(d);
        let (low, tmp) = d.split_at_mut(pivot);
        let (_, high) = tmp.split_at_mut(1);

        TaskResult::Fork(vec![low, high], None)
    }
}

fn quicksort_join(_: &[()]) -> () {}

fn quicksort_seq(d: &mut [usize]) {
    if d.len() > 1 {
        let pivot = partition(d);

        let (low, tmp) = d.split_at_mut(pivot);
        let (_, high) = tmp.split_at_mut(1);

        quicksort_seq(low);
        quicksort_seq(high);
    }
}

fn partition(d: &mut[usize]) -> usize {
    let last = d.len()-1;
    let pi = pick_pivot(d);
    let pv = d[pi];
    d.swap(pi, last); // Put pivot last
    let mut store = 0;
    for i in 0..last {
        if d[i] <= pv {
            d.swap(i, store);
            store += 1;
        }
    }
    if d[store] > pv {
        d.swap(store, last);
        store
    } else {
        last
    }
}

fn pick_pivot(d: &[usize]) -> usize {
    let len = d.len();
    if len < 3 {
        0
    } else {
        let is = [0, len/2, len-1];
        let mut vs = [d[0], d[len/2], d[len-1]];
        vs.sort();
        for i in is.iter() {
            if d[*i] == vs[1] {
                return *i;
            }
        }
        unreachable!();
    }
}
