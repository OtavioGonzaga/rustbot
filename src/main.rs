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
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    time::Duration,
};
use termimad::MadSkin;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key: String = env::var("API_KEY")?;
    let api_url: String = env::var("API_URL")? + api_key.as_str();

    let skin: MadSkin = MadSkin::default();
    let path: &str = "messages/messages.json";
    let mut file: File;
    let mut contents: Contents = Contents::new(vec![]);

    if Path::new(path).exists() {
        file = File::open(path)?;
        contents = serde_json::from_reader(file)?;
    } else {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }

        file = File::create(path)?;
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
            .post(&api_url)
            .body(serde_json::to_string(&contents)?)
            .send()?;
        let json: Value = res.json()?;

        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            skin.print_text(text);
            println!();

            contents.add(Content::new(vec![Part::from(text.trim())], Role::Model));

            let mut file: File = File::create(path)?;
            file.write_all(serde_json::to_string(&contents)?.as_bytes())?;
        } else {
            println!("Texto da resposta não encontrado.");
        }
    }

    Ok(())
}
