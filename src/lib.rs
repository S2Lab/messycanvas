#[macro_use]
extern crate clap;
extern crate failure;
extern crate simple_logger;
#[macro_use]
extern crate failure_derive;
extern crate toml;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
//extern crate futures;
//extern crate hyper;
//#[cfg(unix)]
//extern crate hyperlocal;

#[macro_use]
mod priv_macro;

pub mod cfg;
mod cli;
mod logging;
mod srv;

// Daemon Entry
pub mod daemon;
// Client Entry
pub mod client;
