pub mod core;
mod engine;
mod utils;

#[cfg(test)]
mod tests {

    use crate::{
        core::similarity::SimilarityEngine,
        engine::cosine_similarity_engine::CosineSimilarityEngineInMemory,
    };

    #[test]
    fn test_on_test_data() {
        // create engine
        let mut engine = CosineSimilarityEngineInMemory::new();

        engine.train();
    }
}
