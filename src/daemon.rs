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
/// 5.  [x] Bind address(es) and start server
/// 6.  [x] Run forever
/// ...
/// 7.  [ ] Interrupt/Signal received
/// 8.  [ ] Prevent new requests & unbind address(es)
/// 9.  [ ] Wait current requests to finish
/// 10. [ ] Unregister backends(Drop)
/// 11. [x] Return
pub fn main() {
    let parsed_args =
        cli::CanvasdArgs::parse_args().unwrap_or_else(|e| exit::cleanup_and_exit(e.exit_code()));
    logging::init_logger(parsed_args.verbose);

    loop {
        let config = match cfg::Config::try_from_cfg_path(&parsed_args.config_path) {
            Ok(m) => m,
            Err(e) => {
                error!("Cannot read configuration because `{}`", e);
                exit::cleanup_and_exit(1);
            }
        };

        match srv::Server::run_forever(config) {
            Ok(_) => break,
            Err(e) => match e {
                srv::ServerSpawnError::AddressChanged => continue,
                srv::ServerSpawnError::Io(io_e) => {
                    debug!("Cannot create server because `{}`", io_e);
                    exit::cleanup_and_exit(1);
                }
            },
        };
    }
}
