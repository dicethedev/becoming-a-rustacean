
//Even/Odd checker
fn main() {
    let number = 7;

    if number % 2 == 0 {
        println!("{number} is even");
    } else {
        println!("{number} is odd");
    }
}
//result: 7 is odd

//Retry loop(real use case)
fn main() {
    let mut attempts = 0;

    loop {
        attempts += 1;
        println!("Attempt {attempts}: connecting...");

        // Fake success on attempt 2
        if attempts == 2 {
            println!("✅ Connected successfully!");
            break;
        }

        if attempts >= 3 {
            println!("❌ Failed to connect after {attempts} attempts.");
            break;
        }
    }
}
// Attempt 1: connecting...
// Attempt 2: connecting...
// ✅ Connected successfully!
// Attempt 3: connecting...
// ❌ Failed to connect after 3 attempts.


//Count down with for statement
fn main() {
    for second in (1..=5).rev() {
        println!("Launching in {second}...");
    }
    println!("🚀 LIFTOFF!");
}
