use rocket_contrib::json::Json;

mod model;
mod database;

use model::Sensor;

#[post("/sensor", data = "<sensor>")]
pub fn create_entry(sensor: Json<Sensor>) -> &'static str {
    let sensor = sensor.into_inner();
    // create the database
    let db_connection = database::SensorsDB::new().expect("Failed to connect to the database!");
    // insert the sensor into it
    db_connection.insert(sensor).expect("Failed to insert a document into the database!");
    // Create a response
    "Insertion successful"
}