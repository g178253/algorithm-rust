use crate::sort::common;

pub fn sort(a: &mut Vec<i32>, lo: usize, hi: usize) {
    if hi <= lo { return; }

    let j = partition(a, lo, hi);
    sort(a, lo, j - 1);
    sort(a, j + 1, hi);
}

fn partition(a: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    let v = a[lo];
    let mut k = lo;

    for i in lo+1..=hi {
        if a[i] < v {
            common::swap(a, k, i);
            k += 1;
        }
    }

    k
}