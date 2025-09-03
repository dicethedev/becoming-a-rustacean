
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } 

    let mut prev = 0;
    let cur curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn main() {
    for i in 0..=10 {
        println!("Fibonacci({}) = {}", i, fibonacci(i));
    }
}

//A Fibonacci generator is a program (or function) that produces numbers in the Fibonacci sequence.

// 🔢 Fibonacci Sequence

// The Fibonacci sequence is a series of numbers where:

// The first two numbers are 0 and 1.

// Every number after that is the sum of the two before it.

// So it looks like this:

// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...

// It means:

// fibonacci(0) → 0

// fibonacci(1) → 1

// fibonacci(2) → 1

// fibonacci(3) → 2

// fibonacci(4) → 3

// fibonacci(5) → 5

// …and so on.

// Basically, the function generates the nth Fibonacci number.


