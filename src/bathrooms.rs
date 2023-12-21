use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Bathrooms {
    pub id: i64,
    pub bathroom: String,
    pub occupied: bool,
    pub time: String,
}

impl Bathrooms {
    pub fn to_string(&self) -> String {
        format!(
            "{{ \"id\": {}, \"bathroom\": \"{}\", \"occupied\": {}, \"time\": \"{}\" }},",
            &self.id, &self.bathroom, &self.occupied, &self.time
        )
    }
}

pub static QUERY_SELECT: &'static str = "SELECT * FROM bathrooms";
