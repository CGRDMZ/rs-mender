use std::fs::File;

use itertools::Itertools;
use ndarray::{Array, Array1, Array2};
use ndarray_rand::{
    rand,
    rand_distr::{Distribution, Standard, Uniform},
    RandomExt,
};
use sprs::TriMat;

use crate::{
    core::{
        dataset::Dataset,
        item_index::ItemIndex,
        model::{Event, RecommendationResponse},
        similarity::SimilarityEngine,
    },
    utils::approx_equal,
};

pub struct MatrixFactorizationEngine {
    dataset: Dataset,
    u_matrix: Option<Array2<f64>>,
    v_matrix: Option<Array2<f64>>,
}

impl MatrixFactorizationEngine {
    pub fn new(dataset: Dataset) -> Self {
        Self {
            dataset,
            u_matrix: None,
            v_matrix: None,
        }
    }

    fn internal_predict(&self, user_idx: usize) -> Array1<(usize, f64)> {
        // if the model is not trained yet we panic, we should handle this gracefully in the future.
        match (&self.u_matrix, &self.v_matrix) {
            (Some(u_matrix), Some(v_matrix)) => {
                let item_recs = u_matrix
                    .row(user_idx)
                    .dot(&v_matrix.t())
                    .indexed_iter()
                    .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
                    .map(|(a, &b)| (a, b))
                    .collect::<Array1<_>>();

                item_recs
            }
            (_, _) => {
                panic!("you should train the model before making a prediction")
            }
        }
    }

    pub fn calculate_mpr(&self) -> f64 {
        // if the model is not trained yet we panic, we should handle this gracefully in the future.
        match (&self.u_matrix, &self.v_matrix) {
            (Some(_), Some(_)) => {
                let mut total_mpr = 0f64;
                for user_idx in 0..self.dataset.user_idx.size() {
                    let actual = self.dataset.cui.outer_view(user_idx).unwrap();
                    let recommendations = self.internal_predict(user_idx);

                    let mut percentile_rank_summation = 0f64;
                    for (actual_item_idx, _) in actual.iter() {
                        let rank = recommendations
                            .iter()
                            .position(|(item_idx, _)| actual_item_idx == *item_idx)
                            .unwrap();

                        let percentile_rank = (rank + 1) as f64 / recommendations.len() as f64;
                        percentile_rank_summation += percentile_rank;
                    }
                    let user_mpr = percentile_rank_summation / actual.iter().count() as f64;
                    total_mpr += user_mpr;
                }

                let mpr = total_mpr / self.dataset.user_idx.size() as f64;

                println!("Mean Percentile Rank (MPR): {:.4}", mpr);

                mpr
            }
            (_, _) => {
                panic!("you should train the model before calculating mpr")
            }
        }
    }
}

impl SimilarityEngine for MatrixFactorizationEngine {
    fn train(&mut self) {
        let latent_factors = 30;

        let learning_rate = 0.01;
        let lambda = 0.01;

        let n_iter = 100;

        let user_size = self.dataset.user_idx.size();
        let item_size = self.dataset.item_idx.size();

        let mut u_matrix = Array::random((user_size, latent_factors), Uniform::new(-0.1, 0.1));
        let mut v_matrix = Array::random((item_size, latent_factors), Uniform::new(-0.1, 0.1));
        let mut previous_validation_err = f64::MAX;

        let mut patience_count = 0;

        let non_zero_value_count = self.dataset.cui.iter().count();

        for _ in 0..n_iter {
            let mut validation_err = 0.0;
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

                validation_err += error * error;
            }
            validation_err /= non_zero_value_count as f64;
            validation_err = validation_err.sqrt();

            println!("RMSE: {}", validation_err);

            if validation_err < previous_validation_err
                && !approx_equal(validation_err, previous_validation_err, 1e-3)
            {
                previous_validation_err = validation_err;

                self.u_matrix = Some(u_matrix.clone());
                self.v_matrix = Some(v_matrix.clone());

                patience_count = 0;
            } else {
                patience_count += 1;
                if patience_count >= 5 {
                    println!("early breaking...");
                    break;
                }
            }
        }

        // should not have any NaN values
        assert_eq!(
            0,
            self.u_matrix
                .clone()
                .unwrap()
                .iter()
                .filter(|v| v.is_nan())
                .count()
        );
        assert_eq!(
            0,
            self.v_matrix
                .clone()
                .unwrap()
                .iter()
                .filter(|v| v.is_nan())
                .count()
        );

        self.calculate_mpr();

        serde_json::to_writer_pretty(
            File::create("./data/best_u_matrix.json").unwrap(),
            &u_matrix,
        )
        .unwrap();
        serde_json::to_writer_pretty(
            File::create("./data/best_v_matrix.json").unwrap(),
            &v_matrix,
        )
        .unwrap();
    }

    fn find_similar_by_user_id(
        &mut self,
        user_id: String,
        n_items: usize,
    ) -> Result<RecommendationResponse, ()> {
        let user_idx = self.dataset.user_idx.get_idx(user_id);

        let binding = self.internal_predict(user_idx);
        let predictions = binding
            .iter()
            .map(|(i, v)| {
                let item = self.dataset.item_idx.get_item(*i);
                (item, v)
            })
            .take(n_items)
            .collect_vec();
        println!("{:?}", predictions);
        Err(())
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
