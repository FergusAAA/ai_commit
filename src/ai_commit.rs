// ===================================================================
// AI Commit Message Generation
// ===================================================================

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

pub async fn generate_commit_message(
    diff: &str,
    api_key: &str,
    language: &str,
    prompt: &str,
    url: &str,
    model: &str,
) -> Result<String, String> {
    let client = Client::new();

    let system_prompt = format!(
        "You are a helpful assistant that generates commit messages in {}. \
        The user will provide a git diff, and you should generate a concise and informative commit message. {}",
        language, prompt
    );

    let user_prompt = format!("Here is the git diff:\n```\n{}\n```", diff);

    let request = OpenAiRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ],
    };

    let res = client
        .post(url)
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = res.status();
    let body = res
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    if status.is_success() {
        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response_json) => {
                if response_json.choices.is_empty() {
                    Err("API response is empty.".to_string())
                } else {
                    Ok(response_json.choices[0].message.content.clone())
                }
            }
            Err(e) => Err(format!(
                "Failed to parse JSON response: {}. \nRaw response: {}",
                e, body
            )),
        }
    } else {
        Err(format!(
            "API request failed with status {}. \nResponse: {}",
            status, body
        ))
    }
}
