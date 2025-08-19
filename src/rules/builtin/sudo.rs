use crate::command::Command;
use crate::engine::Rule;

pub struct SudoRule;

impl Rule for SudoRule {
    fn name(&self) -> &'static str { "sudo" }
    fn priority(&self) -> i32 { 100 }
    fn matches(&self, command: &Command) -> bool {
        if let Some(out) = &command.output {
            let out_l = out.to_lowercase();
            out_l.contains("permission denied") || out.contains("EACCES")
        } else { false }
    }
    fn generate(&self, command: &Command) -> Vec<String> {
        vec![format!("sudo {}", command.script)]
    }
}

