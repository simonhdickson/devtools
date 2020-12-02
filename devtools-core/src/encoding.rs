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
    #[error("failed to decode base64")]
    Decode(#[from] base64::DecodeError),
    #[error("failed to convert to UTF-8")]
    UTF8(#[from] std::string::FromUtf8Error),
}

#[derive(Clone, Copy)]
pub enum Kind {
    Auto,
    Base64
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Auto
    }
}

pub trait ViewModel {
    fn set_encoded_text(&mut self, encoded_text: &str);

    fn set_plain_text(&mut self, plain_text: &str);

    fn set_kind(&mut self, kind: Kind);

    fn encoded_text(&self) -> Result<String, EncodeError>;

    fn plain_text(&self) -> Result<String, DecodeError>;

    fn kind(&self) -> Kind;
}

enum Content {
    None,
    EncodedText(String),
    PlainText(String)
}

impl Default for Content {
    fn default() -> Self {
        Content::None
    }
}

pub fn create() -> ViewModelImpl {
    ViewModelImpl::default()
}

#[derive(Default)]
pub struct ViewModelImpl {
    content: Content,
    kind: Kind
}

impl ViewModel for ViewModelImpl {
    fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }

    fn set_encoded_text(&mut self, encoded_text: &str) {
        self.content = Content::EncodedText(encoded_text.to_owned());
    }

    fn set_plain_text(&mut self, plain_text: &str) {
        self.content = Content::PlainText(plain_text.to_owned());
    }

    fn encoded_text(&self) -> Result<String, EncodeError> {
        match &self.content {
            Content::None => Ok("".to_owned()),
            Content::EncodedText(encoded_text) => Ok(encoded_text.to_owned()),
            Content::PlainText(plain_text) => {
                match self.kind {
                    Kind::Auto => Ok("".to_owned()),
                    Kind::Base64 => {
                        let encoded_text = base64::encode(plain_text);
                        Ok(encoded_text)
                    }
                }
            },
        }
    }

    fn plain_text(&self) -> Result<String, DecodeError> {
        match &self.content {
            Content::None => Ok("".to_owned()),
            Content::PlainText(plain_text) => Ok(plain_text.to_owned()),
            Content::EncodedText(encoded_text) => {
                match self.kind {
                    Kind::Auto => Ok("".to_owned()),
                    Kind::Base64 => {
                        let bytes = base64::decode(encoded_text)?;
                        let plain_text = String::from_utf8(bytes)?;
                        Ok(plain_text)
                    }
                }
            },
        }
    }

    fn kind(&self) -> Kind {
        self.kind
    }
}
