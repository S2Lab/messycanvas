use super::cfg;

use std::result;

pub type ServerSpawnResult<T> = result::Result<T, ServerSpawnError>;

#[derive(Fail, Debug)]
pub enum ServerSpawnError {
    #[fail(display = "Listening address changed, restarting server...")]
    AddressChanged,
}

pub struct Server;

impl Server {
    pub fn run_forever(config: cfg::Config) -> ServerSpawnResult<()> {
        Ok(())
    }
}
