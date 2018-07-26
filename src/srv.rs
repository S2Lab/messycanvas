use super::cfg;

use std::io;
use std::result;

use actix_web;

pub type ServerSpawnResult<T> = result::Result<T, ServerSpawnError>;

#[derive(Fail, Debug)]
pub enum ServerSpawnError {
    #[fail(display = "Listening address changed, restarting server...")]
    AddressChanged,
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
}

impl_from!(ServerSpawnError::Io, io::Error);

pub struct Server;

impl Server {
    pub fn run_forever(config: cfg::Config) -> ServerSpawnResult<()> {
        actix_web::server::new(|| {
            actix_web::App::new().resource("/", |r| r.f(|_req| "Hello world!"))
        }).bind(&config.listen_addr)
            .map_err(|e| {
                debug!("Cannot bind address `{}`", &config.listen_addr);
                e
            })
            .map(|m| {
                debug!("Listening at: {}", &config.listen_addr);
                m
            })?
            .run();

        Ok(())
    }
}
