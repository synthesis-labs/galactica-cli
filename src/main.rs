extern crate rocket;

use std::io::{self, Read};

use clap::{Arg, Command};
use colored::Colorize;
use galactica::{self, config, discord_login, galactica_api, updates};
use galactica::{config::Config, errors::ClientError};
use galactica_lib::specs::{Agent, HistoryEntry, Instruction};
use tokio::runtime::Builder;

#[cfg(windows)]
use colored::control;

fn cli() -> Command {
    Command::new("cli")
        .about("GPT-3.5 at your fingertips!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("login").about("Login via Discord"))
        .subcommand(
            Command::new("chat")
                .about("Open ended chat including history and context")
                .arg(Arg::new("prompt").num_args(1..)),
        )
        .subcommand(
            Command::new("code")
                .about("Generate code based on requirements (no history)")
                .arg(Arg::new("prompt").num_args(1..)),
        )
        .subcommand(
            Command::new("stream")
                .about("Open ended chat including history and context")
                .arg(Arg::new("prompt").num_args(1..)),
        )
        .subcommand(Command::new("history").about("Show history"))
        .subcommand(Command::new("reset").about("Reset history"))
        .subcommand(Command::new("update").about("Update the tool"))
        .subcommand(Command::new("version").about("Get the version"))
}

async fn invoke() -> Result<(), ClientError> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("login", _submatches)) => {
            // Read the config, or use a default one if not available
            //
            let config = config::read().unwrap_or(Config::default());
            let new_config = discord_login::perform_login(config).await?;
            config::write(&new_config)?;
        }
        Some(("reset", _submatches)) => {
            // Read the config, or use a default one if not available
            //
            let mut new_config = config::read().unwrap_or(Config::default());
            new_config.history = vec![];
            config::write(&new_config)?;

            println!("History cleared");
        }
        Some(("history", _submatches)) => {
            // Read the config, or use a default one if not available
            //
            let config = config::read().unwrap_or(Config::default());

            for entry in config.history {
                match entry.agent {
                    Agent::User => println!("{}", entry.content.blue()),
                    Agent::Galactica => println!("{}", entry.content.green()),
                }
            }
        }
        Some(("update", _submatches)) => {
            updates::update().await;
        }
        Some(("version", _submatches)) => {
            updates::version();
        }
        Some(("chat", submatches)) => {
            let config = config::read()?;

            if config.token.is_none() {
                return Err(ClientError::NotLoggedIn("Please login first!".to_string()));
            }

            let prompt = get_prompt(submatches)?;

            // Do we have data passed to us via stdin?
            //
            let specific = match get_stdin() {
                Some(stdin) => Instruction::ConversationWithReference(prompt.clone(), stdin),
                None => Instruction::Conversation(prompt.clone()),
            };

            let replies =
                galactica_api::instruction(&config, specific, 1, config.history.clone()).await?;

            // Update history
            //
            let mut mut_config = config::read()?;

            mut_config.history.push(HistoryEntry {
                agent: Agent::User,
                content: prompt,
            });

            for reply in replies.iter() {
                mut_config.history.push(HistoryEntry {
                    agent: Agent::Galactica,
                    content: reply.clone(),
                });

                config::write(&mut_config)?;

                println!("{}", reply.green());
            }
        }
        Some(("code", submatches)) => {
            let config = config::read()?;

            if config.token.is_none() {
                return Err(ClientError::NotLoggedIn("Please login first!".to_string()));
            }

            let prompt = get_prompt(submatches)?;

            // Do we have data passed to us via stdin?
            //
            let specific = match get_stdin() {
                Some(stdin) => Instruction::GenerateCodeWithReference(prompt, stdin),
                None => Instruction::GenerateCode(prompt),
            };

            // No history...
            //
            let replies = galactica_api::instruction(&config, specific, 1, vec![]).await?;

            for reply in replies.iter() {
                println!("{}", reply.green());
            }
        }
        Some(("stream", submatches)) => {
            let config = config::read()?;

            if config.token.is_none() {
                return Err(ClientError::NotLoggedIn("Please login first!".to_string()));
            }

            let prompt = get_prompt(submatches)?;

            // Do we have data passed to us via stdin?
            //
            let specific = match get_stdin() {
                Some(stdin) => Instruction::ConversationWithReference(prompt.clone(), stdin),
                None => Instruction::Conversation(prompt.clone()),
            };

            let reply =
                galactica_api::instruction_stream(&config, specific, 1, config.history.clone())
                    .await?;

            // Update history
            //
            let mut mut_config = config::read()?;

            mut_config.history.push(HistoryEntry {
                agent: Agent::User,
                content: prompt,
            });

            mut_config.history.push(HistoryEntry {
                agent: Agent::Galactica,
                content: reply.clone(),
            });

            config::write(&mut_config)?;

            //println!("{}", reply.bright_green());
        }
        Some((cmd, _submatches)) => {
            println!("Not sure how to process cmd {}", cmd);
        }
        _ => unreachable!(),
    }

    Ok(())
}

// A handy little fn to get the arguments as a single long string ("prompt")
//
fn get_prompt(submatches: &clap::ArgMatches) -> Result<String, ClientError> {
    Ok(submatches
        .get_many("prompt")
        .ok_or(ClientError::CommandError(
            "No arguments provided".to_string(),
        ))?
        .map(|s: &String| s.clone())
        .collect::<Vec<String>>()
        .join(" "))
}

fn get_stdin() -> Option<String> {
    if atty::isnt(atty::Stream::Stdin) {
        let mut stdin_buffer = String::new();
        io::stdin().read_to_string(&mut stdin_buffer).ok()?;
        Some(stdin_buffer)
    } else {
        None
    }
}

fn main() {
    // Make colourize work in windows
    //
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let r = rt.block_on(invoke());

    match r {
        Err(err) => {
            println!("{}", err.to_string().red())
        }
        Ok(_) => {}
    }
}
