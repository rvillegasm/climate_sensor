use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Sensor {
    temperature: f32,
    humidity: f32,
}