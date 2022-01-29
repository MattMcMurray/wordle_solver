use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = read_lines_from_file(Path::new(&config.wordfile));

    println!("Read {} words from {}", lines.len(), config.wordfile)
}

struct Config {
    wordfile: String,
    init_guess: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        const NUM_ARGS: usize = 3;

        if args.len() < NUM_ARGS {
            return Err("not enough arguments");
        }

        let wordfile = args[1].clone();
        let init_guess = args[2].clone();

        Ok(Config {
            wordfile,
            init_guess,
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

enum Correctness {
    Correct,
    IncorrectPlacement,
    Incorrect,
}

fn check_guess(guess: &str, word: &str) -> Vec<Correctness> {
    let guess_chars: Vec<_> = guess.chars().collect();
    let word_chars: Vec<_> = word.chars().collect();

    let mut correctness: Vec<Correctness> = Vec::new();

    for i in 0..guess_chars.len() {
        if guess_chars[i] == word_chars[i] {
            correctness.push(Correctness::Correct);
        } else if word.contains(guess_chars[i]) {
            // TODO: (maybe) need to account for case where letter is correctly placed elsewhere:
            // e.g., double-letter word guesses
            correctness.push(Correctness::IncorrectPlacement);
        } else {
            correctness.push(Correctness::Incorrect);
        }
    }

    correctness
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_should_return_all_correct() {
        let result = check_guess("salty", "salty");
        for r in result {
            assert!(matches!(r, Correctness::Correct))
        }
    }

    #[test]
    fn it_should_return_all_incorrect() {
        let result = check_guess("skirt", "lynch");
        for r in result {
            assert!(matches!(r, Correctness::Incorrect))
        }
    }

    #[test]
    fn it_should_return_correct_mixed_results() {
        let result = check_guess("skirt", "shirt");
        assert!(matches!(result[0], Correctness::Correct));
        assert!(matches!(result[1], Correctness::Incorrect));
        assert!(matches!(result[2], Correctness::Correct));
        assert!(matches!(result[3], Correctness::Correct));
        assert!(matches!(result[4], Correctness::Correct));
    }
}
