use rand::seq::SliceRandom;
use std::collections::HashSet;

pub const GREEN_SQUARE: char = '🟩';
pub const WHITE_SQUARE: char = '⬜';
pub const YELLOW_SQUARE: char = '🟨';

pub struct Wordle {
  pub guesses: Vec<Guess>,
  pub dictionary: Vec<String>,
  pub incorrect_letters: Vec<char>,
  pub correct_letters: Vec<(char, u32)>,
  pub misplaced_letters: Vec<char>,
}

impl Wordle {
  pub fn new(dictionary: Vec<String>) -> Wordle {
    Wordle {
      guesses: vec![],
      dictionary: dictionary,
      incorrect_letters: vec![],
      correct_letters: vec![],
      misplaced_letters: vec![],
    }
  }

  pub fn add_guess(&mut self, guess: Guess) {
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

  pub fn is_solved(&self) -> bool {
    let last_guess = self.guesses.last().unwrap();

    for r in &last_guess.result {
      if !matches!(r, Correctness::Correct) {
        return false;
      }
    }

    true
  }
}

pub struct Guess {
  pub guess: String,
  pub result: Vec<Correctness>,
}

impl Guess {
  pub fn get_formatted_result(&self) -> String {
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

#[derive(Copy, Clone)]
pub enum Correctness {
  Correct,
  IncorrectPlacement,
  Incorrect,
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

// TODO: encapsulate this and make it private (?)
pub fn check_guess(guess: &String, word: &String) -> Vec<Correctness> {
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

fn has_double_letter(word: &String) -> bool {
  let mut set = HashSet::new();

  for c in word.chars() {
    if set.contains(&c) {
      return true;
    } else {
      set.insert(c);
    }
  }

  return false;
}

pub fn choose_next_guess(dict: &Vec<String>) -> &String {
  let mut num_choices = 0;

  loop {
    let mut rng = rand::thread_rng();
    let choice = dict.choose(&mut rng).unwrap();

    num_choices = num_choices + 1;

    if dict.len() < 10 || !has_double_letter(choice) || num_choices > 4 {
      return choice;
    }
  }
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
  fn it_should_filter_the_word_if_it_does_not_have_correctly_placed_letter() {
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
  fn it_should_not_filter_the_word_if_it_does_not_have_correctly_placed_letter() {
    let word = String::from("hello");
    let correct_letters = vec![('e', 1)];

    assert!(filter_dictionary(&word, &vec!(), &vec!(), &correct_letters));
  }

  #[test]
  fn it_should_return_true_if_the_word_contains_double_letters() {
    assert!(has_double_letter(&String::from("hello")))
  }

  #[test]
  fn it_should_return_false_if_the_word_does_not_contain_double_letters() {
    assert!(!has_double_letter(&String::from("friend")))
  }
}
