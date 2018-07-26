use super::cli;
use super::exit;
use super::logging;

/// Canvasctl main function flow:
/// 1. Parse command line arguments
/// 2. Initialize logger
/// 3. Dispatch command to corresponding function
/// 4. Return
pub fn main() {
    let parsed_args =
        cli::CanvasCtlArgs::parse_args().unwrap_or_else(|e| exit::cleanup_and_exit(e.exit_code()));
    logging::init_logger(parsed_args.verbose);
}
