use clap::Parser;
use std::fs;

const GAME_WORD_LENGTH : usize = 5;

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

struct Knowledge
{
    guessed_words : Vec<String>,
}

struct Solution
{
    guess_sequence : Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("word list {}!", args.word_list);

    let word_list = read_word_list(args.word_list);
    let solver : Box<dyn Fn(&str) -> GuessResponse> =
        match args.answer {
            Some(word) => { Box::new(move |guess:&str|
                                         {
                                             let answer_word = word.clone();
                                             non_interactive_solver(guess, answer_word)
                                         })}
            None => { Box::new(|_: &str| GuessResponse{ is_correct: true})}
        };


    let sln = solve(&word_list, solver);
    println!("Solution: {:?}", sln.guess_sequence);
}

fn non_interactive_solver(guess :&str, answer : String) -> GuessResponse
{
    GuessResponse{ is_correct: guess == answer }
}

struct GuessResponse
{
    is_correct : bool,
}

fn is_guess_correct(response: &GuessResponse) -> bool
{
    response.is_correct
}

fn solve<T>(possbile_words : &Vec<String>, verifier : T) -> Solution
where T: Fn(&str) -> GuessResponse{
    let knowledge = Knowledge{ guessed_words: vec![] };
    solve_rec(possbile_words, verifier, &knowledge)
}

fn solve_rec<T>(possbile_words : &Vec<String>, verifier : T, starting_knowledge : &Knowledge) -> Solution
    where T: Fn(&str) -> GuessResponse{
    let guess  = make_guess(possbile_words, starting_knowledge);
    let response = verifier(&guess);
    let new_knowledge = apply_learning(starting_knowledge, &guess, &response);

    match is_guess_correct(&response)
    {
        true => { Solution{ guess_sequence: new_knowledge.guessed_words } }
        false => { solve_rec(possbile_words, verifier, &new_knowledge) }
    }
}

fn apply_learning(knowledge:&Knowledge, guess:&String, _: &GuessResponse) -> Knowledge
{
    let mut new_words = knowledge.guessed_words.clone();
    new_words.push(guess.to_string());
    Knowledge {
        guessed_words: new_words
    }
}

fn make_guess(possbile_words : &Vec<String>, knowledge:&Knowledge) -> String
{
    possbile_words.iter()
        .find(|w| !knowledge.guessed_words.contains(w))
        .expect("Surely must be something to guess")
        .to_string()
}

fn read_word_list(word_list_file_name : String) -> Vec<String>
{
    fs::read_to_string(word_list_file_name).expect("Error reading file")
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

