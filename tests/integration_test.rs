mod test_utils;

use std::{path::Path, sync::{Arc, Mutex}};

use csv::Error;
use rs_mender::{engine::cosine_similarity_engine::CosineSimilarityEngineInMemory, core::{similarity::SimilarityEngine, model::Event}};
use test_utils::{read_test_data, read_ecommerce_data};

use crate::test_utils::TestEvent;

#[test]
fn foo() -> Result<(), Error> {
    let engine = Arc::new(Mutex::new(CosineSimilarityEngineInMemory::new()));
    
    read_test_data(Path::new("data/test_data.jsonl"), 3, |e| {
        let events = e.to_events();
        events.iter().for_each(|e| engine.lock().unwrap().add_event(e.clone()));
    });


    engine.lock().unwrap().train();


    Ok(())
}

#[test]
fn bar() -> Result<(), Error> {
    let engine = Arc::new(Mutex::new(CosineSimilarityEngineInMemory::new()));
    
    read_ecommerce_data(Path::new("data/data.csv"), |e| {
        let e = e;
        engine.lock().unwrap().add_event(e)
    });


    engine.lock().unwrap().train();


    Ok(())
}