use std::env;
use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::models::general::llm::{ChatCompletion, Message};

// Call LLM
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API Key and Host
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env");
    let api_host = env::var("OPENAI_API_HOST").expect("OPENAI_API_HOST not found in .env");

    // Confirm endpoint
    let url: &str = &format!("{api_host}/v1/chat/completions");

    // Create headers
    let mut headers = HeaderMap::new();

    headers.insert("authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1
    };

    // Troubleshooting
    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test, Give me a short response.".to_string()
        };

        let messages = vec![message];

        call_gpt(messages).await;
    }
}
