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

pub type Id = i32;
pub type Url = String;

#[derive(Debug)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn now() -> Self {
        Self(Local::now().timestamp_millis())
    }

    pub fn from(dur: i64) -> Self {
        Self(dur)
    }

    pub fn count(&self) -> i64 {
        self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::from(0)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = match Utc.timestamp_millis_opt(self.count()) {
            LocalResult::Single(date) => date,
            _ => panic!("Incorrect timestamp_millis"),
        };
        write!(f, "{:?}", date)
    }
}
