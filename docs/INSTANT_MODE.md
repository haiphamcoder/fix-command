# Instant Mode (Experimental)

Instant mode avoids re-running the last command by reading a live session log (e.g., via a pty recorder like `script`). In this version, instant mode is a stub that falls back to the default re-run output reader.

Enable in alias printing and at runtime with:

```bash
fix --alias --enable-experimental-instant-mode
fix --enable-experimental-instant-mode --apply "<cmd>"
```

Current status: Not implemented. The code path will attempt instant mode and fall back to re-run automatically.
