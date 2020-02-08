use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    temperature: f64,
    humidity: f64,
    latitude: f64,
    longitude: f64,
    mac_addr: String,
}

impl Sensor {
    pub fn new(id: Option<ObjectId>, temperature: f64, humidity: f64, latitude: f64, longitude: f64, mac_addr: String) -> Sensor {
        Sensor {
            id,
            temperature,
            humidity,
            latitude, 
            longitude, 
            mac_addr,
        }
    }
}
