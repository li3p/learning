use mockall::predicate::*;
use mockall::*;

#[automock]
trait Foo {
    fn open(&self, path: &String) -> Option<u32>;
}

fn main() {
    let mut mock = MockFoo::new();
    mock.expect_open()
        .with(eq(String::from("something.txt")))
        .returning(|_| Some(5));
    mock.expect_open().return_const(None);

    mock.open(&String::from("nothing.txt")); // Always not panic
}
