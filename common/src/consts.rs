/// The run mode of the application.
pub type RunMode = &'static str;

pub const LOCAL: RunMode = "local";
pub const DEV: RunMode = "development";
pub const PROD: RunMode = "production";
pub const TEST: RunMode = "test";
