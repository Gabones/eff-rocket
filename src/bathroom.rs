use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Bathroom {
    pub bathroom: String,
    pub occupied: bool,
    pub time: String,
}

impl Bathroom {
    pub fn to_string(&self) -> String {
        format!(
            "{{ \"bathroom\": \"{}\", \"occupied\": {}, \"time\": \"{}\" }},",
            &self.bathroom, &self.occupied, &self.time
        )
    }
}
