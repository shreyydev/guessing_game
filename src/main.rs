use core::panic;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, stdin};
use stopwatch::Stopwatch;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    println!("NumGuesser!\n");

    game_loop(&get_secret_number(&get_difficulity()));
}

#[derive(Debug, EnumIter)]
enum Difficulity {
    Easy,
    Medium,
    Hard,
}

fn get_difficulity() -> u32 {
    println!("Diffculity options");
    for (i, diffucilty) in Difficulity::iter().enumerate() {
        println!("{}: {:?}", i + 1, diffucilty);
    }
    println!("Please select a diffucility");
    let mut difficulity_selected = String::new();
    stdin()
        .read_line(&mut difficulity_selected)
        .expect("Error reading from stdin");
    let difficulity_selected: u32 = match difficulity_selected.trim().parse() {
        Ok(num) => num,
        Err(error) => panic!("Error while parsing the difficulity. {:?}", error),
    };
    return difficulity_selected;
}

fn get_secret_number(diffculity_selected: &u32) -> u32 {
    let secret_number: u32 = match diffculity_selected {
        1 => rand::thread_rng().gen_range(1, 101),
        2 => rand::thread_rng().gen_range(1, 201),
        3 => rand::thread_rng().gen_range(1, 301),
        _ => panic!("Difficulity does not exist"),
    };
    return secret_number;
}

fn game_loop(secret_number: &u32) {
    let mut sw = Stopwatch::start_new();
    let mut guess_count: i64 = 0;
    loop {
        println!("Please enter a guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess_count = guess_count + 1;
        println!("Time elapsed: {}secs", sw.elapsed_ms() / 1000);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                sw.stop();
                println!("Yay! you guessed it.");
                break;
            }
        }
    }
    println!(
        "Your score is {}",
        get_score(&sw.elapsed_ms(), &guess_count)
    );
}

fn get_score(time_taken: &i64, guess_count: &i64) -> i64 {
    return *time_taken / 1000 * guess_count;
}
