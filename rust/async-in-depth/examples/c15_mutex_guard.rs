use std::{sync::{Arc, Mutex}, collections::HashMap, borrow::Cow, thread};

fn main() {
    // Use Arc to share ownership of a value between threads
    let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            let mut g = m.lock().unwrap();

            //only the thread that obtained the lock can access the data

            let data = &mut *g;

            // Since Cow implements lots of From traits, 
            // we can use .into to create a Cow from a &str as key.
            let entry = data.entry("foo".into()).or_insert(0);

            *entry += 1;

            // MutexGuard will be dropped when it goes out of scope
        });
    }

    thread::sleep(std::time::Duration::from_secs(1));

    println!("metrics{:?}", metrics.lock().unwrap());
}
