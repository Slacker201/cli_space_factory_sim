use std::sync::atomic::{AtomicBool, Ordering};

pub static SHOULD_INFO: AtomicBool = AtomicBool::new(false);
pub static SHOULD_WARN: AtomicBool = AtomicBool::new(false);
pub static SHOULD_ERROR: AtomicBool = AtomicBool::new(false);
pub static SHOULD_DEBUG: AtomicBool = AtomicBool::new(false);

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            if $crate::logging::logger::SHOULD_INFO.load(std::sync::atomic::Ordering::Relaxed) {
                println!("[Info] {}", format!($($arg)*));
            }
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            if $crate::logging::logger::SHOULD_WARN.load(std::sync::atomic::Ordering::Relaxed) {
                println!("[Warn] {}", format!($($arg)*));
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            if $crate::logging::logger::SHOULD_ERROR.load(std::sync::atomic::Ordering::Relaxed) {
                println!("[Error] {}", format!($($arg)*));
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            if $crate::logging::logger::SHOULD_DEBUG.load(std::sync::atomic::Ordering::Relaxed) {
                println!("[Debug] {}", format!($($arg)*));
            }
        }
    };
}
#[allow(dead_code)]
pub enum LoggingLevels {
    Info(bool),
    Warn(bool),
    Error(bool),
    Debug(bool),
}

pub fn set_params(params: Vec<LoggingLevels>) {
    for p in params {
        match p {
            LoggingLevels::Info(val) => {
                SHOULD_INFO.store(val, Ordering::Relaxed);
            }
            LoggingLevels::Warn(val) => {
                SHOULD_WARN.store(val, Ordering::Relaxed);
            }
            LoggingLevels::Error(val) => {
                SHOULD_ERROR.store(val, Ordering::Relaxed);
            }
            LoggingLevels::Debug(val) => {
                SHOULD_DEBUG.store(val, Ordering::Relaxed);
            }
        }
    }
}
