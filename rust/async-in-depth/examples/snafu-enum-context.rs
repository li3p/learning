use snafu::{prelude::*, ErrorCompat};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not read file: {path}"))]
    ConfigFile {
        source: std::io::Error,
        path: String,
    },
}

fn read_config_file(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path).context(ConfigFileSnafu { path })
}

fn main() {
    let res = read_config_file("not-exists.toml");
    println!("res: {:?}", res);
    if let Err(e) = res {
        eprintln!("An error occurred: {}", e);
        if let Some(bt) = ErrorCompat::backtrace(&e) {
            eprintln!("{}", bt);
        }
    }
}
