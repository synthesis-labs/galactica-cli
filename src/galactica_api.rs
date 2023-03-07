use galactica_lib::auth::{DiscordAccessToken, GetTokenRequest, GetTokenResponse};
use galactica_lib::specs::{HistoryEntry, Instruction, InstructionRequest, InstructionResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{config::Config, errors::Error};

pub async fn api_call<'a, REQ, RES>(
    config: &Config,
    endpoint: &str,
    request: &REQ,
) -> Result<RES, Error>
where
    REQ: Serialize,
    RES: DeserializeOwned, // for why -> https://serde.rs/lifetimes.html
{
    let _request_body =
        serde_json::to_string(&request).map_err(|e| Error::UnableToSerialize(e.to_string()))?;

    // println!("Request body => {}", _request_body);

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}{}", config.api_url, endpoint))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| Error::GalacticaApiError(e.to_string()))?;

    let response_body = resp
        .text()
        .await
        .map_err(|e| Error::GalacticaApiError(e.to_string()))?;

    // println!("Response body => {}", response_body);

    let response: RES = serde_json::from_str(&response_body)
        .map_err(|e| Error::UnableToDeserialize(e.to_string(), response_body.clone()))?;

    Ok(response)
}

pub async fn get_token(config: &Config, code: &String) -> Result<DiscordAccessToken, Error> {
    let response: GetTokenResponse = api_call(
        config,
        "/auth/get_token",
        &GetTokenRequest { code: code.clone() },
    )
    .await?;
    Ok(response.token)
}

pub async fn instruction(
    config: &Config,
    instruction: Instruction,
    n: u32,
    history: Vec<HistoryEntry>,
) -> Result<Vec<String>, Error> {
    let response: InstructionResponse = api_call(
        config,
        "/instruction",
        &InstructionRequest {
            token: config.token.as_ref().unwrap().clone(),
            instruction,
            n,
            history,
        },
    )
    .await?;
    Ok(response.content)
}
