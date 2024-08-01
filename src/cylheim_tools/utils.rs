use std::fmt;

#[derive(Debug)]
pub struct CylToolError {
    message: String,
}
impl CylToolError {
    fn new(msg: &str) -> Self {
        CylToolError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for CylToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CylToolError {}

// 实现从 String 类型转换为 CylToolError 类型
impl From<String> for CylToolError {
    fn from(msg: String) -> Self {
        CylToolError { message: msg }
    }
}

// 实现从 &str 类型转换为 CylToolError 类型
impl From<&str> for CylToolError {
    fn from(msg: &str) -> Self {
        CylToolError::new(msg)
    }
}
