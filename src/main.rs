extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

struct Letter {
    character: char,
    revealed: bool
}

fn main() {
    let keyword = keyword();
    let letters = create_letters(&keyword);

    display_progress(&letters);
    println!("[DEBUG] The selected keyword above is {}!", keyword);
}

fn keyword() -> String {
    let mut file = File::open("keyword.txt").expect("Error opening the Keywords!");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("an Error has Occured while reading the file!");
    
    let available_word: Vec<&str> = file_content.trim().split(',').collect();

    let random_index = rand::thread_rng().gen_range(0, available_word.len());

    return String::from(available_word[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars(){
        letters.push(Letter{
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>){
    let mut display_string = String::from("Progress: ");

    for letter in letters{
        display_string.push(' ');

        if letter.revealed{
            display_string.push(letter.character);
        } else{
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}