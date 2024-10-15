use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use tauri::{
    command,
    http::{HeaderMap, HeaderValue},
};

#[command]
pub async fn fetch_notion(año: String, mes: String, dia: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = "https://api.notion.com/v1/databases/5b9f170bb1b14ba8a54b123f5eb85ebc/query";

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_static("Bearer secret_KE8PyouXrMKmYDig3lYIfg12g9ChzbKfYwQVTFTkTXP"),
    );
    headers.insert("Notion-Version", HeaderValue::from_static("2022-06-28"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = json!({
        "filter": {
            "and": [
                {
                    "property": "Fecha",
                    "date": {
                        "on_or_after": format!("{}-{}-01",año,mes)
                    }
                },
                {
                    "property": "Fecha",
                    "date": {
                        "on_or_before": format!("{}-{}-{}",año,mes,dia)
                    }
                }
            ]
        }
    });

    let response = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}
