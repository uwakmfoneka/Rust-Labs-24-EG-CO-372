use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32) -> String {
    // Simulate network latency
    sleep(Duration::from_millis(100)).await;
    format!("Data from source {}", id)
}

#[tokio::main]
async fn main() {
    // Sequential - slow
    let t0 = std::time::Instant::now();

    for id in 1..=4 {
        let data = fetch_data(id).await;
        println!("Sequential: {}", data);
    }

    println!("Sequential time: {:?}", t0.elapsed());

    // Concurrent - fast
    let t1 = std::time::Instant::now();
    let handles: Vec<_> = (1..=4).map(|id| tokio::spawn(fetch_data(id))).collect();

    for h in handles {
        println!("Concurrent: {}", h.await.unwrap());
    }
    println!("Concurrent time: {:?}", t1.elapsed());

    // TODO 5
    let f1 = fetch_data(1);
    let f2 = fetch_data(2);
    let f3 = fetch_data(3);
    let f4 = fetch_data(4);

    let (r1, r2, r3, r4) = tokio::join!(f1, f2, f3, f4);

    println!("Concurrent: {}", r1);
    println!("Concurrent: {}", r2);
    println!("Concurrent: {}", r3);
    println!("Concurrent: {}", r4);

    println!("Concurrent time: {:?}", t1.elapsed());
}