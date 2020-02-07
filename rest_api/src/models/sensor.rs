use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    temperature: f64,
    humidity: f64,
}

impl Sensor {
    pub fn new(id: Option<ObjectId>, temperature: f64, humidity: f64) -> Sensor {
        Sensor {
            id,
            temperature,
            humidity,
        }
    }
}
