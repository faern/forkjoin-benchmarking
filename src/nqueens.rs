use criterion::Bencher;
use forkjoin::{FJData,ForkPool,TaskResult,AlgoStyle,ReduceStyle,Algorithm};
use test;

pub fn seq_nqueens_reduce(b: &mut Bencher, &i: &usize) {
    b.iter(|| {
        let empty = vec![];
        nqueens_reduce(test::black_box(&empty[..]), test::black_box(i))
    });
}

pub fn par_nqueens_reduce(b: &mut Bencher, threads: usize, &i: &usize) {
    let forkpool = ForkPool::with_threads(threads);
    let queenpool = forkpool.init_algorithm(NQUEENS_REDUCE);

    let expected_result = nqueens_reduce(&vec![][..], i);

    b.iter_with_setup_and_verify(|| {}, |()| {
        let empty = vec![];
        let job = queenpool.schedule(test::black_box((empty, i)));
        job.recv().unwrap()
    }, |solutions| {
        assert_eq!(expected_result.len(), solutions.len());
    });
}

pub fn par_nqueens_search(b: &mut Bencher, threads: usize, &i: &usize) {
    let forkpool = ForkPool::with_threads(threads);
    let queenpool = forkpool.init_algorithm(NQUEENS_SEARCH);

    let expected_result = nqueens_reduce(&vec![][..], i);

    b.iter_with_setup_and_verify(|| {}, |()| {
        let empty = vec![];
        let job = queenpool.schedule(test::black_box((empty, i)));
        let mut solutions = vec![];
        while let Ok(solution) = job.recv() {
            solutions.push(solution);
        }
        solutions
    }, |solutions| {
        assert_eq!(expected_result.len(), solutions.len());
    });
}

pub fn par_nqueens_reduce_once(threads: usize, i: usize) {
    let forkpool = ForkPool::with_threads(threads);
    let queenpool = forkpool.init_algorithm(NQUEENS_REDUCE);

    let empty = vec![];
    let job = queenpool.schedule(test::black_box((empty, i)));
    drop(test::black_box(job.recv().unwrap()));
}

const NQUEENS_SEARCH: Algorithm<(Board,usize), Board> = Algorithm {
    fun: nqueens_task_search,
    style: AlgoStyle::Search,
};

const NQUEENS_REDUCE: Algorithm<(Board,usize), Solutions> = Algorithm {
    fun: nqueens_task_reduce,
    style: AlgoStyle::Reduce(ReduceStyle::NoArg(nqueens_join)),
};

pub type Queen = usize;
pub type Board = Vec<Queen>;
pub type Solutions = Vec<Board>;

fn nqueens_task_search((q, n): (Board, usize), _: FJData) -> TaskResult<(Board,usize), Board> {
    if q.len() == n {
        TaskResult::Done(q)
    } else {
        let mut fork_args: Vec<(Board, usize)> = vec![];
        for i in 0..n {
            let mut q2 = q.clone();
            q2.push(i);

            if ok(&q2[..]) {
                fork_args.push((q2, n));
            }
        }
        TaskResult::Fork(fork_args, None)
    }
}

fn nqueens_task_reduce((q, n): (Board, usize), _: FJData) -> TaskResult<(Board,usize), Solutions> {
    if q.len() == n {
        TaskResult::Done(vec![q])
    } else {
        let mut fork_args: Vec<(Board, usize)> = vec![];
        for i in 0..n {
            let mut q2 = q.clone();
            q2.push(i);

            if ok(&q2[..]) {
                fork_args.push((q2, n));
            }
        }
        TaskResult::Fork(fork_args, None)
    }
}

fn nqueens_join(values: &[Solutions]) -> Solutions {
    let mut all_solutions: Solutions = vec![];
    for solutions in values {
        all_solutions.push_all(&solutions[..]);
    }
    all_solutions
}

// fn nqueens_search(q: &[Queen], n: usize) -> Board {
//     if q.len() == n && ok(q) {
//         return vec![q.to_vec()];
//     }
//     let mut solutions: Solutions = vec![];
//     for i in 0..n {
//         let mut q2 = q.to_vec();
//         q2.push(i);
//         let new_q = &q2[..];
//
//         if ok(new_q) {
//             let more_solutions = nqueens_search(new_q, n);
//             solutions.push_all(&more_solutions[..]);
//         }
//     }
//     solutions
// }

fn nqueens_reduce(q: &[Queen], n: usize) -> Solutions {
    if q.len() == n {
        return vec![q.to_vec()];
    }
    let mut solutions: Solutions = vec![];
    for i in 0..n {
        let mut q2 = q.to_vec();
        q2.push(i);
        let new_q = &q2[..];

        if ok(new_q) {
            let more_solutions = nqueens_reduce(new_q, n);
            solutions.push_all(&more_solutions[..]);
        }
    }
    solutions
}

fn ok(q: &[usize]) -> bool {
    for (x1, &y1) in q.iter().enumerate() {
        for (x2, &y2) in q.iter().enumerate() {
            if x2 > x1 {
                let xd = x2-x1;
                if y1 == y2 || y1 == y2 + xd || (y2 >= xd && y1 == y2 - xd) {
                    return false;
                }
            }
        }
    }
    true
}
