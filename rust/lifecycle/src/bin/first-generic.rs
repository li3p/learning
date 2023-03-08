fn foo<T>(x: T) -> T {
    x
}

struct Op {
    op: i32,
}

// impl Add<i32, i32> for Op {}
trait Add<RHS, Output> {
    fn my_add(self, rhs: RHS) -> Output;
}

impl Add<i32, i32> for i32 {
    fn my_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

fn main() {
    assert_eq!(1, foo(1));
    assert_eq!("hello", foo("hello"));

    let lhs = 100;
    let out = lhs.my_add(250);
    assert_eq!(350, out);
}
