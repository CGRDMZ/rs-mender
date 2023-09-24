use std::{fmt::Error, time::Instant};

use rs_mender::{
    core::{dataset::Dataset, similarity::SimilarityEngine},
    engine::matrix_factorization_engine::MatrixFactorizationEngine,
};

#[test]
fn foo2() -> Result<(), Error> {
    let dataset = Dataset::from_jsonl("./data/test_data.jsonl".to_string());

    let mut engine = MatrixFactorizationEngine::new(dataset);

    engine.train();

    let start = Instant::now();
    println!(
        "{:?}, time passed: {:?}ms",
        engine.find_similar_by_user_id("0fb1e031d84a".to_string(), 10),
        start.elapsed().as_millis()
    );

    Ok(())
}

#[test]
fn foo() -> Result<(), Error> {
    let dataset = Dataset::from_csv_example();

    let mut engine = MatrixFactorizationEngine::new(dataset);

    engine.train();

    let start = Instant::now();
    println!(
        "{:?}, time passed: {:?}ms",
        engine.find_similar_by_user_id("17850".to_string(), 10),
        start.elapsed().as_millis()
    );

    Ok(())
}
