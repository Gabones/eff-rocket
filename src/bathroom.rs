use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Bathroom {
    pub bathroom: String,
    pub occupied: Bool,
    pub time: Date
}