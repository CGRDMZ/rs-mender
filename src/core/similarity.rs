use super::model::{Event, RecommendationResponse};

pub trait SimilarityEngine {
    fn add_event(&mut self, event: Event);

    fn train(&mut self);

    fn find_similar_by_user_id(
        user_id: String,
        n_items: usize,
    ) -> Result<RecommendationResponse, ()>;

    fn find_similar_by_target_id(
        target_id: String,
        n_items: usize,
    ) -> Result<RecommendationResponse, ()>;
}
