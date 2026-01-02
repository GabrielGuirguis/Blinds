use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnifiedWindow {
    pub id: String,
    pub name: String,
    pub app_id: String,
    pub is_hidden: bool
}

impl UnifiedWindow {
    pub fn new(id: String, name: String, app_id: String) -> Self {
        Self { id, name, app_id, is_hidden: false }
    }
}
