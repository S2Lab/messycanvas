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

pub mod cfg;
mod cli;
pub mod error;
mod logging;
mod srv;

// Daemon Entry
pub mod daemon;
// Client Entry
pub mod client;
