pub fn log_warning(warn_message: &str) {
    println!("WARN: {}", warn_message);
}

pub fn log_unrecoverable_error(error_message: &str) {
    println!("ERROR: {}", error_message);
    panic!("Unexpected program end.")
}
