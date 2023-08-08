use std::{
    io::prelude::*,
    sync::Arc,
    fs::{OpenOptions, File}, net::SocketAddr, collections::HashMap,
};
use prometheus::register_gauge;
use serde_json::json;
use serde::{Deserialize, Serialize};
use chrono::Local;
use tokio::{net::{TcpStream, TcpListener}, io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};
use commands::*;

mod commands;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package{
    pub header: String,
    pub payload: String
}

const SOCKET: &str = "127.0.0.1:7879";

fn log_activity(file: &Arc<Mutex<File>>, msg: String){
    let time = Local::now().format("[%Y-%m-%d %H:%M:%S]");
    // file.lock().unwrap().write_all(format!("{time} - {msg}\n\n").as_bytes()).unwrap();
    println!("{time} - {msg}\n");
}

async fn handle_connection(id: &mut String, request: Package, metrics: &MetricsHandle)-> Package{
    let mut header = String::from("GOOD");

    let payload = match request.header.as_str(){
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
        "UPDATE_DATA" =>{
            match deserialize_data(request.payload){
                Ok(residence_data) =>{
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

async fn check_connection(mut stream: TcpStream, addr: SocketAddr, file: Arc<Mutex<File>>, metrics: MetricsHandle){
    let mut buf = [0_u8; 4096];
    let mut id = String::new();

    loop{
        match stream.read(&mut buf).await{
            Ok(0) =>{
                log_activity(&file, format!("CONNECTION TERMINATED NORMALLY || With Address: {}, ID: {};", 
                    addr.to_string(),
                    id));
                return;
            }
            Ok(_) =>{
                let mut response = Package{
                    header: String::from("BAD"),
                    payload: json!({ "error": "Request body format is ill-formed!" }).to_string(),
                };

                if let Some(package_end) = buf.iter().position(|x| *x == b'\n'){
                    if let Ok(request) = serde_json::from_slice::<Package>(&buf[..package_end]){
                        log_activity(&file, format!("INCOMING REQUEST || From Address: {}, ID: {}, Header: {}, Payload: {:?};", 
                            addr.to_string(),
                            id, 
                            request.header, 
                            request.payload));
                        response = handle_connection(&mut id, request, &metrics).await;
                    }
                }

                let mut response_bytes = serde_json::to_vec(&response).unwrap();
                response_bytes.push(b'\n');

                if stream.write_all(&response_bytes).await.is_ok(){
                    log_activity(&file, format!("OUTGOING RESPONSE SENT || To Address: {}, ID: {}, Header: {}, Payload: {:?};", 
                        addr.to_string(),
                        id,
                        response.header, 
                        response.payload));
                }
                else{
                    log_activity(&file, format!("OUTGOING RESPONSE FAILED || To Address: {}, ID: {}, Header: {}, Payload: {:?};", 
                        addr.to_string(),
                        id,
                        response.header, 
                        response.payload));
                }
            }
            Err(_) =>{
                log_activity(&file, format!("CONNECTION TERMINATED ABNORMALLY || With Address: {}, ID: {};", 
                    addr.to_string(),
                    id));
                return;
            }
        }
    }
}

#[tokio::main]
async fn main(){
    let file = Arc::new(Mutex::new(OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/tyler/log.txt")
        .unwrap()));

    let metrics: MetricsHandle = Arc::new(Mutex::new(HashMap::new()));

    let socket = "127.0.0.1:7878".parse().unwrap();
    prometheus_exporter::start(socket).expect("Failed to connect to prometheus via 127.0.0.1:7878!");

    let listener = TcpListener::bind(SOCKET).await.unwrap();

    loop{
        let file = Arc::clone(&file);
        let metrics = Arc::clone(&metrics);

        if let Ok((stream, addr)) = listener.accept().await{
            log_activity(&file, format!("CONNECTION ESTABLISHED || With Address: {};", 
                stream.peer_addr().unwrap().to_string()));
            tokio::spawn(check_connection(stream, addr, file, metrics));
        }
        else{
            println!("FAILED TO ESTABLISH CONNECTION WITH CLIENT!");
        }
    }
}
