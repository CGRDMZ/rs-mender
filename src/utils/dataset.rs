
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use serde::Deserialize;

use crate::core::model::Event;

#[derive(Debug, Deserialize)]
pub struct TestEvent {
    user_id: String,
    deliveries: Vec<String>,
}

impl TestEvent {
    pub fn to_events(&self) -> Vec<Event> {
        self.deliveries
            .iter()
            .map(|d| Event::new(self.user_id.clone(), d.clone()))
            .collect()
    }
}

pub fn read_test_data<F: FnMut(TestEvent)>(path: &Path, filter_lt: usize, mut on_event: F) {
    let mut reader = BufReader::new(File::open(path).unwrap());

    let mut buffer = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let s: TestEvent = serde_json::from_str(&buffer).unwrap();

        if s.deliveries.len() >= filter_lt {
            on_event(s);
        }
        buffer.clear();
    }
}

#[derive(Debug, Deserialize)]
pub struct ECommerceRecord {
    #[serde(alias = "CustomerID", deserialize_with = "csv::invalid_option")]
    user_id: Option<String>,
    #[serde(alias = "Description", deserialize_with = "csv::invalid_option")]
    product_id: Option<String>,
}

pub fn read_ecommerce_data<F: FnMut(Event)>(path: &Path, mut on_event: F) {
    let mut reader = csv::Reader::from_path(path).unwrap();

    for record in reader.deserialize() {
        if record.is_err() {
            continue;
        }
        let e: ECommerceRecord = record.unwrap();

        if e.user_id.is_none() || e.product_id.is_none() {
            continue;
        }
        on_event(Event::new(e.user_id.unwrap(), e.product_id.unwrap()));
    }
}