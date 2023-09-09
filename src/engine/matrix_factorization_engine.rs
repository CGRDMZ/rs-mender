use std::fs::File;

use itertools::Itertools;
use ndarray::{Array, Array2};
use ndarray_rand::{
    rand,
    rand_distr::{Distribution, Standard, Uniform},
    RandomExt,
};
use sprs::TriMat;

use crate::core::{
    dataset::Dataset,
    item_index::ItemIndex,
    model::{Event, RecommendationResponse},
    similarity::SimilarityEngine,
};

pub struct MatrixFactorizationEngine {
    dataset: Dataset,
}

impl MatrixFactorizationEngine {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl SimilarityEngine for MatrixFactorizationEngine {
    fn train(&mut self) {
        let latent_factors = 10;

        let learning_rate = 0.01;
        let lambda = 0.1;

        let n_iter = 1000;

        let user_size = self.dataset.user_idx.size();
        let item_size = self.dataset.item_idx.size();

        let mut random = rand::thread_rng();

        let mut u_matrix = Array::random((user_size, latent_factors), Uniform::new(-1f64, 1f64));
        let mut v_matrix = Array::random((item_size, latent_factors), Uniform::new(-1f64, 1f64));

        for _ in 0..n_iter {
            for (v, (i, j)) in self.dataset.cui.iter() {
                let pred = u_matrix.row(i).dot(&v_matrix.row(j).t());
                let error = f64::from(*v) - pred;

                // Update U and V using SGD
                let delta_u = -2.0 * error * &v_matrix.row(j) + 2.0 * lambda * &u_matrix.row(i);
                let delta_v = -2.0 * error * &u_matrix.row(i) + 2.0 * lambda * &v_matrix.row(j);

                let updated_row_u = &(&u_matrix.row(i) - learning_rate * delta_u);
                let updated_row_v = &(&v_matrix.row(j) - learning_rate * delta_v);

                u_matrix.row_mut(i).assign(updated_row_u);
                v_matrix.row_mut(j).assign(updated_row_v);
            }
        }

        // check mpr on train data, should use a seperate data later!
        // Use the factorized matrices for predictions
        let predicted_matrix = u_matrix.dot(&v_matrix.t());

        let mut total_mpr = 0f64;
        for ui in 0..self.dataset.user_idx.size() {
            let actual = self.dataset.cui.outer_view(ui).unwrap();
            let recommendations = predicted_matrix
                .row(ui)
                .iter()
                .enumerate()
                .sorted_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap())
                .map(|(item_idx, _)| item_idx)
                .collect::<Vec<_>>();

            let mut percentile_rank_summation = 0f64;
            for (actual_item_idx, _) in actual.iter() {
                let rank = recommendations
                    .iter()
                    .position(|&rec_item| actual_item_idx == rec_item)
                    .unwrap();

                let percentile_rank = (rank + 1) as f64 / recommendations.len() as f64;
                percentile_rank_summation += percentile_rank;
            }
            let user_mpr = percentile_rank_summation / actual.iter().count() as f64;
            total_mpr += user_mpr;
        }

        let mpr = total_mpr / self.dataset.user_idx.size() as f64;

        println!("Mean Percentile Rank (MPR): {:.4}", mpr);

        // serde_json::to_writer_pretty(File::create("./data/predictions.json").unwrap(), &predicted_matrix).unwrap();
    }

    fn find_similar_by_user_id(
        &self,
        user_id: String,
        n_items: usize,
    ) -> Result<RecommendationResponse, ()> {
        todo!()
    }

    fn find_similar_by_target_id(
        &self,
        target_id: String,
        n_items: usize,
    ) -> Result<RecommendationResponse, ()> {
        todo!()
    }

    fn save(self) {
        todo!()
    }
}
