use std::collections::{
    hash_map::{Entry, OccupiedEntry},
    HashMap,
};

#[derive(Clone, Debug)]
pub struct ItemIndex {
    item_to_index: HashMap<String, usize>,
    index_to_item: Vec<String>,
}

impl ItemIndex {
    pub fn new() -> Self {
        Self {
            item_to_index: HashMap::new(),
            index_to_item: Vec::new(),
        }
    }

    pub fn get_idx(&mut self, item: String) -> usize {
        match self.item_to_index.entry(item) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(v) => {
                self.index_to_item.push(v.key().clone());

                let inserted_idx = v.insert(self.index_to_item.len() - 1);
                *inserted_idx
            }
        }
    }

    pub fn get_item(&self, idx: usize) -> String {
        self.index_to_item.get(idx).unwrap().clone()
    }

    pub fn size(&self) -> usize {
        self.index_to_item.len()
    }
}

#[cfg(test)]
mod test {

    use itertools::Itertools;

    use super::ItemIndex;

    #[test]
    pub fn should_give_indexes() {
        let v = vec!["a", "b", "c", "d", "a", "b", "a", "d"];

        let mut item_idx = ItemIndex::new();

        let ids = v.iter().map(|&i| {item_idx.get_idx(i.to_string())}).collect::<Vec<_>>();
    

        assert_eq!((0..4).collect::<Vec<_>>(), ids.clone().into_iter().unique().collect_vec());
        println!("{:?}", ids);




        let idx = item_idx.get_idx("c".to_string());
        assert_eq!(2, idx);

        let idx = item_idx.get_idx("a".to_string());
        assert_eq!(0, idx);

        let idx = item_idx.get_idx("d".to_string());
        assert_eq!(3, idx);
    }

}
