#![allow(unused)]
extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use voca_rs::*;
use ansi_term::Style;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress{
    InProgress,
    Won,
    Lose
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let keyword = keyword();
    let mut letters = create_letters(&keyword);
    
    loop {
        
        println!("\n{}{}{}", Style::new().bold().paint("You Have "), turns_left, Style::new().bold().paint(" Attempts Left!"));
        println!("{}", Style::new().bold().paint("Give Up by Entering Asterick (*) Character"));
        println!("");
        display_progress(&letters);

        println!("Please Enter a Letter to Guess: ");
        let user_char = read_user_input_character();

        if user_char == '*'{
            break;
        }

        let mut at_least_one_revelaed = false;
        for letter in letters.iter_mut(){
            if letter.character == user_char{
                letter.revealed = true;
                at_least_one_revelaed = true;
            }
        }

        if !at_least_one_revelaed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => { 
                println!("\nCorrect! the Selected Word is {}", keyword);
                break;
            }
            GameProgress::Lose => { 
                println!("\nYou Lose! the Right Word is {}", keyword);
                break;
            }
        }
    }

    println!("Why Giving Up? The Selected Word is {}. Keep Trying", keyword);
    println!("{}", Style::new().bold().paint("Thanks for Guessing."));
    println!("{}", Style::new().bold().paint("Coded by encrypt0r using RUST"));
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
        }
        else{
            display_string.push('_');
        }
        display_string.push(' ');
    }
    println!("{}", display_string);
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input){
        Ok(_) =>{
            match user_input.to_uppercase().chars().next(){
                Some(c) => { return c; }
                None => { return'*'; }
            }
        }
        Err(_) => { return '*'; }
    }


}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_revealed = true;
    for letter in letters{
        if !letter.revealed{
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }
    return GameProgress::Lose;

}