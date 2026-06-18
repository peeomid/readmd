use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, ValueHint};

use crate::error::{ReadmdError, Result};

const HELP_AFTER: &str = r#"Examples:
  readmd README.md
  readmd README.md --theme paper --style notebook
  readmd README.md --output public/readme.html
  readmd README.md --tmp --open
  readmd README.md --stdout
  readmd themes list
  readmd themes print paper
  readmd styles list
  readmd styles print notebook
  readmd config print-default

Theme and style:
  - --theme controls colors: paper, white, graphite, polar, sepia, midnight
  - --style controls typography and width: editorial, notebook, technical, large

Output behavior:
  - default: writes beside the input file with a .html extension
  - --output: writes to the chosen path
  - --tmp: writes to a temp file and prints the path
  - --stdout: prints HTML and writes no file
  - --open: opens the written file after save

Config lookup order:
  1. --config <path>
  2. ./readmd.toml
  3. ~/.config/readmd/config.toml
  4. built-in defaults
"#;

#[derive(Debug, Parser)]
#[command(
    name = "readmd",
    version,
    about = "Render one Markdown file into one standalone HTML reading page.",
    long_about = "readmd turns one Markdown file into one readable HTML page with inline CSS.",
    after_help = HELP_AFTER
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(
        value_name = "INPUT",
        value_hint = ValueHint::FilePath,
        help = "Markdown input file."
    )]
    pub input: Option<PathBuf>,

    #[arg(
        long,
        value_name = "PATH",
        value_hint = ValueHint::FilePath,
        help = "Write HTML to this path."
    )]
    pub output: Option<PathBuf>,

    #[arg(
        long,
        value_name = "NAME",
        help = "Color theme: paper, white, graphite, polar, sepia, or midnight."
    )]
    pub theme: Option<String>,

    #[arg(
        long,
        value_name = "NAME",
        help = "Reader style: editorial, notebook, technical, or large."
    )]
    pub style: Option<String>,

    #[arg(
        long,
        value_name = "PATH",
        value_hint = ValueHint::FilePath,
        help = "Read config from this TOML file."
    )]
    pub config: Option<PathBuf>,

    #[arg(
        long,
        conflicts_with_all = ["output", "tmp", "open"],
        help = "Print HTML to stdout and write no file."
    )]
    pub stdout: bool,

    #[arg(
        long,
        conflicts_with = "output",
        help = "Write HTML to a generated temp file."
    )]
    pub tmp: bool,

    #[arg(long, help = "Open the written HTML file after save.")]
    pub open: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Inspect color themes.")]
    Themes(ThemesArgs),
    #[command(about = "Inspect reader styles.")]
    Styles(StylesArgs),
    #[command(about = "Inspect config.")]
    Config(ConfigArgs),
}

#[derive(Debug, Subcommand)]
pub enum ThemesCommand {
    #[command(about = "List color theme names.")]
    List,
    #[command(about = "Print one color theme as TOML.")]
    Print { name: String },
}

#[derive(Debug, Subcommand)]
pub enum StylesCommand {
    #[command(about = "List reader style names.")]
    List,
    #[command(about = "Print one reader style as TOML.")]
    Print { name: String },
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    #[command(about = "Print built-in default config as TOML.")]
    PrintDefault,
}

#[derive(Debug, clap::Args)]
pub struct ThemesArgs {
    #[command(subcommand)]
    pub command: ThemesCommand,
}

#[derive(Debug, clap::Args)]
pub struct StylesArgs {
    #[command(subcommand)]
    pub command: StylesCommand,
}

#[derive(Debug, clap::Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Themes(command)) => run_themes(command.command),
        Some(Command::Styles(command)) => run_styles(command.command),
        Some(Command::Config(command)) => run_config(command.command),
        None => run_render(cli),
    }
}

fn run_render(cli: Cli) -> Result<()> {
    let input = cli
        .input
        .ok_or_else(|| ReadmdError::Message("missing input file".to_string()))?;

    let config = crate::config::Config::load(cli.config.as_deref())?;
    let html =
        crate::renderer::render(&input, &config, cli.theme.as_deref(), cli.style.as_deref())?;

    if cli.stdout {
        print!("{html}");
        return Ok(());
    }

    let output_path = if cli.tmp {
        temp_output_path(&input)?
    } else {
        cli.output.unwrap_or_else(|| default_output_path(&input))
    };

    write_text(&output_path, &html)?;

    if cli.open {
        open::that(&output_path).map_err(|source| {
            ReadmdError::Message(format!(
                "failed to open {}: {source}",
                output_path.display()
            ))
        })?;
    }

    if cli.tmp {
        println!("{}", output_path.display());
    }

    Ok(())
}

fn run_styles(command: StylesCommand) -> Result<()> {
    match command {
        StylesCommand::List => {
            for name in crate::theme::builtin_style_names() {
                println!("{name}");
            }
            Ok(())
        }
        StylesCommand::Print { name } => {
            let text = crate::theme::builtin_style_toml(&name)
                .ok_or_else(|| ReadmdError::Message(format!("unknown style: {name}")))?;
            print!("{text}");
            Ok(())
        }
    }
}

fn run_themes(command: ThemesCommand) -> Result<()> {
    match command {
        ThemesCommand::List => {
            for name in crate::theme::builtin_theme_names() {
                println!("{name}");
            }
            Ok(())
        }
        ThemesCommand::Print { name } => {
            let text = crate::theme::builtin_theme_toml(&name)
                .ok_or_else(|| ReadmdError::UnknownTheme(name))?;
            print!("{text}");
            Ok(())
        }
    }
}

fn run_config(command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::PrintDefault => {
            let text = crate::config::default_config_toml();
            print!("{text}");
            Ok(())
        }
    }
}

fn default_output_path(input: &Path) -> PathBuf {
    input.with_extension("html")
}

fn temp_output_path(input: &Path) -> Result<PathBuf> {
    let name = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("readmd");

    let temp = tempfile::Builder::new()
        .prefix(name)
        .suffix(".html")
        .tempfile()
        .map_err(|source| ReadmdError::Message(format!("failed to create temp file: {source}")))?;

    let (_, path) = temp
        .keep()
        .map_err(|error| ReadmdError::Message(format!("failed to keep temp file: {error}")))?;

    Ok(path)
}

fn write_text(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text).map_err(|source| ReadmdError::WriteFile {
        path: path.to_path_buf(),
        source,
    })
}
