mod content;

use content::{Content, Part, Role, content::Contents};
use dotenv::dotenv;
use reqwest::{
    blocking::{Client, Response},
    header::HeaderMap,
};
use serde_json::Value;
use std::{
    env,
    error::Error,
    io::{self, Read},
    time::Duration,
};
use termimad::MadSkin;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let skin = MadSkin::default();

    let api_key: String = env::var("API_KEY")?;
    let api_url: String = env::var("API_URL")? + api_key.as_str();

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let client: Client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()?;

    loop {
        println!("User:");

        let mut prompt: String = String::new();
        io::stdin().read_to_string(&mut prompt)?;

        if prompt.trim() == "exit" || prompt.trim().is_empty() {
            break;
        }

        println!();
        println!("Model:");
        let body: Contents = Contents::new(vec![Content::new(
            vec![Part::from(prompt.trim())],
            Role::User,
        )]);

        let res: Response = client
            .post(&api_url)
            .body(serde_json::to_string(&body)?)
            .send()?;
        let json: Value = res.json()?;

        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            skin.print_text(text);
        } else {
            println!("Texto da resposta não encontrado.");
        }
    }

    Ok(())
}
