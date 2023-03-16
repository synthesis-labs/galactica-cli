use crate::errors::ClientError;
use crate::templates;
use clap::ArgMatches;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command as com;

const PRE_COMMIT_HOOK_FILEPATH: &str = ".git/hooks/pre-commit";

pub fn cli_integrations(submatches: &ArgMatches) -> Result<(), ClientError> {
    if let Some(matches) = submatches.subcommand() {
        match matches {
            ("git_commit_hook", submatches) => {
                if let Some(matches) = submatches.subcommand() {
                    match matches {
                        ("install", _) => {
                            create_pre_commit_hook()?;
                        }
                        ("uninstall", _) => {
                            delete_pre_commit_hook()?;
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

fn create_hook(filepath: &str, script: &str) -> Result<(), ClientError> {
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
    //
    if let Err(_e) = writeln!(hook_file, "{}", script) {
        return Err(ClientError::IntegrationError(format!(
            "Failed to write pre-commit hook script"
        )));
    }
    // Make the pre-commit file executable
    //
    #[cfg(target_os = "windows")]
    let os_com = "attrib";
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let os_com = "chmod";
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
    #[cfg(target_os = "windows")]
    let (editor, tty) = ("notepad", "");
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let (editor, tty) = ("vi", "</dev/tty");
    let script = templates::render_git_commit_hook(editor, &tty);

    create_hook(PRE_COMMIT_HOOK_FILEPATH, &script)
}
