use prometheus::{core::{GenericGauge, AtomicF64, GenericCounter}, IntGauge};
use prometheus_exporter::prometheus::{register_counter, register_gauge, register_int_gauge};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{env, error::Error, boxed::Box, collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use reqwest::{Client, StatusCode};

pub type Eval<T> = Result<T, &'static str>;
pub type MetricsHandle = Arc<Mutex<HashMap<String, PrometheusMetrics>>>;

#[derive(Serialize, Deserialize)]
pub struct ResidenceData{
    ds18b20_temperature: [f64; 3],
    dht22_humidity: [f64; 4],
    dht22_temperature: [f64; 4],
    motion_sensor: [bool; 4],
    sound_sensor: [bool; 4],
    soil_moisture: i64
}

pub enum PrometheusMetrics{
    Counter(GenericCounter<AtomicF64>),
    Gauge(GenericGauge<AtomicF64>),
    IntGauge(IntGauge)
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

async fn grafana_create(id: &str)-> Result<(), Box<dyn Error>>{
    let bearer_token = env::var("API_TOKEN")?;
    let metrics = vec![
        vec!["ds18b20_temperature_0", "ds18b20_temperature_1", "ds18b20_temperature_2"],
        vec!["dht22_humidity_equipment", "dht22_humidity_laundry", "dht22_humidity_kitchen", "dht22_humidity_living"],
        vec!["dht22_temperature_equipment", "dht22_temperature_laundry", "dht22_temperature_kitchen", "dht22_temperature_living"],
        vec!["motion_sensor_equipment", "motion_sensor_laundry", "motion_sensor_kitchen", "motion_sensor_living"],
        vec!["sound_sensor_equipment", "sound_sensor_laundry", "sound_sensor_kitchen", "sound_sensor_living"],
        vec!["soil_moisture_0"]
    ];

    let client = Client::new();

    let response = client
        .post("http://bbp-grafana:3000/api/folders")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", bearer_token))
        .json(&json!({
            "title": id
        }))
        .send().await?;

    if response.status() == StatusCode::from_u16(200).unwrap(){
        let folder_id = &response.json::<Value>().await.unwrap()["id"].as_i64().unwrap();

        for sensors in metrics{
            let title = format!("{id}_{}", sensors[0].rsplit_once("_").unwrap().0);

            let mut json = json!({
                "dashboard": {
                    "id": null,
                    "uid": null,
                    "title": title,
                    "timezone": "browser",
                    "schemaVersion": 16,
                    "version": 0,
                    "refresh": "25s",
                    "panels": [
                        {
                            "id": 1,
                            "visualization": "time series",
                            "type": "graph",
                            "title": title,
                            "targets": []
                        },
                    ]
                },
                "folderId": folder_id,
                "message": "Created dashboard",
                "overwrite": false
            });

            let targets = json["dashboard"]["panels"][0]["targets"].as_array_mut().unwrap();
            for sensor in sensors{
                targets.push(json!({
                    "refId": sensor,
                    "expr": format!("{id}_{sensor}") 
                }));
            }

            let response = client
                .post("http://bbp-grafana:3000/api/dashboards/db")
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", bearer_token))
                .json(&json)
                .send().await?;

            println!("Response status: {}", response.status());
            println!("Response payload:\n{}", response.text().await.unwrap());
        }
    }

    Ok(())
}

pub async fn update_data(id: String, residence_data: ResidenceData, metrics: &MetricsHandle)-> Result<(), Box<dyn Error>>{
    let mut metrics = metrics.lock().await;

    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_ds18b20_temperature_0"))
        { register.set(residence_data.ds18b20_temperature[0]); }
    else{ 
        metrics.insert(id.clone() + "_ds18b20_temperature_0", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_ds18b20_temperature_0", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_ds18b20_temperature_1"))
        { register.set(residence_data.ds18b20_temperature[1]); }
    else{ 
        metrics.insert(id.clone() + "_ds18b20_temperature_1", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_ds18b20_temperature_1", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_ds18b20_temperature_2"))
        { register.set(residence_data.ds18b20_temperature[2]); }
    else{ 
        metrics.insert(id.clone() + "_ds18b20_temperature_2", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_ds18b20_temperature_2", "help")?)); 
    }

    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_humidity_equipment"))
        { register.set(residence_data.dht22_humidity[0]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_humidity_equipment", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_humidity_equipment", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_humidity_laundry"))
        { register.set(residence_data.dht22_humidity[1]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_humidity_laundry", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_humidity_laundry", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_humidity_kitchen"))
        { register.set(residence_data.dht22_humidity[2]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_humidity_kitchen", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_humidity_kitchen", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_humidity_living"))
        { register.set(residence_data.dht22_humidity[3]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_humidity_living", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_humidity_living", "help")?)); 
    }

    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_temperature_equipment"))
        { register.set(residence_data.dht22_temperature[0]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_temperature_equipment", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_temperature_equipment", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_temperature_laundry"))
        { register.set(residence_data.dht22_temperature[1]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_temperature_laundry", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_temperature_laundry", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_temperature_kitchen"))
        { register.set(residence_data.dht22_temperature[2]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_temperature_kitchen", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_temperature_kitchen", "help")?)); 
    }
    if let Some(PrometheusMetrics::Gauge(register)) = metrics.get(&(id.clone() + "_dht22_temperature_living"))
        { register.set(residence_data.dht22_temperature[3]); }
    else{ 
        metrics.insert(id.clone() + "_dht22_temperature_living", 
            PrometheusMetrics::Gauge(register_gauge!(id.clone() + "_dht22_temperature_living", "help")?)); 
    }

    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_motion_sensor_equipment"))
        { if residence_data.motion_sensor[0] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_motion_sensor_equipment", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_motion_sensor_equipment", "help")?)); 
    }
    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_motion_sensor_laundry"))
        { if residence_data.motion_sensor[1] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_motion_sensor_laundry", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_motion_sensor_laundry", "help")?)); 
    }
    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_motion_sensor_kitchen"))
        { if residence_data.motion_sensor[2] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_motion_sensor_kitchen", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_motion_sensor_kitchen", "help")?)); 
    }
    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_motion_sensor_living"))
        { if residence_data.motion_sensor[3] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_motion_sensor_living", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_motion_sensor_living", "help")?)); 
    }

    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_sound_sensor_equipment"))
        { if residence_data.sound_sensor[0] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_sound_sensor_equipment", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_sound_sensor_equipment", "help")?)); 
    }
    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_sound_sensor_laundry"))
        { if residence_data.sound_sensor[1] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_sound_sensor_laundry", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_sound_sensor_laundry", "help")?)); 
    }
    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_sound_sensor_kitchen"))
        { if residence_data.sound_sensor[2] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_sound_sensor_kitchen", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_sound_sensor_kitchen", "help")?)); 
    }
    if let Some(PrometheusMetrics::Counter(register)) = metrics.get(&(id.clone() + "_sound_sensor_living"))
        { if residence_data.sound_sensor[3] { register.inc(); }}
    else{ 
        metrics.insert(id.clone() + "_sound_sensor_living", 
            PrometheusMetrics::Counter(register_counter!(id.clone() + "_sound_sensor_living", "help")?)); 
    }

    if let Some(PrometheusMetrics::IntGauge(register)) = metrics.get(&(id.clone() + "_soil_moisture_0"))
        { register.set(residence_data.soil_moisture); }
    else{ 
        metrics.insert(id.clone() + "_soil_moisture_0", 
            PrometheusMetrics::IntGauge(register_int_gauge!(id.clone() + "_soil_moisture_0", "help")?)); 
    }

    grafana_create(&id).await?;

    Ok(())
}
