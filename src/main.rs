mod sort;
use sort::quick;

fn main() {
    let mut list = vec![8,9,6,12,4,13,2,31];
    println!("排序前：{:?}", list);
    quick::sort(&mut list, 0, 7);
    println!("排序后：{:?}", list);
}