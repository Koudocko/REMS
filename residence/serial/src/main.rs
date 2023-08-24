use std::{
    process::{Command, Stdio},
    fs::File,
    io::{BufRead, BufReader, Write}
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResidenceData{
    ds18b20_temperature: [f64; 3],
    dht22_humidity: [f64; 4],
    dht22_temperature: [f64; 4],
    motion_sensor: [bool; 4],
    sound_sensor: [bool; 4],
    soil_moisture: i64
}

fn main(){
    let mut command = Command::new("script")
        .args(&["-q", "-c", "arduino-cli monitor -p /dev/ttyACM0", "/dev/null"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to open /dev/ttyACM0");

    let stdout = command.stdout.take().expect("Failed to open stdout");
    let reader = BufReader::new(stdout);

    let mut json = String::new();
    for line in reader.lines() {
        if let Ok(line) = line{
            if line.contains("{"){
                json.push('{');
            }
            else if line.contains("}"){
                json.push('}');

                if let Ok(json) = serde_json::from_str::<ResidenceData>(&json){
                    let mut file = File::create("/rems/readings/residence.json")
                        .expect("Failed to open /rems/readings/residence.json");

                    file.set_len(0).expect("Failed to clear /rems/readings/residence.json");

                    file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes())
                        .expect("Failed to write to /rems/readings/residence.json");
                }

                json = String::new();
            }
            else{
                if !json.is_empty(){
                    json.push_str(&line);
                }
            }
        }
    }
}
