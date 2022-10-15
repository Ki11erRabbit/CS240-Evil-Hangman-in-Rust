pub mod game;

use std::env;
use std::fs::File;
use std::io;
use std::collections::HashSet;
use std::rc::Rc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dictionary = File::open(&args[1]).expect("Unable to open dictionary");
    let word_length = args[2].parse::<usize>().unwrap();
    let total_guesses = args[3].parse::<u32>().unwrap();

    let mut game = game::evil_hangman_game::EvilHangmanGame::new();

    match game.start_game(&mut dictionary, word_length) {
        Err(x) => panic!("{}",x),
        Ok(_) => {}
    }



    let continue_game = true;
    let mut remaining_guesses = total_guesses;
    let mut current_set = game.get_current_set().clone();
    let mut current_words:Rc<HashSet<String>> = Rc::new(HashSet::new());
    while continue_game && remaining_guesses > 0 {
        println!("You have {} guesses left",remaining_guesses);
        print!("Used letters: [");
        let mut index = 0;
        let used_char = game.get_guessed_letters();
        for letter in used_char {
           print!("{}",letter);
           if index < used_char.len() -1 {
                print!(", ");
           }
           index += 1;
        }
        println!("]");
        println!("Word: {}", current_set);

        let mut valid_input = false;
        let mut input: Vec<String> = Vec::new();
        let mut guess = String::new();
        let mut guess_to_use;

        while !valid_input {
            if input.len() == 0 {
                io::stdin().read_line(&mut guess)
                    .expect("Failed to read line");

                let collected_vals:Vec<&str> = guess.split_whitespace().collect();
                
                for val in collected_vals {
                    input.push(val.to_string());
                }
                if input.len() > 0 {
                    guess_to_use = input.remove(0);
                }
                else {
                    continue;
                }
            }
            else {
                guess_to_use = input.remove(0);
            }
                
            if guess_to_use.len() > 1 {
                println!("Invalid input! Enter guess: ");
            }
            else if guess_to_use.len() == 0 {
                continue;
            }
            else if !guess_to_use.chars().next().unwrap().is_alphabetic() {
                println!("Invalid input! Enter guess: ");
            }
            else if guess_to_use.chars().next().unwrap() == '\n' {
                continue;
            }
            else {
                valid_input = true;

                match game.make_guess(guess_to_use.chars().next().unwrap()) {
                    Err(_) => {
                        println!("Guess already made! Enter guess: ");
                        valid_input = false;
                    }
                    Ok(v) => {
                        current_words = v.clone();
                        if current_set != *game.get_current_set() {
                            current_set = game.get_current_set().clone();
                            let mut count = 0;
                            for i in 0..current_set.len() {
                                if current_set.get(i..=i).unwrap() == guess_to_use.get(0..=0).unwrap() {
                                    count += 1;
                                }
                            }

                            println!("Yes, there is {} {}\n",count,guess_to_use.chars().next().unwrap());
                        }
                        else {
                            println!("Sorry, there are no {}'s\n",guess_to_use.get(0..=0).unwrap());
                            remaining_guesses -= 1;
                        }
                    }
                }
            }
        }
        if !current_set.contains("-") {
            break;
        }
    }
    if !current_set.contains("-") || remaining_guesses > 0 {
        println!("You win! You guessed the word: {}",current_set);
    }
    else {
        println!("You lose!");
        println!("The word was: {}",current_words.iter().next().unwrap())
    }


}
