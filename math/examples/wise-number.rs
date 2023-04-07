fn main() {
    let arr: Vec<_> = (1..10000)
        .filter(|&x| (x >= 3 && x % 2 != 0) || (x > 4 && x % 4 == 0))
        .collect();
    println!("{:?}", arr[2006 - 1]);
}
