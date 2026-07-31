fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // captures n by value
}

pub fn ex_3() {
    // Closures
    let double = |x| x * 2;

    println!(
        "apply_twice(double, 3) = {}",
        apply_twice(double, 3)
    );

    let add10 = make_adder(10);
    println!("add10(5) = {}", add10(5));

    // Chained iterators
    let result: Vec<String> = (1..=20)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5)
        .map(|x| format!("{}", x))
        .collect();

    println!(
        "First 5 even squares: {}",
        result.join(", ")
    );

    // Fold (reduce)
    let product: u64 = (1..=10).fold(1, |acc, x| acc * x);

    println!("10! = {}", product);
}