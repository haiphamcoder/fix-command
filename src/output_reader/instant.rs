use crate::output_reader::OutputReader;
use anyhow::Result;

pub struct InstantOutputReader;

impl OutputReader for InstantOutputReader {
    fn read_output(&self, _raw_cmd: &str, _timeout_secs: u64) -> Result<String> {
        // Experimental stub: real implementation would tail a pty/script log.
        // For now, return Err to signal fallback to rerun.
        Err(anyhow::anyhow!("instant-mode not implemented; falling back"))
    }
}

