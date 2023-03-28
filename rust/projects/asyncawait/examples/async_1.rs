use futures::executor::block_on;

async fn do_something() {
    println!("Doing something");
}

fn main() {
    let fut = do_something();
    block_on(fut);
}
