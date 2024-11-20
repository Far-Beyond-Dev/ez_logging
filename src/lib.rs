use std::sync::OnceLock;
use chrono::Local;

#[cfg(feature = "async-tokio")]
use tokio::fs::File;
#[cfg(feature = "async-tokio")]
use tokio::sync::Mutex;
#[cfg(feature = "async-tokio")]
use tokio::io::AsyncWriteExt;
#[cfg(feature = "async-tokio")]
use tokio::fs::OpenOptions;

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
pub fn log_message(message: &str) -> std::io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("[{}] {}\n", timestamp, message);
    print!("{}", log_message); // Print to console
    
    if let Some(lock) = LOG_FILE.get() {
        let mut file = lock.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to acquire lock")
        })?;
        file.write_all(log_message.as_bytes())?;
        file.flush()?;  // Ensure the data is written to disk
    }
    Ok(())
}

#[cfg(feature = "async-tokio")]
pub async fn log_message(message: &str) -> std::io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("[{}] {}\n", timestamp, message);
    print!("{}", log_message); // Print to console
    
    if let Some(lock) = LOG_FILE.get() {
        let mut file = lock.lock().await;
        file.write_all(log_message.as_bytes()).await?;
        file.flush().await?;  // Ensure the data is written to disk
    }
    Ok(())
}

#[cfg(feature = "async-tokio")]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        if let Err(e) = $crate::log_message(&message).await {
            eprintln!("Logging error: {}", e);
        }
    }};
}

#[cfg(not(feature = "async-tokio"))]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        if let Err(e) = $crate::log_message(&message) {
            eprintln!("Logging error: {}", e);
        }
    }};
}

#[cfg(not(feature = "async-tokio"))]
pub fn init() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("server.log")?;
        
    let log_file: Mutex<File> = Mutex::new(file);
    LOG_FILE.set(log_file)
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initialize LOG_FILE"
        ))?;
        
    println!("Logging system initialized");
    Ok(())
}

#[cfg(feature = "async-tokio")]
pub async fn init() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("server.log")
        .await?;
        
    let log_file: Mutex<File> = Mutex::new(file);
    LOG_FILE.set(log_file)
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initialize LOG_FILE"
        ))?;
        
    println!("Logging system initialized");
    Ok(())
}