fn divide(a: f64, b: f64) -> Option<f64> {
    if  b == 0.0 {
        None
    } else {
        Some(a /b)
    }
}

fn find_first_even(numbers:&[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}
pub fn ex_3() {
    // pattern matching on Option
    match divide(10.0, 3.0) {
        Some(result) => println!("10 / 3 = {:.4}", result),
        None => println!("Cannot divide by zero"),
    }

    // if let
    if let Some(val) = divide(7.0, 0.0) {
        println!("Got: {}", val);
    } else {
        println!("Divison failed (expected)");
    }

    // unwrap_or
    let safe = divide(5.0, 0.0).unwrap_or(f64::INFINITY);
    println!("Safe result:{}", safe);

    let nums = vec![1, 3, 7, 8, 11];

    match find_first_even(&nums) {
        Some(n) => println!("First even: {}", n),
        None => println!("no even numbers"),
    }
}