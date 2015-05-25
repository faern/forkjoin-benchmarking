use criterion::Bencher;
use forkjoin::{TaskResult,ForkPool,AlgoStyle,ReduceStyle,Algorithm};
use std::mem;
use std::ptr::{self, Unique};
use std::slice;

use sortutils::verify_sorted;
use quicksort::quicksort_seq;

pub fn par_mergesort<F>(b: &mut Bencher, threads: usize, size: usize, datafun: F) where
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
            fun: mergesort_task,
            style: AlgoStyle::Reduce(ReduceStyle::NoArg(mergesort_join)),
        });
        let job = sortpool.schedule(&mut data_bench[..]);
        job.recv().unwrap()
    }, |_| {
        verify_sorted(&mut data_verify[..]);
    });

    mem::forget(data_bench);
    mem::forget(data_verify);
}

pub fn seq_mergesort<F>(b: &mut Bencher, size: usize, datafun: F) where
    F: Fn(&mut [usize])
{
    let mut data: Vec<usize> = (0..size).collect();
    let mut data_bench: Vec<usize> = unsafe { Vec::from_raw_parts(data.as_mut_ptr(), data.len(), data.capacity()) };
    let mut data_verify: Vec<usize> = unsafe { Vec::from_raw_parts(data.as_mut_ptr(), data.len(), data.capacity()) };

    b.iter_with_setup_and_verify(|| {
        datafun(&mut data[..]);
    }, |()| {
        mergesort_seq(&mut data_bench[..]);
    }, |()| {
        verify_sorted(&mut data_verify[..]);
    });

    mem::forget(data_bench);
    mem::forget(data_verify);
}

pub fn par_mergesort_once(threads: usize, data: &mut [usize]) {
    let forkpool = ForkPool::with_threads(threads);
    let sortpool = forkpool.init_algorithm(Algorithm {
        fun: mergesort_task,
        style: AlgoStyle::Reduce(ReduceStyle::NoArg(mergesort_join)),
    });
    let job = sortpool.schedule(data);
    job.recv().unwrap();
}

fn mergesort_task(d: &mut [usize]) -> TaskResult<&mut [usize], (Unique<usize>, usize)> {
    let len = d.len();
    if len <= 1000 {
        quicksort_seq(d);
        TaskResult::Done(unsafe{(Unique::new(d.as_mut_ptr()), len)})
    } else {
        let (low, high) = d.split_at_mut(len / 2);
        TaskResult::Fork(vec![low, high], None)
    }
}

fn mergesort_join(xs: &[(Unique<usize>, usize)]) -> (Unique<usize>, usize) {
    assert_eq!(2, xs.len());
    let (ref lowp, lowl) = xs[0];
    let (ref highp, highl) = xs[1];
    let low = unsafe{mem::transmute::<&[usize], &mut [usize]>(slice::from_raw_parts(**lowp, lowl))};
    let high = unsafe{mem::transmute::<&[usize], &mut [usize]>(slice::from_raw_parts(**highp, highl))};
    assert_eq!(unsafe{low.as_ptr().offset(low.len() as isize)}, high.as_ptr());

    merge(low, high);

    unsafe {
        let mut ret = Unique::new(mem::transmute(ptr::null::<usize>()));
        ptr::copy(lowp, &mut ret, 1);
        (ret, lowl + highl)
    }
}

#[test]
fn test_mergesort_join() {
    let mut xs = vec![1,3,5,2,4,6];
    let low = unsafe{Unique::new(xs.as_mut_ptr())};
    let high = unsafe{Unique::new(xs.as_mut_ptr().offset(3))};
    let join_arg = vec![(low, 3), (high, 3)];

    let (result_p, result_len) = mergesort_join(&join_arg[..]);

    assert_eq!(vec![1,2,3,4,5,6], xs);
    assert_eq!(xs.as_mut_ptr(), *result_p);
    assert_eq!(xs.len(), result_len);
}

fn mergesort_seq(d: &mut [usize]) {
    let len = d.len();
    if len < 1000 {
        quicksort_seq(d);
    } else {
        let mid = len / 2;
        let (low, high) = d.split_at_mut(mid);

        mergesort_seq(low);
        mergesort_seq(high);
        merge(low, high);
    }
}

#[test]
fn test_mergesort_seq() {
    let mut xs = [6,4,3,5,1,2];
    mergesort_seq(&mut xs[..]);
    assert_eq!(vec![1,2,3,4,5,6], xs);
}

fn merge (xs1: &mut [usize], xs2: &mut [usize]) {
    let (l1, l2) = (xs1.len(), xs2.len());
    let len = l1+l2;
    let (mut il, mut ir) = (0, 0);
    let mut i = 0;

    let mut buf: Vec<usize> = Vec::with_capacity(len);

    while i < len {
        if il < l1 && (ir >= l2 || xs1[il] <= xs2[ir]) {
            buf.push(xs1[il]);
            il = il + 1;
        } else {
            buf.push(xs2[ir]);
            ir = ir + 1;
        }
        i = i + 1;
    }
    unsafe {
        ptr::copy(buf.as_ptr(), xs1.as_mut_ptr(), l1);
        ptr::copy(buf.as_ptr().offset(l1 as isize), xs2.as_mut_ptr(), l2);
    }
}

#[test]
fn test_merge() {
    let mut xs1 = vec![1,3,5];
    let mut xs2 = vec![2,4,6];
    merge(&mut xs1[..], &mut xs2[..]);
    assert_eq!(vec![1,2,3], xs1);
    assert_eq!(vec![4,5,6], xs2);
}
