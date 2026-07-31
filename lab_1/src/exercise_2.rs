pub fn ex_2() {
    // Move semantics 
    let s1 = String::from("hello");
    let _s2 = s1; // s1 is removed into s2

    //println!("s2 = {}", s2,);

    // clone (deep copy)
    let s3 = String::from("world");
    let s4 = s3.clone();
    
    println!("s3 = {}, s4 = {}", s3, s4);

    //borrowing (Immutable reference)
    let s5 = String::from("Rust is great");
    let length = calculate_length(&s5);

    println!("'{}' has {} characters", s5, length);

    // Todo 5
    let word: &str = first_word(&s5);
    println!("first word: {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}