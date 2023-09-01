use mockall::predicate::*;
use mockall::*;
#[automock]
trait MyTrait {
    fn foo(&self) -> u32;
    fn bar(&self, x: u32, y: u32) -> u32;
}

struct NonClone();
#[automock]
trait Foo {
    fn foo(&self) -> NonClone;
}

///
/// 返回值有两种方法：使用常量(return_const)或闭包(returning)
///

fn main() {
    let mut mock = MockMyTrait::new();
    mock.expect_foo().return_const(42u32);
    mock.expect_bar().returning(|x, y| x + y);

    let a = mock.foo();
    assert_eq!(42, a);
    let a = mock.bar(1, 2);
    assert_eq!(3, a);

    let mut mock = MockFoo::new();
    let r = NonClone {};
    mock.expect_foo().return_once(move || r);
    // let a = mock.foo();
    // assert_eq!(&a as *const NonClone);
    // non-primitive cast: `NonClone` as `*const NonClone`
}
