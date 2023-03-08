const OWNER: &str = "synthesis-labs";
const REPO: &str = "galactica-cli";

pub fn version() -> () {
    println!(
        "Version: {}, Count: {}, Branch: {}, Author: {} ({})",
        env!("VERGEN_GIT_DESCRIBE"),
        env!("VERGEN_GIT_COMMIT_COUNT"),
        env!("VERGEN_GIT_BRANCH"),
        env!("VERGEN_GIT_COMMIT_AUTHOR_NAME"),
        env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL"),
    );
    println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
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
