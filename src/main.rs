use rand::seq::SliceRandom;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    // TODO get lang --> keep it in english for now
    // TODO prompt for number of letters --> start w 5
    // TODO prompt for n of tries --> start w 6, -1 means infinite

    // game parameters
    const WORD_SIZE: usize = 5; // word size
    const MAX_GUESSES: usize = 6; // number of guesses

    // store glyph for empty cell
    const EMPTY : char = '\u{2B1B}';

    // load word list
    let filename: &str = "./words.txt";
    let corpus: Vec<String> = load_corpus(filename);
    
    // vectorize and hash word list
    let possible_words: Vec<String> = corpus.clone()
                                            .into_iter()
                                            .filter(|x| x.chars().count() == WORD_SIZE)
                                            .collect();
    let hash: HashMap<_, _> = possible_words.clone()
                                            .into_iter()
                                            .map(|x| (x, 1))
                                            .collect();

    // Start up Wordle
    println!("Let's play Wordle!");
    println!("You have {} tries to guess a {} letter english word.", MAX_GUESSES, WORD_SIZE);

    loop { // Start a new game!
        let mut win: bool = false;
        // randomly select answer 
        let answer: &String = choose_word(&possible_words);
        // store guesses in mutable vector
        let mut guesses: Vec<String> = Vec::new();
        // generate scoreboard
        let mut score_board: Vec<Vec<char>> = vec![vec![EMPTY as char; WORD_SIZE]; MAX_GUESSES];
        print_score(&score_board, &guesses, &MAX_GUESSES);
        // store unused letters
        let mut alphabet : Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        print_available_chars(&alphabet);

        // loop until win or out of guesses
        for i in 0..MAX_GUESSES {
            // read  in user guess word
            println!("\nGuess a word:\t");
            let mut guess : String = input_guess(&hash);
            
            // grade the guess
            score_board[i] = grade(&guess, &answer);
            // remove guessed letters from unused letters
            alphabet.retain(|c| !guess.contains(*c) );
            // store guess in vector
            guesses.push(guess);
            
            // display game status
            print_score(&score_board, &guesses, &MAX_GUESSES);
            print_available_chars(&alphabet);
            
            // win condition
            if guesses[i] == *answer {
                win = true;
                break;
            }

        }

        // Display win/loss text
        if win {println!("\nCongratulations, you win!");}
        else {println!("\nToo bad! The correct word was {}.", answer.to_uppercase());}

        // Prompt for a new game
        println!("\nPlay again? (Y/n)");
        let mut again : String = String::new();
        io::stdin().read_line(&mut again)
                   .expect("Failed to read line");
        // Continue (new game) if input starts with 'y'
        if again.trim().to_lowercase().chars().next().unwrap() == 'y' { continue; } 
        else { break; }
    }
}

fn input_guess(viable_words: &HashMap<_,_>) -> String {
    // read in user guess and validate word is valid
    loop { // loop until we get a valid guess
        // read in guess
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Failed to read line");
        let guess : String = input.trim().to_lowercase();
        // validate input word
        if viable_words.contains_key(&guess) { return guess; } 
        else { println!("\nThat's not a valid word. Try again..."); }
    }
}

fn grade(guess: &String, answer: &String) -> Vec<char> {
    // grade the current guess, returning a row for the scoreboard
    let (good, lost, bad) = ('\u{1F7E9}', '\u{1F7E8}', '\u{1F7E5}'); // glyphs
    // convert strings to vec<char>
    let guess_chars: Vec<_> = guess.chars().collect();
    let ans_chars: Vec<_> = answer.chars().collect();
    // store corresponding glyphs in new vec<char>
    let mut score: Vec<char> = vec![' '; answer.len()];
    for c in 0..answer.len() {
        // for each column, get guess + answer characters
        let guess_char : char = guess_chars[c];
        let ans_char : char = ans_chars[c];
        // grade each guess char as good, bad, or lost
        if guess_char == ans_char {
            score[c] = good;
        } else if answer.contains(guess_char) {
            score[c] = lost;
        } else {
            score[c] = bad;
        }
    }
    return score;

}

fn print_score(score_board: &Vec<Vec<char>>, guesses: &Vec<String>, max_guesses: &usize) {
    // print out and format the scoreboard
    println!("");
    let n_guesses : usize = guesses.len(); 
    let word_size : usize = score_board[0].len();
    for row in 0..*max_guesses { // for each row
        for col in 0..word_size { // print out glyph for each cell
            print!("{}", score_board[row][col]);
        }
        // If there's a guess, print it on it's corresponding row
        if row < n_guesses { print!("  {}", guesses[row].to_uppercase()); }
        print!("\n");
    }
}

fn print_available_chars(characters: &Vec<char>) {
    // print out unguessed characters
    println!("\nUnused characters:");
    for c in characters{ print!("{} ", c.to_uppercase()); }
    println!();
}

fn choose_word(word_list: &Vec<String>) -> &String {
    // choose a word from the word list
    word_list.choose(&mut rand::thread_rng()).unwrap()
}

fn load_corpus(filename: &str) -> Vec<String> {
    // load word list from file name
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}
