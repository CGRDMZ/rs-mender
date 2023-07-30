use std::{collections::HashMap, fs::File, io::BufWriter, path::Path, time::Instant};

use crate::{
    core::{
        self,
        model::{Event, RecommendationResponse},
        similarity, DetailedRecommendations,
    },
    utils::{
        construct_item_user_matrix, construct_user_item_matrix, math::cosine_similarity,
        ItemUserMatrix, UserItemMatrix,
    },
};

pub struct CosineSimilarityEngineInMemory {
    item_user_matrix: Option<ItemUserMatrix>,
    events: Vec<Event>,
}

impl CosineSimilarityEngineInMemory {
    pub fn new() -> Self {
        Self {
            events: vec![],
            item_user_matrix: None, // lazy initialized
        }
    }

    fn item_user_matrix(&mut self) -> &ItemUserMatrix {
        self.item_user_matrix
            .get_or_insert(construct_item_user_matrix(&self.events))
    }
}

impl similarity::SimilarityEngine for CosineSimilarityEngineInMemory {
    fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    fn train(&mut self) {
        // create item-user matrix from the events, for now we do not have different event types but we can have them in the future!
        let item_user_matrix = self.item_user_matrix();

        let start = Instant::now();
        // find similarities pair-wise
        let mut similarities: DetailedRecommendations = HashMap::new();
        for (item_id, users) in item_user_matrix {
            let a = similarities.entry(item_id.clone()).or_default();
            for (item_id_2, users_2) in item_user_matrix {
                let c_sim = cosine_similarity(users, users_2);
                if c_sim == 0f64 || item_id == item_id_2 {
                    continue;
                }
                a.push((item_id_2.clone(), cosine_similarity(users, users_2)));
            }
            a.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
        }
        println!("took {:?} to train", start.elapsed());

        // println!("{:?}", similarities)// [&"Lyft".to_string()][&"Uber".to_string()]);
        let path = Path::new("./data/output.json");
        let file = File::create(path).unwrap();
        let writer_buf = BufWriter::new(file);
        let filter_no_recs= similarities
            .iter()
            .filter(|i| i.1.len() != 0)
            .map(|(product_id, recs)| (product_id.clone(), recs.iter().take(10).collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>();

        println!(
            "{:?}",
            serde_json::to_writer_pretty(writer_buf, &filter_no_recs)
        );
    }

    fn find_similar_by_user_id(
        _user_id: String,
        _n_items: usize,
    ) -> Result<core::model::RecommendationResponse, ()> {
        todo!()
    }

    fn find_similar_by_target_id(
        _target_id: String,
        _n_items: usize,
    ) -> Result<core::model::RecommendationResponse, ()> {
        todo!()
    }
}

#[cfg(test)]
mod cosine_sim_tests {
    use crate::core::{model::Event, similarity::SimilarityEngine};

    use super::CosineSimilarityEngineInMemory;

    #[test]
    fn should_be_correctly_constructing_user_item_matrix() {
        let events = [
            Event::new("user-1".to_string(), "product-1".to_string()),
            Event::new("user-1".to_string(), "product-2".to_string()),
            Event::new("user-1".to_string(), "product-3".to_string()),
            Event::new("user-2".to_string(), "product-3".to_string()),
        ];

        let mut engine = CosineSimilarityEngineInMemory::new();

        events.iter().for_each(|e| engine.add_event(e.clone()));

        engine.train();
    }
}
