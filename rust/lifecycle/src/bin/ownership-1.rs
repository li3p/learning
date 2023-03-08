fn main() {
    let s1 = give_ownership();
    let s2 = String::from("Hi, there!");
    let s3 = take_and_give_back(s2);
    assert_eq!("Some String", s1);
    // assert_eq!("", s2);
    assert_eq!("Hi, there!", s3);

    let x = bar(&6u32);
    assert_eq!(6, *x);

    let y = foo(&8u32);
    assert_eq!(8, *y);
}
fn bar(x: &u32) -> &u32 {
    x
}
fn foo<'a>(x: &'a u32) -> &'a u32 {
    x
}
fn give_ownership() -> String {
    let some = String::from("Some String");
    some
}

fn take_and_give_back(s: String) -> String {
    s
}
