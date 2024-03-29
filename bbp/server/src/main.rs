use std::{
    sync::Arc,
    fs::File, net::SocketAddr, collections::HashMap, io::Write,
};
use local_ip_address::linux::local_ip;
use serde_json::json;
use serde::{Deserialize, Serialize};
use chrono::Local;
use tokio::{net::{TcpStream, TcpListener}, io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};
use commands::*;

mod commands;

type FileHandle = Arc<Mutex<Option<File>>>;

// JSON payload format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package{
    pub header: String,
    pub payload: String
}

// Log activity via stdout and optionally a log file
async fn log_activity(file: &FileHandle, msg: String){
    let time = Local::now().format("[%Y-%m-%d %H:%M:%S]");
    if let Some(file) = file.lock().await.as_mut(){
        file.write_all(format!("{time} - {msg}\n\n").as_bytes()).unwrap();
    }
    println!("{time} - {msg}\n");
}

// Handle validated packege sent from client
async fn handle_connection(id: &mut String, request: Package, metrics: &MetricsHandle)-> Package{
    let mut header = String::from("GOOD");

    // List of valid headers the client can use
    let payload = match request.header.as_str(){
        // Links a residence id with the TCP connection for later use
        "SET_ID" =>{
            match set_id(request.payload){
                Ok(residence_id) =>{
                    *id = residence_id;
                    String::new()
                }
                Err("INVALID_FORMAT") =>{
                    header = String::from("BAD");
                    json!({ "error": "Request body format is ill-formed!" }).to_string()
                }
                _ => String::new(),
            }
        }
        // Updates database with JSON provided by client 
        "UPDATE_DATA" =>{
            // Deserialization check
            match deserialize_data(request.payload){
                Ok(residence_data) =>{
                    // Updates databse using now deserialized data
                    if update_data(id.to_owned(), residence_data, metrics).await.is_ok(){
                        String::new()
                    }
                    else{
                        header = String::from("BAD");
                        json!({ "error": "Error updating prometheus database!" }).to_string()
                    }
                }
                Err("INVALID_FORMAT") =>{
                    header = String::from("BAD");
                    json!({ "error": "Request body format is ill-formed!" }).to_string()
                }
                _ => String::new(),
            }
        }
        _ => String::new(),
    };

    Package{ header, payload }
}

// Validator for a client's incoming and outgoing packets
async fn check_connection(mut stream: TcpStream, addr: SocketAddr, file: FileHandle, metrics: MetricsHandle){
    let mut buf = [0_u8; 4096];
    let mut id = String::new();

    // Listen to connect client's requests
    loop{
        match stream.read(&mut buf).await{
            // Ok(0): 0 bytes to read, client closed stream
            Ok(0) =>{
                log_activity(&file, format!("CONNECTION TERMINATED NORMALLY || With Address: {}, ID: {};", 
                    addr.to_string(),
                    id)).await;
                return;
            }
            // Ok(_): bytes received to process
            Ok(_) =>{
                // Default to ill-formed package layout
                let mut response = Package{
                    header: String::from("BAD"),
                    payload: json!({ "error": "Request body format is ill-formed!" }).to_string(),
                };

                // Deserialize package into Package struct and handler request
                if let Some(package_end) = buf.iter().position(|x| *x == b'\n'){
                    if let Ok(request) = serde_json::from_slice::<Package>(&buf[..package_end]){
                        log_activity(&file, format!("INCOMING REQUEST || From Address: {}, ID: {}, Header: {}, Payload: {:?};", 
                            addr.to_string(),
                            id, 
                            request.header, 
                            request.payload)).await;
                        response = handle_connection(&mut id, request, &metrics).await;
                    }
                }

                let mut response_bytes = serde_json::to_vec(&response).unwrap();
                response_bytes.push(b'\n');

                // Respond to client's request accordingly
                if stream.write_all(&response_bytes).await.is_ok(){
                    log_activity(&file, format!("OUTGOING RESPONSE SENT || To Address: {}, ID: {}, Header: {}, Payload: {:?};", 
                        addr.to_string(),
                        id,
                        response.header, 
                        response.payload)).await;
                }
                else{
                    log_activity(&file, format!("OUTGOING RESPONSE FAILED || To Address: {}, ID: {}, Header: {}, Payload: {:?};", 
                        addr.to_string(),
                        id,
                        response.header, 
                        response.payload)).await;
                }
            }
            // Err(_): client terminated abnormally, connection closed
            Err(_) =>{
                log_activity(&file, format!("CONNECTION TERMINATED ABNORMALLY || With Address: {}, ID: {};", 
                    addr.to_string(),
                    id)).await;
                return;
            }
        }
    }
}

#[tokio::main]
async fn main(){
    // Load .env file variables
    dotenvy::dotenv().unwrap();

    // Commented out log file for space conservation
    let file = Arc::new(Mutex::new(None));
        // if let Ok(file) = OpenOptions::new()
        //     .create(true)
        //     .append(true)
        //     .open("/rems/logs/log.txt"){
        //     Some(file)
        // }
        // else
        //     { None }));

    let ip = local_ip().unwrap().to_string();

    let metrics: MetricsHandle = Arc::new(Mutex::new(HashMap::new()));

    // Enable runtime prometheus exporter
    prometheus_exporter::start((ip.clone() + ":7878").parse().unwrap())
        .expect("Failed to connect to prometheus!");

    let listener = TcpListener::bind(ip + ":7879").await.unwrap();

    // Main TCP listener loop
    loop{
        let file = Arc::clone(&file);
        let metrics = Arc::clone(&metrics);

        if let Ok((stream, addr)) = listener.accept().await{
            log_activity(&file, format!("CONNECTION ESTABLISHED || With Address: {};", 
                stream.peer_addr().unwrap().to_string())).await;
            tokio::spawn(check_connection(stream, addr, file, metrics));
        }
        else{
            println!("FAILED TO ESTABLISH CONNECTION WITH CLIENT!");
        }
    }
}
