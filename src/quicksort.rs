
use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};

pub fn parqsort_inv(b: &mut Bencher, &size: &usize) {
    let mut data: Vec<usize> = (size..0).collect();
    b.iter(|| {
        quicksort_par(&mut data[..], 4);
    });
}

pub fn parqsort(b: &mut Bencher, &size: &usize) {
    let mut data: Vec<usize> = (0..size).collect();
    b.iter(|| {
        quicksort_par(&mut data[..], 4);
    });
}

pub fn seqqsort_inv(b: &mut Bencher, &size: &usize) {
    let mut data: Vec<usize> = (size..0).collect();
    b.iter(|| {
        quicksort_seq(&mut data[..]);
    });
}

pub fn seqqsort(b: &mut Bencher, &size: &usize) {
    let mut data: Vec<usize> = (0..size).collect();
    b.iter(|| {
        quicksort_seq(&mut data[..]);
    });
}

fn quicksort_par(d: &mut[usize], threads: usize) {
    let forkpool = ForkPool::with_threads(threads);
    let sortpool = forkpool.init_algorithm(Algorithm {
        fun: quicksort_task,
        style: AlgoStyle::Summa(SummaStyle::NoArg(quicksort_join)),
    });

    let job = sortpool.schedule(&mut d[..]);
    job.recv().unwrap();
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
    let pv = d[last];
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
