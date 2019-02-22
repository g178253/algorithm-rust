pub fn sort(a: &mut Vec<i32>) {
    if a.len() < 2 { return; }

    for i in 0..a.len() {
        let mut min = i;
        for j in i+1..a.len() {
            if a[min] > a[j] {
                min = j;
            }
        }
        swap(a, min, i);
    }
}

fn swap(a: &mut Vec<i32>, left: usize, right: usize) {
    let temp = a[left];
    a[left] = a[right];
    a[right] = temp;
}