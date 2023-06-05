use std::cell::RefCell;

struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: RefCell::new(0),
        }
    }

    fn increment(&self) {
        let mut count = self.count.borrow_mut();
        *count += 1;
    }

    fn count(&self) -> i32 {
        *self.count.borrow()
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Count: {}", counter.count());
}
