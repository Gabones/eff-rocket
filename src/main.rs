use rocket::serde::json::{json, serde_json, Json, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::vec;

mod bathroom;
use bathroom::Bathroom;

#[macro_use]
extern crate rocket;

fn read_json() -> Vec<Bathroom> {
    let file = OpenOptions::new().read(true).open("bathrooms.json");

    if file.is_ok() {
        let serialized: Result<Vec<Bathroom>, serde_json::Error> =
            serde_json::from_reader(file.unwrap());
        if serialized.is_ok() {
            return serialized.unwrap();
        }
    }

    let result: Vec<Bathroom> = vec![];
    return result;
}

#[post("/", data = "<bathroom>")]
fn insert(bathroom: Json<Bathroom>) -> Value {
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
