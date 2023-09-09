use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FeatureItem {
    pub title: String,
    pub description: String,
}
