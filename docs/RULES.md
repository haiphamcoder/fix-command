# Rules

## Builtin rules

- `sudo` (priority 100, requires output): if output contains "permission denied" or `EACCES`, suggests `sudo <command>`.
- `cd_correction` (priority 150, requires output): when `cd <dir>` fails with not found, suggests closest directories in CWD.
- `no_command` (priority 200, requires output): when output looks like command-not-found, suggests close matches of common binaries on PATH.

## Rule API

```rust
pub trait Rule {
    fn name(&self) -> &'static str;
    fn requires_output(&self) -> bool { true }
    fn priority(&self) -> i32 { 1000 }
    fn enabled_by_default(&self) -> bool { true }
    fn matches(&self, command: &Command) -> bool;
    fn generate(&self, command: &Command) -> Vec<String>;
    fn side_effect(&self, _old: &Command, _new_script: &str) -> Result<()> { Ok(()) }
}
```

## Custom/scripted rules

Planned optional support via `rhai` feature. Not implemented in this release.
