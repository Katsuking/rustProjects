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
