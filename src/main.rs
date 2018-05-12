#[macro_use]
extern crate messycanvascore as core;

#[macro_use]
extern crate log;

use core::cli;
use core::logging;
use core::cfg;

use std::process;

fn main() {
    let crate_info = canvas_crate_info!();

    let parsed_args = cli::CanvasdArgs::parse_args(&crate_info);
    logging::init_logger(parsed_args.verbose);

    debug!("Read configuration from path: {}", &parsed_args.config_path);
    let config = match cfg::Config::try_from_cfg_file(&parsed_args.config_path) {
        Ok(m) => m,
        Err(e) => {
            error!("Cannot load config file because `{}`", e);
            process::exit(1);
        }
    };

    debug!("Listen at: {}", &config.listen_addr);
}
