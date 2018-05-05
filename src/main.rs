#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate pretty_logger;

mod cli;

fn init_logger(parsed_args: &cli::ParsedArgs) {
    if parsed_args.verbose {
        pretty_logger::init_level(log::LogLevelFilter::Debug).unwrap();
    } else {
        pretty_logger::init_level(log::LogLevelFilter::Info).unwrap();
    }
}

fn main() {
    let parsed_args = cli::parse_args();
    init_logger(&parsed_args);

    debug!("Read configuration from path: {}", &parsed_args.config_path);
}
