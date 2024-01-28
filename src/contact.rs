use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
}
