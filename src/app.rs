use crate::command::Command;
use crate::engine::{Engine, Rule};
use crate::output_reader::{OutputMode, OutputReader};
use crate::output_reader::rerun::RerunOutputReader;
#[cfg(feature = "instant-mode")]
use crate::output_reader::instant::InstantOutputReader;
use crate::rules::builtin::{sudo::SudoRule, no_command::NoCommandRule, cd_correction::CdCorrectionRule};
use crate::settings::Settings;
use crate::tui::select_suggestion;
use crate::util::execute_script;
use anyhow::{anyhow, Result};
use log::debug;

pub struct AppConfig {
    pub yeah: bool,
    pub debug: bool,
    pub apply_script: Option<String>,
    pub output_mode: OutputMode,
}

pub struct App;

impl App {
    pub fn run(cfg: AppConfig) -> Result<i32> {
        let (settings, _path) = Settings::load(cfg.debug)?;

        let raw = cfg.apply_script.ok_or_else(|| anyhow!("--apply is required (use --alias to set shell function)"))?;

        // Timeout selection: consider slow commands
        let head = raw.split_whitespace().next().unwrap_or("");
        let is_slow = settings.slow_commands.as_ref().map(|lst| lst.iter().any(|s| s == head)).unwrap_or(false);
        let timeout = if is_slow { settings.wait_slow_command.unwrap_or(20) } else { settings.wait_command.unwrap_or(10) };
        let reader = match cfg.output_mode {
            OutputMode::Rerun => Reader::Rerun,
            OutputMode::Instant => Reader::Instant,
        };
        debug!("Collecting output using {:?}", cfg.output_mode);
        let output = reader.read(&raw, timeout);
        debug!("Captured output len={} (ok={})", output.as_ref().map(|s| s.len()).unwrap_or(0), output.is_ok());

        let command = Command::new(raw.clone(), output.ok()).unwrap_or(Command { script: raw.clone(), output: None, parts: vec![] });
        let rules: Vec<&dyn Rule> = vec![&SudoRule, &CdCorrectionRule, &NoCommandRule];
        let engine = Engine::new(rules);
        let suggestions = engine.suggest(&command);
        if suggestions.is_empty() {
            println!("No suggestions.");
            return Ok(0);
        }
        let scripts: Vec<String> = suggestions.iter().map(|c| c.new_script.clone()).collect();
        let run_script = if cfg.yeah || settings.require_confirmation.unwrap_or(true) == false {
            scripts.first().cloned()
        } else {
            select_suggestion(&scripts)?
        };
        if let Some(script) = run_script {
            debug!("Executing: {}", script);
            let code = execute_script(&script)?;
            return Ok(code);
        }
        println!("Aborted");
        Ok(0)
    }
}

enum Reader {
    Rerun,
    Instant,
}

impl Reader {
    fn read(&self, raw: &str, timeout: u64) -> Result<String> {
        match self {
            Reader::Rerun => {
                let r = RerunOutputReader;
                r.read_output(raw, timeout)
            }
            Reader::Instant => {
                #[cfg(feature = "instant-mode")]
                {
                    let r = InstantOutputReader;
                    r.read_output(raw, timeout).or_else(|_| {
                        let rr = RerunOutputReader;
                        rr.read_output(raw, timeout)
                    })
                }
                #[cfg(not(feature = "instant-mode"))]
                {
                    let rr = RerunOutputReader;
                    rr.read_output(raw, timeout)
                }
            }
        }
    }
}

