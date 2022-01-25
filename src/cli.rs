use crate::game;
use crate::game::GuessResponse;
use crate::interactive_solver;
use crate::non_interactive_solver;
use crate::player;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(long, default_value = "words_alpha.txt")]
    word_list_path: String,

    #[clap(long)]
    answer: Option<String>,
}

pub fn run_cli() {
    let args = Args::parse();

    let word_list = read_word_list(args.word_list_path);
    let verifier: Box<dyn Fn(&str) -> GuessResponse> = match args.answer {
        Some(word) => Box::new(move |guess: &str| {
            let answer_word = word.clone();
            non_interactive_solver::non_interactive_solver(guess, answer_word)
        }),
        None => Box::new(move |guess: &str| interactive_solver::interactive_solver(&guess)),
    };

    let sln = player::solve(&word_list, verifier);
    println!("Solution: {:?}", sln.guess_sequence);
}

fn read_word_list(word_list_file_name: String) -> Vec<String> {
    fs::read_to_string(word_list_file_name)
        .expect("Error reading file")
        .split_ascii_whitespace()
        .filter(|s| s.len() == game::GAME_WORD_LENGTH)
        .filter(|s| s.chars().all(|c| c.is_alphabetic()))
        .map(str::to_string)
        .collect()
}
