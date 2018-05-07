#[macro_use]
extern crate messycanvascore;

#[macro_use]
extern crate log;
extern crate pretty_logger;
extern crate time;

mod logging;

use messycanvascore::cli;

fn main() {
    let crate_info = crate_info!();

    let canvasd_args = cli::CanvasdArgs::parse_args(&crate_info);
    logging::init_logger(&canvasd_args);

    debug!(
        "Read configuration from path: {}",
        &canvasd_args.config_path
    );
}
