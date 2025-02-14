use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use tokio;
use colored::*;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::env;

// Structures for Gemini API
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    top_k: i32,
    top_p: f32,
    max_output_tokens: i32,
    response_mime_type: String,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize, Debug)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize, Debug)]
struct ResponsePart {
    text: String,
}

struct AppConfig {
    api_key: String,
}

impl AppConfig {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let api_key = env::var("GEMINI_API_KEY")
            .expect("GEMINI_API_KEY must be set in environment");
        Ok(AppConfig { api_key })
    }
}

async fn query_gemini(
    config: &AppConfig,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Enhance the prompt with context
    let enhanced_prompt = format!("As an exotic guide to Tokyo, can you provide insights on: {}", prompt);

    let request = GeminiRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: enhanced_prompt,
            }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.7,  // Adjust for creativity
            top_k: 50,
            top_p: 0.95,
            max_output_tokens: 150,  // Adjust based on expected response length
            response_mime_type: "text/plain".to_string(),
        },
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-pro-exp-02-05:generateContent?key={}",
        config.api_key
    );

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await?
        .json::<GeminiResponse>()
        .await?;

    Ok(response.candidates[0].content.parts[0].text.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::new()?;
    
    println!("{}", "Welcome to Tokyo Explorer + AI!".bright_cyan().bold());
    println!("{}", "体験を楽しんでください (Enjoy the experience!)".bright_magenta());

    loop {
        display_menu();
        let choice = get_user_input()?;

        match choice.as_str() {
            "1" => display_shibuya_crossing(),
            "2" => display_mount_fuji(),
            "3" => show_random_fact().await,
            "4" => display_shinkansen(),
            "5" => ai_tokyo_guide(&config).await?,
            "6" => ai_travel_planner(&config).await?,
            "7" => break,
            _ => println!("Invalid choice! Please try again."),
        }
        thread::sleep(Duration::from_secs(2));
    }

    Ok(())
}

fn display_menu() {
    println!("\n{}", "=== Tokyo Explorer + AI Menu ===".green());
    println!("1. View Shibuya Crossing ASCII Art");
    println!("2. View Mount Fuji ASCII Art");
    println!("3. Get a Random Tokyo Fact");
    println!("4. View Shinkansen ASCII Art");
    println!("5. AI Tokyo Guide");
    println!("6. AI Travel Planner");
    println!("7. Exit");
    print!("\nEnter your choice (1-7): ");
    io::stdout().flush().unwrap();
}

async fn ai_tokyo_guide(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("What would you like to know about Tokyo? ");
    let query = get_user_input()?;
    
    println!("\n{}", "Asking AI...".bright_yellow());
    let prompt = format!(
        "Act as a knowledgeable Tokyo guide. Answer this question about Tokyo: {}",
        query
    );
    
    let response = query_gemini(config, &prompt).await?;
    display_ai_response_ascii_box(&response, &query);
    Ok(())
}

async fn ai_travel_planner(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("How many days will you be in Tokyo? ");
    let days = get_user_input()?;
    
    println!("What are your main interests? (e.g., culture, food, technology) ");
    let interests = get_user_input()?;
    
    println!("\n{}", "Creating your personalized itinerary...".bright_yellow());
    let prompt = format!(
        "Create a detailed {}-day Tokyo itinerary for someone interested in {}. Include specific locations, estimated times, and travel tips.",
        days, interests
    );
    
    let response = query_gemini(config, &prompt).await?;
    display_ai_response_ascii_box(&response, &interests);
    Ok(())
}

fn display_shibuya_crossing() {
    println!("{}", r#"
      _______|_______|_______
     /       |       |       \
    /   🚶   |   🚶‍♀️   |   🚶‍♂️    \
   /    🚶‍♂️   |   🚶   |   🚶‍♀️     \
  /     🚶   |   🚶‍♂️   |   🚶      \
 /_______|_______|_______|_\
    "#.bright_yellow());
    println!("Welcome to the famous Shibuya Crossing!");
}

fn display_mount_fuji() {
    println!("{}", r#"
         /\
        /  \
       /    \
      /      \
     /   ❄️   \
    /        \
   /          \
  /____________\
    "#.bright_white());
    println!("Mount Fuji - Japan's iconic mountain!");
}

async fn show_random_fact() {
    let facts = vec![
        "Tokyo is the world's largest metropolitan area.",
        "Tokyo was formerly known as Edo.",
        "Tokyo has over 200 stations on its metro system.",
        "Tokyo Tower is 333 meters tall.",
        "Tokyo hosts the world's busiest pedestrian crossing.",
    ];
    
    println!("{}", "\nDid you know?".bright_cyan());
    println!("{}", facts.choose(&mut rand::thread_rng()).unwrap().bright_yellow());
}

fn display_shinkansen() {
    println!("{}", r#"
   _____________________
  /_____________________/|
 /____________________ /||
|  _________________  |||
| |                 | |||
| |_________________| ||/
|_____________________|/
    "#.bright_blue());
    println!("The famous Japanese bullet train!");
}

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn generate_ascii_art(input: &str) {
    for c in input.chars() {
        println!("{}", c); // Simple representation, can be enhanced
    }
}

fn display_ai_response_ascii_box(response: &str, user_input: &str) {
    generate_ascii_art(user_input); // Generate ASCII art from user input
    let box_width = response.len() + 4; // Adding padding for the box
    println!("{}",
             "+".repeat(box_width));
    println!("| {} |", response); // Centered response
    println!("{}",
             "+".repeat(box_width));
}