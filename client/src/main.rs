use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{
    env,
    fs,
    io::prelude::*,
    net::TcpStream,
};
use std::error::Error;

static SOCKET: &'static str = "127.0.0.1:7878";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package{
    pub header: String,
    pub payload: String
}

#[derive(Serialize, Deserialize)]
pub struct ResidenceData{
    ds18b20_temperature: [f64; 3],
    dht22_humidity: [f64; 4],
    dht22_temperature: [f64; 4],
    motion_sensor: [bool; 4],
    sound_sensor: [bool; 4],
    soil_moisture: i64
}

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
    if let Ok(mut stream) = TcpStream::connect(SOCKET){
        if let Ok(env_var) = env::var("RESIDENT_ID"){
            write_stream(
                &mut stream,
                String::from("SET_ID"),
                json!({"residence_id": &env_var}).to_string()
            ).is_ok();

            loop{
                if let Ok(residence_data) = fs::read_to_string("/rems/readings/log1.txt"){
                    if serde_json::from_str::<ResidenceData>(&residence_data).is_ok(){
                        write_stream(
                            &mut stream,
                            String::from("UPDATE_DATA"),
                            residence_data
                        ).is_ok();
                    }
                }
                else{
                    panic!("File at /rems/readings/log1.txt does not exist!");
                }
            }
        }
        else{
            panic!("Environment variable RESIDENT_ID is not set!");
        }
    }
    else{
        panic!("Failed to connect to server via {}", SOCKET);
    }
}
