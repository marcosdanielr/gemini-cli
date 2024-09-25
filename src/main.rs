mod services;

use services::generate_ai_content_service;

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Gemini CLI");
    println!("To exit type 'quit()'");

    let mut config_dir = dirs::home_dir().unwrap();
    config_dir.push(".config/gemini-cli");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let mut api_key_file_path = config_dir;
    api_key_file_path.push("api_key.txt");

    let mut api_key = String::new();

    if File::open(&api_key_file_path)
        .and_then(|mut file| file.read_to_string(&mut api_key))
        .is_err()
    {
        println!("Please enter your Gemini API KEY to use this application:");
        io::stdin().read_line(&mut api_key).unwrap();
        let mut api_key_file = File::create(&api_key_file_path)?;
        api_key_file.write_all(api_key.trim().as_bytes())?;
        println!("API Key saved.");
    }

    println!("Type 'remove_api_key()' to remove the stored API key.");

    println!("Please enter something below:");

    loop {
        let mut prompt = String::new();

        io::stdin().read_line(&mut prompt).unwrap();

        if prompt.trim().eq_ignore_ascii_case("quit()") {
            break;
        }

        if prompt.trim().eq_ignore_ascii_case("remove_api_key()") {
            fs::remove_file(&api_key_file_path)?;
            println!("...");
            break;
        }

        println!("...");

        let response = generate_ai_content_service::execute(&prompt, &api_key).await?;

        println!("{}", response);
    }

    Ok(())
}
