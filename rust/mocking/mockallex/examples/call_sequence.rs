use mockall::*;
#[automock]
trait Foo {
    fn foo(&self);
}

fn main() {
    let mut seq = Sequence::new();

    let mut mock1 = MockFoo::new();
    mock1
        .expect_foo()
        .times(1)
        .in_sequence(&mut seq)
        .returning(|| ());

    let mut mock2 = MockFoo::new();
    mock2
        .expect_foo()
        .times(1)
        .in_sequence(&mut seq)
        .returning(|| ());

    mock2.foo(); // Panics!  mock1.foo should've been called first.
}
