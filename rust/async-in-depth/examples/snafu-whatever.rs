use snafu::{prelude::*, ErrorCompat, Whatever};

fn is_valid_id(id: u16) -> Result<(), Whatever> {
    if id < 10 {
        whatever!("ID may not be less than 10, but it was {}", id);
    }
    Ok(())
}

fn read_config_file(path: &str) -> Result<String, Whatever> {
    std::fs::read_to_string(path).with_whatever_context(|_| format!("Could not read file {}", path))
}

fn main() {
    let res = is_valid_id(5);
    println!("res: {:?}", res);

    let res = read_config_file("not-exists.toml");
    println!("res: {:?}", res);
    if let Err(e) = res {
        eprintln!("An error occurred: {}", e);
        if let Some(bt) = ErrorCompat::backtrace(&e) {
            eprintln!("{}", bt);
        }
    }
}
