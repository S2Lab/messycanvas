use messycanvascore::cli;

use pretty_logger;
use log;

pub fn init_logger(parsed_args: &cli::CanvasdArgs) {
    if parsed_args.verbose {
        pretty_logger::init_level(log::LogLevelFilter::Debug).unwrap();
    } else {
        pretty_logger::init_level(log::LogLevelFilter::Info).unwrap();
    }
}
