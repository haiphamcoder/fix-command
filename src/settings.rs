use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub rules: Option<Vec<String>>,            // FIX_RULES
    pub exclude_rules: Option<Vec<String>>,    // FIX_EXCLUDE_RULES
    pub require_confirmation: Option<bool>,    // FIX_REQUIRE_CONFIRMATION
    pub wait_command: Option<u64>,             // FIX_WAIT_COMMAND
    pub wait_slow_command: Option<u64>,        // FIX_WAIT_SLOW_COMMAND
    pub slow_commands: Option<Vec<String>>,    // FIX_SLOW_COMMANDS
    pub no_colors: Option<bool>,               // FIX_NO_COLORS
    pub num_close_matches: Option<usize>,      // FIX_NUM_CLOSE_MATCHES
    pub debug: Option<bool>,                   // via CLI
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rules: None,
            exclude_rules: None,
            require_confirmation: Some(true),
            wait_command: Some(10),
            wait_slow_command: Some(20),
            slow_commands: Some(vec!["gradle".into(), "react-native".into()]),
            no_colors: Some(false),
            num_close_matches: Some(3),
            debug: Some(false),
        }
    }
}

impl Settings {
    pub fn load(debug_flag: bool) -> Result<(Self, PathBuf)> {
        let proj = ProjectDirs::from("com", "example", "fix-command");
        let mut path = if let Some(d) = directories::BaseDirs::new() { d.config_dir().to_path_buf() } else { PathBuf::from("~/.config") };
        if let Some(proj) = proj {
            path = proj.config_dir().to_path_buf();
        }
        let file = path.join("config.toml");
        let mut settings: Settings = if file.exists() {
            let s = fs::read_to_string(&file)?;
            toml::from_str(&s)?
        } else {
            Settings::default()
        };
        settings.apply_env();
        if debug_flag { settings.debug = Some(true); }
        Ok((settings, file))
    }

    fn apply_env(&mut self) {
        if let Ok(val) = env::var("FIX_RULES") {
            let list = val.split(':').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            self.rules = Some(list);
        }
        if let Ok(val) = env::var("FIX_EXCLUDE_RULES") {
            let list = val.split(':').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            self.exclude_rules = Some(list);
        }
        if let Ok(val) = env::var("FIX_REQUIRE_CONFIRMATION") {
            self.require_confirmation = Some(is_true(&val));
        }
        if let Ok(val) = env::var("FIX_WAIT_COMMAND") {
            if let Ok(v) = val.parse() { self.wait_command = Some(v); }
        }
        if let Ok(val) = env::var("FIX_WAIT_SLOW_COMMAND") {
            if let Ok(v) = val.parse() { self.wait_slow_command = Some(v); }
        }
        if let Ok(val) = env::var("FIX_SLOW_COMMANDS") {
            let list = val.split(':').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            self.slow_commands = Some(list);
        }
        if let Ok(val) = env::var("FIX_NO_COLORS") {
            self.no_colors = Some(is_true(&val));
        }
        if let Ok(val) = env::var("FIX_NUM_CLOSE_MATCHES") {
            if let Ok(v) = val.parse() { self.num_close_matches = Some(v); }
        }
    }
}

fn is_true(s: &str) -> bool {
    matches!(s.to_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

