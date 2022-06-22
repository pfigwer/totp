use std::process;
use clap::Parser;
use confy::ConfyError;
use crate::authenticator::google_authenticator::Authenticator;

use crate::cli::args::{Cli, Commands, ConfigCommands};
use crate::configuration::configuration::{Configuration, load_configuration, save_configuration};

pub fn handle_cli_commands() {
    let args = Cli::parse();
    match args.command {
        Commands::Config(config) => {
            let config_cmd = config.command.unwrap();
            match config_cmd {
                ConfigCommands::Append {key, secret } => {
                    handle_config_append(key, secret);
                }
                ConfigCommands::Remove { key } => {
                    handle_config_removal(&key);
                }
            }
        },
        Commands::Generate { key } => {
            handle_generate_code(key)
        }
    }
}

fn handle_generate_code(key: Option<String>) {
    let authenticator = Authenticator::default();
    match load_configuration() {
        Ok(config) => {
            match key {
                Some(key) => {
                    match config.secrets.get(&key) {
                        Some(secret) => {
                            match authenticator.generator.get_code(secret) {
                                Ok(code) => println!("{}: {}", key, code),
                                Err(error) => println!("Failed to generate code: {}", error)
                            }
                        }
                        None => {
                            println!("no configuration found for: {}", key);
                            process::exit(exitcode::CONFIG);
                        }
                    }
                }
                None => {
                    for (key, secret) in &config.secrets {
                        match authenticator.generator.get_code(secret) {
                            Ok(code) => println!("{}: {}", key, code),
                            Err(error) => println!("Failed to generate code: {}", error)
                        }
                    }
                }
            }
        },
        Err(error) => handle_parse_file_error(error)
    }
}

fn handle_config_removal(key: &String) {
    match load_configuration() {
        Ok(mut config) => {
            config.secrets.remove(key);
            update_configuration_file(&mut config);
        }
        Err(error) => handle_parse_file_error(error)
    }
}

fn handle_config_append(key: String, secret: String) {
    match load_configuration() {
        Ok(mut config) => {
            config.secrets.insert(key, secret);
            update_configuration_file(&mut config);
        }
        Err(error) => {
            println!("Parse error: {}", error);
            process::exit(exitcode::IOERR);
        }
    }
}

fn update_configuration_file(config: &mut Configuration) {
    match save_configuration(&config) {
        Ok(_) => println!("Configuration was saved to the file."),
        Err(error) => handle_parse_file_error(error)
    }
}

fn handle_parse_file_error(error: ConfyError) {
    println!("Parse error: {}", error);
    process::exit(exitcode::IOERR);
}
