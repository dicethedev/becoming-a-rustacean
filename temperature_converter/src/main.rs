use std::io;

//Formular:  Celsius → Fahrenheit: F = C * 9/5 + 32
//Fahrenheit → Celsius: C = (F - 32) * 5/9

fn main() {
    println!("Enter temperature in Fahrenheit:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let fahrenheit: f64 = input.trim().parse().expect("Please enter a valid number");
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F = {celsius:.2}°C");
}

// 📝 What this does:

// Reads input from the user.

// Parses it into a floating-point number.

// Converts F → C.

// Prints the result with 2 decimal places.