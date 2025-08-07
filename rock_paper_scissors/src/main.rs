// Import standard library's I/O module for reading user input
use std::io;

// Import random number generation tools from the `rand` crate
use rand::Rng;

// Enum representing the possible moves
#[derive(Debug, PartialEq)] // Allows us to print and compare Move values easily
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// Convert a string into a `Move`
    /// Returns Some(Move) if valid, None if invalid
    fn from_str(input: &str) -> Option<Move> {
        match input.trim().to_lowercase().as_str() {
            "rock" => Some(Move::Rock),
            "paper" => Some(Move::Paper),
            "scissors" => Some(Move::Scissors),
            _ => None, // Anything else is invalid
        }
    }

    /// Generate a random move for the computer
    fn random() -> Move {
        // Pick a number between 0 and 2
        let num = rand::thread_rng().gen_range(0..3);
        // Convert the number to a Move
        match num {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissors,
        }
    }
}

/// Decide the winner of a round
/// Returns a message describing the result
fn decide_winner(player: &Move, computer: &Move) -> &'static str {
    // If both moves are the same, it's a tie
    if player == computer {
        "It's a tie!"
    } else {
        // Check all player win conditions
        match (player, computer) {
            (Move::Rock, Move::Scissors) => "You win! Rock beats Scissors",
            (Move::Paper, Move::Rock) => "You win! Paper beats Rock",
            (Move::Scissors, Move::Paper) => "You win! Scissors beats Paper",
            // Otherwise, the player loses
            _ => "You lose!",
        }
    }
}

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");
    println!("Type 'rock', 'paper', or 'scissors'. Type 'quit' to stop.");

    // Game loop
    loop {
        let mut input = String::new();

        println!("\nYour move:");

        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Check if user wants to quit
        if input.trim().eq_ignore_ascii_case("quit") {
            println!("Thanks for playing!");
            break;
        }

        // Convert input into a Move
        let player_move = match Move::from_str(&input) {
            Some(m) => m,
            None => {
                println!("Invalid move. Try again.");
                continue;
            }
        };

        // Generate computer's move
        let computer_move = Move::random();

        // Show choices
        println!("You played: {:?}", player_move);
        println!("Computer played: {:?}", computer_move);

        // Decide and print result
        let result = decide_winner(&player_move, &computer_move);
        println!("{}", result);
    }
}
