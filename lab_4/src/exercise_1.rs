pub fn stats(data: &[f64]) -> (f64, f64, f64) {
    let sum: f64 = data.iter().sum();
    let mean = sum / data.len() as f64;

    let min = data
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);

    let max = data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    (mean, min, max)
}

pub fn main() {
    let mut scores: Vec<f64> = vec![85.0, 92.0, 78.5, 95.0, 60.0, 88.0];

    // Sorting
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Sorted: {:?}", scores);

    // Iterator chain
    let high_scores: Vec<f64> = scores
        .iter()
        .filter(|&&s| s >= 80.0)
        .copied()
        .collect();

    println!("High scores: {:?}", high_scores);

    let (mean, min, max) = stats(&scores);

    println!("Mean = {:.2}", mean);
    println!("Min  = {:.2}", min);
    println!("Max  = {:.2}", max);

    // Median
    let median = if scores.len() % 2 == 0 {
        (scores[scores.len() / 2 - 1] + scores[scores.len() / 2]) / 2.0
    } else {
        scores[scores.len() / 2]
    };

    println!("Median = {:.2}", median);

    // Variance
    let variance: f64 = scores
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>()
        / scores.len() as f64;

    // Standard deviation
    let std_dev = variance.sqrt();

    println!("Variance = {:.2}", variance);
    println!("Standard Deviation = {:.2}", std_dev);
}