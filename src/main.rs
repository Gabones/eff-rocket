use rocket::serde::json::{serde_json, Json, Value};
use sqlite::State;
use std::vec;
use rocket::http::Status;

mod bathrooms;
use bathrooms::Bathrooms;

#[macro_use]
extern crate rocket;

fn read_json() -> Vec<Bathrooms> {
    let connection = sqlite::open("database.db").unwrap();

    let mut statement = connection.prepare(bathrooms::QUERY_SELECT).unwrap();

    let mut result: Vec<Bathrooms> = vec![];
    while let Ok(State::Row) = statement.next() {
        result.push(Bathrooms {
            id: statement.read::<i64, _>("id").unwrap(),
            bathroom: statement.read::<String, _>("bathroom").unwrap(),
            occupied: if statement.read::<i64, _>("occupied").unwrap() != 0 {
                true
            } else {
                false
            },
            time: statement.read::<String, _>("time").unwrap(),
        });
    }

    return result;
}

#[post("/", data = "<bathroom>")]
fn insert(bathroom: Json<Bathrooms>) -> Status {

    let sensor_data = bathroom.0;

    let connection = sqlite::open("database.db").unwrap();

    connection.execute(format!(
        "INSERT INTO bathrooms (bathroom, occupied, time) VALUES(\"{}\", {}, \"{}\")",
        sensor_data.bathroom,
        if sensor_data.occupied { 1 } else { 0 },
        sensor_data.time
    )).unwrap();

    Status::Ok
}

#[get("/")]
fn index() -> Value {
    let vec_bath = read_json();
    let json = serde_json::to_string(&vec_bath).unwrap();
    let v: Value = serde_json::from_str(&json).unwrap();
    return v;
}

#[delete("/<id>")]
fn delete(id: i64) -> Status {
	let connection = sqlite::open("database.db").unwrap();
	connection.execute(format!(
		"DELETE FROM bathrooms WHERE id = {}",
		id
	)).unwrap();

	Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, insert, delete])
}
