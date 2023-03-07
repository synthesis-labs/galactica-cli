use std::fmt::Display;

type ConfigFileName = String;

#[derive(Debug)]
pub enum ClientError {
    ConfigError(ConfigFileName, String),
    GalacticaApiError(String),
    GalacticaApiReturnedError(galactica_lib::errors::Error),
    UnableToSerialize(String),
    UnableToDeserialize(String, String),
    CommandError(String),
    ParsingError(String),
    UnableToLaunchWebServer(String),
    StdinError(String),
    NotLoggedIn(String),
    NotImplemented,
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigError(filename, msg) => {
                writeln!(f, "ConfigError: {} ({})", msg, filename)
            }
            Self::GalacticaApiError(msg) => {
                writeln!(f, "GalacticaApiError: {}", msg)
            }
            Self::GalacticaApiReturnedError(inner) => {
                writeln!(f, "GalacticaApiReturnedError: {:?}", inner)
            }
            Self::UnableToSerialize(msg) => {
                writeln!(f, "UnableToSerialize: {}", msg)
            }
            Self::UnableToDeserialize(msg, body) => {
                writeln!(f, "UnableToDeserialize: {}\nBody: {}", msg, body)
            }
            Self::CommandError(msg) => {
                writeln!(f, "CommandError: {}", msg)
            }
            Self::ParsingError(msg) => {
                writeln!(f, "ParsingError: {}", msg)
            }
            Self::UnableToLaunchWebServer(msg) => {
                writeln!(f, "UnableToLaunchWebServer: {}", msg)
            }
            Self::StdinError(msg) => {
                writeln!(f, "StdinError: {}", msg)
            }
            Self::NotLoggedIn(msg) => {
                writeln!(f, "NotLoggedIn: {}", msg)
            }
            Self::NotImplemented => write!(f, "NotImplemented"),
        }
    }
}
