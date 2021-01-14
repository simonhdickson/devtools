use chrono::{Local, TimeZone, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetUnixTimeString {
    #[error("failed to parse unix time")]
    Parse(#[from] std::num::ParseIntError),
}

pub trait UnixTimeViewModel {
    fn set_unix_time(&mut self, v: i64);

    fn set_unix_time_string(&mut self, s: String);

    fn set_unix_time_to_now(&mut self);

    fn get_unix_time(&self) -> i64;

    fn get_utc_time(&self) -> Result<String, SetUnixTimeString>;

    fn get_local_time(&self) -> String;
}

#[derive(Clone)]
pub enum Content {
    UnixTime(i64),
    UtcTime(String),
}

#[derive(Default)]
pub struct ViewModelImpl {
    content: Option<Content>,
}

pub fn create() -> ViewModelImpl {
    ViewModelImpl::default()
}

impl UnixTimeViewModel for ViewModelImpl {
    fn set_unix_time(&mut self, v: i64) {
        self.content = Some(Content::UnixTime(v));
    }

    fn set_unix_time_string(&mut self, s: String) {
        self.content = Some(Content::UtcTime(s));
    }

    fn set_unix_time_to_now(&mut self) {
        self.set_unix_time(Utc::now().timestamp())
    }

    fn get_unix_time(&self) -> i64 {
        match &self.content {
            Some(Content::UnixTime(unix_time)) => *unix_time,
            Some(Content::UtcTime(s)) => {
                let time = s.parse().unwrap();
                time
            }
            None => 0,
        }
    }

    fn get_utc_time(&self) -> Result<String, SetUnixTimeString> {
        match &self.content {
            Some(Content::UnixTime(unix_time)) => Ok(Utc.timestamp(*unix_time, 0).to_string()),
            Some(Content::UtcTime(s)) => {
                let time = s.parse()?;
                Ok(Utc.timestamp(time, 0).to_string())
            }
            None => Ok("".to_owned()),
        }
    }

    fn get_local_time(&self) -> String {
        match &self.content {
            Some(Content::UnixTime(unix_time)) => Local.timestamp(*unix_time, 0).to_string(),
            Some(Content::UtcTime(s)) => {
                let time = s.parse().unwrap();
                Local.timestamp(time, 0).to_string()
            }
            None => "".to_owned(),
        }
    }
}
