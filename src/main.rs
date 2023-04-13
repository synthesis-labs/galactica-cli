extern crate rocket;

use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use galactica::sessions;
use galactica::{self, config, discord_login, galactica_api, integrations, updates};
use galactica::{config::Config, errors::ClientError};
use galactica_lib::specs::{Agent, HistoryEntry, Instruction};
use std::io::{self, Read};
use tokio::runtime::Builder;

#[cfg(windows)]
use colored::control;

async fn call_instruction(
    prompt: &String,
    instruction: &Instruction,
    stream: bool,
    add_history: bool,
    session_name: &String,
) -> Result<String, ClientError> {
    let config = config::read()?;

    if config.token.is_none() {
        return Err(ClientError::NotLoggedIn("Please login first!".to_string()));
    }

    let session = sessions::read(session_name)?;
    let reply = if stream {
        galactica_api::instruction_stream(&config, instruction.clone(), 1, session.history.clone())
            .await?
    } else {
        galactica_api::instruction(&config, instruction.clone(), 1, session.history.clone()).await?
            [0]
        .clone()
    };

    // If we're streaming we can assume the output has been written
    //
    if !stream {
        println!("{}", reply.green());
    }

    // Update session
    //
    if add_history {
        let mut mut_session = sessions::read(session_name)?;

        mut_session.history.push(HistoryEntry {
            agent: Agent::User,
            content: prompt.clone(),
        });

        mut_session.history.push(HistoryEntry {
            agent: Agent::Galactica,
            content: reply.clone(),
        });

        sessions::write(session_name, &mut_session)?;
    }

    Ok(reply)
}

fn cli() -> Command {
    let prompt_arg = Arg::new("prompt").num_args(1..);
    let session_arg = Arg::new("session")
        .short('s')
        .long("session")
        .help("Continue from named session")
        .action(ArgAction::Set)
        .default_value("default");
    let no_stream_arg = Arg::new("no-stream")
        .short('b')
        .long("no-stream")
        .help("Do not stream results")
        .action(ArgAction::SetTrue)
        .required(false);
    let no_history_arg = Arg::new("no-history")
        .short('a')
        .long("no-history")
        .help("Do not store result in history")
        .action(ArgAction::SetTrue)
        .required(false);

    Command::new("cli")
        .about("AI at your fingertips!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("login").about("Login via Discord"))
        .subcommand(
            Command::new("chat")
                .about("Open ended chat including history and context")
                .arg(&session_arg)
                .arg(&no_stream_arg)
                .arg(&no_history_arg)
                .arg(&prompt_arg),
        )
        .subcommand(
            Command::new("code")
                .about("Generate code based on requirements")
                .arg(&session_arg)
                .arg(&no_stream_arg)
                .arg(&no_history_arg)
                .arg(&prompt_arg),
        )
        .subcommand(
            Command::new("explain")
                .about("EXPERIMENTAL: Work in progress. Ask for a detailed explanation with reflexion.")
                .arg(&session_arg)
                .arg(&no_stream_arg)
                .arg(&no_history_arg)
                .arg(&prompt_arg),
        )
        .subcommand(sessions::cli_session_cmd())
        .subcommand(Command::new("history").about("Show history (deprecating...)"))
        .subcommand(Command::new("reset").about("Reset history (deprecating...)"))
        .subcommand(
            Command::new("integration")
                .about("Manage integrations, e.g. git and others")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("git_commit_hook")
                        .about("Git stuff")
                        .subcommand(Command::new("install"))
                        .subcommand(Command::new("uninstall")),
                ),
        )
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
            let config = config::read()?;

            let current_version = updates::get_current_version();

            let available_version = galactica_api::update(&config, current_version).await?;
            updates::print_update_banner(available_version);
        }
        Some(("version", _submatches)) => {
            println!("{}", updates::get_current_version());
        }
        Some(("chat", submatches)) => {
            let prompt = get_prompt(submatches)?;

            // Do we have data passed to us via stdin?
            //
            let specific = match get_stdin() {
                Some(stdin) => Instruction::ConversationWithReference(prompt.clone(), stdin),
                None => Instruction::Conversation(prompt.clone()),
            };

            let session: &String = submatches.get_one("session").unwrap();
            let no_stream = submatches.get_flag("no-stream");
            let no_history = submatches.get_flag("no-history");

            call_instruction(&prompt, &specific, !no_stream, !no_history, &session).await?;
        }
        Some(("code", submatches)) => {
            let prompt = get_prompt(submatches)?;

            // Do we have data passed to us via stdin?
            //
            let specific = match get_stdin() {
                Some(stdin) => Instruction::GenerateCodeWithReference(prompt.clone(), stdin),
                None => Instruction::GenerateCode(prompt.clone()),
            };

            let session: &String = submatches.get_one("session").unwrap();
            let no_stream = submatches.get_flag("no-stream");
            let no_history = submatches.get_flag("no-history");

            call_instruction(&prompt, &specific, !no_stream, !no_history, &session).await?;
        }
        Some(("explain", submatches)) => {
            let prompt = get_prompt(submatches)?;

            // Do we have data passed to us via stdin?
            //
            let instruction = match get_stdin() {
                Some(stdin) => Instruction::ExplainWithReference(prompt.clone(), stdin),
                None => Instruction::Explain(prompt.clone()),
            };

            let session: &String = submatches.get_one("session").unwrap();
            let no_stream = submatches.get_flag("no-stream");
            let no_history = submatches.get_flag("no-history");

            call_instruction(&prompt, &instruction, !no_stream, !no_history, &session).await?;
        }
        Some(("session", submatches)) => {
            sessions::cli_session_handle(submatches)?;
        }
        Some(("integration", submatches)) => {
            integrations::cli_integrations(submatches)?;
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

    // Be a good citizen and return an error code
    //
    match r {
        Err(err) => {
            eprintln!("❌ {}", err.to_string().red());
            std::process::exit(-1)
        }
        Ok(_) => std::process::exit(0),
    }
}
