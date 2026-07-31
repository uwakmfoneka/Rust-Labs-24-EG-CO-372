use std::fmt::Display;

// Generic function with multiple trait bounds
pub fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    if list.is_empty() {
        return;
    }

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    println!("The largest is {}", largest);
}

// Generic struct
#[derive(Debug)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    pub fn cmp_display(&self) {
        if self.first >= self.second {
            println!("First is larger: {}", self.first);
        } else {
            println!("Second is larger: {}", self.second);
        }
    }
}

// Generic function
pub fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    let min_len = std::cmp::min(a.len(), b.len());
    let mut result = Vec::with_capacity(min_len);

    for i in 0..min_len {
        result.push(f(&a[i], &b[i]));
    }

    result
}

// Function to run the exercise
pub fn ex_2() {
    let numbers = vec![3, 7, 2, 9, 5];
    print_largest(&numbers);

    let pair = Pair::new(10, 20);
    pair.cmp_display();

    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let result = zip_with(&a, &b, |x, y| x + y);

    println!("Result: {:?}", result);
}