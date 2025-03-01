use serde::Serialize;

/// The run mode of the application.
pub type RunMode = &'static str;

pub const DEV: RunMode = "development";
pub const PROD: RunMode = "production";
pub const TEST: RunMode = "test";
#[repr(usize)]
#[derive(Debug, Serialize)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}
