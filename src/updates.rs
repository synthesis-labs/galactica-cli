const OWNER: &str = "synthesis-labs";
const REPO: &str = "galactica-cli";

pub fn get_current_version() -> String {
    // galactica-x86_64-apple-darwin-0.1.0-build.16.a52800d
    format!(
        "galactica-{}-{}-build.{}.{}",
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_GIT_COMMIT_COUNT"),
        env!("VERGEN_GIT_DESCRIBE")
    )
}

pub fn version() -> () {
    println!(
        "Version: {}, Count: {}, Branch: {}, Author: {} ({})",
        env!("VERGEN_GIT_DESCRIBE"),
        env!("VERGEN_GIT_COMMIT_COUNT"),
        env!("VERGEN_GIT_BRANCH"),
        env!("VERGEN_GIT_COMMIT_AUTHOR_NAME"),
        env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL"),
    );
    println!(
        "Build: {}, Arch: {}",
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_CARGO_TARGET_TRIPLE")
    );
    println!("Cargo Pkg Version: {}", env!("CARGO_PKG_VERSION"));
}

pub async fn update() -> () {
    let octocrab = octocrab::instance();
    if let Ok(page) = octocrab.repos(OWNER, REPO).releases().list().send().await {
        for item in page.items {
            println!(
                "Release Tag => {} Name => {}",
                item.tag_name,
                item.name.unwrap_or("None".to_string())
            );

            for asset in item.assets {
                println!("  Asset => {} {} ", asset.name, asset.browser_download_url);
            }
        }
        //println!("Page => {:?}", page);
    }
}
