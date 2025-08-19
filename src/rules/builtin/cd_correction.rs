use crate::command::Command;
use crate::engine::Rule;
use std::fs;
use strsim::levenshtein;

pub struct CdCorrectionRule;

impl Rule for CdCorrectionRule {
    fn name(&self) -> &'static str { "cd_correction" }
    fn priority(&self) -> i32 { 150 }
    fn matches(&self, command: &Command) -> bool {
        // Trigger on failed cd-like commands
        if command.parts.first().map(|s| s == "cd").unwrap_or(false) {
            // If output indicates no such file or directory
            if let Some(out) = &command.output {
                let out_l = out.to_lowercase();
                return out_l.contains("no such file") || out_l.contains("not found")
                    || out_l.contains("cd: ") || out_l.contains("cannot find the path");
            }
        }
        false
    }

    fn generate(&self, command: &Command) -> Vec<String> {
        if command.parts.len() < 2 { return vec![]; }
        let target = &command.parts[1];
        let mut candidates: Vec<(i32, String)> = Vec::new();
        if let Ok(entries) = fs::read_dir(".") {
            for e in entries.flatten() {
                if let Ok(md) = e.metadata() {
                    if md.is_dir() {
                        if let Some(name) = e.file_name().to_str() {
                            let d = levenshtein(target, name) as i32;
                            candidates.push((d, name.to_string()));
                        }
                    }
                }
            }
        }
        candidates.sort_by_key(|(d, _)| *d);
        candidates.into_iter().take(5).map(|(_, name)| format!("cd {}", name)).collect()
    }
}

