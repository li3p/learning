use core::fmt::Debug;
// In rust, the default type of generic parameter is Sized.
// for situation that generic parameter is not Sized, we can use ?Sized to indicate it.
fn print_info<T: Debug + ?Sized>(value: &T) {
    println!("{:?}", value);
}

fn main() {
    let mut string = format!("str is not implement Sized trait");
    let data: &mut str = string.as_mut();
    print_info(data);

    // In the following code, which the compiler will report an error:
    //
    // 14 |     let data: &str = format!("str is not implement Sized trait").as_mut();
    //    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^         - temporary value is freed at the end of this statement
    //    |                      |
    //    |                      creates a temporary value which is freed while still in use
    // 15 |     print_info(data);
    //    |                ---- borrow later used here
    // the type of data is &str, which is Sized.
    //
    //
    // which means: the format! macro will return a temporary value, and the temporary value will
    // be freed at the end of this statement.

    // let data: &str = format!("str is not implement Sized trait").as_mut();
    // print_info(data);

    let data: &str = &format!("str2 is not implement Sized trait");
    print_info(data);
}
