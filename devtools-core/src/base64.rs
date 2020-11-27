use base64::{decode, encode};
use thiserror::Error;

#[derive(Default)]
pub struct Base64 {
    base64: String,
    plain_text: String,
}

#[derive(Error, Debug)]
pub enum SetBase64Error {
    #[error("failed to decode base64")]
    Decode(#[from] base64::DecodeError),
    #[error("failed to convert to UTF-8")]
    UTF8(#[from] std::string::FromUtf8Error),
}

impl Base64 {
    pub fn set_base64(&mut self, s: &str) -> Result<(), SetBase64Error> {
        let bytes = decode(&s)?;
        let text = String::from_utf8(bytes)?;

        self.plain_text = text;

        Ok(())
    }

    pub fn set_plain_text(&mut self, s: &str) {
        self.base64 = encode(&s);
        self.plain_text = s.to_owned();
    }

    pub fn get_base64(&self) -> &str {
        &self.base64
    }

    pub fn get_plain_text(&self) -> &str {
        &self.plain_text
    }
}
