use crate::errors::ClientError;
use crate::templates;
use clap::ArgMatches;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command as com;

const PRE_COMMIT_HOOK_FILEPATH: &str = ".git/hooks/pre-commit";
// const PREPARE_COMMIT_MSG_HOOK_FILEPATH: &str = ".git/hooks/prepare-commit-msg";

pub fn cli_integrations(submatches: &ArgMatches) -> Result<(), ClientError> {
    if let Some(matches) = submatches.subcommand() {
        match matches {
            ("git_commit_hook", submatches) => {
                // Naughty for now, but this should live in it's own place one day
                if let Some(matches) = submatches.subcommand() {
                    match matches {
                        ("install", _) => {
                            create_pre_commit_hook()?;
                            // create_prepare_commit_hook()?;
                        }
                        ("uninstall", _) => {
                            delete_pre_commit_hook()?;
                            // delete_prepare_commit_hook()?;
                        }
                        _ => {}
                    }
                } else {
                }
            }
            _ => {}
        }
    } else {
    }

    Ok(())
}

fn delete_pre_commit_hook() -> Result<(), ClientError> {
    if let Err(_e) = fs::remove_file(PRE_COMMIT_HOOK_FILEPATH) {
        return Err(ClientError::IntegrationError(format!(
            "Failed to delete pre-commit hook. Maybe you don't have one?"
        )));
    }
    println!("Pre-commit hook deleted successfully");
    Ok(())
}

fn create_hook(filepath: &str, script: &str, os: &str) -> Result<(), ClientError> {
    let hook_file_path = Path::new(filepath);

    // Check if the pre-commit file already exists
    //
    if hook_file_path.exists() {
        return Err(ClientError::IntegrationError(
            "A pre-commit hook script already exists. Maybe it's already installed?".to_string(),
        ));
    }

    // Create the pre-commit file
    //
    let mut hook_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(&hook_file_path)
    {
        Ok(file) => file,
        Err(e) => {
            return Err(ClientError::IntegrationError(format!(
                "Failed to create pre-commit hook script: {}",
                e
            )))
        }
    };

    // Write the script to the pre-commit file
    if let Err(_e) = writeln!(hook_file, "{}", script) {
        return Err(ClientError::IntegrationError(format!(
            "Failed to write pre-commit hook script"
        )));
    }
    // Make the pre-commit file executable
    let mut os_com = "chmod";
    if os == "windows" {
        os_com = "attrib";
    }
    let output = com::new(os_com)
        .arg("+x")
        .arg(hook_file_path.to_str().unwrap())
        .output()
        .unwrap();
    if !output.status.success() {
        return Err(ClientError::IntegrationError(format!(
            "Failed to make pre-commit hook script executable: {:?}",
            output
        )));
    }
    println!("Created {}", filepath);

    Ok(())
}

fn create_pre_commit_hook() -> Result<(), ClientError> {
    let os = std::env::consts::OS; //this allows the executable to run on any machine as it checks at runtime as opposed to compile time with cfg!(target_os)

    let editor = match os {
        "linux" => "vi",
        "macos" => "open -W -e",
        "windows" => "notepad",
        _ => "vi",
    };

    let script = templates::render_git_commit_hook(editor);

    create_hook(PRE_COMMIT_HOOK_FILEPATH, &script, os)
}
