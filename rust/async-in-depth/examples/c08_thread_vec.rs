use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let arr = Arc::new(Mutex::new(vec![10086]));
    let mut handles = vec![];
    for i in 0..5 {
        let arr = arr.clone();
        let handle = thread::spawn(move || {
            let mut arr_locked = arr.lock().unwrap();
            arr_locked.push(i);
            println!("Thread {} added value: {}", i, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_data = arr.lock().unwrap();
    println!("Final data: {:?}", *final_data);
}
