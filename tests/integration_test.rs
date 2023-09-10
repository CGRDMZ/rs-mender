use std::fmt::Error;

use rs_mender::{engine::matrix_factorization_engine::MatrixFactorizationEngine, core::{dataset::Dataset, similarity::SimilarityEngine}};

#[test]
fn foo2() -> Result<(), Error> {

    let dataset = Dataset::from_jsonl("./data/test_data.jsonl".to_string());


    let mut engine = MatrixFactorizationEngine::new(dataset);

    engine.train();

    Ok(())
}

#[test]
fn foo() -> Result<(), Error> {

    let dataset = Dataset::from_csv_example();


    let mut engine = MatrixFactorizationEngine::new(dataset);

    engine.train();

    Ok(())
}