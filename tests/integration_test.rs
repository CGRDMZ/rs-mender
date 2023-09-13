use std::fmt::Error;

use rs_mender::{engine::matrix_factorization_engine::MatrixFactorizationEngine, core::{dataset::Dataset, similarity::SimilarityEngine}};

#[test]
fn foo2() -> Result<(), Error> {

    let dataset = Dataset::from_jsonl("./data/test_data.jsonl".to_string());


    let mut engine = MatrixFactorizationEngine::new(dataset);


    engine.train();

    println!("{:?}", engine.find_similar_by_user_id("0fb1e031d84a".to_string(), 10));

    Ok(())
}

#[test]
fn foo() -> Result<(), Error> {

    let dataset = Dataset::from_csv_example();


    let mut engine = MatrixFactorizationEngine::new(dataset);

    engine.train();

    println!("{:?}", engine.find_similar_by_user_id("17850".to_string(), 10));
    Ok(())
}