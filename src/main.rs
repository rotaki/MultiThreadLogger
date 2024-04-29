use chrono::Local;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Mutex;
use std::thread;

thread_local! {
    static LOG_FILE: RefCell<io::Result<File>> = RefCell::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!("log-{:?}.txt", std::thread::current().id()))
    );
}

#[inline]
fn log(level: &str, file: &str, line: u32, message: &str) {
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!(
        "{} [{}] {}:{} - {}\n",
        formatted_time, level, file, line, message
    );

    LOG_FILE.with(|log_file| {
        if let Ok(writer) = &mut *log_file.borrow_mut() {
            let _ = writer.write_all(log_message.as_bytes());
            let _ = writer.flush();
        }
    });
}

/*
lazy_static! {
    static ref LOG_FILE: Mutex<io::Stdout> = Mutex::new(io::stdout());
}

#[inline]
fn log(level: &str, file: &str, line: u32, message: &str) {
    let now = Local::now();
    let thread_id = thread::current().id();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!(
        "{} [{}] [thread {:?}] {}:{} - {}\n",
        formatted_time, level, thread_id, file, line, message
    );

    // Write log_message to stdout
    let mut stdout = LOG_FILE.lock().unwrap();
    let _ = stdout.write_all(log_message.as_bytes());
    let _ = stdout.flush();
}
*/

macro_rules! log_error {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "log_error", feature = "log_warn", feature = "log_info", feature = "log_debug", feature = "log_trace"))]
        log("ERROR", file!(), line!(), &format!($($arg)*))
    }
}

macro_rules! log_warn {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "log_warn", feature = "log_info", feature = "log_debug", feature = "log_trace"))]
        log("WARN ", file!(), line!(), &format!($($arg)*))
    }
}

macro_rules! log_info {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "log_info", feature = "log_debug", feature = "log_trace"))]
        log("INFO ", file!(), line!(), &format!($($arg)*))
    }
}

macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "log_debug", feature = "log_trace"))]
        log("DEBUG", file!(), line!(), &format!($($arg)*))
    }
}

macro_rules! log_trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "log_trace")]
        log("TRACE", file!(), line!(), &format!($($arg)*))
    }
}

fn main() {
    // multiple threads
    let handles: Vec<_> = (0..5)
        .map(|_| {
            std::thread::spawn(|| {
                log_error!("This is an error message");
                log_warn!("This is a warning message");
                log_info!("This is an info message");
                log_debug!("This is a debug message");
                log_trace!("This is a trace message");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
