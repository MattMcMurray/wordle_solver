use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process,
};

const GREEN_SQUARE: char = '🟩';
const WHITE_SQUARE: char = '⬜';
const YELLOW_SQUARE: char = '🟨';

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
        result: check_guess(&config.init_guess, &config.target),
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
        let next_word = wordle.dictionary.first().unwrap();
        let next_guess = Guess {
            guess: next_word.clone(),
            result: check_guess(&next_word, &config.target),
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

struct Wordle {
    guesses: Vec<Guess>,
    dictionary: Vec<String>,
    incorrect_letters: Vec<char>,
    correct_letters: Vec<(char, u32)>,
    misplaced_letters: Vec<char>,
}

impl Wordle {
    fn new(dictionary: Vec<String>) -> Wordle {
        Wordle {
            guesses: vec![],
            dictionary: dictionary,
            incorrect_letters: vec![],
            correct_letters: vec![], // TODO: this probably needs to be a hash map? or use tuples?
            misplaced_letters: vec![],
        }
    }

    fn add_guess(&mut self, guess: Guess) {
        self.guesses.push(guess);

        let g: &Guess = self.guesses.last().unwrap();

        for (i, c) in g.guess.chars().enumerate() {
            if matches!(g.result[i], Correctness::Correct) {
                self.correct_letters.push((c, i.try_into().unwrap()));
            } else if matches!(g.result[i], Correctness::IncorrectPlacement) {
                self.misplaced_letters.push(c);
            } else {
                self.incorrect_letters.push(c)
            }
        }

        self.dictionary.retain(|word| {
            filter_dictionary(
                word,
                &self.incorrect_letters,
                &self.misplaced_letters,
                &self.correct_letters,
            ) && word != &g.guess
        });
    }

    fn is_solved(&self) -> bool {
        let last_guess = self.guesses.last().unwrap();

        for r in &last_guess.result {
            if !matches!(r, Correctness::Correct) {
                return false;
            }
        }

        true
    }
}

fn filter_dictionary(
    word: &String,
    incorrect_letters: &Vec<char>,
    misplaced_letters: &Vec<char>,
    correct_letters: &Vec<(char, u32)>,
) -> bool {
    for c in incorrect_letters {
        if word.contains(*c) {
            return false;
        }
    }

    for c in misplaced_letters {
        if !word.contains(*c) {
            return false;
        }
    }

    for (c, i) in correct_letters {
        if word.chars().nth(*i as usize).unwrap() != *c {
            return false;
        }
    }

    true
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

struct Guess {
    guess: String,
    result: Vec<Correctness>,
}

impl Guess {
    fn get_formatted_result(&self) -> String {
        let mut result: String = String::new();

        for r in &self.result {
            if matches!(r, Correctness::Correct) {
                result.push(GREEN_SQUARE);
            } else if matches!(r, Correctness::IncorrectPlacement) {
                result.push(YELLOW_SQUARE);
            } else {
                result.push(WHITE_SQUARE);
            }
        }

        result
    }
}

fn read_lines_from_file(filename: &Path) -> Vec<String> {
    let file = File::open(&filename).unwrap_or_else(|_| panic!("No such file"));

    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Copy, Clone)]
enum Correctness {
    Correct,
    IncorrectPlacement,
    Incorrect,
}

fn check_guess(guess: &String, word: &String) -> Vec<Correctness> {
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
    fn it_should_return_all_correct() {
        let result = check_guess(&String::from("salty"), &String::from("salty"));
        for r in result {
            assert!(matches!(r, Correctness::Correct))
        }
    }

    #[test]
    fn it_should_return_all_incorrect() {
        let result = check_guess(&String::from("skirt"), &String::from("lynch"));
        for r in result {
            assert!(matches!(r, Correctness::Incorrect))
        }
    }

    #[test]
    fn it_should_return_correct_mixed_results() {
        let result = check_guess(&String::from("skirt"), &String::from("shirt"));
        assert!(matches!(result[0], Correctness::Correct));
        assert!(matches!(result[1], Correctness::Incorrect));
        assert!(matches!(result[2], Correctness::Correct));
        assert!(matches!(result[3], Correctness::Correct));
        assert!(matches!(result[4], Correctness::Correct));
    }

    #[test]
    fn it_should_render_a_correct_result_string() {
        let guess = Guess {
            guess: String::from("testing"),
            result: vec![
                Correctness::Correct,
                Correctness::Incorrect,
                Correctness::Correct,
                Correctness::IncorrectPlacement,
                Correctness::Incorrect,
            ],
        };

        let expected_result = format!(
            "{}{}{}{}{}",
            GREEN_SQUARE, WHITE_SQUARE, GREEN_SQUARE, YELLOW_SQUARE, WHITE_SQUARE
        );

        assert_eq!(guess.get_formatted_result(), expected_result);
    }

    #[test]
    fn it_should_not_filter_the_word_if_no_incorrect_letters() {
        let word = String::from("hello");
        let incorrect_letters = vec!['a'];

        assert!(filter_dictionary(
            &word,
            &incorrect_letters,
            &vec!(),
            &vec!()
        ));
    }

    #[test]
    fn it_should_filter_the_word_if_it_contains_incorrect_letters() {
        let word = String::from("hello");
        let incorrect_letters = vec!['o'];

        assert!(!filter_dictionary(
            &word,
            &incorrect_letters,
            &vec!(),
            &vec!()
        ));
    }

    #[test]
    fn it_should_filter_the_word_if_it_does_not_contain_the_misplaced_letter() {
        let word = String::from("hello");
        let misplaced_letters = vec!['a'];

        assert!(!filter_dictionary(
            &word,
            &vec!(),
            &misplaced_letters,
            &vec!()
        ))
    }

    #[test]
    fn it_should_not_filter_the_word_if_it_does_not_contain_the_misplaced_letter() {
        let word = String::from("hello");
        let misplaced_letters = vec!['l'];

        assert!(filter_dictionary(
            &word,
            &vec!(),
            &misplaced_letters,
            &vec!()
        ))
    }

    #[test]
    fn it_should_filter_the_word_if_it_does_not_have_correctly_placed_lettrr() {
        let word = String::from("hello");
        let correct_letters = vec![('a', 1)];

        assert!(!filter_dictionary(
            &word,
            &vec!(),
            &vec!(),
            &correct_letters
        ))
    }

    #[test]
    fn it_should_not_filter_the_word_if_it_does_not_have_correctly_placed_lettrr() {
        let word = String::from("hello");
        let correct_letters = vec![('e', 1)];

        assert!(filter_dictionary(
            &word,
            &vec!(),
            &vec!(),
            &correct_letters
        ));
    }
}
