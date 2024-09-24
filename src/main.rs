mod services;

use services::generate_ai_content_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response =
        generate_ai_content_service::execute("Explain why Rust is so incredible").await?;

    println!("{}", response);

    Ok(())
}
