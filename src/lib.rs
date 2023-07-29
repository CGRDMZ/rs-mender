mod core;
mod engine;
mod utils;

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, BufRead},
        path::Path,
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH, Instant},
    };

    use serde_json::Value;

    use crate::{
        core::{model::Event, similarity::SimilarityEngine},
        engine::cosine_similarity_engine::CosineSimilarityEngineInMemory,
    };

    #[test]
    fn test_on_test_data() {
        // create engine
        let mut engine = CosineSimilarityEngineInMemory::new();

        engine.train();

    }
}
