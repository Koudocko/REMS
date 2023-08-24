use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{
    env,
    fs,
    io::prelude::*,
    net::TcpStream,
};
use std::error::Error;

// TCP package format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package{
    pub header: String,
    pub payload: String
}

// Format to equate to equivalent deserialized JSON
#[derive(Serialize, Deserialize)]
pub struct ResidenceData{
    ds18b20_temperature: [f64; 3],
    dht22_humidity: [f64; 4],
    dht22_temperature: [f64; 4],
    motion_sensor: [bool; 4],
    sound_sensor: [bool; 4],
    soil_moisture: i64
}

// Writes the package safely across the TCP stream
pub fn write_stream(stream: &mut TcpStream, header: String, payload: String)-> Result<(), Box<dyn Error>>{
    let package = Package{
        header,
        payload
    };

    let mut buf = serde_json::to_vec(&package).unwrap();
    buf.push(b'\n');
    stream.write_all(&mut buf)?;

    Ok(())
}

fn main() {
    // Load .env file variables
    dotenvy::dotenv().unwrap();

    // Checks that variables are present and viable
    if let Ok(server_socket) = env::var("SERVER_IP"){
        if let Ok(mut stream) = TcpStream::connect(&server_socket){
            if let Ok(residence_id) = env::var("RESIDENCE_ID"){
                // Initial set residence id request before providing sensor data
                write_stream(
                    &mut stream,
                    String::from("SET_ID"),
                    json!({
                        "residence_id": &residence_id
                    }).to_string()
                ).unwrap();

                // Enter loop of readings from the sensor data file and sending it to BBP
                loop{
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    if let Ok(residence_data) = fs::read_to_string("/rems/readings/residence.json"){
                        // Ensures the JSON is valid before sending
                        if serde_json::from_str::<ResidenceData>(&residence_data).is_ok(){
                            write_stream(
                                &mut stream,
                                String::from("UPDATE_DATA"),
                                residence_data
                            ).unwrap();
                        }
                    }
                    else{
                        panic!("File at /rems/readings/residence.json does not exist!");
                    }
                }
            }
            else{
                panic!("Environment variable RESIDENCE_ID is not set!");
            }
        }
        else{
            panic!("Failed to connect to server via {}", server_socket);
        }
    }
    else{
        panic!("Environment variable SERVER_IP is not set!");
    }

}
