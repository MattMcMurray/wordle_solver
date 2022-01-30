use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process,
};

use clap::Parser;

mod wordle;

use wordle::Guess;
use wordle::Wordle;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The relative path to the dictionary/wordlist
    wordlist: Option<String>,

    /// The starting word
    first_guess: Option<String>,

    /// The solution
    target: Option<String>,
}

fn main() {
    let args = Args::parse();

    let wordlist_path = args.wordlist.unwrap_or_else(|| {
        println!("Missing `wordlist` arg");
        process::exit(1);
    });
    let first_guess = args.first_guess.unwrap_or_else(|| {
        println!("Missing `first_guess` arg");
        process::exit(1);
    });
    let target = args.target.unwrap_or_else(|| {
        println!("Missing `target` arg");
        process::exit(1);
    });

    let config = Config::new(wordlist_path, first_guess, target);
    let lines = read_lines_from_file(Path::new(&config.wordfile));
    println!("Read {} words from {}", lines.len(), config.wordfile);

    let mut wordle = Wordle::new(lines);

    let init_guess = Guess {
        guess: config.init_guess.clone(),
        result: wordle::check_guess(&config.init_guess, &config.target),
    };
    println!("Initial guess: {}", &init_guess.guess);
    println!("Result: {}", &init_guess.get_formatted_result());

    let dict_size_before = wordle.dictionary.len();
    wordle.add_guess(init_guess);

    let dict_size_after = wordle.dictionary.len();
    println!(
        "Removed {} words from dict after first guess",
        dict_size_before - dict_size_after
    );
    println!(
        "There are {} words remaining after {} guess(es)",
        &wordle.dictionary.len(),
        &wordle.guesses.len()
    );

    while &wordle.dictionary.len() > &0 {
        let next_word = wordle::choose_next_guess(&wordle.dictionary);
        let next_guess = Guess {
            guess: next_word.clone(),
            result: wordle::check_guess(&next_word, &config.target),
        };

        println!("Next guess: {}", &next_guess.guess);
        println!("Result: {}", &next_guess.get_formatted_result());

        let dict_size_before = wordle.dictionary.len();
        wordle.add_guess(next_guess);

        if wordle.is_solved() {
            break
        }

        let dict_size_after = wordle.dictionary.len();
        println!(
            "Removed {} words from dict after guess: {}",
            dict_size_before - dict_size_after,
            &wordle.guesses.len(),
        );
        println!(
            "There are {} words remaining after {} guess(es)",
            &wordle.dictionary.len(),
            &wordle.guesses.len()
        );
        println!();
    }

    if wordle.is_solved() {
        println!("The correct word is {:?}.", &wordle.guesses.last().unwrap().guess);
        println!("It took {:?} guesses to find it.", &wordle.guesses.len());
    }
}

struct Config {
    wordfile: String,
    init_guess: String,
    target: String,
}

impl Config {
    fn new(wordfile: String, init_guess: String, target: String) -> Config {
        Config {
            wordfile,
            init_guess,
            target,
        }
    }
}

fn read_lines_from_file(filename: &Path) -> Vec<String> {
    let file = File::open(&filename).unwrap_or_else(|_| panic!("No such file"));

    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
