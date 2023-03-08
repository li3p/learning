fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    for i in v.iter() {
        println!("{i}");
    }
    let mut vi = v.iter();
    let a = vi.next();
    assert_eq!(1, *a.unwrap());
    println!("{:?}", v);
}
