use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("failed to encode")]
    Decode(String),
    #[error("failed to convert to UTF-8")]
    UTF8(#[from] std::string::FromUtf8Error),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("failed to decode")]
    Decode(String),
    #[error("failed to convert to UTF-8")]
    UTF8(#[from] std::string::FromUtf8Error),
}

#[derive(Clone, Copy)]
pub enum Kind {
    Auto,
    Base64,
    RFC4648Base32,
    CrockfordBase32,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Auto
    }
}

pub trait EncodingViewModel {
    fn set_encoded_text(&mut self, encoded_text: &str);

    fn set_plain_text(&mut self, plain_text: &str);

    fn set_kind(&mut self, kind: Kind);

    fn encoded_text(&self) -> Result<String, EncodeError>;

    fn plain_text(&self) -> Result<String, DecodeError>;

    fn kind(&self) -> Kind;
}

enum Content {
    EncodedText(String),
    PlainText(String),
}

pub fn create() -> ViewModelImpl {
    ViewModelImpl::default()
}

#[derive(Default)]
pub struct ViewModelImpl {
    content: Option<Content>,
    kind: Kind,
}

impl EncodingViewModel for ViewModelImpl {
    fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }

    fn set_encoded_text(&mut self, encoded_text: &str) {
        self.content = Some(Content::EncodedText(encoded_text.to_owned()));
    }

    fn set_plain_text(&mut self, plain_text: &str) {
        self.content = Some(Content::PlainText(plain_text.to_owned()));
    }

    fn encoded_text(&self) -> Result<String, EncodeError> {
        match &self.content {
            None => Ok("".to_owned()),
            Some(Content::EncodedText(encoded_text)) => Ok(encoded_text.to_owned()),
            Some(Content::PlainText(plain_text)) => match self.kind {
                Kind::Auto => base64::encode(plain_text),
                Kind::Base64 => base64::encode(plain_text),
                Kind::RFC4648Base32 => base32_rfc4648::encode(plain_text),
                Kind::CrockfordBase32 => base32_crockford::encode(plain_text),
            },
        }
    }

    fn plain_text(&self) -> Result<String, DecodeError> {
        match &self.content {
            None => Ok("".to_owned()),
            Some(Content::PlainText(plain_text)) => Ok(plain_text.to_owned()),
            Some(Content::EncodedText(encoded_text)) => match self.kind {
                Kind::Auto => auto_decode(encoded_text),
                Kind::Base64 => base64::decode(encoded_text),
                Kind::RFC4648Base32 => base32_rfc4648::decode(encoded_text),
                Kind::CrockfordBase32 => base32_crockford::decode(encoded_text),
            },
        }
    }

    fn kind(&self) -> Kind {
        self.kind
    }
}

fn auto_decode(encoded_text: &str) -> Result<String, DecodeError> {
    if let Ok(plain_text) = base64::decode(encoded_text) {
        return Ok(plain_text);
    }

    if let Ok(plain_text) = base32_crockford::decode(encoded_text) {
        return Ok(plain_text);
    }

    if let Ok(plain_text) = base32_rfc4648::decode(encoded_text) {
        return Ok(plain_text);
    }

    Err(DecodeError::Decode(
        "couldn't determine encoding type".to_owned(),
    ))
}

mod base64 {
    use super::{DecodeError, EncodeError};

    pub fn encode(plain_text: &str) -> Result<String, EncodeError> {
        Ok(base64::encode(plain_text))
    }

    pub fn decode(encoded_text: &str) -> Result<String, DecodeError> {
        let bytes =
            base64::decode(encoded_text).map_err(|err| DecodeError::Decode(err.to_string()))?;
        let plain_text = String::from_utf8(bytes)?;
        Ok(plain_text)
    }
}

mod base32_crockford {
    use base32::Alphabet::Crockford;

    use super::{DecodeError, EncodeError};

    pub fn encode(plain_text: &str) -> Result<String, EncodeError> {
        Ok(base32::encode(Crockford, plain_text.as_bytes()))
    }

    pub fn decode(encoded_text: &str) -> Result<String, DecodeError> {
        let bytes = base32::decode(Crockford, encoded_text)
            .ok_or_else(|| DecodeError::Decode("failed to decode base32_crockford".to_owned()))?;
        let plain_text = String::from_utf8(bytes)?;
        Ok(plain_text)
    }
}

mod base32_rfc4648 {
    use base32::Alphabet::RFC4648;

    use super::{DecodeError, EncodeError};

    pub fn encode(plain_text: &str) -> Result<String, EncodeError> {
        Ok(base32::encode(
            RFC4648 { padding: true },
            plain_text.as_bytes(),
        ))
    }

    pub fn decode(encoded_text: &str) -> Result<String, DecodeError> {
        let bytes = base32::decode(RFC4648 { padding: true }, encoded_text)
            .ok_or_else(|| DecodeError::Decode("failed to decode base32_rcf4648".to_owned()))?;
        let plain_text = String::from_utf8(bytes)?;
        Ok(plain_text)
    }
}
