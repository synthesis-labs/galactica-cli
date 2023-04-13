use colored::Colorize;

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

pub fn print_update_banner(available_version: Option<String>) -> () {
    match available_version {
        Some(newer) => {
            println!("{}", "------------------------------------------".yellow());
            println!("{}", "A newer version of galactica is available!".yellow());
            println!("\n==> {} <==\n", newer.yellow());
            println!(
                "{}",
                "Please visit our release page and install the latest version:".yellow()
            );
            println!(
                "{}",
                "https://github.com/synthesis-labs/galactica-cli/releases".blue()
            );
            println!("\n");
            println!(
                "{}",
                "If you're on mac and installed via homebrew you can simply:".blue()
            );
            println!("{}", "$ brew upgrade galactica".blue());
            println!("{}", "------------------------------------------".yellow());
        }
        None => {
            println!(
                "{} ({})",
                "You are on the latest version!".green(),
                get_current_version().blue()
            );
        }
    }
}
