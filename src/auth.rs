use reqwest::{self, StatusCode};
use serde_json;
use std::{env, fs, io};

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

pub async fn get_auth_token(client: &reqwest::Client, user: &User) -> Result<(), reqwest::Error> {
    println!("Authenticating...");
    let server_url = env::var("TOBSMG_SERVER_URL").expect("Server url has not been set");
    let auth_url = format!("{}/api/auth.login", &server_url);
    let res = client
        .post(auth_url)
        .json(&serde_json::json!({
            "email": user.email,
            "password": user.password
        }))
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => {
            let json: serde_json::Value = res.json().await?;
            let token = json["result"]["data"]["value"].to_string();
            save_token(&token).unwrap();
            println!("Auth token has been saved");
        }
        _ => println!("Request Failed with status code {:?}", res.status()),
    }

    return Ok(());
}

fn save_token(token: &String) -> io::Result<()> {
    let server_url = &env::var("TOBSMG_SERVER_URL").unwrap();
    let data = format!(
        "TOBSMG_SERVER_URL=\"{}\"\nTOBSMG_TOKEN={}",
        server_url, token
    );
    let env_file_path = env::var("TOBSMG_ENV_PATH").expect("Tobsmg ENV file path not set");
    fs::write(env_file_path, data.as_bytes())?;
    return Ok(());
}
