# Architecture

Flow: alias → capture previous command → construct `Command` → Rule Engine → TUI picker → execute.

## Command model

`Command { script: String, output: Option<String>, parts: Vec<String> }`, where `parts` is shell-aware split.

## Rule engine

- `Rule` trait: `name`, `requires_output` (default true), `priority` (lower is better), `enabled_by_default` (default true), `matches(&Command)`, `generate(&Command) -> Vec<String>`, optional `side_effect`.
- Engine gathers suggestions from all rules that match. Suggestions are sorted by priority; if a rule returns multiple suggestions, later ones get a slight priority offset.

## Output acquisition

- Default: re-run the previous command in a shell with timeout. On timeout, kill the whole process tree. Two timeouts exist: `wait_command` and `wait_slow_command` (if command head matches entries in `slow_commands`).
- Instant mode (experimental feature): a placeholder that would read from a session log to avoid re-run. Currently falls back to re-run.

## Shell integration

`fix --alias [--shell ...] [--enable-experimental-instant-mode]` prints a function/alias for bash/zsh/fish/PowerShell/cmd that retrieves the previous command and calls `fix --apply "<raw>"`.

## UX

- Interactive by default: shows a list, navigate with ↑/↓ and Enter to run.
- `-y/--yeah` skips the picker and runs the top suggestion.
- If no suggestions, prints "No suggestions." and exits 0.
