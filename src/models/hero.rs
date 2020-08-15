use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Hero {
    pub id: Option<u32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: u32
}
