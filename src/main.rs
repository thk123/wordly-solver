mod game {
    pub const GAME_WORD_LENGTH: usize = 5;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub enum LetterResponse {
        Correct,
        InWord,
        NotInWord,
    }

    pub struct GuessResponse {
        pub letter_responses: Vec<LetterResponse>,
    }

    pub fn is_guess_correct(response: &GuessResponse) -> bool {
        response
            .letter_responses
            .iter()
            .all(|&l| l == LetterResponse::Correct)
    }

    #[cfg(test)]
    mod is_guess_correct_tests {
        use super::*;

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
}

mod cli {
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
        /// Name of the person to greet
        #[clap(long, default_value = "words_alpha.txt")]
        word_list: String,

        /// Number of times to greet
        #[clap(long)]
        answer: Option<String>,
    }

    pub fn run_cli() {
        let args = Args::parse();
        println!("word list {}!", args.word_list);

        let word_list = read_word_list(args.word_list);
        let solver: Box<dyn Fn(&str) -> GuessResponse> = match args.answer {
            Some(word) => Box::new(move |guess: &str| {
                let answer_word = word.clone();
                non_interactive_solver::non_interactive_solver(guess, answer_word)
            }),
            None => Box::new(move |guess: &str| interactive_solver::interactive_solver(&guess)),
        };

        let sln = player::solve(&word_list, solver);
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
}

mod interactive_solver {
    use crate::game;
    use crate::game::GuessResponse;
    use crate::game::LetterResponse;
    use std::io;

    pub fn interactive_solver(guess: &str) -> GuessResponse {
        println!("Guess: {}", guess);
        let mut response = String::new();
        println!("Type a 5 letter response - y: correct, . - in word, x - not involved");

        loop {
            io::stdin()
                .read_line(&mut response)
                .expect("Failed to read line");
            response.pop();
            match response.len() {
                game::GAME_WORD_LENGTH => {
                    let parsed_response = parse_user_response(&response);
                    match parsed_response {
                        Ok(response) => return response,
                        Err(e) => println!("Invalid string: {}", e),
                    }
                }

                _ => {
                    println!("Enter exactly 5 characters, {}", response.len())
                }
            }
        }
    }

    fn parse_user_response(response_str: &String) -> Result<GuessResponse, String> {
        if response_str.len() != game::GAME_WORD_LENGTH {
            panic!("Must call parse_user_response with exactly 5 characters");
        }
        let mapped_response: Result<Vec<_>, _> = response_str
            .chars()
            .map(|char| match char {
                'y' => Ok(LetterResponse::Correct),
                '.' => Ok(LetterResponse::InWord),
                'x' => Ok(LetterResponse::NotInWord),
                x => Err(format!("Invalid character: {}", x)),
            })
            .collect();

        return match mapped_response {
            Ok(x) => Ok(GuessResponse {
                letter_responses: x.to_vec(),
            }),
            Err(e) => Err(format!("Invalid response: {}", e)),
        };
    }

    #[cfg(test)]
    mod parse_user_response_tests {
        use super::*;

        #[test]
        fn valid_string_parsed_correctly() {
            // If I inline this, it is apparently still needed for checking response.is_ok (lazy eval of parse_user_response?)
            let user_input = String::from("y.x.x");
            let response = parse_user_response(&user_input);
            assert!(response.is_ok());
            assert_eq!(
                response.unwrap().letter_responses,
                vec![
                    LetterResponse::Correct,
                    LetterResponse::InWord,
                    LetterResponse::NotInWord,
                    LetterResponse::InWord,
                    LetterResponse::NotInWord
                ]
            );
        }

        #[test]
        fn one_invalid_char_invalid_response() {
            let user_input = String::from("y.xax");
            let response = parse_user_response(&user_input);
            assert!(response.is_err());
        }
    }
}

mod non_interactive_solver {
    use crate::game::GuessResponse;
    use crate::game::LetterResponse;
    pub fn non_interactive_solver(guess: &str, answer: String) -> GuessResponse {
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
}

mod player {
    use crate::game;
    use crate::game::GuessResponse;
    use crate::game::LetterResponse;
    use itertools::Itertools;
    use std::collections::HashMap;

    struct Knowledge {
        guessed_words: Vec<String>,
        correct_letters: Vec<(char, usize)>,
        contained_letters: HashMap<char, Vec<usize>>,
    }

    pub struct Solution {
        pub guess_sequence: Vec<String>,
    }

    fn build_empty_knowledge() -> Knowledge {
        Knowledge {
            guessed_words: vec![],
            correct_letters: vec![],
            contained_letters: HashMap::new(),
        }
    }

    pub fn solve<T>(possbile_words: &Vec<String>, verifier: T) -> Solution
    where
        T: Fn(&str) -> GuessResponse,
    {
        solve_rec(possbile_words, verifier, &build_empty_knowledge())
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

        match game::is_guess_correct(&response) {
            true => Solution {
                guess_sequence: new_knowledge.guessed_words,
            },
            false => solve_rec(possbile_words, verifier, &new_knowledge),
        }
    }

    fn apply_learning(
        knowledge: &Knowledge,
        guess: &String,
        response: &GuessResponse,
    ) -> Knowledge {
        let mut new_words = knowledge.guessed_words.clone();
        new_words.push(guess.to_string());
        Knowledge {
            guessed_words: new_words,
            correct_letters: knowledge
                .correct_letters
                .iter()
                .map(|t| t.clone())
                .chain(
                    response
                        .letter_responses
                        .iter()
                        .enumerate()
                        .filter(|(_, &char)| char == LetterResponse::Correct)
                        .map(|(index, &_)| (guess.chars().nth(index).expect(""), index))
                        .collect::<Vec<(char, usize)>>(),
                )
                .collect::<Vec<(char, usize)>>(),
            contained_letters: merge_contained_letters_map(
                &knowledge.contained_letters,
                response
                    .letter_responses
                    .iter()
                    .enumerate()
                    .filter(|(_, &char)| char == LetterResponse::InWord)
                    .map(|(index, &_)| (guess.chars().nth(index).unwrap(), index)),
            ),
        }
    }

    fn merge_contained_letters_map<I>(
        original: &HashMap<char, Vec<usize>>,
        newly_tried_letters: I,
    ) -> HashMap<char, Vec<usize>>
    where
        I: Iterator<Item = (char, usize)>,
    {
        let mut new_map = original.clone();
        for (char, pos_tried) in newly_tried_letters {
            if new_map.contains_key(&char) {
                let mut current_vec = new_map[&char].clone();
                current_vec.push(pos_tried);
                new_map.insert(char, current_vec);
            } else {
                new_map.insert(char, vec![pos_tried]);
            }
        }
        new_map
    }
    #[cfg(test)]
    mod apply_learning_tests {
        use super::*;

        #[test]
        fn empty_knowledge_correct_letter_added_to_list_of_correct_letters() {
            let knowledge = build_empty_knowledge();
            let response = GuessResponse {
                letter_responses: [
                    LetterResponse::Correct,
                    LetterResponse::NotInWord,
                    LetterResponse::NotInWord,
                ]
                .to_vec(),
            };
            let new_knowledge = apply_learning(&knowledge, &String::from("abc"), &response);
            assert!(new_knowledge.correct_letters.len() == 1);
            assert!(new_knowledge.correct_letters[0] == ('a', 0));
        }

        #[test]
        fn empty_knowledge_contained_letters_added_to_list_of_contained_letters() {
            let knowledge = build_empty_knowledge();
            let response = GuessResponse {
                letter_responses: [
                    LetterResponse::InWord,
                    LetterResponse::NotInWord,
                    LetterResponse::NotInWord,
                ]
                .to_vec(),
            };
            let new_knowledge = apply_learning(&knowledge, &String::from("abc"), &response);
            assert!(new_knowledge.correct_letters.len() == 0);
            assert!(new_knowledge.contained_letters.len() == 1);
            assert!(new_knowledge.contained_letters.contains_key(&'a'));
            assert_eq!(new_knowledge.contained_letters[&'a'], vec![0]);
        }

        #[test]
        fn knowledge_about_letter_in_word_extended() {
            let knowledge = Knowledge {
                contained_letters: HashMap::from([('a', vec![0])]),
                ..build_empty_knowledge()
            };
            let response = GuessResponse {
                letter_responses: [
                    LetterResponse::NotInWord,
                    LetterResponse::InWord,
                    LetterResponse::NotInWord,
                ]
                .to_vec(),
            };
            let new_knowledge = apply_learning(&knowledge, &String::from("bac"), &response);
            assert!(new_knowledge.correct_letters.len() == 0);
            assert!(new_knowledge.contained_letters.len() == 1);
            assert!(new_knowledge.contained_letters.contains_key(&'a'));
            assert_eq!(new_knowledge.contained_letters[&'a'], vec![0, 1]);
        }
    }

    fn make_guess(possbile_words: &Vec<String>, knowledge: &Knowledge) -> String {
        let valid_words = possbile_words
            .iter()
            .filter(|w| {
                knowledge
                    .correct_letters
                    .iter()
                    .all(|(char, index)| w.chars().nth(*index).unwrap() == *char)
            })
            .filter(|w| {
                knowledge
                    .contained_letters
                    .iter()
                    .all(|char| w.chars().contains(char.0))
            })
            .filter(|w| {
                knowledge
                    .contained_letters
                    .iter()
                    .all(|(char, tried_indexes)| {
                        tried_indexes
                            .iter()
                            .all(|tried_index| w.chars().nth(*tried_index).unwrap() != *char)
                    })
            })
            .filter(|w| !knowledge.guessed_words.contains(w))
            .cloned()
            .collect::<Vec<String>>();
        if valid_words.len() > 2 {
            let guess = revealing_word(&valid_words, &knowledge);
            println!("{} possibilities, trying {}", valid_words.len(), guess);
            guess
        } else {
            valid_words.first().unwrap().to_string()
        }
    }

    fn revealing_word(possbile_words: &Vec<String>, knowledge: &Knowledge) -> String {
        let letter_frequency = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|c| (c, possbile_words.iter().map(|w| w.matches(c).count()).sum()))
            .collect::<HashMap<char, usize>>();

        possbile_words
            .iter()
            .max_by(|w1, w2| {
                let w1_score = word_score(&w1, &letter_frequency, &knowledge);
                let w2_score = word_score(&w2, &letter_frequency, &knowledge);
                w1_score.cmp(&w2_score)
            })
            .unwrap()
            .clone()
    }

    fn word_score(
        word: &String,
        char_frequence: &HashMap<char, usize>,
        knowledge: &Knowledge,
    ) -> usize {
        word.chars()
            .unique() // each letter only gets scored once
            .map(|c_in_word| {
                if knowledge.contained_letters.contains_key(&c_in_word) {
                    return char_frequence[&c_in_word];
                } else if knowledge
                    .correct_letters
                    .iter()
                    .any(|(letter, _)| *letter == c_in_word)
                {
                    return char_frequence[&c_in_word];
                } else if knowledge
                    .guessed_words
                    .iter()
                    .any(|w| w.contains(c_in_word))
                {
                    return 0;
                }
                return char_frequence[&c_in_word];
            })
            .sum()
    }

    #[cfg(test)]
    mod make_guess_tests {
        use super::*;

        #[test]
        fn guessed_one_word_dont_guess_again() {
            let words = [String::from("foo"), String::from("bar")].to_vec();
            let knowledge = Knowledge {
                guessed_words: [String::from("foo")].to_vec(),
                ..build_empty_knowledge()
            };
            let guess = make_guess(&words, &knowledge);
            assert!(guess == "bar");
        }

        #[test]
        fn guessed_one_correct_letter_guess_next_valid_word() {
            let words = [
                String::from("abc"),
                String::from("bcd"),
                String::from("abd"),
            ]
            .to_vec();
            let knowledge = Knowledge {
                guessed_words: vec![String::from("abc")],
                correct_letters: vec![('a', 0)],
                ..build_empty_knowledge()
            };
            let guess = make_guess(&words, &knowledge);
            assert!(guess == "abd");
        }

        #[test]
        fn guessed_one_contained_letter_guess_next_word_that_contains_in_different_position() {
            // let words = "abcdef".chars().iter().permutations(3).unique().collect();
            let words = vec![
                String::from("abc"),
                String::from("bcd"),
                String::from("acd"),
                String::from("bac"),
            ];
            let knowledge = Knowledge {
                guessed_words: vec![String::from("abc")],
                contained_letters: HashMap::from([('a', vec![0])]),
                ..build_empty_knowledge()
            };
            let guess = make_guess(&words, &knowledge);
            assert!(guess == "bac");
        }
    }
}

fn main() {
    cli::run_cli();
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
