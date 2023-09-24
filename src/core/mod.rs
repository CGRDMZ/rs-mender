use std::collections::HashMap;

pub mod dataset;
pub mod item_index;
pub mod model;
pub mod similarity;

pub type DetailedRecommendations = HashMap<String, Vec<(String, f64)>>;
