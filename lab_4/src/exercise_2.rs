use std::collections::HashMap;

pub fn ex_2() {
    let text = "the quick brown fox jumps over the lazy dog the fox was very quick the dog was lazy";

    let freq = word_frequency(text);

    println!("Word Frequencies:");
    for (word, count) in &freq {
        println!("{}: {}", word, count);
    }

    println!("\nTop 5 words:");
    for (word, count) in top_n(&freq, 5) {
        println!("{}: {}", word, count);
    }
}

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();

    for word in text.split_whitespace() {
        let clean = word.to_lowercase();
        *freq.entry(clean).or_insert(0) += 1;
    }

    freq
}

fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut words: Vec<(String, usize)> =
        freq.iter().map(|(w, c)| (w.clone(), *c)).collect();

    words.sort_by(|a, b| b.1.cmp(&a.1));

    words.into_iter().take(n).collect()
}