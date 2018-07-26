#[macro_use]
extern crate clap;
extern crate simple_logger;
extern crate toml;
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure_derive;
extern crate failure;

extern crate actix_web;
//extern crate futures;

#[macro_use]
mod priv_macro;

pub mod cfg;
mod cli;
mod exit;
mod logging;
mod srv;

// Daemon Entry
pub mod daemon;
// Client Entry
pub mod client;
