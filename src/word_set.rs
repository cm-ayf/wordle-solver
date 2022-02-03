use std::collections::HashMap;
use std::collections::HashSet;

use super::*;

pub trait WordSet {
  fn full() -> Self;
  fn filter(&mut self, word: &Word, status: &Status) -> usize;
  fn answer(&self) -> Option<&Word>;
  fn suggest<'a>(&self, full_set: &'a Self) -> &'a Word;
  fn matches_less_than(&self, word: &Word, limit: usize) -> Option<usize>;
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

  fn suggest<'a>(&self, full_set: &'a Self) -> &'a Word {
    let mut iter = full_set.iter();
    let mut word = iter.next().unwrap();
    let mut max = full_set.len();

    loop {
      match iter.next() {
        Some(w) => if let Some(u) = self.matches_less_than(w, max) {
          word = w;
          max = u;
        },
        None => break,
      }
    }

    word
  }

  fn matches_less_than(&self, word: &Word, limit: usize) -> Option<usize> {
    let mut map: HashMap<Status, usize> = HashMap::new();

    for w in self {
      let status = w.status(word);
      if let Some(u) = map.get_mut(&status) {
        *u += 1;
        if *u > limit {
          return None;
        }
      } else {
        map.insert(status, 1);
      }
    }

    map
      .into_values()
      .reduce(|u1, u2| if u1 < u2 { u2 } else { u1 })
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
