use dotenv;
use reqwest::{self, StatusCode};
use serde_json;
use std::path::PathBuf;
use std::{env, fs, io};
mod upload;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let command: Option<&str> = args.get(1).map(|x| &**x);
    if command.is_some() {
        let command = command.unwrap();
        let client = reqwest::Client::new();
        match command {
            "--help" | "-h" => display_help_message(),
            _ => match command {
                // TODO: check for "env" file else create it
                "--upload" | "-u" | "--temp-upload" => {
                    if args.len() < 3 {
                        display_help_message()
                    }
                    for path in &args[2..] {
                        let abs_path = fs::canonicalize(PathBuf::from(path));
                        match abs_path {
                            Ok(path) => {
                                upload::upload_image_at_path(
                                    &client,
                                    &path,
                                    if command == "--temp-upload" {
                                        true
                                    } else {
                                        false
                                    },
                                )
                                .await;
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
                "--server" | "-s" => {
                    let server_url = args.get(2);
                    match server_url {
                        None => {
                            let server_url = env::var("TOBSMG_SERVER_URL").ok();
                            println!("Server url is set as {:?}", server_url);
                        }
                        Some(server_url) => {
                            set_server_url(server_url).unwrap();
                        }
                    }
                }
                _ => display_help_message(),
            },
        }
    } else {
        display_help_message();
    }
}

fn set_server_url(server_url: &String) -> io::Result<()> {
    let data = format!("TOBSMG_SERVER_URL=\"{}\"\n", server_url);
    fs::write(".env", data.as_bytes())?;
    return Ok(());
}

#[derive(Debug)]
struct User {
    email: String,
    password: String,
}

async fn get_auth_token(client: &reqwest::Client, user: &User) -> Result<(), reqwest::Error> {
    println!("Authenticating with {:?}", user);
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
    -u, --upload                Upload files to the server. E.g. -u ./path/to/img1 ../path/to/img2 ...
    -t, --temp-upload           Temporarily Upload files to the server 
    "
    );
}
