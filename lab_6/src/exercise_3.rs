use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
enum WorkResult {
    Sum(u64),
    Error(String),
}

fn worker(id: usize, data: Vec<u64>, tx: mpsc::Sender<WorkResult>) {
    let sum = data.iter().sum();

    println!("Worker {} computed sum = {}", id, sum);

    if sum > 30000 {
        tx.send(WorkResult::Error(format!(
            "Worker {}: sum {} is greater than 30000",
            id, sum
        )))
        .unwrap();
    } else {
        tx.send(WorkResult::Sum(sum)).unwrap();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let dataset: Vec<Vec<u64>> = (0..4)
        .map(|i| (i * 250 + 1..=(i + 1) * 250).collect())
        .collect();

    for (id, chunk) in dataset.into_iter().enumerate() {
        let tx_clone = tx.clone();
        thread::spawn(move || worker(id, chunk, tx_clone));
    }

    drop(tx); // close the original sender

    let mut total = 0u64;

    for result in rx {
        match result {
            WorkResult::Sum(s) => {
                total += s;
            }
            WorkResult::Error(msg) => {
                println!("Error: {}", msg);
            }
        }
    }

    println!("Grand total: {}", total);
}