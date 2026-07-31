fn add(a: i32, b: i32) ->i32 {
    a + b 
}
fn greet(name: &str) -> String {
    format!("Hello , {}!", name)
}
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n -1)
    }
}

pub fn ex_3() {
    println!("{}", add(3, 7));
    println!("{}", greet("Rustacean"));
    println!("factorial(10) = {}", factorial(10));
}