use super::model::{Event, RecommendationResponse};

pub trait SimilarityEngine {
    fn train(&mut self);

    fn find_similar_by_user_id(
        &self,
        user_id: String,
        n_items: usize,
    ) -> Result<RecommendationResponse, ()>;

    fn find_similar_by_target_id(
        &self,
        target_id: String,
        n_items: usize,
    ) -> Result<RecommendationResponse, ()>;

    fn save(self);
}
