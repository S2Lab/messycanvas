use std::collections;
use std::fs;
use std::io;
use std::path;
use std::result;

use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Toml(#[cause] toml::de::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Entry `{}` {}.", entry, explanation)]
    Integrity { entry: String, explanation: String },
}

impl Error {
    pub fn entry_required<T: AsRef<str>>(entry: T) -> Self {
        Error::Integrity {
            entry: entry.as_ref().to_owned(),
            explanation: "is required".to_owned(),
        }
    }

    pub fn type_mismatch<T: AsRef<str>, U: AsRef<str>>(
        entry: T,
        type_expected: U,
        val: &toml::Value,
    ) -> Self {
        Error::Integrity {
            entry: entry.as_ref().to_owned(),
            explanation: format!(
                "has to be a(n) {}, found {}",
                type_expected.as_ref(),
                val.type_str()
            ),
        }
    }
}

impl_from!(Error::Toml, toml::de::Error);
impl_from!(Error::Io, io::Error);

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub raw_map: collections::BTreeMap<String, toml::Value>,
}

impl Config {
    pub fn try_from_toml_value(v: toml::Value) -> Result<Config> {
        let root_map = match v {
            toml::Value::Table(m) => m,
            _ => return Err(Error::type_mismatch("(root)", "table", &v).into()),
        };

        let listen_addr;
        {
            let general_map = root_map
                .get("general")
                .ok_or(Error::entry_required("general"))
                .and_then(|m| {
                    m.as_table()
                        .ok_or(Error::type_mismatch("general", "table", m))
                })?;

            listen_addr = general_map
                .get("listen_addr")
                .ok_or(Error::entry_required("general.listen_addr"))
                .and_then(|m| {
                    m.as_str()
                        .ok_or(Error::type_mismatch("general.listen_addr", "string", m))
                })?
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

    pub fn try_from_cfg_path<T: AsRef<path::Path>>(path: T) -> Result<Config> {
        use std::io::Read;

        let mut cfg_file = fs::File::open(path.as_ref())?;
        let mut cfg_string = String::new();

        cfg_file.read_to_string(&mut cfg_string)?;

        let config = Self::try_from_str(&cfg_string);
        debug!(
            "Reading configuration from path: {}",
            path.as_ref().display()
        );
        config
    }
}
