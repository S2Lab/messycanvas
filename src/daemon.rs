use super::cfg;
use super::cli;
use super::logging;

use std::process;

/// Canvasd main function flow:
/// 1.  [x] Parse command line arguments
/// 2.  [x] Initialize logger
/// 3.  [x] Read & parse config
/// 4.  [ ] Register backends
/// 5.  [ ] Bind address and start server
/// 6.  [ ] Run forever
/// ...
/// 7.  [ ] Interrupt/Signal received
/// 8.  [ ] Prevent new requests & unbind address
/// 9.  [ ] Wait current requests to finish
/// 10. [ ] Unregister backends
/// 11. [ ] Shutdown server
/// 12. [ ] Return
pub fn main() {
    let parsed_args = match cli::CanvasdArgs::parse_args() {
        Ok(m) => m,
        Err(e) => {
            return e.print_and_exit();
        }
    };
    logging::init_logger(parsed_args.verbose);
    debug!(
        "Reading configuration from path: {}",
        &parsed_args.config_path
    );
    let config = match cfg::Config::try_from_cfg_path(&parsed_args.config_path) {
        Ok(m) => m,
        Err(e) => {
            error!("Cannot read configuration because `{}`", e);
            process::exit(1);
        }
    };
    debug!("Listening at: {}", &config.listen_addr);
}
