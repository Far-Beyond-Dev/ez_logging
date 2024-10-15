use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use chrono::Local;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG_FILE: Mutex<std::fs::File> = Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("server.log")
            .expect("Failed to open log file")
    );
}

pub fn log_message(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("[{}] {}\n", timestamp, message);
    
    print!("{}", log_message);  // Print to console
    
    if let Ok(mut file) = LOG_FILE.lock() {
        let _ = file.write_all(log_message.as_bytes());
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::log_message(&message);
    }};
}

pub fn init() {
    hlog!("Logging system initialized");
}