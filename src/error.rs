use std::io;
use std::result;

use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Toml(#[cause] toml::de::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "For entry {}, {}.", entry, explanation)]
    CfgIntegrity { entry: String, explanation: String },
}

impl Error {
    pub fn cfg_integrity_entry_required<T: AsRef<str>>(entry: T) -> Self {
        Error::CfgIntegrity {
            entry: entry.as_ref().to_owned(),
            explanation: "this entry is required".to_owned(),
        }
    }

    pub fn cfg_integrity_type_mismatch<T: AsRef<str>, U: AsRef<str>>(
        entry: T,
        type_expected: U,
        val: &toml::Value,
    ) -> Self {
        Error::CfgIntegrity {
            entry: entry.as_ref().to_owned(),
            explanation: format!(
                "this entry has to be a(n) {}, found {}",
                type_expected.as_ref(),
                val.type_str()
            ),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Error {
        Error::Toml(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
