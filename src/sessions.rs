use std::{
    fs::{self, read_to_string},
    path::Path,
};

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use galactica_lib::specs::{Agent, HistoryEntry};
use glob::glob;
use serde::{Deserialize, Serialize};

use crate::errors::ClientError;

const SESSION_PATH: &str = ".galactica/sessions";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub name: String,
    pub history: Vec<HistoryEntry>,
}

pub fn cli_session_cmd() -> Command {
    Command::new("session")
        .about("Manage active sessions")
        .arg_required_else_help(true)
        .subcommand(Command::new("ls").about("List active sessions"))
        .subcommand(
            Command::new("rm")
                .about("Delete a session")
                .arg(Arg::new("name").help("The named session to delete"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("cat")
                .about("Display contents of a session")
                .arg(
                    Arg::new("name")
                        .help("The named session to display")
                        .default_value("default"),
                ),
        )
}

pub fn cli_session_handle(submatches: &ArgMatches) -> Result<(), ClientError> {
    if let Some(matches) = submatches.subcommand() {
        match matches {
            ("ls", submatches) => {
                for session_name in list() {
                    let session = read(&session_name).unwrap();
                    println!(
                        "{} -> {} entries (approx {} tokens)",
                        session_name.blue(),
                        session.history.len(),
                        approx_tokens(&session)
                    );
                }
            }
            ("rm", submatches) => {
                let name: &String = submatches.get_one("name").unwrap();
                remove(name);
            }
            ("cat", submatches) => {
                let name: &String = submatches.get_one("name").unwrap();

                if exists(name) {
                    let session = read(name).unwrap();

                    for entry in session.history {
                        match entry.agent {
                            Agent::User => println!("{}", entry.content.blue()),
                            Agent::Galactica => println!("{}", entry.content.green()),
                        }
                    }
                } else {
                    return Err(ClientError::SessionError(
                        session_file_path(name),
                        format!("Session '{}' does not exist", name),
                    ));
                }
            }
            _ => {}
        }
    } else {
    }

    Ok(())
}

pub fn read(name: &String) -> Result<Session, ClientError> {
    if exists(name) {
        let contents = read_to_string(Path::new(&session_file_path(name))).map_err(|e| {
            ClientError::SessionError(
                session_file_path(name),
                format!("Error while reading: {}", e.to_string()),
            )
        })?;

        let session: Session = serde_json::from_str(contents.as_str()).map_err(|e| {
            ClientError::SessionError(
                session_file_path(name),
                format!("Error while deserializing: {}", e.to_string()),
            )
        })?;

        Ok(session)
    } else {
        // A new session!
        //
        Ok(Session {
            name: name.clone(),
            history: vec![],
        })
    }
}

pub fn write(name: &String, session: &Session) -> Result<(), ClientError> {
    let json = serde_json::to_string(session).map_err(|_| {
        ClientError::SessionError(
            session_file_path(name),
            "Unable to serialize session object".to_string(),
        )
    })?;

    if let Some(parent_dir) = Path::new(&session_file_path(name)).parent() {
        fs::create_dir_all(parent_dir).map_err(|e| {
            ClientError::SessionError(
                session_file_path(name),
                format!(
                    "Unable to create directories while attempting to write session file due to {}",
                    e.to_string()
                ),
            )
        })?;
    }

    fs::write(session_file_path(name), json.as_str()).map_err(|e| {
        ClientError::SessionError(
            session_file_path(name),
            format!("Unable to write to session file due to {}", e.to_string()),
        )
    })?;

    Ok(())
}

pub fn list() -> Vec<String> {
    glob(
        format!(
            "{}/{}/*.json",
            dirs::home_dir().unwrap().to_str().unwrap(),
            SESSION_PATH
        )
        .as_str(),
    )
    .expect("Session directory does not exist, maybe run a chat first")
    .flat_map(|session_file| {
        if let Ok(path) = session_file {
            let session_name = Path::file_stem(&path).unwrap().to_str().unwrap();
            vec![session_name.to_string()]
        } else {
            vec![]
        }
    })
    .collect()
}

pub fn remove(name: &String) {
    fs::remove_file(session_file_path(name));
}

pub fn exists(name: &String) -> bool {
    Path::new(&session_file_path(name)).exists()
}

pub fn session_file_path(name: &String) -> String {
    format!(
        "{}/{}/{}.json",
        dirs::home_dir().unwrap().to_str().unwrap(),
        SESSION_PATH,
        name
    )
}

pub fn approx_tokens(session: &Session) -> usize {
    let mut tokens: usize = 0;
    for entry in session.history.as_slice() {
        tokens += entry.content.split(" ").collect::<Vec<&str>>().len();
    }
    tokens
}
