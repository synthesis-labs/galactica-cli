use colored::Colorize;
use galactica_lib::parser::parse;
use galactica_lib::version_parser::{self};

const OWNER: &str = "synthesis-labs";
const REPO: &str = "galactica-cli";

pub fn get_current_version() -> String {
    // galactica-x86_64-apple-darwin-0.1.0-build.16.a52800d
    format!(
        "galactica-{}-{}+build.{}.{}",
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_GIT_COMMIT_COUNT"),
        env!("VERGEN_GIT_DESCRIBE")
    )
}

pub async fn update() -> () {
    let octocrab = octocrab::instance();
    if let Ok(page) = octocrab.repos(OWNER, REPO).releases().list().send().await {
        let available = page
            .items
            .iter()
            .map(|item| {
                // println!(
                //     "Release Tag => {} Name => {}",
                //     item.tag_name,
                //     item.name.unwrap_or("None".to_string())
                // );
                item.assets
                    .iter()
                    .map(|asset| asset.clone().name)
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();

        // println!("Available releases => {:?}", available);

        let mut has_update = false;
        // Check if any of these available releases are newer than ours?
        if let Ok((_, current)) = parse(version_parser::filename_parser(), &get_current_version()) {
            for asset in available {
                if let Ok((_, version)) = parse(version_parser::filename_parser(), &asset) {
                    // Just check for build number for now
                    if let (Ok(current_build), Ok(version_build)) =
                        (current.build.parse::<i32>(), version.build.parse::<i32>())
                    {
                        if current_build < version_build {
                            has_update = true;
                        }
                    }
                }
            }
        }

        if has_update {
            println!("{}", "------------------------------------------".yellow());
            println!("{}", "A newer version of galactica is available!".yellow());
            println!(
                "{}",
                "Please visit our release page and install the latest version:".yellow()
            );
            println!(
                "{}",
                "https://github.com/synthesis-labs/galactica-cli/releases".blue()
            );
            println!("{}", "------------------------------------------".yellow());
        } else {
            println!(
                "{} ({})",
                "You are on the latest version!".green(),
                get_current_version().blue()
            );
        }
    } else {
        println!("Unable to connect to github?");
    }
}
