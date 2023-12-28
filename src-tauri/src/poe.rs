use anyhow::Result;
use reqwest::cookie::Cookie;
use reqwest::Client;

const POE_HOST: &str = "https://poe.game.qq.com";
const TRADE_URL: &str = &format!("{}{}", POE_HOST, "/trade");

fn new_poe_client(session: &str) -> Client {
    let cookie = Cookie::build("POESESSID", session)
        .domain(POE_HOST)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();
}

pub async fn is_effective_session(session: &str) -> Result<bool> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
}
