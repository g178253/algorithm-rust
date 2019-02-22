mod selection;

fn main() {
    let mut list = vec![8,7,6,5,4,3,2,1];
    println!("排序前：{:?}", list);
    selection::sort(&mut list);
    println!("排序后：{:?}", list);
}