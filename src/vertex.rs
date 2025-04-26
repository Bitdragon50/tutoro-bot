use std::env;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
    usage_metadata: UsageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    content: Content,
    finish_reason: String,
    index: Option<i32>,
    safety_ratings: Vec<SafetyRating>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageMetadata {
    prompt_token_count: i32,
    candidates_token_count: i32,
    total_token_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SafetyRating {
    category: String,
    probability: String,
    severity: String,
}

pub async fn call_vertex_client(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // Replace with your actual project ID and location
    let project_id = "kubernetes-the-hardway-357008";
    let location = "us-central1";
    let model_name = "gemini-2.5-flash-preview-04-17";
    
    let access_token = fetch_token().await.unwrap();

    // Construct the API endpoint URL
    let endpoint_url = format!(
        "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
        project_id, location, model_name
    );

    // Construct the request body, mirroring your Python configuration
    let request_body = generate_request_body(prompt);

    // Make the API request
    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint_url)
        .bearer_auth(access_token)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Check for errors
    if !response.status().is_success() {
        let err_text = &response.text().await?;
        eprintln!("Error Body: {}", err_text);
        return Err("API request failed".into());
    }
    dbg!(&response);
    // Parse the JSON response
    let response_body = response.json::<GenerateContentResponse>().await.unwrap();

    // Print the generated text
    let filtered_content = response_body.candidates.into_iter().map(|candidate| candidate.content );
    let filtered_parts = filtered_content.map(|content|
        content.parts.iter().map(|part| part.text.clone()).collect::<Vec<String>>(),
            
        );
    let data_packages = filtered_parts.flatten().collect::<Vec<String>>();

    Ok(data_packages.join("\n"))
}

async fn fetch_token() -> Result<String, Box<dyn std::error::Error>> {
    let path = env::var("CREDENTIAL_PATH").expect("Expected CREDENTIAL_PATH in the environment");
    let reader = std::fs::File::open(path).expect("Failed to open credentials file");
    // Read the credentials from the JSON file  
    let authorized_user: Value = serde_json::from_reader(reader)?;
    let token_manager = google_cloud_auth::credentials::user_account::Builder::new(authorized_user).build();
    let access_token = token_manager.unwrap().token().await?;
    Ok(access_token.token)
}

fn generate_request_body(prompt: String) -> Value {
    json!({
        "contents": [
            {
                "role": "user",
                "parts": [{"text":prompt}] // Add your input text here
            }
        ],
        "generationConfig": {
            "temperature": 1.0,
            "topP": 1.0,
            "seed": 0,
            "maxOutputTokens": 65535,
        },
        "safetySettings": [
            {"category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_NONE"},
            {"category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_NONE"},
            {"category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_NONE"},
            {"category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_NONE"}
        ]
    })
}
