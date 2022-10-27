use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    prompt: String,

    #[arg(short, long, default_value_t = 512)]
    max_token: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Item {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    choices: Vec<Item>,
}

pub fn get_args() -> Args {
    Args::parse()
}

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut body = <HashMap<&str, serde_json::Value>>::new();

    body.insert("model", json!("text-davinci-002"));
    body.insert(
        "prompt",
        json!(format!(
            "Correct this to standard English:\n\n{}",
            args.prompt
        )),
    );
    body.insert("temperature", json!(0.7));
    body.insert("max_tokens", json!(args.max_token));

    let resp = client
        .post("https://api.openai.com/v1/completions")
        .bearer_auth("")
        .json(&body)
        .send()
        .await?;

    println!(
        "corrected text: {}",
        resp.json::<APIResponse>()
            .await?
            .choices
            .first()
            .unwrap()
            .text
    );

    Ok(())
}
