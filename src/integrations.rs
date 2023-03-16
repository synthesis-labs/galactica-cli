use clap::ArgMatches;
use reqwest::Client;
use std::io::Write;
use std::path::Path;
use std::process::Command as com;
use crate::errors::ClientError;
use std::fs;
use std::fs::OpenOptions;

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
    if let Err(_e) = fs::remove_file(PRE_COMMIT_HOOK_FILEPATH){
        return Err(ClientError::IntegrationError(format!(
            "Failed to delete pre-commit hook. Maybe you don't have one?"
        )));
    }
    println!("Pre-commit hook deleted successfully");
    Ok(())
}

fn delete_prepare_commit_hook() -> Result<(), ClientError> {
    Err(ClientError::NotImplemented)
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
    if let Err(_e) = writeln!(hook_file, "{}", script){
        return Err(ClientError::IntegrationError(format!(
            "Failed to make write pre-commit hook script executable"
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
    let mut editor = "";
    let os = std::env::consts::OS; //this allows the executable to run on any machine as it checks at runtime as opposed to compile time with cfg!(target_os)
    match os {
        "linux" => editor = "vi",
        "macos" => editor = "TextEdit",
        "windows" => editor = "notepad",
        _ => editor = "vi",
    }
  
    let tmp_file_creator = r#"#!/bin/bash
if [ -n "$GIT_EDITOR" ]; then
exit 0
fi
TMPFILE=$(mktemp) || { echo "Failed to create temp file"; exit 1; }
git diff --staged | galactica code 'provide 1 sentence as a summary of the changes made to this code. Then skip a line and provide a short description of why the major changes were made, using bullet points if necessary.' > "$TMPFILE"
${EDITOR:-$(git config --get core.editor || echo '"#.to_string();
    let commit_string = r#"')} "$TMPFILE"
COMMIT_MSG=$(cat "$TMPFILE")
rm "$TMPFILE"
echo "$COMMIT_MSG" | git commit -F -"#.to_string();
    
    let script = format!("{}{}{}",tmp_file_creator, editor, commit_string);

    create_hook(
        PRE_COMMIT_HOOK_FILEPATH,
        &script,
        os,
    )
}



// fn create_prepare_commit_hook() -> Result<(), ClientError> {
//     // Windows script
//     //
//     if cfg!(target_os = "windows") {
//         create_hook(
//             PREPARE_COMMIT_MSG_HOOK_FILEPATH,
//             r#"#!/bin/bash
    
//         COMMIT_MSG_FILE=$1
//         COMMIT_SOURCE=$2
//         SHA1=$3
        
//         echo Running Galactica prepare-commit-hook on $COMMIT_MSG_FILE...
//         if [ -f "$COMMIT_MSG_FILE" ] && [ "$COMMIT_SOURCE" = "message" ]; then
//             # Get the editor configured in Git or use the system default
//             EDITOR=$(git config --get core.editor || echo 'notepad')
//             # Open the commit message file in the editor
//             "$EDITOR" "$COMMIT_MSG_FILE"
//         fi"#,
//         )
//     } 
//     else{
  
//         create_hook(
//             PREPARE_COMMIT_MSG_HOOK_FILEPATH,
//             r#"#!/bin/bash
    
//         COMMIT_MSG_FILE=$1
//         COMMIT_SOURCE=$2
//         SHA1=$3
        
//         echo Running Galactica prepare-commit-hook on $COMMIT_MSG_FILE...
//         if [ -f "$COMMIT_MSG_FILE" ] && [ "$COMMIT_SOURCE" = "message" ]; then
//             # Get the editor configured in Git or use the system default
//             EDITOR=$(git config --get core.editor || echo 'vi')
//             # Open the commit message file in the editor
//             "$EDITOR" "$COMMIT_MSG_FILE"
//         fi"#,
//         )
//     }
// }
