use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process,
};

mod wordle;

use wordle::Guess;
use wordle::Wordle;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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

    while !&wordle.is_solved() && &wordle.dictionary.len() > &0 {
        let next_word = wordle::choose_next_guess(&wordle.dictionary);
        let next_guess = Guess {
            guess: next_word.clone(),
            result: wordle::check_guess(&next_word, &config.target),
        };

        println!("Next guess: {}", &next_guess.guess);
        println!("Result: {}", &next_guess.get_formatted_result());

        let dict_size_before = wordle.dictionary.len();
        wordle.add_guess(next_guess);
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
    }
}

struct Config {
    wordfile: String,
    init_guess: String,
    target: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        const NUM_ARGS: usize = 3;

        if args.len() < NUM_ARGS {
            return Err("not enough arguments");
        }

        let wordfile = args[1].clone();
        let init_guess = args[2].clone();
        let target = args[3].clone();

        Ok(Config {
            wordfile,
            init_guess,
            target,
        })
    }
}

fn read_lines_from_file(filename: &Path) -> Vec<String> {
    let file = File::open(&filename).unwrap_or_else(|_| panic!("No such file"));

    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
