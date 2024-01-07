use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct NavigationItem {
    pub id: i32,
    pub note: String,
}
