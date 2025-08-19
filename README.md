# Fix-Command

Fix-command is a Rust CLI that suggests and runs corrections for your previous console command. It mirrors the core principles of the well-known tool but uses respectful naming. Binary: `fix`.

MSRV: Rust 1.70. Edition 2021.

## Install

```bash
cargo install --path .
```

## Quick start

Add the alias to your shell:

```bash
eval "$(fix --alias)"
# Or with instant-mode flag (experimental stub):
# eval "$(fix --alias --enable-experimental-instant-mode)"
```

Use it:

```bash
# 1) Mistyped command
gti status
fix            # shows suggestions (↑/↓ to navigate, Enter to run)

# 2) Permission denied
cat /root/secret
fix            # suggests: sudo cat /root/secret

# 3) cd typo
cd srcc
fix            # suggests closest directories in CWD
```

Flags:

- `-y, --yeah`: auto-run top suggestion
- `--debug`: verbose logs
- `--alias [--shell <bash|zsh|fish|pwsh|cmd>] [--enable-experimental-instant-mode]`: print alias for your shell

Examples:

```bash
gti status && fix
cat /root/secret && fix
cd srcc && fix
```

## Build

```bash
cargo build --release
```

## Notes

- Instant mode is experimental and currently a stub; it falls back to re-run.
- Cross-platform: Linux/macOS/Windows (cmd and PowerShell supported via alias printouts).

See `docs/` for architecture, rules, settings and shell integration details.
