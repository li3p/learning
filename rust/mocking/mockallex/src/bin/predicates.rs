use mockall::predicate::*;
use mockall::*;

#[automock]
trait Foo {
    fn foo(&self, x: u32, y: u32);
}
fn main() {
    let mut mock = MockFoo::new();
    mock.expect_foo()
        .withf(|x: &u32, y: &u32| x == y)
        .return_const(());

    mock.foo(2 + 2, 5); // Panics!
}
