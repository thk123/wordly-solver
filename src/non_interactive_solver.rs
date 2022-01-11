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
