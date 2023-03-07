// After auth we will receive the code via:
// http://127.0.0.1:32888/discord/oauth?code=iHJpDsv0Nc4zfmhg9qoEoqOtDKQ95p

use std::{net::Ipv4Addr, sync::RwLock};

use rocket::{get, info, routes, Shutdown, State};

use crate::{config::Config, errors::Error, galactica_api};

const DISCORD_LOGIN: &str = "https://discord.com/api/oauth2/authorize?client_id=1081168959941918801&redirect_uri=http%3A%2F%2F127.0.0.1%3A32888%2Fdiscord%2Foauth&response_type=code&scope=identify%20email";

pub async fn perform_login(current_config: Config) -> Result<Config, Error> {
    println!("Please click the following link:\n\n{}\n", DISCORD_LOGIN);

    match open::that(DISCORD_LOGIN) {
        Err(err) => {
            println!("Error while opening browser: {}", err);
            println!(
                "Failed to automatically open browser\nPlease open the url directly!\n\n{}",
                DISCORD_LOGIN
            );
        }
        Ok(_) => (),
    }

    // Launch the webserver and inject the current config as a rocket managed state
    //
    let rocket = rocket::build()
        .configure(rocket::Config {
            port: 32888,
            address: Ipv4Addr::new(127, 0, 0, 1).into(),
            ..rocket::Config::debug_default()
        })
        .manage(RwLock::new(current_config))
        .mount("/", routes![discord_oauth_code_receive])
        .launch()
        .await
        .map_err(|e| Error::UnableToLaunchWebServer(e.to_string()))?;

    //
    // The webserver will block here until it shuts down...
    //

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

            String::from(
                "OK - you are logged in to Galactica! You are welcome to close this window.",
            )
        }
    }
}
