use clap::ValueEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ShellKind {
    Bash,
    Zsh,
    Fish,
    Pwsh,
    Cmd,
}

pub fn alias_snippet(shell: Option<ShellKind>, enable_instant: bool) -> String {
    let sk = shell.unwrap_or_else(detect_shell);
    match sk {
        ShellKind::Bash | ShellKind::Zsh => bash_like_alias(enable_instant),
        ShellKind::Fish => fish_alias(enable_instant),
        ShellKind::Pwsh => pwsh_alias(enable_instant),
        ShellKind::Cmd => cmd_alias(),
    }
}

fn bash_like_alias(_instant: bool) -> String {
    // instant ignored in stub
    let body = r#"fix() {
  local previous=$(fc -ln -2 | head -n 1)
  FIX_HISTORY=$(history 2 | sed 's/^ *[0-9]\+ *//') command fix --apply "$previous" "$@"
}
alias fx=fix
"#;
    body.to_string()
}

fn fish_alias(_instant: bool) -> String {
    let body = r#"function fix
  set -l prev (history | head -n 1)
  env FIX_HISTORY=(history | string join "\n") fix --apply "$prev" $argv
end
alias fx=fix
"#;
    body.to_string()
}

fn pwsh_alias(_instant: bool) -> String {
    let body = r#"function global:fix { param([Parameter(ValueFromRemainingArguments=$true)][string[]]$args)
  $prev = (Get-History -Count 1).CommandLine
  $env:FIX_HISTORY = (Get-History | ForEach-Object CommandLine) -join "`n"
  fix --apply "$prev" @args
}
Set-Alias fx fix
"#;
    body.to_string()
}

fn cmd_alias() -> String {
    // A .bat wrapper would be more accurate; here we print a FOR macro snippet.
    let body = r#"@echo off
REM Add to cmd alias system manually; for demo, call PowerShell wrapper if available.
"#;
    body.to_string()
}

fn detect_shell() -> ShellKind {
    // Simple heuristic: check SHELL/ComSpec
    if let Ok(s) = std::env::var("SHELL") {
        if s.contains("zsh") { return ShellKind::Zsh; }
        if s.contains("bash") { return ShellKind::Bash; }
        if s.contains("fish") { return ShellKind::Fish; }
    }
    if std::env::var("ComSpec").is_ok() { return ShellKind::Cmd; }
    ShellKind::Bash
}

