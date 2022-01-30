use std::collections::HashSet;

pub fn filter_dictionary(
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

pub fn has_double_letter(word: &String) -> bool {
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

mod tests {
  #[test]
  fn it_should_not_filter_the_word_if_no_incorrect_letters() {
    let word = String::from("hello");
    let incorrect_letters = vec!['a'];

    assert!(super::filter_dictionary(
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

    assert!(!super::filter_dictionary(
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

    assert!(!super::filter_dictionary(
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

    assert!(super::filter_dictionary(
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

    assert!(!super::filter_dictionary(
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

    assert!(super::filter_dictionary(&word, &vec!(), &vec!(), &correct_letters));
  }

  
  #[test]
  fn it_should_return_true_if_the_word_contains_double_letters() {
    assert!(super::has_double_letter(&String::from("hello")))
  }

  #[test]
  fn it_should_return_false_if_the_word_does_not_contain_double_letters() {
    assert!(!super::has_double_letter(&String::from("friend")))
  }
}