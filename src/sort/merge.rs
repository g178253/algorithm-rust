pub fn sort(a: &mut Vec<i32>) {
    let n = a.len();
    if n < 2 { return; }

    let mut aux = vec![0; n];
    sort_internal(a, 0, n-1, &mut aux);
}

fn sort_internal(a: &mut Vec<i32>, lo: usize, hi: usize, b: &mut Vec<i32>) {
    println!("sort_internal(lo:{0}, hi:{1})", lo, hi);
    if hi <= lo { return; }

    let mid = lo + (hi - lo) / 2;
    sort_internal(a, lo, mid, b);
    sort_internal(a, mid+1, hi, b);
    merge(a, lo, mid, hi, b);
}

fn merge(a: &mut Vec<i32>, lo: usize, mid: usize, hi: usize, b: &mut Vec<i32>) {
    println!("merge(lo:{0}, mid:{1}, hi:{2})", lo, mid, hi);
    let mut i = lo;
    let mut j = mid + 1;

    for k in i..=hi {
        b[k] = a[k];
    }

    for k in i..=hi {
        if i > mid          { a[k] = b[j]; j+=1; } // 左半边用尽，取右
        else if j > hi      { a[k] = b[i]; i+=1; } // 右半边用尽，取左
        else if b[i] > b[j] { a[k] = b[j]; j+=1; } // 左边元素大，取右
        else                { a[k] = b[i]; i+=1; } // 右边元素大，取左
    }
}