use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Config(Config),
    Generate {
        #[clap(short, long, value_parser)]
        key: Option<String>,
    }
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Config {
    #[clap(subcommand)]
    pub(crate) command: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    #[clap(arg_required_else_help = true)]
    Append {
        #[clap(short, long, value_parser)]
        key: String,

        #[clap(short, long, value_parser)]
        secret: String,
    },
    Remove {
        #[clap(short, long, value_parser)]
        key: String,
    },
}
