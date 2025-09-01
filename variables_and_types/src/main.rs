
//Immutability vs. Mutability
fn main() {
    let x = 10;        // immutable
    // x = 20;         // ❌ won't compile if you uncomment this

    let mut y = 10;     // mutable
    y = 20;             // ✅ allowed
    println!("x = {x}, y = {y}");
}

//Constant & Variables
const SECONDS_IN_MINUTE: u32 = 60;

fn main() {
    let mut counter = 0;  
    counter += 1; // ✅ can change because it's mutable

    // SECONDS_IN_MINUTE = 100; ❌ not allowed, constants never change
    println!("Counter = {counter}");
    println!("Seconds in a minute = {SECONDS_IN_MINUTE}");
}

//Shadowing (with type change)
fn main() {
    let spaces = "   ";        // spaces is &str
    println!("spaces as text: '{}'", spaces);

    let spaces = spaces.len(); // shadows previous spaces, now usize
    println!("spaces as number: {}", spaces);
}

//Shadowing
fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    // This is a shadowing example, not a mutability example

}
