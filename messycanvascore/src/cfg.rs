use std::result;
use std::error;
use std::fmt;
use std::collections;
use std::path;
use std::fs;
use std::io;

use toml;

#[derive(Debug)]
pub struct IntegrityError {
    entry: String,
    explanation: String,
    cause: Option<Box<error::Error + Send + Sync>>,
}

impl fmt::Display for IntegrityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "For entry {}, {}.", &self.entry, &self.explanation)
    }
}

impl error::Error for IntegrityError {
    fn description(&self) -> &str {
        "entry is not valid"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.cause {
            Some(ref m) => Some(m.as_ref()),
            None => None,
        }
    }
}

impl IntegrityError {
    pub fn new<T: AsRef<str>>(entry: T, explanation: T) -> Self {
        Self {
            entry: entry.as_ref().to_owned(),
            explanation: explanation.as_ref().to_owned(),
            cause: None,
        }
    }

    pub fn new_with_cause<T: AsRef<str>>(
        entry: T,
        explanation: T,
        cause: Box<error::Error + Send + Sync>,
    ) -> IntegrityError {
        Self {
            entry: entry.as_ref().to_owned(),
            explanation: explanation.as_ref().to_owned(),
            cause: Some(cause),
        }
    }

    pub fn entry_required<T: AsRef<str>>(entry: T) -> Self {
        Self {
            entry: entry.as_ref().to_owned(),
            explanation: "this entry is required".to_owned(),
            cause: None,
        }
    }

    pub fn type_mismatch<T: AsRef<str>>(entry: T, type_expected: T, val: &toml::Value) -> Self {
        Self {
            entry: entry.as_ref().to_owned(),
            explanation: format!(
                "this entry has to be a(n) {}, found {}",
                type_expected.as_ref(),
                val.type_str()
            ),
            cause: None,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Toml(toml::de::Error),
    Integrity(IntegrityError),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Toml(ref e) => fmt::Display::fmt(&e, f),
            Error::Integrity(ref e) => fmt::Display::fmt(&e, f),
            Error::Io(ref e) => fmt::Display::fmt(&e, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Toml(ref e) => e.description(),
            Error::Integrity(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Toml(ref e) => Some(e),
            Error::Integrity(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Error {
        Error::Toml(e)
    }
}

impl From<IntegrityError> for Error {
    fn from(e: IntegrityError) -> Error {
        Error::Integrity(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub raw_map: collections::BTreeMap<String, toml::Value>,
}

impl Config {
    pub fn try_from_toml_value(v: toml::Value) -> Result<Config> {
        let root_map = match v {
            toml::Value::Table(m) => m,
            _ => return Err(IntegrityError::type_mismatch("(root)", "table", &v).into()),
        };

        let listen_addr;
        {
            let general_val = root_map
                .get("general")
                .ok_or(IntegrityError::entry_required("general"))?;

            let general_map = general_val.as_table().ok_or(IntegrityError::type_mismatch(
                "general",
                "table",
                general_val,
            ))?;

            let listen_addr_val = general_map
                .get("listen_addr")
                .ok_or(IntegrityError::entry_required("general.listen_addr"))?;

            listen_addr = listen_addr_val
                .as_str()
                .ok_or(IntegrityError::type_mismatch(
                    "general.listen_addr",
                    "string",
                    listen_addr_val,
                ))?
                .to_owned();
        }

        Ok(Self {
            listen_addr: listen_addr,
            raw_map: root_map,
        })
    }
    pub fn try_from_str<T: AsRef<str>>(s: T) -> Result<Config> {
        Self::try_from_toml_value(s.as_ref().parse::<toml::Value>()?)
    }

    pub fn try_from_cfg_file<T: AsRef<path::Path>>(path: T) -> Result<Config> {
        use std::io::Read;

        let mut cfg_file = fs::File::open(path.as_ref())?;
        let mut cfg_string = String::new();

        cfg_file.read_to_string(&mut cfg_string)?;

        Self::try_from_str(&cfg_string)
    }
}
