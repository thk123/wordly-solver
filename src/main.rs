use clap::Parser;
use std::fs;

const GAME_WORD_LENGTH: usize = 5;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    #[clap(long, default_value = "words_alpha.txt")]
    word_list: String,

    /// Number of times to greet
    #[clap(long)]
    answer: Option<String>,
}

struct Knowledge {
    guessed_words: Vec<String>,
    correct_letters : Vec<(char, usize)>,
}

struct Solution {
    guess_sequence: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("word list {}!", args.word_list);

    let word_list = read_word_list(args.word_list);
    let solver: Box<dyn Fn(&str) -> GuessResponse> = match args.answer {
        Some(word) => Box::new(move |guess: &str| {
            let answer_word = word.clone();
            non_interactive_solver(guess, answer_word)
        }),
        None => Box::new(|_: &str| GuessResponse {
            letter_responses: [LetterResponse::Correct; GAME_WORD_LENGTH].to_vec(),
        }),
    };

    let sln = solve(&word_list, solver);
    println!("Solution: {:?}", sln.guess_sequence);
}

fn non_interactive_solver(guess: &str, answer: String) -> GuessResponse {
    GuessResponse {
        letter_responses: guess
            .chars()
            .enumerate()
            .map(|(index, char)| {
                if answer
                    .chars()
                    .nth(index)
                    .expect("Word length different from guess length")
                    == char
                {
                    return LetterResponse::Correct;
                } else if answer.contains(char) {
                    return LetterResponse::InWord;
                } else {
                    return LetterResponse::NotInWord;
                }
            })
            .collect(),
    }
}
#[cfg(test)]
mod non_interactive_solver_tests {
    use super::*;
    #[test]
    fn words_same_all_correct() {
        let guess = "hello";
        let answer = "hello";
        let result = non_interactive_solver(&guess, String::from(answer));
        assert!(result
            .letter_responses
            .iter()
            .all(|&l| l == LetterResponse::Correct))
    }

    #[test]
    fn letter_in_not_right_place() {
        let guess = "abc";
        let answer = "dea";
        let result = non_interactive_solver(&guess, String::from(answer));
        assert!(result.letter_responses[0] == LetterResponse::InWord);
        assert!(result.letter_responses[1] == LetterResponse::NotInWord);
        assert!(result.letter_responses[2] == LetterResponse::NotInWord);
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum LetterResponse {
    Correct,
    InWord,
    NotInWord,
}

struct GuessResponse {
    letter_responses: Vec<LetterResponse>,
}

fn is_guess_correct(response: &GuessResponse) -> bool {
    response
        .letter_responses
        .iter()
        .all(|&l| l == LetterResponse::Correct)
}

#[cfg(test)]
mod is_guess_correct_tests {
    use crate::{is_guess_correct, GuessResponse, LetterResponse};

    #[test]
    fn all_correct_is_correct() {
        let response = GuessResponse {
            letter_responses: [LetterResponse::Correct; 5].to_vec(),
        };
        assert!(is_guess_correct(&response));
    }

    #[test]
    fn one_in_correct_is_not_correct() {
        let response = GuessResponse {
            letter_responses: [
                LetterResponse::Correct,
                LetterResponse::Correct,
                LetterResponse::Correct,
                LetterResponse::InWord,
                LetterResponse::Correct,
            ]
            .to_vec(),
        };
        assert!(!is_guess_correct(&response));
    }
}

fn solve<T>(possbile_words: &Vec<String>, verifier: T) -> Solution
where
    T: Fn(&str) -> GuessResponse,
{
    let knowledge = Knowledge {
        guessed_words: vec![],
        correct_letters: vec![]
    };
    solve_rec(possbile_words, verifier, &knowledge)
}

fn solve_rec<T>(
    possbile_words: &Vec<String>,
    verifier: T,
    starting_knowledge: &Knowledge,
) -> Solution
where
    T: Fn(&str) -> GuessResponse,
{
    let guess = make_guess(possbile_words, starting_knowledge);
    let response = verifier(&guess);
    let new_knowledge = apply_learning(starting_knowledge, &guess, &response);

    match is_guess_correct(&response) {
        true => Solution {
            guess_sequence: new_knowledge.guessed_words,
        },
        false => solve_rec(possbile_words, verifier, &new_knowledge),
    }
}

fn apply_learning(knowledge: &Knowledge, guess: &String, response: &GuessResponse) -> Knowledge {
    let mut new_words = knowledge.guessed_words.clone();
    new_words.push(guess.to_string());
    Knowledge {
        guessed_words: new_words,
        correct_letters: knowledge.correct_letters.iter()
            .map(|t| t.clone())
            .chain(
            response.letter_responses.iter()
                .enumerate()
                .filter(|(index, &char)| char == LetterResponse::Correct)
                .map(|(index, &char_response)| (guess.chars().nth(index).expect(""), index))
                .collect::<Vec<(char, usize)>>())
            .collect::<Vec<(char, usize)>>()
    }
}

#[cfg(test)]
mod apply_learning_tests
{
    use super::*;

    #[test]
    fn empty_knowledge_correct_letter_added_to_list_of_correct_letters() {
        let knowledge = Knowledge{
            guessed_words: vec![],
            correct_letters: vec![]
        };
        let response = GuessResponse{
            letter_responses: [LetterResponse::Correct, LetterResponse::NotInWord, LetterResponse::NotInWord].to_vec(),
        };
        let new_knowledge = apply_learning(&knowledge, &String::from("abc"), &response);
        assert!(new_knowledge.correct_letters.len() == 1);
        assert!(new_knowledge.correct_letters[0] == ('a', 0));
    }
}

fn make_guess(possbile_words: &Vec<String>, knowledge: &Knowledge) -> String {
    possbile_words
        .iter()
        .filter(|w| knowledge.correct_letters.iter().all((|(char, index)| w.chars().nth(*index).unwrap() == *char)))
        .find(|w| !knowledge.guessed_words.contains(w))
        .expect("Surely must be something to guess")
        .to_string()
}

#[cfg(test)]
mod make_guess_tests
{
    use super::*;

    #[test]
    fn guessed_one_word_dont_guess_again()
    {
        let words = [String::from("foo"), String::from("bar")].to_vec();
        let knowledge = Knowledge{
            guessed_words: [String::from("foo")].to_vec(),
            correct_letters: vec![]
        };
        let guess = make_guess(&words, &knowledge);
        assert!(guess == "bar");
    }

    #[test]
    fn guessed_one_correct_letter_guess_next_valid_word()
    {
        let words = [String::from("abc"), String::from("bcd"), String::from("abd")].to_vec();
        let knowledge = Knowledge{
            guessed_words: vec![String::from("abc")],
            correct_letters: vec![('a', 0)]
        };
        let guess = make_guess(&words, &knowledge);
        assert!(guess == "abd");
    }
}

fn read_word_list(word_list_file_name: String) -> Vec<String> {
    fs::read_to_string(word_list_file_name)
        .expect("Error reading file")
        .split_ascii_whitespace()
        .filter(|s| s.len() == GAME_WORD_LENGTH)
        .map(str::to_string)
        .collect()
}
// Full solution

// Accept argument --word-list for specifying a words list (default words.list)
// accept argument --answer for providing the answer (otherwise default to interactive)

// Read word list
// ** Compute letter frequency
// ** Find word with the most distinct high frequency letters
// ***Tie break words using word frequency
// Validate guess
//    interactive - read response to find out correct state
//    non-interactive compute response
// * Apply knowledge from guess
// Loop until guess the right word
