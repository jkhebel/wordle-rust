use rand::seq::SliceRandom;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = "./en_10000.txt";
    let corpus: Vec<_> = load_corpus(filename);

    let word_size: usize = 5;

    let possible_words: Vec<_> = corpus
        .into_iter()
        .filter(|x| x.chars().count() == word_size)
        .collect();
    let hash: HashMap<_, _> = possible_words.clone().into_iter().map(|x| (x, 1)).collect();

    let answer: &String = possible_words.choose(&mut rand::thread_rng()).unwrap();

    println!("{:?}", hash.contains_key(answer));
}

fn load_corpus(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}
