use std::{
    sync::Arc,
    fs::{OpenOptions, File}, net::SocketAddr, collections::HashMap, num::NonZeroU16,
};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use chrono::Local;
use tokio::{net::{TcpStream, TcpListener}, io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};
use commands::*;
use reqwest::{Client, StatusCode};

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
    // let file = Arc::new(Mutex::new(OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("/home/tyler/log.txt")
    //     .unwrap()));

    // let metrics: MetricsHandle = Arc::new(Mutex::new(HashMap::new()));

    let socket = "127.0.0.1:7878".parse().unwrap();
    prometheus_exporter::start(socket).expect("Failed to connect to prometheus via 127.0.0.1:7878!");

    // let listener = TcpListener::bind(SOCKET).await.unwrap();

    // loop{
    //     let file = Arc::clone(&file);
    //     let metrics = Arc::clone(&metrics);

    //     if let Ok((stream, addr)) = listener.accept().await{
    //         log_activity(&file, format!("CONNECTION ESTABLISHED || With Address: {};", 
    //             stream.peer_addr().unwrap().to_string()));
    //         tokio::spawn(check_connection(stream, addr, file, metrics));
    //     }
    //     else{
    //         println!("FAILED TO ESTABLISH CONNECTION WITH CLIENT!");
    //     }
    // }

    let bearer_token = "glsa_SMNy2JM7lnhNTI34s56xpwoNMWNYeTC0_9ad1c23f";

    let client = Client::new();

    let response = client
        .post("http://127.0.0.1:3000/api/folders")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", bearer_token))
        .json(&json!({
            "title": "testfolder"
        }))
        .send().await.unwrap();

        if response.status() == StatusCode::from_u16(200).unwrap(){
            let folder_id = &response.json::<Value>().await.unwrap()["id"].as_i64().unwrap();

            let response = client
                .post("http://127.0.0.1:3000/api/dashboards/db")
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", bearer_token))
                .json(&json!({
                    "dashboard": {
                        "id": null,
                        "uid": null,
                        "title": "testdashboard",
                        "timezone": "browser",
                        "schemaVersion": 16,
                        "version": 0,
                        "refresh": "25s",
                        "panels": [
                            {
                                "id": 1,
                                "visualization": "time series",
                                "type": "graph",
                                "title": "residence_0_ds18b20_temperature",
                                "targets": [
                                    {
                                        "refId": "ds18b20_temperature_0",
                                        "expr": "residence_0_ds18b20_temperature_0"
                                    },
                                    {
                                        "refId": "ds18b20_temperature_1",
                                        "expr": "residence_0_ds18b20_temperature_1"
                                    },
                                    {
                                        "refId": "ds18b20_temperature_avg",
                                        "expr": "avg(residence_0_ds18b20_temperature_0 + residence_0_ds18b20_temperature_1)"
                                    }
                                ]
                            },
                        ]
                    },
                    "folderId": folder_id,
                    "message": "Created dashboard",
                    "overwrite": false
                }))
                .send().await.unwrap();

            println!("Response status: {}", response.status());
            println!("Response payload:\n{}", response.text().await.unwrap());
        }

    loop{}
}
