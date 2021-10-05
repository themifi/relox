use std::fmt::Display;

pub fn report<T: Display>(e: T) {
    eprintln!("{}", e);
    unsafe {
        HAD_ERROR = true;
    }
}

pub static mut HAD_ERROR: bool = false;

pub fn format_error<T: AsRef<str>>(line: usize, message: T) -> String {
    format!("[line {}] Error: {}", line, message.as_ref())
}
