
//simple function example
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let number = 5;
    let new_number = plus_one(number);

    println!("Original: {number}, Plus One: {new_number}");
}
//output: Original: 5, Plus One: 6

//Real-Use Case Example: Temperature Conversion
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let today_c = 30.0;
    let today_f = celsius_to_fahrenheit(today_c);

    println!("Today's temperature: {today_c}°C = {today_f}°F");
}
//output: Today's temperature: 30°C = 86°F


//Function with Multiple Parameters
fn rectangle_area(length: f64, width: f64) -> f64 {
    length * width
}

fn main() {
    let length = 10.0;
    let width = 5.0;
    let area = rectangle_area(length, width);

    println!("Area of the rectangle: {area}");
}
//output: Area of the rectangle: 50
