use dotenv;
use reqwest;
use std::path::PathBuf;
use std::{env, fs, io};
mod auth;
mod upload;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Option<&str> = args.get(1).map(|x| &**x);
    if command.is_some() {
        let command = command.unwrap();
        let client = reqwest::Client::new();
        match command {
            "--help" | "-h" => display_help_message(),
            _ => {
                load_tobsmg_env_vars();
                match command {
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
                        let user = auth::User {
                            email: args[2].clone(),
                            password: args[3].clone(),
                        };
                        auth::get_auth_token(&client, &user).await.unwrap();
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
                }
            }
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

fn load_tobsmg_env_vars() {
    let env_file_path = if env::var("IS_DEV").is_ok() {
        String::from(".env")
    } else {
        format!(
            "{}/.tobsmg/.env",
            env::var("HOME").expect("no HOME set in env")
        )
    };
    println!("env path is {}", env_file_path);
    dotenv::from_path(env_file_path).ok();
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
