use clap::{Arg, Command};
use reqwest;
use serde_json::Value;
use tokio;
use dotenv::dotenv;
use std::env;
use regex::Regex;

#[tokio::main]
async fn main() {
    let matches = Command::new("Libsug")
        .version("1.0")
        .author("Serhat Polat")
        .about("Suggests libraries based on technology and purpose")
        .arg(Arg::new("technology")
            .help("The technology (e.g., Python, React.js)")
            .required(true)
            .index(1))
        .arg(Arg::new("purpose")
            .help("The purpose (e.g., dashboard data visualization)")
            .required(true)
            .index(2))
        .get_matches();

    let technology = matches.get_one::<String>("technology").unwrap();
    let purpose = matches.get_one::<String>("purpose").unwrap();

    let prompt = format!(
        "Suggest three libraries for {} to {} with short descriptions. Provide them in a numbered list. Be concise and clear.",
        technology, purpose
    );
    
    match fetch_suggestions(&prompt).await {
        Ok(response) => println!("{}", clean_response(&response)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn fetch_suggestions(prompt: &str) -> Result<String, reqwest::Error> {
    dotenv().ok();

    let api_token = env::var("API_TOKEN").expect("API_TOKEN not set in .env file");
    const API_BASE_URL: &str = "https://api-inference.huggingface.co/models/";
    const MODEL: &str = "deepseek-ai/DeepSeek-R1-Distill-Qwen-32B";

    let client = reqwest::Client::new();
    let body = serde_json::json!({ "inputs": prompt });

    let res = client
        .post(format!("{}{}", API_BASE_URL, MODEL))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&body)
        .send()
        .await?;

    let json: Value = res.json().await?;

    if let Some(text) = json[0]["generated_text"].as_str() {
        Ok(text.to_string())
    } else {
        Ok("No suggestions found".to_string())
    }
}

// purpose: cleaning the introduction part of AI response and having the core part as output
fn clean_response(response: &str) -> String {
    let pattern = r"(?is)^.*?</think>\s*";
    let re = Regex::new(pattern).unwrap();

    let cleaned_response = re.replace_all(response, "");
    cleaned_response.to_string()
}
