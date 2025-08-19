use anyhow::Result;
use std::process::Command as PCommand;

pub fn execute_script(script: &str) -> Result<i32> {
    #[cfg(target_os = "windows")]
    let status = PCommand::new("cmd").arg("/C").arg(script).status()?;
    #[cfg(not(target_os = "windows"))]
    let status = PCommand::new("sh").arg("-c").arg(script).status()?;
    Ok(status.code().unwrap_or(-1))
}

