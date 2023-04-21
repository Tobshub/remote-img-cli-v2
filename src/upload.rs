use base64::{engine::general_purpose, Engine as _};
use mime_guess;
use serde_json;
use std::{env, fs, path::PathBuf};

use reqwest::{self, StatusCode};

pub async fn upload_image_at_path(client: &reqwest::Client, path: &PathBuf, is_temp: bool) {
    println!("Uploading file at {:?}", path);
    let data = read_image_data_at_path(path);

    match data {
        Some(file_data) => {
            let file_type = mime_guess::from_path(path).first();
            match file_type {
                None => println!("Unknown file type: file must be an image"),
                Some(file_type) => {
                    let file_type = file_type.to_string();
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let server_url =
                        env::var("TOBSMG_SERVER_URL").expect("Server url has not been set");
                    let upload_url = format!(
                        "{}/api/upload.{}",
                        &server_url,
                        if is_temp { "tempUpload" } else { "permUpload" }
                    );
                    let auth_token = env::var("TOBSMG_TOKEN").expect("Auth token is missing");
                    let res = client
                        .post(upload_url)
                        .header("Authorization", auth_token)
                        .json(&serde_json::json!({
                            "data": file_data,
                            "type": file_type,
                            "name": file_name
                        }))
                        .send()
                        .await;

                    if res.is_ok() {
                        let res = res.unwrap();
                        match res.status() {
                            StatusCode::OK => {
                                let json: serde_json::Value = res.json().await.unwrap();
                                let image_url = &json["result"]["data"]["value"].to_string();
                                // TODO: remove '"' from image_url
                                println!(
                                    "Success: Image is available at {}/img/{}",
                                    server_url, image_url
                                );
                            }
                            _ => println!("Request Failed with status code {:?}", res.status()),
                        };
                    };
                }
            }
        }
        _ => {}
    };
}

fn read_image_data_at_path(path: &PathBuf) -> Option<String> {
    let data = fs::read(path);
    match data {
        Ok(data) => {
            let data = general_purpose::STANDARD_NO_PAD.encode(data);
            return Some(data);
        }
        Err(_) => {
            println!("Failed to read file data");
            return None;
        }
    }
}
