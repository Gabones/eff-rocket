use rocket::http::Status;
use rocket::serde::json::{Value, Json, json};
use std::fs::OpenOptions;
use std::io::Write;

mod bathroom;
use bathroom::Bathroom;

#[macro_use]
extern crate rocket;

#[post("/post", data = "<bathroom>")]
fn insert(bathroom: Json<Bathroom>) { /* .. */ }

#[get("/?<msg>")]
fn index(msg: Option<&str>) -> Value {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("my-file.txt");
    
    let result = match file {
        Ok(mut file) => {
            if let Some(msg) = msg {
                if let Err(e) = writeln!(file, "{msg}") {
                    eprintln!("Couldn't write to file: {}", e);
                }
                return json!({
                    "File": "Writen!"
                });
                
            }
            return json!({
                "File": "No message to write!"
            });

        },
        _ => json!({ "File": "Unable to write" })
    };

    return result;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}