use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("IO error: {source}")]
    MyIoError {
        #[from]
        source: std::io::Error,
    },

    #[error("custom error: {0}")]
    CustomError(String),
}
fn read_file(path: &str) -> Result<String, MyError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let result = read_file("test.txt");
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_file_error() {
        let err = MyError::InvalidArgument("invalid value".to_string());
        println!("Error: {}", err); // 使用 Display
        println!("Debug: {:?}", err); // 使用 Debug
    }
}
