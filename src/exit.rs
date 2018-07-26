use std::process;

pub fn cleanup_and_exit(exit_code: i32) -> ! {
    process::exit(exit_code);
}
