use std::{
    process::{Command, Stdio},
    fs::File,
    io::{BufRead, BufReader, Write}
};
use serde::{Serialize, Deserialize};

// JSON package payload format
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
    // Fork a new process running the arduino-cli monitor and pipe its stdout into this stdin
    let mut command = Command::new("/rems/files/serial/arduino-monitor")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to open /dev/ttyACM0");

    let stdout = command.stdout.take().expect("Failed to open stdout");
    let reader = BufReader::new(stdout);

    let mut json = String::new();
    // Iterate over the stdout of the monitor
    for line in reader.lines(){
        if let Ok(line) = line{
            // Start building if opening brace is found
            if line.contains("{"){
                json.push('{');
            }
            // Write to file if closing brace is found and cleanup
            else if line.contains("}"){
                json.push('}');

                if let Ok(json) = serde_json::from_str::<ResidenceData>(&json){
                    let mut file = File::create("/rems/readings/residence.json")
                        .expect("Failed to open /rems/readings/residence.json");

                    let json_string = serde_json::to_string_pretty(&json).unwrap();

                    file.write_all(json_string.as_bytes())
                        .expect("Failed to write to /rems/readings/residence.json");

                    println!("{json_string}");
                }

                json = String::new();
            }
            // If started building, add inner data
            else{
                if !json.is_empty(){
                    json.push_str(&line);
                }
            }
        }
    }
}
