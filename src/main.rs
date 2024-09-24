mod services;

use services::generate_ai_content_service;

use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Gemini CLI");
    println!("To exit type quit()");

    loop {
        let mut prompt = String::new();

        io::stdin().read_line(&mut prompt).unwrap();

        if prompt.trim().eq_ignore_ascii_case("quit()") {
            break;
        }

        println!("...");

        let response = generate_ai_content_service::execute(prompt.trim()).await?;

        println!("{}", response);
    }

    Ok(())
}
