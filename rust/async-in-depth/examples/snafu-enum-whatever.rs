use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },

    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}

fn is_valid_id(id: u16) -> Result<(), Error> {
    ensure!(id >= 10, InvalidIdSnafu { id });
    whatever!("Just kidding... this function always fails!");
}

fn main() {
    let res = is_valid_id(5);
    println!("res: {:?}", res);
    println!("res: {}", res.unwrap_err());
}
