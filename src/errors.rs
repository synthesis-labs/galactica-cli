use std::fmt::Display;

type ConfigFileName = String;

#[derive(Debug)]
pub enum Error {
    ConfigError(ConfigFileName, String),
    OpenAIError(String),
    GalacticaApiError(String),
    UnableToSerialize(String),
    UnableToDeserialize(String, String),
    CommandError(String),
    ParsingError(String),
    UnableToLaunchWebServer(String),
    StdinError(String),
    NotLoggedIn(String),
    NotImplemented,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigError(filename, msg) => {
                writeln!(f, "ConfigError: {} ({})", msg, filename)
            }
            Error::OpenAIError(msg) => {
                writeln!(f, "OpenAIError: {}", msg)
            }
            Error::GalacticaApiError(msg) => {
                writeln!(f, "GalacticaApiError: {}", msg)
            }
            Error::UnableToSerialize(msg) => {
                writeln!(f, "UnableToSerialize: {}", msg)
            }
            Error::UnableToDeserialize(msg, body) => {
                writeln!(f, "UnableToDeserialize: {}\nBody: {}", msg, body)
            }
            Error::CommandError(msg) => {
                writeln!(f, "CommandError: {}", msg)
            }
            Error::ParsingError(msg) => {
                writeln!(f, "ParsingError: {}", msg)
            }
            Error::UnableToLaunchWebServer(msg) => {
                writeln!(f, "UnableToLaunchWebServer: {}", msg)
            }
            Error::StdinError(msg) => {
                writeln!(f, "StdinError: {}", msg)
            }
            Error::NotLoggedIn(msg) => {
                writeln!(f, "NotLoggedIn: {}", msg)
            }
            Error::NotImplemented => write!(f, "NotImplemented"),
        }
    }
}
