use mockall::predicate::*;
use mockall::*;

#[automock]
trait Foo {
    fn foo(&self, x: u32);
}

fn main() {
    let mut mock = MockFoo::new();
    mock.expect_foo().times(1).return_const(());

    mock.foo(0); // Ok
    mock.foo(1); // Panics!
}
