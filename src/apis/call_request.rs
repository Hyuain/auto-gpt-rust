use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

// Call LLM
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API Key and Host
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env");
    let api_host = env::var("OPENAI_API_HOST").expect("OPENAI_API_HOST not found in .env");

    // Confirm endpoint
    let url: &str = &format!("{api_host}/v1/chat/completions");

    // Create headers
    let mut headers = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();
    //
    // dbg!(res_raw.text().await.unwrap());

    // Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send Response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test, Give me a short response.".to_string(),
        };

        let messages = vec![message];

        let res = call_gpt(messages).await;

        if let Ok(res_str) = res {
            dbg!(res_str);
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
