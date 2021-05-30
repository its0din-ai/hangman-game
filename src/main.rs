extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let keyword = keyword();

    println!("The Generated Word is {}", keyword);
}

fn keyword() -> String {
    let mut file = File::open("keyword.txt").expect("Error opening the Keywords!");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("an Error has Occured while reading the file!");
    
    let available_word: Vec<&str> = file_content.trim().split(',').collect();

    let random_index = rand::thread_rng().gen_range(0, available_word.len());

    return String::from(available_word[random_index]);
}
