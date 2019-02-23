use crate::sort::common;

pub fn sort(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 { return; }

    let mut h = 1;
    while h < n/3 { h = 3*h + 1; }
    while h >= 1 {
        for i in h..n {
            let mut j = i;
            while j >= h && a[j] < a[j - h] {
                common::swap(a, j, j-h);
                j -= h;
            }
        }
        h /= 3;
    }
}