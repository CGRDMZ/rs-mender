use std::collections::HashMap;

pub mod model;
pub mod similarity;
pub mod item_index;
pub mod dataset;

pub type DetailedRecommendations = HashMap<String, Vec<(String, f64)>>;
