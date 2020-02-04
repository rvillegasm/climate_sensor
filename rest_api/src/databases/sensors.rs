use bson;
use mongodb::{options::ClientOptions, Client};

use super::super::models::sensor::Sensor;

pub struct SensorsDB {
    _db: mongodb::Database,
    collection: mongodb::Collection,
}

impl SensorsDB {
    /// Create a new connection to the sensor_data collection
    /// in the climate_sensor database.
    pub fn new() -> Result<SensorsDB, Box<dyn std::error::Error>> {
        // parse the connection string
        let client_options = ClientOptions::parse("mongodb+srv://admin:kcR8BUQVQqLmeTd@tt1-dkho2.mongodb.net/test?retryWrites=true&w=majority")?;
        // create a client
        let client = Client::with_options(client_options)?;
        // get the db and the collection
        let db = client.database("climate_sensor");
        let collection = db.collection("sensor_data");

        Ok(SensorsDB {
            _db: db,
            collection,
        })
    }

    /// Insert data into the collection.
    pub fn insert(&self, sensor: Sensor) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_sensor = bson::to_bson(&sensor)?;

        if let bson::Bson::Document(document) = serialized_sensor {
            self.collection.insert_one(document, None)?;
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }

        Ok(())
    }

    /// Insert data into the collection.
    pub fn find(&self) -> Result<Vec<Sensor>, Box<dyn std::error::Error>> {
        let cursor = self.collection.find(None, None)?;

        let mut sensors: Vec<Sensor> = Vec::new();

        for result in cursor {
            match result {
                Ok(document) => {
                    let sensor = Sensor::new(
                        document
                            .get("temperature")
                            .and_then(bson::Bson::as_f64)
                            .unwrap(),
                        document
                            .get("humidity")
                            .and_then(bson::Bson::as_f64)
                            .unwrap(),
                    );
                    sensors.push(sensor);
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(sensors)
    }
}
