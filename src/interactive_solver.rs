use crate::game;
use crate::game::GuessResponse;
use crate::game::LetterResponse;
use std::io;

pub fn interactive_solver(guess: &str) -> GuessResponse {
    println!("Guess: {}", guess);
    let mut response = String::new();
    println!("Type a {} letter response - y: correct, . - in word, x - not involved", game::GAME_WORD_LENGTH);

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
                println!("Enter exactly {} characters, {}", game::GAME_WORD_LENGTH, response.len())
            }
        }
    }
}

fn parse_user_response(response_str: &str) -> Result<GuessResponse, String> {
    if response_str.len() != game::GAME_WORD_LENGTH {
        panic!("Must call parse_user_response with exactly {} characters", game::GAME_WORD_LENGTH);
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
