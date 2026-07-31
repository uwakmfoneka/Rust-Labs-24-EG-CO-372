pub fn main () {
    // Immutable variable
let x = 5;
    println!("x = {}", x);

    // Mutable variable 
let mut y = 10;
    println!("y before = {}", y);
    y += 5;
    println!("y after = {}", y);  

    // Todo 1: Declare a float f64 called 'pi' with value 3.14159
let pi: f64 = 3.14159;


    // Todo 2: Declare a boolean called 'is_ learning' set to true
let is_learning = true; 


    // Todo 3: Declare a char called grade 
let grade = 'A';

    // Todo 4: print all three variables 
println!("pi = {}", pi);
println!("is_learning = {}", is_learning);
println!("grade = {}", grade);

    // shadowing 
let z = "42"; // &str
let z: u32 = z.parse().expect("Not a number!");
println!("parsed z = {}", z);
}