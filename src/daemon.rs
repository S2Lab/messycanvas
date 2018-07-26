use super::cfg;
use super::cli;
use super::exit;
use super::logging;
use super::srv;

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
/// 10. [ ] Unregister backends(Drop)
/// 11. [ ] Return
pub fn main() {
    let parsed_args =
        cli::CanvasdArgs::parse_args().unwrap_or_else(|e| exit::cleanup_and_exit(e.exit_code()));
    logging::init_logger(parsed_args.verbose);
    debug!(
        "Reading configuration from path: {}",
        &parsed_args.config_path
    );

    loop {
        let config = match cfg::Config::try_from_cfg_path(&parsed_args.config_path) {
            Ok(m) => m,
            Err(e) => {
                error!("Cannot read configuration because `{}`", e);
                exit::cleanup_and_exit(1);
            }
        };
        debug!("Listening at: {}", &config.listen_addr);
        match srv::Server::run_forever(config) {
            Ok(_) => exit::cleanup_and_exit(0),
            Err(e) => match e {
                srv::ServerSpawnError::AddressChanged => {
                    continue;
                }
            },
        };
    }
}
