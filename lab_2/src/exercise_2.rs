#[derive(Debug)]
enum Direction{
    North,
    South,
    East,
    West,
} 
#[derive(Debug)]
enum Shape {
Circle(f64),                    // radius
Rectangle(f64, f64),            // width, height
Triangle(f64, f64, f64),        // sides a, b, c    
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        
        //Triangle area usiing Heron's formula
        Shape::Triangle(a, b, c) => {
        let s = (a + b + c) / 2.0; 
        (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
 }

 fn describe_dirction(d: &Direction) {
    match d {
        Direction::North =>println!("Heading North - towards the mountains"),
        Direction::South =>println!("Heading South - towards the bench"),
        Direction::East =>println!("Heading East - towards the sunrise"),
        Direction::West =>println!("Heading West - towards the sunset"),
    }
 }

 pub fn ex_2() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for s in &shapes {
        println!("{:?} => area = {:.2}", s, area(s));
    }

    println!();
    
    describe_dirction(&Direction::North);
    describe_dirction(&Direction::South);
    describe_dirction(&Direction::East);
    describe_dirction(&Direction::West);
 }