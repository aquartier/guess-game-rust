use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

const MAX_NUMBER: i32 = 100;
const MAX_ATTEMPTS: i32 = 10;

enum State {
    TooHigh,
    TooLow,
    Won,
    Lost
}

struct GuessGame {
    secret_number: i32,
    attempts: i32,
}

impl GuessGame {
    fn guess(&self, number: i32) -> State {
        if self.attempts == MAX_ATTEMPTS {
            return State::Lost;
        }
        match number.cmp(&self.secret_number) {
            Ordering::Less => State::TooLow,
            Ordering::Greater => State::TooHigh,
            Ordering::Equal => State::Won,
        }
    }
}

fn main() {
    let mut game = GuessGame {
        secret_number: rand::thread_rng().gen_range(0, MAX_NUMBER),
        attempts: 0,
    };
    println!("Welcome to Guess Game! Let guess a secret number between 0 to {:?}.", MAX_NUMBER);

    loop {
        print!("Attempts {:?} of {:?}: ", game.attempts, MAX_ATTEMPTS);
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read input");

        let number: i32 = input.trim().parse().expect("Input is not a number");
        game.attempts += 1;

        match game.guess(number) {
            State::TooLow => println!("Your guess was too low."),
            State::TooHigh => println!("Your guess was too high."),
            State::Won => {
                println!("You win!");
                break;
            },
            State::Lost => {
                println!("You lost. The secret number was: {:?}", game.secret_number);
                break;
            },
        }
    }
}