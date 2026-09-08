use std::env;

use anyhow::anyhow;
use reqwest::{header, Client};

pub async fn bunpro_client() -> anyhow::Result<Client> {
    let frontend_session_token = get_frontend_auth_token().await?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Token token={}", frontend_session_token)
            .parse()
            .unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}

async fn get_frontend_auth_token() -> anyhow::Result<String> {
    const TOKEN_NAME: &str = "frontend_api_token";

    let bunpro_grammar_cookie = env::var("BUNPRO_GRAMMAR_COOKIE")?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        format!("_grammar_app_session={}", bunpro_grammar_cookie).parse()?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let bunpro_login = client
        .get("https://bunpro.jp/login")
        .send()
        .await?
        .error_for_status()?;

    let cookie = bunpro_login.cookies().find_map(|cookie| {
        if cookie.name() == TOKEN_NAME {
            Some(cookie.value().to_string())
        } else {
            None
        }
    });

    cookie.ok_or(anyhow!(format!("{} cookie could not be found", TOKEN_NAME)))
}
