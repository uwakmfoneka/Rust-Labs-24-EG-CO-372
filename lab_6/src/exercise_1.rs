use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("   [thread] count = {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("[main] count = {}", i);
        thread::sleep(Duration::from_millis(80));
    }

    handle.join().expect("Thread panicked");
    println!("All done");

    // TODO 1: Spawn 4 threads, each computing the sum of a different
    // quarter of (1..=1000). Collect all 4 handles and join them.
    // Print the total. Use `move` to capture ranges by value.

    let mut handles = Vec::new();

    let ranges = vec![
        (1, 250),
        (251, 500),
        (501, 750),
        (751, 1000),
    ];

    for (start, end) in ranges {
        let handle = thread::spawn(move || {
            let mut sum = 0;

            for i in start..=end {
                sum += i;
            }

            println!("Sum from {} to {} = {}", start, end, sum);
            sum
        });

        handles.push(handle);
    }

    let mut total = 0;

    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("Total sum = {}", total);
}