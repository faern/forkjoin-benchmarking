use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};

pub fn seq_qsort_sorted(b: &mut Bencher, &size: &usize) {
    seq_qsort(b, move || (0..size).collect::<Vec<usize>>());
}

pub fn par_qsort_t1_sorted(b: &mut Bencher, &size: &usize) {
    par_qsort(b, 1, move || (0..size).collect::<Vec<usize>>());
}

pub fn par_qsort_t4_sorted(b: &mut Bencher, &size: &usize) {
    par_qsort(b, 4, move || (0..size).collect::<Vec<usize>>());
}


pub fn seq_qsort_rnd(b: &mut Bencher, &size: &usize) {
    seq_qsort(b, move || create_vec_rnd(893475343, size));
}

pub fn par_qsort_t1_rnd(b: &mut Bencher, &size: &usize) {
    par_qsort(b, 1, move || create_vec_rnd(893475343, size));
}

pub fn par_qsort_t4_rnd(b: &mut Bencher, &size: &usize) {
    par_qsort(b, 4, move || create_vec_rnd(893475343, size));
}


fn par_qsort<F>(b: &mut Bencher, threads: usize, datafun: F) where
    F: Fn() -> Vec<usize>
{
    b.iter_with_setup_and_verify(|| {
        datafun()
    }, |mut data| {
        {
            let forkpool = ForkPool::with_threads(threads);
            let sortpool = forkpool.init_algorithm(Algorithm {
                fun: quicksort_task,
                style: AlgoStyle::Summa(SummaStyle::NoArg(quicksort_join)),
            });
            let job = sortpool.schedule(&mut data[..]);
            job.recv().unwrap()
        }
        data
    }, |data| {
        verify_sorted(data);
    });
}

fn seq_qsort<F>(b: &mut Bencher, datafun: F) where
    F: Fn() -> Vec<usize>
{
    b.iter_with_setup_and_verify(|| {
        datafun()
    }, |mut data| {
        quicksort_seq(&mut data[..]);
        data
    }, |data| {
        verify_sorted(data);
    });
}

fn verify_sorted(data: Vec<usize>) {
    if data.len() > 0 {
        let mut last = data[0];
        for d in data {
            assert!(last <= d);
            last = d;
        }
    }
}

fn create_vec_rnd(mut x: usize, n: usize) -> Vec<usize> {
    let mut d: Vec<usize> = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        let num = (i * n ^ x) % n;
        d.push(num);
        x ^= i*num;
        i += 1;
    }
    d
}

fn quicksort_task(d: &mut [usize]) -> TaskResult<&mut [usize], ()> {
    let len = d.len();
    if len <= 1 {
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
