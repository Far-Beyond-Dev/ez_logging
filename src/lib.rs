use std::sync::OnceLock;
use chrono::Local;

// import async specific stuff, if enabled
#[cfg(feature = "async-tokio")]
use tokio::fs::File;
#[cfg(feature = "async-tokio")]
use tokio::sync::Mutex;
#[cfg(feature = "async-tokio")]
use tokio::io::AsyncWriteExt;
#[cfg(feature = "async-tokio")]
use tokio::fs::OpenOptions;

// and then blocking specific stuff, if enabled
#[cfg(not(feature = "async-tokio"))]
use std::fs::File;
#[cfg(not(feature = "async-tokio"))]
use std::sync::Mutex;
#[cfg(not(feature = "async-tokio"))]
use std::io::Write;
#[cfg(not(feature = "async-tokio"))]
use std::fs::OpenOptions;

static LOG_FILE: OnceLock<Mutex<File>> = OnceLock::new();

#[cfg(not(feature = "async-tokio"))]
pub fn log_message(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("[{}] {}\n", timestamp, message);

    print!("{}", log_message);  // Print to console

    if let Some(lock) = LOG_FILE.get() {
        if let Ok(mut file) = lock.lock() {
            let _ = file.write_all(log_message.as_bytes());
        }
    }
}

#[cfg(feature = "async-tokio")]
pub async fn log_message(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("[{}] {}\n", timestamp, message);

    print!("{}", log_message);  // Print to console

    if let Some(lock) = LOG_FILE.get() {
        let mut file = lock.lock().await;
        let _ = file.write_all(log_message.as_bytes()).await;
    }
}

#[cfg(feature = "async-tokio")]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::log_message(&message).await;
    }};
}
#[cfg(not(feature = "async-tokio"))]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::log_message(&message);
    }};
}

#[cfg(not(feature = "async-tokio"))]
pub fn init() {
    let log_file: Mutex<File> = Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("server.log")
            .expect("Failed to open log file")
            .into()
    );
    LOG_FILE.set(log_file).expect("Failed to initialize LOG_FILE");

    println!("Logging system initialized");
}

#[cfg(feature = "async-tokio")]
pub async fn init() {
    let log_file: Mutex<File> = Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("server.log")
            .await
            .expect("Failed to open log file")
            .into()
    );
    LOG_FILE.set(log_file).expect("Failed to initialize LOG_FILE");

    println!("Logging system initialized");
}

