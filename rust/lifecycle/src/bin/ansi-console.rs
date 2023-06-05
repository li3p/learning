use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    print!("Countdown: ");
    io::stdout().flush().unwrap();
    for i in (1..=10).rev() {
        print!("\x1B[1K\r{} ", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    print!("\nDone!\n");
}
