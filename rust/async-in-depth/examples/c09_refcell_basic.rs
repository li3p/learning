use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);

    // why wrap in a block?
    // because according to Ownership rules, 
    // we can't have one mutable borrow and one immutable borrow at the same time
    // of course we also can't have two mutable borrows at the same time

    // {
        let mut v= data.borrow_mut();
        *v += 1;
    // }

    println!("data: {:?}", data);
}