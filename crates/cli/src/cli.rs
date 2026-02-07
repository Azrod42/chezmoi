use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "cealum", version, about = "Cealum CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Login(LoginArgs),
    Register(RegisterArgs),
    #[command(alias = "a")]
    Ask(AskArgs),
    #[command(alias = "t")]
    Translate(TranslateArgs),
    #[command(alias = "c")]
    Correct(CorrectArgs),
    Config(ConfigArgs),
}

#[derive(Args)]
pub struct LoginArgs {
    #[arg(long)]
    pub email: Option<String>,
    #[arg(long)]
    pub password: Option<String>,
}

#[derive(Args)]
pub struct RegisterArgs {
    #[arg(long)]
    pub email: Option<String>,
    #[arg(long)]
    pub password: Option<String>,
}

#[derive(Args)]
pub struct AskArgs {
    #[arg(required = true, num_args = 0..)]
    pub text: Vec<String>,
    #[arg(long, default_value_t = true)]
    pub format: bool,
}

#[derive(Args)]
pub struct TranslateArgs {
    pub language: String,
    #[arg(required = true, num_args = 1..)]
    pub text: Vec<String>,
    #[arg(long)]
    pub format: bool,
}

#[derive(Args)]
pub struct CorrectArgs {
    pub language: String,
    #[arg(required = true, num_args = 1..)]
    pub text: Vec<String>,
    #[arg(long)]
    pub format: bool,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    Set(ConfigSetArgs),
    Show,
}

#[derive(Args)]
pub struct ConfigSetArgs {
    pub target: ModelTarget,
    pub model: String,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ModelTarget {
    #[value(alias = "a")]
    Ask,
    #[value(alias = "t")]
    Translate,
    #[value(alias = "c")]
    Correct,
}
