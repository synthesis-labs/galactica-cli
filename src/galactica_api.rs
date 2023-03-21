use std::io::{self, Write};

use colored::Colorize;
use galactica_lib::auth::{DiscordAccessToken, GetTokenRequest, GetTokenResponse};
use galactica_lib::parser;
use galactica_lib::specs::{
    ErrorResponse, HistoryEntry, Instruction, InstructionChunk, InstructionRequest,
    InstructionResponse, UpdateRequest, UpdateResponse,
};
use galactica_lib::stream_data_parser::stream_data_parser;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{config::Config, errors::ClientError};

pub async fn api_call<'a, REQ, RES>(
    config: &Config,
    endpoint: &str,
    request: &REQ,
) -> Result<RES, ClientError>
where
    REQ: Serialize,
    RES: DeserializeOwned, // for why -> https://serde.rs/lifetimes.html
{
    let _request_body = serde_json::to_string(&request)
        .map_err(|e| ClientError::UnableToSerialize(e.to_string()))?;

    // println!("Request body => {}", _request_body);

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}{}", config.api_url, endpoint))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| ClientError::GalacticaApiError(e.to_string()))?;

    let response_body = resp
        .text()
        .await
        .map_err(|e| ClientError::GalacticaApiError(e.to_string()))?;

    // println!("Response body => {}", response_body);

    // Test whether we have received an error from the Galactica API
    //
    if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_body) {
        Err(ClientError::GalacticaApiReturnedError(error_response.error))
    } else {
        let response: RES = serde_json::from_str(&response_body)
            .map_err(|e| ClientError::UnableToDeserialize(e.to_string(), response_body.clone()))?;

        Ok(response)
    }
}

pub async fn stream_call<'a, REQ>(
    config: &Config,
    endpoint: &str,
    request: &REQ,
) -> Result<reqwest::Response, ClientError>
where
    REQ: Serialize,
{
    let _request_body = serde_json::to_string(&request)
        .map_err(|e| ClientError::UnableToSerialize(e.to_string()))?;

    // println!("Request body => {}", _request_body);

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}{}", config.api_url, endpoint))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| ClientError::GalacticaApiError(e.to_string()))?;

    Ok(resp)
}

pub async fn get_token(config: &Config, code: &String) -> Result<DiscordAccessToken, ClientError> {
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
) -> Result<Vec<String>, ClientError> {
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

pub async fn instruction_stream(
    config: &Config,
    instruction: Instruction,
    n: u32,
    history: Vec<HistoryEntry>,
) -> Result<String, ClientError> {
    let mut response: reqwest::Response = stream_call(
        config,
        "/instruction_stream",
        &InstructionRequest {
            token: config.token.as_ref().unwrap().clone(),
            instruction,
            n,
            history,
        },
    )
    .await
    .unwrap();

    // Accumulate into this
    let mut result = String::new();

    // Drain from this
    let mut buffer = String::new();

    // To fix, must eat until the first newline, else buffer

    while let Some(chunk) = response.chunk().await.unwrap() {
        let chunk_str: &str = std::str::from_utf8(&chunk).unwrap();

        // Accumulate to the buffer continuously
        buffer.push_str(chunk_str);

        // Run the stream data parser to grab out complete packets
        if let Ok((consumed, packets)) = parser::parse(stream_data_parser(), &buffer) {
            // println!("Consumed: {}, Packets: {:?}", consumed, packets);

            // Drop this many consumed chars from buffer
            buffer.drain(..consumed);
            // println!("Buffer now: [{}]", buffer);

            // Process the packets
            //
            for packet in packets {
                // println!("Packet ==>{:?}<==", packet);
                let data_obj: InstructionChunk = serde_json::from_str(&packet).unwrap();
                print!("{}", data_obj.content.green());
                io::stdout().flush().unwrap();

                // Accumulate to the result
                result.push_str(&data_obj.content)
            }
        }
    }

    Ok(result)
}

pub async fn update(
    config: &Config,
    current_version: String,
) -> Result<Option<String>, ClientError> {
    let response: UpdateResponse = api_call(
        config,
        "/update",
        &UpdateRequest {
            token: config.token.clone(),
            current_version,
        },
    )
    .await?;
    Ok(response.update_available)
}
