use crate::sort::common;

pub fn sort(a: &mut Vec<i32>) {
    if a.len() < 2 { return; }

    for i in 1..a.len() {
        let mut j = i;
        while j > 0 && a[j] < a[j-1] {
            common::swap(a, j, j-1);
            j -= 1;
        }
    }
}