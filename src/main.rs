use clap::{Parser, ValueEnum};
use fix_command::app::{App, AppConfig};
use fix_command::output_reader::OutputMode;
use fix_command::shell::alias::{alias_snippet, ShellKind};

#[derive(Parser, Debug)]
#[command(name = "fix", version, about = "Suggest and run corrections for your previous shell command.")]
struct Cli {
    /// Apply to provided raw command (used by shell alias)
    #[arg(long = "apply")]
    apply: Option<String>,

    /// Print shell alias/function
    #[arg(long = "alias")]
    alias: bool,

    /// Shell kind for alias printing
    #[arg(long = "shell")]
    shell: Option<ShellOpt>,

    /// Enable experimental instant mode in alias or runtime
    #[arg(long = "enable-experimental-instant-mode")]
    instant_mode: bool,

    /// Auto-accept and run first suggestion
    #[arg(short = 'y', long = "yeah")]
    yeah: bool,

    /// Enable debug logging
    #[arg(long = "debug")]
    debug: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ShellOpt { Bash, Zsh, Fish, Pwsh, Cmd }

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.alias {
        let kind = cli.shell.map(|s| match s {
            ShellOpt::Bash => ShellKind::Bash,
            ShellOpt::Zsh => ShellKind::Zsh,
            ShellOpt::Fish => ShellKind::Fish,
            ShellOpt::Pwsh => ShellKind::Pwsh,
            ShellOpt::Cmd => ShellKind::Cmd,
        });
        let snippet = alias_snippet(kind, cli.instant_mode);
        println!("{}", snippet);
        return Ok(());
    }

    if cli.debug { env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init(); }
    else { env_logger::init(); }

    let cfg = AppConfig {
        yeah: cli.yeah,
        debug: cli.debug,
        apply_script: cli.apply,
        output_mode: if cli.instant_mode { OutputMode::Instant } else { OutputMode::Rerun },
    };
    let code = App::run(cfg)?;
    std::process::exit(code);
}
