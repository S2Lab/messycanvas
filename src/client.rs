use super::cli;
use super::logging;

use std::process;

/// Canvasctl main function flow:
/// 1. Parse command line arguments
/// 2. Initialize logger
/// 3. Dispatch command to corresponding function
/// 4. Return
pub fn main() {
    let parsed_args = match cli::CanvasCtlArgs::parse_args() {
        Some(m) => m,
        None => process::exit(1),
    };
    logging::init_logger(parsed_args.verbose);
}

//use core::cfg;
//use core::cli;
//use core::logging;

//use std::process;

// main_old() {
//  let crate_info = canvas_crate_info!();
//
//  let parsed_args = cli::CanvasdArgs::parse_args(&crate_info);
//  logging::init_logger(parsed_args.verbose);
//
//  debug!(
//      "Reading configuration from path: {}",
//      &parsed_args.config_path
//  );
//  let config = match cfg::Config::try_from_cfg_file(&parsed_args.config_path) {
//      Ok(m) => m,
//      Err(e) => {
//          error!("Cannot read configuration because `{}`", e);
//          process::exit(1);
//      }
//  };
//
//  debug!("Listening at: {}", &config.listen_addr);
//  let srv = server::CanvasApiServer::try_from_listen_addr(&config.listen_addr);
//
