
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
