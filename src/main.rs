mod config;
mod content;

use config::Config;
use content::{Content, Part, Role, content::Contents};
use reqwest::{
    Url,
    blocking::{Client, Response},
    header::HeaderMap,
};
use serde_json::Value;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    time::Duration,
};
use termimad::MadSkin;

fn main() -> Result<(), Box<dyn Error>> {
    let skin: MadSkin = MadSkin::default();

    let api_url: Url = Url::parse_with_params(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent",
        &[("key", Config::load().unwrap().api_key)],
    )?;

    let config_path: PathBuf = dirs::config_dir().expect("Unable to find config directory");
    let path: PathBuf = config_path.join("rustbot/messages.json");
    let mut file: File;
    let mut contents: Contents = Contents::new(vec![]);

    if path.exists() {
        file = File::open(&path)?;
        contents = serde_json::from_reader(file)?;
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        file = File::create(&path)?;
        file.write_all(serde_json::to_string(&contents)?.as_bytes())?;
    }

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

        contents.add(Content::new(vec![Part::from(prompt.trim())], Role::User));

        println!();
        println!("Model:");

        let res: Response = client
            .post(api_url.clone())
            .body(serde_json::to_string(&contents)?)
            .send()?;
        let json: Value = res.json()?;

        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            skin.print_text(text);
            println!();

            contents.add(Content::new(vec![Part::from(text.trim())], Role::Model));

            let mut file: File = File::create(&path)?;
            file.write_all(serde_json::to_string(&contents)?.as_bytes())?;
        } else {
            println!("Response body not found.");
        }
    }

    Ok(())
}
