use std::{
    error, fs,
    io::{self, ErrorKind},
    path::Path,
};

fn read_file(file_path: &Path) -> Result<String, io::Error> {
    if !file_path.is_file() {
        let not_a_file_error = io::Error::new(
            ErrorKind::InvalidInput,
            format!("Not a file: {}", file_path.display()),
        );
        return Err(not_a_file_error);
    }

    if file_path.extension().unwrap() != "txt" {
        let incorrect_ext_error = io::Error::new(
            ErrorKind::InvalidInput,
            "The file should have txt extension!",
        );
        return Err(incorrect_ext_error);
    }

    let str = fs::read_to_string(file_path)?;
    Ok(str)
}

fn get_number(str: &str) -> Result<u32, std::num::ParseIntError> {
    let result = str.trim().parse::<u32>()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    // we will write our tests her
    use super::*;

    #[test]
    fn read_file_should_return_correct_str() {
        let actual_str = read_file(Path::new("test_data/correct.txt")).unwrap();
        let expected_str = "5\n".to_string();
        assert_eq!(actual_str, expected_str);
    }

    #[test]
    fn read_file_should_return_err_if_not_file() {
        assert!(read_file(Path::new("test_data/")).is_err());
    }

    #[test]
    #[should_panic]
    fn read_file_should_panic_if_not_file() {
        read_file(Path::new("test_data/")).unwrap();
    }
}
