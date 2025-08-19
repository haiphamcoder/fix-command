pub mod rerun;
pub mod instant;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    Rerun,
    Instant,
}

pub trait OutputReader {
    fn read_output(&self, raw_cmd: &str, timeout_secs: u64) -> Result<String>;
}

