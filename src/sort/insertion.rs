use crate::sort::common;

pub fn sort(a: &mut Vec<i32>) {
    if a.len() < 2 { return; }

    for i in 1..a.len() {
        for j in (1..=i).rev() {
            if a[j] < a[j-1] {
                common::swap(a, j, j-1);
            }
        }
    }
}