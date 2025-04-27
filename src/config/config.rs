use std::fs::{self, File};
use std::io::{Read, Result, Write};
use std::path::PathBuf;
use toml::{self, Value};

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path: PathBuf = Self::get_config_path();

        if !path.exists() {
            let api_key: String = Self::prompt_api_key();
            Self::save(&api_key)?;
        }

        let mut file: File = File::open(path)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;

        let value: Value = contents
            .parse::<Value>()
            .expect("Error parsing config.toml");
        let api_key: String = value
            .get("api_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "api_key not founded"))?
            .to_string();

        Ok(Config { api_key })
    }

    fn save(api_key: &str) -> std::io::Result<()> {
        let path: PathBuf = Self::get_config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file: File = File::create(path)?;
        writeln!(file, "api_key = \"{}\"", api_key)?;

        Ok(())
    }

    fn get_config_path() -> PathBuf {
        let mut path: PathBuf = dirs::config_dir().expect("Unable to find config directory");
        path.push("rustbot");
        path.push("config.toml");
        path
    }

    fn prompt_api_key() -> String {
        print!("Enter your Gemini API KEY: ");
        std::io::stdout().flush().unwrap();

        let mut api_key: String = String::new();
        std::io::stdin()
            .read_line(&mut api_key)
            .expect("Error reading API KEY");

        api_key.trim().to_string()
    }
}
