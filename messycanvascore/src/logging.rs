use simple_logger;
use log;

pub fn init_logger(verbose: bool) {
    if verbose {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    }
}
