use prometheus::{register_counter, register_gauge, register_int_gauge};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, boxed::Box};

pub type Eval<T> = Result<T, &'static str>;

#[derive(Serialize, Deserialize)]
pub struct ResidenceData{
    ds18b20_temperature: [f64; 3],
    dht22_humidity: [f64; 4],
    dht22_temperature: [f64; 4],
    motion_sensor: [bool; 4],
    sound_sensor: [bool; 4],
    soil_moisture: i64
}

pub fn set_id(payload: String)-> Eval<String>{
    if let Ok(payload) = serde_json::from_str::<Value>(&payload){
       if let Some(residence_id) = payload["residence_id"].as_str(){
           return Ok(residence_id.to_owned());
       }
    }
    
    Err("INVALID_FORMAT")
}

pub fn deserialize_data(payload: String)-> Eval<ResidenceData>{
    if let Ok(residence_data) = serde_json::from_str::<ResidenceData>(&payload){
        return Ok(residence_data);
    }

    Err("INVALID_FORMAT")
}

pub fn update_data(id: String, residence_data: ResidenceData)-> Result<(), Box<dyn Error>>{
    register_gauge!(id.clone() + "_ds18b20_temperature_0", "help")?
        .set(residence_data.ds18b20_temperature[0]);
    register_gauge!(id.clone() + "_ds18b20_temperature_1", "help")?
        .set(residence_data.ds18b20_temperature[1]);
    register_gauge!(id.clone() + "_ds18b20_temperature_2", "help")?
        .set(residence_data.ds18b20_temperature[2]);

    register_gauge!(id.clone() + "_dht22_humidity_equipment", "help")?
        .set(residence_data.dht22_humidity[0]);
    register_gauge!(id.clone() + "_dht22_humidity_laundry", "help")?
        .set(residence_data.dht22_humidity[1]);
    register_gauge!(id.clone() + "_dht22_humidity_kitchen", "help")?
        .set(residence_data.dht22_humidity[2]);
    register_gauge!(id.clone() + "_dht22_humidity_living", "help")?
        .set(residence_data.dht22_humidity[3]);

    register_gauge!(id.clone() + "_dht22_temperature_equipment", "help")?
        .set(residence_data.dht22_temperature[0]);
    register_gauge!(id.clone() + "_dht22_temperature_laundry", "help")?
        .set(residence_data.dht22_temperature[1]);
    register_gauge!(id.clone() + "_dht22_temperature_kitchen", "help")?
        .set(residence_data.dht22_temperature[2]);
    register_gauge!(id.clone() + "_dht22_temperature_living", "help")?
        .set(residence_data.dht22_temperature[3]);

    if residence_data.motion_sensor[0]{
        register_counter!(id.clone() + "_motion_sensor_equipment", "help")?.inc();
    }
    if residence_data.motion_sensor[1]{
        register_counter!(id.clone() + "_motion_sensor_laundry", "help")?.inc();
    }
    if residence_data.motion_sensor[2]{
        register_counter!(id.clone() + "_motion_sensor_kitchen", "help")?.inc();
    }
    if residence_data.motion_sensor[3]{
        register_counter!(id.clone() + "_motion_sensor_living", "help")?.inc();
    }

    if residence_data.sound_sensor[0]{
        register_counter!("sound_sensor_equipment", "help")?.inc();
    }
    if residence_data.sound_sensor[1]{
        register_counter!(id.clone() + "_sound_sensor_laundry", "help")?.inc();
    }
    if residence_data.sound_sensor[2]{
        register_counter!(id.clone() + "_sound_sensor_kitchen", "help")?.inc();
    }
    if residence_data.sound_sensor[3]{
        register_counter!(id.clone() + "_sound_sensor_living", "help")?.inc();
    }

    register_int_gauge!(id.clone() + "_soil_moisture", "help")?
        .set(residence_data.soil_moisture);

    Ok(())
}
