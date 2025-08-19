# Shell Integration

Print alias/function snippet:

```bash
fix --alias [--shell bash|zsh|fish|pwsh|cmd] [--enable-experimental-instant-mode]
```

## Bash / Zsh

Uses `fc -ln -1` to fetch the previous command. Example:

```bash
eval "$(fix --alias)"
```

## Fish

```bash
fix --alias --shell fish
```

## PowerShell

```powershell
fix --alias --shell pwsh | Invoke-Expression
```

## Windows cmd

Command prompt has limited aliasing. Printing a small wrapper is supported; for robust usage, prefer PowerShell.
