use anyhow::Context;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("This is a custom error: {message}")]
struct CustomError {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error>>,
}

fn do_something() -> anyhow::Result<()> {
    let result: Result<(), std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "IO error"));
    result.map_err(|err| CustomError {
        message: "An error occurred".to_string(),
        source: Some(Box::new(err)),
    })?;

    Ok(())
}

fn main() {
    if let Err(err) = do_something() {
        eprintln!("Error: {}", err);
        if let Some(source) = err.source() {
            eprintln!("Source: {}", source);
        }
    }
}
