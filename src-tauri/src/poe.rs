use std::mem;
use std::sync::Arc;

use anyhow::{Ok, Result};
use reqwest::cookie::Jar;
use reqwest::{StatusCode, Url};
use tokio::sync::{RwLock, RwLockReadGuard};

pub const TX_POE_HOST: &str = "poe.game.qq.com";

const PROFILE_PATH: &str = "/api/profile";
const GET_CHARACTERS_PATH: &str = "/character-window/get-characters";
const GET_PASSIVE_SKILLS_PATH: &str = "/character-window/get-passive-skills";
const GET_ITEMS_PATH: &str = "/character-window/get-items";

const COOKIE_NAME_POESESSID: &str = "POESESSID";

const ERR_UNAUTHORIZED: &str = "POESESSID已失效，请更新";
const ERR_GET_CHARACTERS_FORBIDDEN: &str = "你查看的用户不存在或已隐藏";
const ERR_RATE_LIMIT: &str = "请求过于频繁，请稍后再试";
const ERR_UNKNOWN: &str = "未预期的错误";
pub struct PoeClient {
    client: reqwest::Client,
    profile_url: String,
    get_characters_url: String,
    get_passive_skills_url: String,
    get_items_url: String,
}

impl PoeClient {
    pub fn new(poe_host: &str, poesessid: &str) -> Result<PoeClient> {
        let cookie = format!("{}={}", COOKIE_NAME_POESESSID, poesessid);
        let url = format!("https://{}", poe_host).parse::<Url>().unwrap();

        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &url);

        let client = reqwest::Client::builder()
            .cookie_provider(Arc::new(jar))
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let profile_url = url.join(PROFILE_PATH).unwrap().to_string();
        let get_characters_url = url.join(GET_CHARACTERS_PATH).unwrap().to_string();
        let get_passive_skills_url = url.join(GET_PASSIVE_SKILLS_PATH).unwrap().to_string();
        let get_items_url = url.join(GET_ITEMS_PATH).unwrap().to_string();

        let poe_client = PoeClient {
            client,
            profile_url,
            get_characters_url,
            get_passive_skills_url,
            get_items_url,
        };

        return Ok(poe_client);
    }

    pub async fn get_profile(&self) -> Result<String> {
        let resp = self.client.get(&self.profile_url).send().await?;
        Self::check_status(&resp.status())?;

        let body = resp.text().await?;

        Ok(body)
    }

    fn check_status(status: &StatusCode) -> Result<()> {
        if status.is_success() {
            return Ok(());
        }

        let code = status.as_u16();
        if code == 401 {
            return Err(anyhow::anyhow!(ERR_UNAUTHORIZED));
        }
        if code == 403 {
            return Err(anyhow::anyhow!(ERR_GET_CHARACTERS_FORBIDDEN));
        }
        if code == 429 {
            return Err(anyhow::anyhow!(ERR_RATE_LIMIT));
        }

        return Err(anyhow::anyhow!("{}: {}", ERR_UNKNOWN, code));
    }

    pub async fn get_characters(&self, account: &str, realm: &str) -> Result<String> {
        let params: [(&str, &str); 2] = [("accountName", account), ("realm", realm)];

        let resp = self
            .client
            .post(&self.get_characters_url)
            .form(&params)
            .send()
            .await?;

        Self::check_status(&resp.status())?;

        let body = resp.text().await?;

        Ok(body)
    }

    pub async fn get_items(&self, account: &str, character: &str, realm: &str) -> Result<String> {
        let params: [(&str, &str); 3] = [
            ("accountName", account),
            ("character", character),
            ("realm", realm),
        ];

        let resp = self
            .client
            .post(&self.get_items_url)
            .form(&params)
            .send()
            .await?;

        Self::check_status(&resp.status())?;

        let body = resp.text().await?;

        Ok(body)
    }

    pub async fn get_passive_skills(
        &self,
        account: &str,
        character: &str,
        realm: &str,
    ) -> Result<String> {
        let params: [(&str, &str); 3] = [
            ("accountName", account),
            ("character", character),
            ("realm", realm),
        ];

        let resp = self
            .client
            .post(&self.get_passive_skills_url)
            .form(&params)
            .send()
            .await?;

        Self::check_status(&resp.status())?;

        let body = resp.text().await?;

        Ok(body)
    }
}

pub struct PoeClientManager {
    lock: RwLock<PoeClient>,
}

impl PoeClientManager {
    pub fn new(poesessid: &str) -> Result<PoeClientManager> {
        let client = PoeClient::new(TX_POE_HOST, poesessid)?;
        let lock = RwLock::new(client);
        return Ok(PoeClientManager { lock });
    }

    pub async fn get(&self) -> RwLockReadGuard<PoeClient> {
        return self.lock.read().await;
    }

    pub async fn set(&self, mut new: PoeClient) {
        let mut old = self.lock.write().await;
        mem::swap(&mut (*old), &mut new);
    }
}

#[cfg(test)]
mod tests {
    use super::{PoeClient, TX_POE_HOST};

    #[tokio::test]
    async fn it_works() {
        let poesessid = std::env::var("poesessid").unwrap();
        let client = PoeClient::new(TX_POE_HOST, &poesessid).unwrap();
        client.get_profile().await.unwrap();

        let account = "盲将盲将";
        let character = "B站高远寒_S24";
        let realm = "pc";
        client.get_characters(account, realm).await.unwrap();
        client.get_items(account, character, realm).await.unwrap();
        client
            .get_passive_skills(account, character, realm)
            .await
            .unwrap();
    }
}
