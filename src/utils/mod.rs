use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::core::model::Event;

pub type UserItemMatrix = HashMap<String, HashMap<String, u32>>;
pub type ItemUserMatrix = HashMap<String, HashMap<String, u32>>;

pub fn print_hashmap<T: Display + Debug>(hm: HashMap<String, HashMap<String, T>>) {
    println!("{:?}", hm)
}

pub fn construct_user_item_matrix(events: &Vec<Event>) -> UserItemMatrix {
    let mut user_item_matrix = UserItemMatrix::new();

    for event in events {
        let items_of_user = user_item_matrix
            .entry(event.user_id().to_string())
            .or_default();

        // increase the number for the interaction or if it is the first insert 1
        *items_of_user
            .entry(event.target_id().to_string())
            .or_insert(0) += 1;
    }
    user_item_matrix
}

pub fn construct_item_user_matrix(events: &Vec<Event>) -> ItemUserMatrix {
    let mut item_user_matrix = ItemUserMatrix::new();

    for event in events {
        let users_of_item = item_user_matrix
            .entry(event.target_id().to_string())
            .or_default();

        // increase the number for the interaction or if it is the first insert 1
        *users_of_item
            .entry(event.user_id().to_string())
            .or_insert(0) += 1;
    }
    item_user_matrix
}

pub mod math;

#[cfg(test)]
mod util_tests {
    use std::vec;

    use crate::utils::math::{cosine_similarity, norm, dot};

    use super::*;
    #[test]
    fn should_be_correctly_constructing_user_item_matrix() {
        let events = vec![
            Event::new("user-1".to_string(), "product-1".to_string()),
            Event::new("user-1".to_string(), "product-2".to_string()),
            Event::new("user-2".to_string(), "product-1".to_string()),
            Event::new("user-1".to_string(), "product-1".to_string()),
        ];

        let user_item_matrix = construct_user_item_matrix(events.as_ref());

        println!("{:?}", user_item_matrix);
        assert_eq!(2, user_item_matrix["user-1"]["product-1"]);
        assert_eq!(1, user_item_matrix["user-1"]["product-2"]);

        assert_eq!(1, user_item_matrix["user-2"]["product-1"]);
    }

    #[test]
    fn cosine_similarity_should_work() {
        let a = HashMap::from([
            ("1".to_string(), 1),
            ("2".to_string(), 2),
            ("3".to_string(), 3),
            ("4".to_string(), 4),
            ("5".to_string(), 5),
        ]);

        let b = HashMap::from([
            ("1".to_string(), 5),
            ("2".to_string(), 4),
            ("3".to_string(), 3),
            ("4".to_string(), 2),
            ("5".to_string(), 1),
        ]);

        let c = cosine_similarity(&a, &b);

        assert_eq!(0.6363636363636364, c)
    }

    #[test]
    fn norm_should_work() {
        let a = HashMap::from([
            ("1".to_string(), 1),
            ("2".to_string(), 0),
            ("3".to_string(), 2),
            ("4".to_string(), 1),
            ("5".to_string(), 3),
        ]);

        let n = norm(&a);

        assert_eq!(3.872983346207417, n)
    }

    #[test]
    fn dot_product_should_work() {
        let a = HashMap::from([
            ("1".to_string(), 1),
            ("2".to_string(), 0),
            ("3".to_string(), 2),
            ("4".to_string(), 1),
            ("5".to_string(), 3),
        ]);

        let b = HashMap::from([
            ("1".to_string(), 0),
            ("2".to_string(), 0),
            ("3".to_string(), 2),
            ("4".to_string(), 1),
            ("5".to_string(), 6),
        ]);

        let d = dot(&a, &b);

        assert_eq!(23, d)
    }
}
