mod game;
mod cli;
mod interactive_solver;
mod non_interactive_solver;
mod player;

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
