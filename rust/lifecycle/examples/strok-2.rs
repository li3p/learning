pub fn strtok<'a>(s: &'a mut &str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');

    // 报错：cannot borrow `s1` as immutable because it is also borrowed as mutable
    // immutable borrow occurs her

    // pub fn strtok<'a>(s: &'a mut &str, delimiter: char) -> &'a str {
    // 因为上面的'a 将 s的 生命周期跟返回值的生命周期绑定了，也就是
    //  let hello = strtok(&mut s1, ' '); 中的 hello 绑定了s1 的生命周期
    // 所以在 hello 生命周期有效其间，也存在一个生效的 s1 可变引用。
    // 随后又想在 println!中同时使用 hello 和 s1的不可变引用，违反了借用规则

    // println!("hello is: {}, s1: {}, s: {}", hello, s1, s);

    // 解决方法，将作用域分开即可：

    println!("hello is: {}, s: {}", hello, s);
    println!(" s1: {}", s1);
}
