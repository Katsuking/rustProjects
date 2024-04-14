//
// sushi.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct AddSushiRequest {
    #[validate(length(min = 1, message = "sushi name required"))]
    pub sushi_name: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateSushiURL {
    // #[validate(length(min = 1, message = "uuid required"))]
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Sushi {
    pub uuid: String,
    pub sushi_name: String,
}

impl Sushi {
    pub fn new(uuid: String, sushi_name: String) -> Sushi {
        Sushi { uuid, sushi_name }
    }
}
