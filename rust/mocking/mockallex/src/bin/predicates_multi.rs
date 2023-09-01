use mockall::predicate::*;
use mockall::*;

#[automock]
trait Foo {
    fn foo(&self, x: u32) -> u32;
}

fn main() {
    let mut mock = MockFoo::new();
    mock.expect_foo().with(eq(5)).return_const(50u32);
    mock.expect_foo().with(eq(6)).return_const(60u32);

    mock.foo(6); // Ok
    mock.foo(7); // Panics!
}
