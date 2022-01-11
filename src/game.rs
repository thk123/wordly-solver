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
