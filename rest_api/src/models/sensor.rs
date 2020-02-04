use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sensor {
    temperature: f64,
    humidity: f64,
}

impl Sensor {
    pub fn new(temperature: f64, humidity: f64) -> Sensor {
        Sensor {
            temperature,
            humidity,
        }
    }
}
