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

    const WORD_SIZE: usize = 5; // word size
    const MAX_GUESSES: usize = 6; // number of guesses

    const EMPTY : char = '\u{2B1B}';

    let filename: &str = "./words.txt";
    let corpus: Vec<String> = load_corpus(filename);
    
    let possible_words: Vec<String> = corpus.clone()
                                            .into_iter()
                                            .filter(|x| x.chars().count() == WORD_SIZE)
                                            .collect();
    let hash: HashMap<_, _> = possible_words.clone()
                                            .into_iter()
                                            .map(|x| (x, 1))
                                            .collect();
    
    println!("Let's play Wordle!");
    println!("You have {} tries to guess a {} letter english word.", MAX_GUESSES, WORD_SIZE);

    loop { // Start a new game!
        let mut win: bool = false;

        // randomly select answer 
        let answer: &String = choose_word(&possible_words);
        // store guesses in mutable vector
        let mut guesses: Vec<String> = Vec::new();
        let mut score_board: Vec<Vec<char>> = vec![vec![EMPTY as char; WORD_SIZE]; MAX_GUESSES];
        print_score(&score_board, &guesses, &MAX_GUESSES);

        let mut alphabet : Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        print_available_chars(&alphabet);

        for i in 0..MAX_GUESSES {
            println!("\nGuess a word:\t");
            let mut guess: String;
            
            loop { // loop until we get a valid guess
                // read in guess
                let mut input: String = String::new();
                io::stdin().read_line(&mut input)
                           .expect("Failed to read line");
                guess = input.trim().to_lowercase();

                if hash.contains_key(&guess) { break; } 
                else { println!("\nThat's not a valid word. Try again..."); }
            }
            
            score_board[i] = grade(&guess, &answer);
            alphabet.retain(|c| !guess.contains(*c) );
            guesses.push(guess);
            
            print_score(&score_board, &guesses, &MAX_GUESSES);
            print_available_chars(&alphabet);
            
            if guesses[i] == *answer {
                win = true;
                break;
            }

        }

        if win {println!("\nCongratulations, you win!");}
        else {println!("\nToo bad! The correct word was {}.", answer.to_uppercase());}

        println!("\nPlay again? (Y/n)");
        let mut again : String = String::new();
        io::stdin().read_line(&mut again)
                   .expect("Failed to read line");

        if again.trim().to_lowercase().chars().next().unwrap() == 'y' {
            continue;
        } else {
            break;
        }
    }
}

fn grade(guess: &String, answer: &String) -> Vec<char> {
    let (good, lost, bad) = ('\u{1F7E9}', '\u{1F7E8}', '\u{1F7E5}');
    let guess_chars: Vec<_> = guess.chars().collect();
    let ans_chars: Vec<_> = answer.chars().collect();
    let mut score: Vec<char> = vec![' '; answer.len()];
    for c in 0..answer.len() {
        let guess_char : char = guess_chars[c];
        let ans_char : char = ans_chars[c];

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
    println!("");
    let n_guesses : usize = guesses.len(); 
    let word_size : usize = score_board[0].len();
    for row in 0..*max_guesses {
        for col in 0..word_size {
            print!("{}", score_board[row][col]);
        }
        if row < n_guesses { print!("  {}", guesses[row].to_uppercase()); }
        print!("\n");
    }
}

fn print_available_chars(characters: &Vec<char>) {
    println!("\nUnused characters:");
    for c in characters{
        print!("{} ", c);
    }
    println!();
}

fn choose_word(word_list: &Vec<String>) -> &String {
    word_list.choose(&mut rand::thread_rng()).unwrap()
}

fn load_corpus(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}
