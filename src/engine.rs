use crate::command::Command;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CorrectedCommand {
    pub rule_name: String,
    pub new_script: String,
    pub priority: i32,
}

pub trait Rule {
    fn name(&self) -> &'static str;
    fn requires_output(&self) -> bool { true }
    fn priority(&self) -> i32 { 1000 }
    fn enabled_by_default(&self) -> bool { true }
    fn matches(&self, command: &Command) -> bool;
    fn generate(&self, command: &Command) -> Vec<String>;
    fn side_effect(&self, _old: &Command, _new_script: &str) -> Result<()> { Ok(()) }
}

pub struct Engine<'a> {
    pub rules: Vec<&'a dyn Rule>,
}

impl<'a> Engine<'a> {
    pub fn new(rules: Vec<&'a dyn Rule>) -> Self { Self { rules } }

    pub fn suggest(&self, command: &Command) -> Vec<CorrectedCommand> {
        let mut suggestions: Vec<CorrectedCommand> = Vec::new();
        for rule in &self.rules {
            if rule.requires_output() && command.output.is_none() { continue; }
            if !rule.matches(command) { continue; }
            let base_priority = rule.priority();
            for (idx, new_script) in rule.generate(command).into_iter().enumerate() {
                // Slightly increase internal order for later suggestions from same rule
                let effective_pri = base_priority + (idx as i32);
                suggestions.push(CorrectedCommand {
                    rule_name: rule.name().to_string(),
                    new_script,
                    priority: effective_pri,
                });
            }
        }
        suggestions.sort_by_key(|c| c.priority);
        suggestions
    }
}

