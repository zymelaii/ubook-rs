use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Status {
    pub httpCode: u32,
    pub errorCode: i32,
    pub msgType: i32,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: Status,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct InternalError {
    pub Message: String,
    pub ExceptionMessage: String,
    pub ExceptionType: String,
    pub StackTrace: String,
}
