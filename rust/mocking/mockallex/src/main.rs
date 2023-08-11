use mockall::predicate::*;
use mockall::*;
#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

fn call_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}
fn main() {
    println!("Hello, world!");

    let mut mock = MockMyTrait::new();
    mock.expect_foo()
        .with(predicate::eq(4))
        .times(1)
        .returning(|x| x + 2);
    assert_eq!(5, call_with_four(&mock));
}
