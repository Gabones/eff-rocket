use rocket::serde::json::{json, serde_json, Json, Value};
use sqlite::State;
use std::fs::OpenOptions;
use std::io::Write;
use std::vec;

mod bathroom;
use bathroom::Bathrooms;

#[macro_use]
extern crate rocket;

fn read_json() -> Vec<Bathrooms> {
    let connection = sqlite::open("database.db").unwrap();

    let mut statement = connection.prepare(bathroom::QUERY_SELECT).unwrap();

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
fn insert(bathroom: Json<Bathrooms>) -> Value {
    let mut vec_bath = read_json();

    vec_bath.push(bathroom.0);
    let json = serde_json::to_string(&vec_bath).unwrap();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("bathrooms.json");

    if let Err(e) = write!(file.unwrap(), "{}", json) {
        eprintln!("Couldn't write to file: {}", e);
        return json!({ "File": "Unable to write the file" });
    }

    let v: Value = serde_json::from_str(&json).unwrap();
    return v;
}

#[get("/")]
fn index() -> Value {
    let vec_bath = read_json();
    let json = serde_json::to_string(&vec_bath).unwrap();
    let v: Value = serde_json::from_str(&json).unwrap();
    return v;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, insert])
}
