#[derive(Clone, Debug)]
pub struct Event {
    user_id: String,
    target_id: String
}

impl Event {
    pub fn new(user_id: String, target_id: String) -> Self { Self { user_id, target_id } }

    pub fn user_id(&self) -> &str {
        self.user_id.as_ref()
    }

    pub fn target_id(&self) -> &str {
        self.target_id.as_ref()
    }
}

pub struct RecommendationResponse {
    recommendations: Vec<String>
}

impl RecommendationResponse {
    /// Creates a new [`RecommendationResponse`].
    pub fn new(recommendations: Vec<String>) -> Self { Self { recommendations } }
}