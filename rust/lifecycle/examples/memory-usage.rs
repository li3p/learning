/// example code of https://deepu.tech/memory-management-in-rust/
///
struct Employee<'a> {
    name: &'a str,
    sales: i32,
    salary: i32,
    bonus: i32,
}

const BONUS_PERCENTAGE: i32 = 10;

fn get_bonus_percentage(salary: &i32) -> i32 {
    return salary * BONUS_PERCENTAGE / 100;
}

fn find_employee_bonus(salary: &i32, no_of_sales: i32) -> i32 {
    let bonus_percentage = get_bonus_percentage(salary);
    return bonus_percentage * no_of_sales;
}

fn main() {
    // variable is declared as mutable
    let mut john = Employee {
        name: &format!("{}", "John"), // explicitly making the value dynamic
        salary: 5000,
        sales: 5,
        bonus: 0,
    };

    // salary is borrowed while sales is copied since i32 is a primitive
    john.bonus = find_employee_bonus(&john.salary, john.sales);
    println!("Bonus for {} is {}", john.name, john.bonus);
}
