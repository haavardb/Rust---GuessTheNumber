extern crate rand;
extern crate ansi_term;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use ansi_term::Colour::{Red, Blue, Yellow, Green};

fn main() {
    println!("{}", Blue.bold().underline().paint("\n\n-- Guess the number --\n\n"));
    println!("Please input your guess, between 1-100.");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!("{}", Red.paint("Please type a BETWEEN 1-100 you idiot..."));
                    100
                } else {
                    num
                }
            },
            Err(_) => {
                println!("{}", Red.paint("Please type a number you idiot..."));
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("{}", Yellow.paint("Too small.. Guess again:")),
            Ordering::Greater => println!("{}", Yellow.paint("Too big.. Guess again:")),
            Ordering::Equal   => {
                println!("{}", Green.bold().paint("\n\nTA-DA-RA-DUM!\nYou win!!\n\n\n"));
                break;
            }
        }
    }
}
