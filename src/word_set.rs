use std::collections::HashSet;

use super::*;

pub trait WordSet {
  fn full() -> Self;
  fn suggest(&self) -> &Word;
  fn filter(&mut self, word: &Word, status: &Status) -> usize;
  fn answer(&self) -> Option<&Word>;
}

impl WordSet for HashSet<Word> {
  fn full() -> Self {
    let s = std::fs::read_to_string("data/wordlist").unwrap();
    let mut set = HashSet::new();

    for word in s.split(',') {
      set.insert(word.parse().unwrap());
    }

    set
  }

  fn suggest(&self) -> &Word {
    todo!()
  }

  fn filter(&mut self, word: &Word, status: &Status) -> usize {
    let mut to_be_removed = Vec::new();
    for w in self.iter() {
      if &word.status(&w) != status {
        to_be_removed.push(w.clone());
      }
    }

    for w in to_be_removed {
      self.remove(&w);
    }

    self.len()
  }

  fn answer(&self) -> Option<&Word> {
    match self.len() {
      1 => self.iter().next(),
      _ => None,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn full() {
    let set = HashSet::full();
    assert_eq!(set.len(), 2315);
  }

  #[test]
  fn filter_1() {
    let mut set = HashSet::full();
    let mut ans = HashSet::new();
    let word = "hello".parse().unwrap();

    assert_eq!(set.filter(&word, &"ggggg".parse().unwrap()), 1);
    ans.insert(word);

    assert_eq!(set, ans);
  }

  #[test]
  fn filter_2() {
    let mut set = HashSet::full();
    let word1 = "hello".parse().unwrap();
    let word2 = "rusty".parse().unwrap();
    assert_eq!(set.filter(&word1, &"_____".parse().unwrap()), 406);
    assert!(!set.contains(&word1));
    assert!(set.contains(&word2));
  }

  #[test]
  fn answer_1() {
    let set = HashSet::new();
    assert_eq!(set.answer(), None);
  }

  #[test]
  fn answer_2() {
    let mut set = HashSet::new();
    let word: Word = "hello".parse().unwrap();
    set.insert(word.clone());
    assert_eq!(set.answer(), Some(&word));
  }

  #[test]
  fn answer_3() {
    let mut set = HashSet::new();
    let word1: Word = "hello".parse().unwrap();
    let word2: Word = "world".parse().unwrap();
    set.insert(word1);
    set.insert(word2);
    assert_eq!(set.answer(), None);
  }
}
