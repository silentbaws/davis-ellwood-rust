use serde::{Serialize, Deserialize};

use crate::work::FeatureItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkItem {
    pub image_url: String,
    pub img_desc: String,
    pub title: String,
    pub description: String,
    pub id: String,
    pub technologies: Vec<String>,
    pub features: Vec<FeatureItem>,
}
