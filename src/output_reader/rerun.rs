use crate::output_reader::OutputReader;
use anyhow::{anyhow, Context, Result};
use std::io::Read;
use std::process::{Command as PCommand, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;
#[cfg(unix)]
use std::os::unix::process::CommandExt;

pub struct RerunOutputReader;

impl OutputReader for RerunOutputReader {
    fn read_output(&self, raw_cmd: &str, timeout_secs: u64) -> Result<String> {
        // Execute via user's shell to interpret as typed
        #[cfg(target_os = "windows")]
        let mut child = PCommand::new("cmd")
            .arg("/C").arg(raw_cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn cmd for: {}", raw_cmd))?;

        #[cfg(not(target_os = "windows"))]
        let mut child = {
            let mut cmd = PCommand::new("sh");
            cmd.arg("-c").arg(raw_cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            #[cfg(unix)]
            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }
            cmd.spawn().with_context(|| format!("Failed to spawn shell for: {}", raw_cmd))?
        };

        let status_opt = child.wait_timeout(Duration::from_secs(timeout_secs))?;
        match status_opt {
            Some(_status) => {
                let mut buf = String::new();
                if let Some(mut out) = child.stdout.take() { let _ = out.read_to_string(&mut buf); }
                if let Some(mut err) = child.stderr.take() { let _ = err.read_to_string(&mut buf); }
                Ok(buf)
            }
            None => {
                // Timed out. Kill process group on Unix, process on Windows.
                #[cfg(unix)]
                unsafe {
                    let pgid = -(child.id() as i32);
                    libc::kill(pgid, libc::SIGKILL);
                }
                #[cfg(windows)]
                {
                    let _ = child.kill();
                }
                let _ = child.wait();
                Err(anyhow!("Timed out after {}s", timeout_secs))
            }
        }
    }
}

