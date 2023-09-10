use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use sprs::{CsMat, CsMatBase, TriMat};

use crate::utils::dataset;

use super::item_index::ItemIndex;

pub struct Dataset {
    pub cui: CsMat<u32>,
    ciu: CsMat<u32>,
    pub user_idx: ItemIndex,
    pub item_idx: ItemIndex,
}

impl Dataset {
    pub fn new(cui: CsMat<u32>, ciu: CsMat<u32>, user_idx: ItemIndex, item_idx: ItemIndex) -> Self {
        Self {
            cui,
            ciu,
            user_idx,
            item_idx,
        }
    }

    pub fn from_jsonl(path: String) -> Self {
        let mut user_idx = ItemIndex::new();
        let mut item_idx = ItemIndex::new();

        // learn the indexes in the data. Also gives us the information about how many users and items in it.
        // might need to find a better solution in the future because we need to go through the dataset twice in this case.
        dataset::read_test_data(Path::new(&path), 2, |e| {
            let events = e.to_events();

            events.iter().for_each(|e| {
                let _user_idx = user_idx.get_idx(e.user_id().to_string());
                let _item_idx = item_idx.get_idx(e.target_id().to_string());
            });
        });

        let user_size = user_idx.size();
        let item_size = item_idx.size();

        // construct csr matrix
        let mut cui_trimat: TriMat<u32> = TriMat::new((user_size, item_size));

        dataset::read_test_data(Path::new(&path), 2, |e| {
            let events = e.to_events();

            events.iter().for_each(|e| {
                let user_idx = user_idx.get_idx(e.user_id().to_string());
                let item_idx = item_idx.get_idx(e.target_id().to_string());

                cui_trimat.add_triplet(user_idx, item_idx, 1);
            });
        });

        let cui: CsMat<u32> = cui_trimat.to_csr();
        let ciu: CsMat<u32> = cui.clone();

        println!("shape of the user-item matrix: {:?}", cui.shape());

        Dataset {
            cui: cui,
            ciu: ciu, // is this field necessary? If so do not forget to provide the correct value!
            user_idx,
            item_idx,
        }
    }

    pub fn from_csv_example() -> Self {
        let mut user_idx = ItemIndex::new();
        let mut item_idx = ItemIndex::new();

        // learn the indexes in the data. Also gives us the information about how many users and items in it.
        // might need to find a better solution in the future because we need to go through the dataset twice in this case.
        dataset::read_ecommerce_data(Path::new("./data/data.csv"), |e| {
            let _user_idx = user_idx.get_idx(e.user_id().to_string());
            let _item_idx = item_idx.get_idx(e.target_id().to_string());
        });

        let user_size = user_idx.size();
        let item_size = item_idx.size();

        // construct csr matrix
        let mut cui_trimat: TriMat<u32> = TriMat::new((user_size, item_size));

        dataset::read_ecommerce_data(Path::new("./data/data.csv"), |e| {
            let user_idx = user_idx.get_idx(e.user_id().to_string());
            let item_idx = item_idx.get_idx(e.target_id().to_string());

            cui_trimat.add_triplet(user_idx, item_idx, 1);
        });

        let cui: CsMat<u32> = cui_trimat.to_csr();
        let ciu: CsMat<u32> = cui.clone();

        println!("shape of the user-item matrix: {:?}", cui.shape());

        Dataset {
            cui: cui,
            ciu: ciu, // is this field necessary? If so do not forget to provide the correct value!
            user_idx,
            item_idx,
        }
    }

    pub fn user_idx(&self) -> &ItemIndex {
        &self.user_idx
    }
}

#[cfg(test)]
mod dataset_test {
    use super::Dataset;

    #[test]
    fn test_loading_jsonl() {
        let dataset = Dataset::from_jsonl("./data/test_data.jsonl".to_string());
    }
}
