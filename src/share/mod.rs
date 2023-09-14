mod author;
mod misc;
mod novel;
mod user;

pub use author::*;
pub use misc::*;
pub use novel::*;
pub use user::*;

use chrono::{Local, LocalResult, TimeZone, Utc};
use std::fmt;
use url;

pub type Id = i32;
pub type Url = url::Url;

#[derive(Debug)]
pub struct Timestamp {
    value: i64,
}

impl Timestamp {
    pub fn now() -> Self {
        Timestamp {
            value: Local::now().timestamp_millis(),
        }
    }

    pub fn count(&self) -> i64 {
        self.value
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = match Utc.timestamp_millis_opt(self.value) {
            LocalResult::Single(date) => date,
            _ => panic!("Incorrect timestamp_millis"),
        };
        write!(f, "{:?}", date)
    }
}
