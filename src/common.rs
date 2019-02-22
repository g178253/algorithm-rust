pub fn swap(a: &mut Vec<i32>, left: usize, right: usize) {
    let temp = a[left];
    a[left] = a[right];
    a[right] = temp;
}