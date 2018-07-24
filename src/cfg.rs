use super::error::{Error, Result};

use std::collections;
use std::fs;
use std::path;

use toml;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub raw_map: collections::BTreeMap<String, toml::Value>,
}

impl Config {
    pub fn try_from_toml_value(v: toml::Value) -> Result<Config> {
        let root_map = match v {
            toml::Value::Table(m) => m,
            _ => return Err(Error::cfg_integrity_type_mismatch("(root)", "table", &v).into()),
        };

        let listen_addr;
        {
            let general_val = root_map
                .get("general")
                .ok_or(Error::cfg_integrity_entry_required("general"))?;

            let general_map = general_val
                .as_table()
                .ok_or(Error::cfg_integrity_type_mismatch(
                    "general",
                    "table",
                    general_val,
                ))?;

            let listen_addr_val = general_map
                .get("listen_addr")
                .ok_or(Error::cfg_integrity_entry_required("general.listen_addr"))?;

            listen_addr = listen_addr_val
                .as_str()
                .ok_or(Error::cfg_integrity_type_mismatch(
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
