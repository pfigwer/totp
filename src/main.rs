use crate::cli::commands::handle_cli_commands;

mod authenticator;
mod config;
mod cli;

fn main() {
    handle_cli_commands()
}
