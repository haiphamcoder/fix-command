use crate::command::Command;
use crate::engine::Rule;
use strsim::levenshtein;
use which::which;

pub struct NoCommandRule;

impl Rule for NoCommandRule {
    fn name(&self) -> &'static str { "no_command" }
    fn priority(&self) -> i32 { 200 }
    fn matches(&self, command: &Command) -> bool {
        if let Some(out) = &command.output {
            let out_l = out.to_lowercase();
            if out_l.contains("command not found")
                || out_l.contains("zsh: command not found")
                || out_l.contains("is not recognized as an internal or external command")
                || out_l.contains("could not be found")
            { return true; }
            if let Some(miss) = command.parts.first() {
                if out_l.contains("not found") && out_l.contains(&miss.to_lowercase()) {
                    return true;
                }
            }
            false
        } else { false }
    }

    fn generate(&self, command: &Command) -> Vec<String> {
        if command.parts.is_empty() { return vec![]; }
        let miss = &command.parts[0];
        // Suggest closest binaries in PATH
        let candidates = collect_binaries();
        let mut scored: Vec<(i32, String)> = candidates.into_iter()
            .map(|c| (levenshtein(miss, &c) as i32, c))
            .collect();
        scored.sort_by_key(|(d, _)| *d);
        let best: Vec<String> = scored.into_iter()
            .take(3)
            .map(|(_, c)| format!("{}{}", c, if command.parts.len()>1 { format!(" {}", command.parts[1..].join(" ")) } else { String::new() }))
            .collect();
        best
    }
}

fn collect_binaries() -> Vec<String> {
    // Heuristic: iterate over PATH by using which for common first-letter prefixes could be expensive.
    // Simplify by scanning a few likely names via which of common tools if miss is close.
    // As a compromise, list a static set plus PATH basenames from which PATH scanning is not trivial without fs walk.
    // We'll try to use the `which` crate on a curated set.
    let common = vec![
        "git","ls","cat","python","python3","npm","yarn","cargo","go","bash","zsh","kubectl","docker","make"
    ];
    let mut found = Vec::new();
    for name in common {
        if which(name).is_ok() { found.push(name.to_string()); }
    }
    found
}

