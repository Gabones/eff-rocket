use rocket::serde::json::{Value, Json, json, serde_json, serde_json::Error};
use std::fs::OpenOptions;
use std::io::Write;

mod bathroom;
use bathroom::Bathroom;

#[macro_use]
extern crate rocket;

#[post("/", data = "<bathroom>")]
fn insert(bathroom: Json<Bathroom>) -> Value { 
	println!("{:?}", bathroom);
	let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("bathrooms.json");
    
    match file {
        Ok(mut file) => {
			let serialized:Result<Vec<Bathroom>, serde_json::Error> = serde_json::from_reader(file.by_ref());
			println!("{:?}", serialized);
			match serialized {
				Ok(mut s) => {
					s.push(bathroom.0);
					let json = serde_json::to_string(&s).unwrap();


					// if let Err(e) = writeln!(file, "{}", json) {
					// 	eprintln!("Couldn't write to file: {}", e);
					// }
					let v: Value = serde_json::from_str(&json).unwrap();
					return v;
				},
				_ => json!({ "File": "Unable to find the file" })
			}

			
			// if let Err(e) = writeln!(file, "{}", bathroom.to_string()) {
			// 	eprintln!("Couldn't write to file: {}", e);
			// 	return json!({ "File": "Unable to write" });
			// }
			// return json!({ "File": "Writen with success!" });
        },
        _ => json!({ "File": "Não achou o arquivo" })
    }
}

#[get("/")]
fn index() -> Value {
    let file = OpenOptions::new()
        .read(true)
        .open("bathrooms.json");

    match file {
		Ok(mut file) => {
			let serialized:Result<Vec<Bathroom>, serde_json::Error> = serde_json::from_reader(file.by_ref());
			if let Err(e) = serialized {
				eprintln!("Couldn't write to file: {}", e);
				return json!({ "File": "Unable to write" });
			}
			
			if let Ok(e) = serialized {
				let json = serde_json::to_string(&e).unwrap();
				let v: Value = serde_json::from_str(&json).unwrap();
				return v;
			}
			return json!({ "File": "File printed" });
		},
		_ => json!({ "File": "Unable to read the file" })
	}
}

#[launch]
fn rocket() -> _ {
    rocket::build()
		.mount("/", routes![index, insert])
}