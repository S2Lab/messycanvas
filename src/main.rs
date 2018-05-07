#[macro_use]
extern crate messycanvascore as core;

#[macro_use]
extern crate log;

use core::cli;
use core::logging;

fn main() {
    let crate_info = canvas_crate_info!();

    let parsed_args = cli::CanvasdArgs::parse_args(&crate_info);
    logging::init_logger(parsed_args.verbose);

    debug!("Read configuration from path: {}", &parsed_args.config_path);
}
