// After auth we will receive the code via:
// http://127.0.0.1:32888/discord/oauth?code=iHJpDsv0Nc4zfmhg9qoEoqOtDKQ95p

use colored::Colorize;
use std::{net::Ipv4Addr, sync::RwLock, time::Duration};
use tokio::time::sleep;

use rocket::{get, info, response::Redirect, routes, Shutdown, State};

use crate::{config::Config, errors::Error, galactica_api};

const DISCORD_LOGIN: &str = "https://discord.com/api/oauth2/authorize?client_id=1081168959941918801&redirect_uri=http%3A%2F%2F127.0.0.1%3A32888%2Fdiscord%2Foauth&response_type=code&scope=identify%20email";

pub async fn open_browser() -> () {
    sleep(Duration::from_secs(1)).await;
    println!("Attempting to automatically open your browser, please wait...");
    sleep(Duration::from_secs(1)).await;

    match open::that("http://127.0.0.1:32888/") {
        Err(err) => {
            println!("Error while opening browser: {}", err);
            println!(
                "Failed to automatically open browser\nPlease open the url directly!\n\n{}",
                DISCORD_LOGIN
            );
        }
        Ok(_) => (),
    }
}

pub async fn launch_rocket(current_config: Config) -> Result<Config, Error> {
    // Launch the webserver and inject the current config as a rocket managed state
    //
    let rocket_async = rocket::build()
        .configure(rocket::Config {
            port: 32888,
            address: Ipv4Addr::new(127, 0, 0, 1).into(),
            ..rocket::Config::debug_default()
        })
        .manage(RwLock::new(current_config))
        .mount("/", routes![root, discord_oauth_code_receive])
        .launch();

    //
    // The webserver will block here until it shuts down...
    //
    let rocket = rocket_async
        .await
        .map_err(|e| Error::UnableToLaunchWebServer(e.to_string()))?;

    // Grab the updated config (modified by the webserver hopefully)
    //
    let updated_config = rocket
        .state::<RwLock<Config>>()
        .unwrap()
        .read()
        .unwrap()
        .clone();
    Ok(updated_config)
}

pub async fn perform_login(current_config: Config) -> Result<Config, Error> {
    tokio::join!(launch_rocket(current_config), open_browser()).0
}

#[get("/")]
async fn root() -> Redirect {
    Redirect::to(DISCORD_LOGIN)
}

#[get("/discord/oauth?<code>")]
async fn discord_oauth_code_receive(
    shutdown: Shutdown,
    rwlock_config: &State<RwLock<Config>>,
    code: &str,
) -> String {
    info!("Received code after auth from discord -> {}", code);

    // Grab a readonly copy of the config from the rwlock
    //
    let readable_config = rwlock_config.read().unwrap().clone();

    // Now get the token from the webservice

    match galactica_api::get_token(&readable_config, &code.to_string()).await {
        Err(err) => format!(
            "An error occured while attempting to login: {}\nPlease retry?",
            err.to_string()
        ),
        Ok(token) => {
            // Write the token to a writeable config
            //
            let mut writeable_config = rwlock_config.write().unwrap();
            *writeable_config = Config {
                token: Some(token),
                ..readable_config
            };

            shutdown.notify();

            println!("You are now logged in!\n");
            println!("------------------------------------------------------");
            println!("{}", "Please feel free to join our discord at:".green());
            println!("{}", "https://discord.gg/5eC72nkpw6".bright_green());
            println!("------------------------------------------------------");

            String::from("OK - you are logged in to Galactica! You can now close this window.")
        }
    }
}
