use std::thread;

fn main() {
    let mut x = vec![1, 2, 3, 4];
    thread::spawn(move || {
        x.push(5);
        println!("{x:#?}")
    })
    .join()
    .unwrap();

    // x.push(6);
}
