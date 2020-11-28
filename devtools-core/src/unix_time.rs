use chrono::{Local, TimeZone, Utc};
use thiserror::Error;

#[derive(Default)]
pub struct UnixTime {
    unix_time: i64,
    utc_time: String,
    local_time: String,
}

#[derive(Error, Debug)]
pub enum SetUnixTimeString {
    #[error("failed to parse unix time")]
    Parse(#[from] std::num::ParseIntError),
}

impl UnixTime {
    pub fn set_unix_time(&mut self, v: i64) {
        self.unix_time = v;
        self.utc_time = Utc.timestamp(self.unix_time, 0).to_string();
        self.local_time = Local.timestamp(self.unix_time, 0).to_string();
    }

    pub fn set_unix_time_string(&mut self, s: &str) -> Result<(), SetUnixTimeString> {
        let new_time = s.parse()?;
        self.set_unix_time(new_time);
        Ok(())
    }

    pub fn set_unix_time_to_now(&mut self) {
        self.set_unix_time(Utc::now().timestamp())
    }

    pub fn get_unix_time(&self) -> i64 {
        self.unix_time
    }

    pub fn get_utc_time(&self) -> &str {
        &self.utc_time
    }

    pub fn get_local_time(&self) -> &str {
        &self.local_time
    }
}
