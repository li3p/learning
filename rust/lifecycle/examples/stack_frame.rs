#[derive(Debug)]
struct Employee<'a> {
    name: &'a str,
    salary: i32,
    sales: i32,
    bonus: i32,
}

fn main() {
    let john = Employee {
        name: &format!("{}", "John Doe"),
        salary: 1000,
        sales: 100,
        bonus: 200,
    };
    println!("{:?}", john.name);
    println!("{:?}", john.salary);
    println!("{:?}", john.sales);
    println!("{:?}", john.bonus);
}
