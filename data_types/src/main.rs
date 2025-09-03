
//Scalar Types
fn main() {
    // Integer (i32 by default)
    let a: i32 = 42;
    let b = 100; // Rust infers i32
    println!("a = {}, b = {}", a, b);

    // Floating-point
    let x = 3.14;      // f64 by default
    let y: f32 = 2.5;  // explicitly f32
    println!("x = {}, y = {}", x, y);

    // Boolean
    let is_active = true;
    let is_complete: bool = false;
    println!("is_active = {}, is_complete = {}", is_active, is_complete);

    // Character
    let c = 'A';
    let emoji: char = '😻';
    println!("c = {}, emoji = {}", c, emoji);
}

//Tuple (compound type)
// fn main() {
//     let person: (&str, i32, bool) = ("Alice", 30, true);
    
//     // Destructuring
//     let (name, age, is_student) = person;
//     println!("Name: {}, Age: {}, Student? {}", name, age, is_student);

//     // Access by index
//     println!("Access by index: {} is {}", person.0, person.1);
// }

//Array (compound type)
// fn main() {
//     let numbers: [i32; 5] = [10, 20, 30, 40, 50];
//     println!("First number: {}", numbers[0]);

//     // Array initialized with the same value
//     let ones = [1; 5]; // same as [1, 1, 1, 1, 1]
//     println!("Array of ones: {:?}", ones);

//     // Sum first and last element
//     println!("Sum: {}", numbers[0] + numbers[4]);
// }

//Mixed example of all types
// fn main() {
//     let age: u8 = 25;          // integer
//     let temperature = 36.6;    // float
//     let is_raining = false;    // boolean
//     let letter = 'R';          // char
//     let readings = [20, 25, 30]; // array
//     let person = ("Bob", age);    // tuple

//     println!("{} is {} years old", person.0, person.1);
//     println!("Today's temperature: {}", temperature);
//     println!("Rain? {}", is_raining);
//     println!("Letter: {}", letter);
//     println!("First reading: {}", readings[0]);
// }

