use rocket_contrib::json::Json;

use super::super::databases::sensors::SensorsDB;
use super::super::models::sensor::Sensor;

#[get("/data")]
pub fn get_data() -> Json<Vec<Sensor>> {
    let db_connection = SensorsDB::new().expect("Failed to connect to the database!");
    let sensors = db_connection
        .find()
        .expect("Failed to find all the documents!");

    Json(sensors)
}
