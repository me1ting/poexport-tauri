use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

static API_ARRAY: &'static [&str] = &[
    "http://42.193.7.36:8888/api/version/export",
    "https://poe.pathof.top/api/version/export",
];

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    pub latest: String,
    pub changelog: String,
}

pub async fn check_for_update() -> Result<Latest> {
    for api in API_ARRAY {
        let resp = request_latest(api).await;
        match resp {
            Result::Ok(v) => return Ok(v),
            Err(e) => log::error!("{:#}", e),
        }
    }
    Err(anyhow::anyhow!("check for update failed"))
}

async fn request_latest(url: &str) -> Result<Latest> {
    let resp = reqwest::get(url).await?;
    if !resp.status().is_success() {
        return Err(anyhow::anyhow!(
            "request {}: {}",
            url,
            resp.status().as_u16()
        ));
    }

    let body = resp.text().await?;
    let v: Latest = serde_json::from_str(&body)?;

    Ok(v)
}
