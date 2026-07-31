pub fn main() {
    // Use your reg number value
    let number = 373;

    // if an expression
    let description = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("{} is {}", number, description);

    // loop with break value
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    // while
    let mut n = 1;

    while n < 100 {
        n *= 2;
    }
    println!("First power of 2 >= 100: {}", n);

    // for over a range 
    let sum: i32 = (1..=100).sum();

    println!("Sum 1..=100 = {}", sum);

    //TODO 1: Multiplication table for 7
    println!("\nMultiplication Table of 7");

    for i in 1..=12 {
        println!("7 x {} = {}", i, 7 * i);
    }

    // using my reg number value (373)
    println!("\nDivison of your reg number value:");

    for i in 1..=12 {
        println!("373 x {} = {}", i, 373.0 / i as f64);
    }
}