# Settings

Location: `$XDG_CONFIG_HOME/fix-command/config.toml` (typically `~/.config/fix-command/config.toml`).

Sample `config.toml`:

```toml
require_confirmation = true
wait_command = 10
wait_slow_command = 20
slow_commands = ["react-native", "gradle"]
no_colors = false
num_close_matches = 3
# rules = ["sudo", "no_command", "cd_correction"]
# exclude_rules = ["some_rule"]
```

Environment variables (prefix `FIX_`):

- `FIX_RULES`: colon-separated list of enabled rules.
- `FIX_EXCLUDE_RULES`: colon-separated list of disabled rules.
- `FIX_REQUIRE_CONFIRMATION`: `true/false`.
- `FIX_WAIT_COMMAND`: seconds.
- `FIX_WAIT_SLOW_COMMAND`: seconds.
- `FIX_SLOW_COMMANDS`: colon-separated list.
- `FIX_NO_COLORS`: `true/false`.
- `FIX_NUM_CLOSE_MATCHES`: integer.
