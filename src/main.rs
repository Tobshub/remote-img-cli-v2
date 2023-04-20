use reqwest::{self, StatusCode};
use serde_json;
use std::path::PathBuf;
use std::{env, fs, io};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Option<&str> = args.get(1).map(|x| &**x);
    if command.is_some() {
        let command = command.unwrap();
        let client = reqwest::Client::new();
        match command {
            "--upload" | "-u" => {
                for path in &args[2..] {
                    let abs_path = fs::canonicalize(PathBuf::from(path));
                    match abs_path {
                        Ok(_path) => {
                            // upload_file_at_path(&client, &path)
                            // .await
                            // .unwrap_or_else(|e| println!("Failed to upload file: {}", e));
                        }
                        Err(_) => println!("Could not find file: {}", path),
                    }
                }
            }
            "--auth" => {
                let user = User {
                    email: args[2].clone(),
                    password: args[3].clone(),
                };
                get_auth_token(&client, &user).await.unwrap();
            }
            "--help" | "-h" | _ => display_help_message(),
        }
    } else {
        display_help_message();
    }
}

#[derive(Debug)]
struct User {
    email: String,
    password: String,
}

async fn get_auth_token(client: &reqwest::Client, user: &User) -> Result<(), reqwest::Error> {
    println!("Authenticating with {:?}", user);
    let url = String::from("http://localhost:4000/api/auth.login");
    let res = client
        .post(url)
        .json(&serde_json::json!({
            "email": user.email,
            "password": user.password
        }))
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => {
            let json: serde_json::Value = res.json().await?;
            let mut token = json["result"]["data"]["value"].to_string();
            save_token(&mut token).unwrap();
        }
        _ => println!("Request Failed with status code {:?}", res.status()),
    }

    return Ok(());
}

fn save_token(token: &mut String) -> io::Result<()> {
    let data = format!("TOBSMG_TOKEN={}", token);
    fs::write(".env", data.as_bytes())?;
    return Ok(());
}

fn display_help_message() {
    println!(
        "\
Tobsmg CLI - v2.0.0

Usage:

    --auth <email> <password>   Authenticate user and get auth token from server                    
    --server <server-url>       Configure the remote server url
    -u, --upload                Upload files to the server
    -t, --temp-upload           Temporarily Upload files to the server 
    "
    );
}
