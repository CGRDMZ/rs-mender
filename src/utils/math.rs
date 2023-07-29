use std::collections::HashMap;

pub fn cosine_similarity(a: &HashMap<String, u32>, b: &HashMap<String, u32>) -> f64 {
    let dot = dot(a, b);

    let norms = norm(a) * norm(b);

    f64::from(dot) / norms
}

pub fn norm(a: &HashMap<String, u32>) -> f64 {
    let sum_of_squares: u64 = a
        .values()
        .into_iter()
        .map(|&i| i as u64)
        .map(|v| v * v)
        .sum();
    (sum_of_squares as f64).sqrt()
}

pub fn dot(a: &HashMap<String, u32>, b: &HashMap<String, u32>) -> u32 {
    let mut result = 00;
    for (ak, av) in a {
        if let Some(bv) = b.get(ak) {
            result += av * bv;
        }
    }
    result
}
