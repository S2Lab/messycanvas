use super::cli;
use super::logging;

/// Canvasctl main function flow:
/// 1. Parse command line arguments
/// 2. Initialize logger
/// 3. Dispatch command to corresponding function
/// 4. Return
pub fn main() {
    let parsed_args = match cli::CanvasCtlArgs::parse_args() {
        Ok(m) => m,
        Err(e) => {
            return e.print_and_exit();
        }
    };
    logging::init_logger(parsed_args.verbose);
}
