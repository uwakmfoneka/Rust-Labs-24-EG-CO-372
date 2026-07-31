// Returns the longest of two string slices
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// Struct holding a reference
pub struct Important<'a> {
    pub content: &'a str,
}

impl<'a> Important<'a> {
    pub fn summarise(&self) -> &str {
        &self.content[..self.content.len().min(80)]
    }
}

// Returns the first sentence in a string
pub fn first_sentence<'a>(text: &'a str) -> &'a str {
    match text.find('.') {
        Some(index) => &text[..index],
        None => text,
    }
}

pub fn main() {
    let s1 = String::from("long string is long");
    let result;

    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);
    }

    // println!("{}", result); // This would fail because s2 has been dropped.

    let article = String::from("Rust 2024 edition brings many improvements...");
    let imp = Important { content: &article };

    println!("Summary: {}", imp.summarise());

    let text_with_period =
        String::from("This is the first sentence. This is the second.");
    let text_no_period =
        String::from("No period here just words");

    println!("First sentence: {}", first_sentence(&text_with_period));
    println!(
        "First sentence (no period): {}",
        first_sentence(&text_no_period)
    );
}