use anyhow::Result;
use shell_words::split as shell_split;

#[derive(Debug, Clone)]
pub struct Command {
    pub script: String,
    pub output: Option<String>,
    pub parts: Vec<String>,
}

impl Command {
    pub fn new(script: String, output: Option<String>) -> Result<Self> {
        let parts = parse_script_parts(&script)?;
        Ok(Self { script, output, parts })
    }
}

fn parse_script_parts(script: &str) -> Result<Vec<String>> {
    // shell-aware split
    let parts = shell_split(script)?;
    Ok(parts)
}

