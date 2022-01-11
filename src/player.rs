use crate::game;
use crate::game::GuessResponse;
use crate::game::LetterResponse;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Solution {
    pub guess_sequence: Vec<String>,
}

struct Knowledge {
    guessed_words: Vec<String>,
    correct_letters: Vec<(char, usize)>,
    contained_letters: HashMap<char, Vec<usize>>,
}

fn build_empty_knowledge() -> Knowledge {
    Knowledge {
        guessed_words: vec![],
        correct_letters: vec![],
        contained_letters: HashMap::new(),
    }
}

pub fn solve<VFn>(possbile_words: &Vec<String>, verifier: VFn) -> Solution
where
    VFn: Fn(&str) -> GuessResponse,
{
    solve_rec(possbile_words, verifier, &build_empty_knowledge())
}

fn solve_rec<VFn>(
    possbile_words: &Vec<String>,
    verifier: VFn,
    starting_knowledge: &Knowledge,
) -> Solution
where
    VFn: Fn(&str) -> GuessResponse,
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

fn apply_learning(knowledge: &Knowledge, guess: &String, response: &GuessResponse) -> Knowledge {
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
                    .map(|(index, &_)| (guess.chars().nth(index).unwrap(), index))
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
