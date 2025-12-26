use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnifiedWindow {
    pub id: String,
    pub name: String,
    pub app_name: String,
    pub app_id: String,
    pub is_visible: bool,

}

impl UnifiedWindow {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self { title, width, height }
    }
}

