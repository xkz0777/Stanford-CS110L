// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn show_string_in_vec(s: &mut Vec<char>) {
    for c in s {
        print!("{}", c);
    }
    print!("\n");
}

fn show_word_so_far(word_so_far: &mut Vec<char>) {
    print!("The word so far is ");
    show_string_in_vec(word_so_far);
}

fn show_guessed(guessed: &mut Vec<char>) {
    print!("You have guessed the following letters: ");
    show_string_in_vec(guessed);
}

fn read_guess(guess: &mut String) {
    io::stdout().flush().expect("Error flushing stdout.");
    io::stdin().read_line(guess).expect("Error reading line.");
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :
    let mut chances = NUM_INCORRECT_GUESSES;
    let l = secret_word.len();
    println!("Welcome to CS110L Hangman!");
    let mut word_so_far: Vec<char> = Vec::new();
    for _ in 0..l {
        word_so_far.push('-');
    }
    let mut guessed = Vec::new();

    loop {
        show_word_so_far(&mut word_so_far);
        show_guessed(&mut guessed);
        println!("You have {} guesses left", chances);
        print!("Please guess a letter: ");

        let mut guess = String::new();
        // Make sure the prompt from the previous line gets displayed:
        read_guess(&mut guess);
        let guess_char: Vec<char> = guess.chars().collect();
        let mut right_guess = false;
        for i in 0..l {
            if secret_word_chars[i] == guess_char[0] {
                right_guess = true;
                word_so_far[i] = guess_char[0];
            }
        }
        if !right_guess {
            chances -= 1;
            println!("Sorry, that letter is not in the word");
        }
        print!("\n");
        if chances == 0 {
            println!("Sorry, you ran out of guesses!");
            break;
        } else if !word_so_far.contains(&'-') {
            println!(
                "Congratulations you guessed the secret word: {}!",
                secret_word
            );
            break;
        }
    }
}
