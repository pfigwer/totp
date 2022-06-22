use crate::cli::commands::handle_cli_commands;

mod authenticator;
mod configuration;
mod cli;

fn main() {
    handle_cli_commands()
}
