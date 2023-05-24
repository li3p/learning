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
}
