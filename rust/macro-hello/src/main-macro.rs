use macro_hello::HelloMacro;
use macro_hello_derive::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();
}
