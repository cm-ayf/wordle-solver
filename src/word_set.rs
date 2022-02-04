use std::collections::HashMap;
use std::collections::HashSet;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct WordSet {
  data: HashSet<Word>,
}

impl WordSet {
  pub fn from_file(path: &'static str) -> Self {
    let s = std::fs::read_to_string(path).unwrap();
    let mut data = HashSet::new();

    for word in s.split(',') {
      data.insert(word.parse().unwrap());
    }

    Self { data }
  }

  pub fn filter(&mut self, word: &Word, status: &Status) -> usize {
    let mut to_be_removed = Vec::new();
    for w in self.data.iter() {
      if &word.status(&w) != status {
        to_be_removed.push(w.clone());
      }
    }

    for w in to_be_removed {
      self.data.remove(&w);
    }

    self.data.len()
  }

  pub fn answer(&self) -> Option<&Word> {
    match self.data.len() {
      1 => self.data.iter().next(),
      _ => None,
    }
  }

  pub fn suggest<'a>(&self, available: &'a Self) -> &'a Word {
    let mut iter = available.data.iter();
    let mut word = iter.next().unwrap();
    let mut max = available.data.len();

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

    for w in &self.data {
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

  impl WordSet {
    fn new() -> Self {
      Self {
        data: HashSet::new(),
      }
    }
  }

  #[test]
  fn full() {
    let set = WordSet::from_file("data/candidates");
    assert_eq!(set.data.len(), 2315);
  }

  #[test]
  fn filter_1() {
    let mut set = WordSet::from_file("data/candidates");
    let mut ans = HashSet::new();
    let word = "hello".parse().unwrap();

    assert_eq!(set.filter(&word, &"ggggg".parse().unwrap()), 1);
    ans.insert(word);

    let ans = WordSet { data: ans };

    assert_eq!(set, ans);
  }

  #[test]
  fn filter_2() {
    let mut set = WordSet::from_file("data/candidates");
    let word1 = "hello".parse().unwrap();
    let word2 = "rusty".parse().unwrap();
    assert_eq!(set.filter(&word1, &"_____".parse().unwrap()), 406);
    assert!(!set.data.contains(&word1));
    assert!(set.data.contains(&word2));
  }

  #[test]
  fn answer_1() {
    let set = WordSet::new();
    assert_eq!(set.answer(), None);
  }

  #[test]
  fn answer_2() {
    let mut set = WordSet::new();
    let word: Word = "hello".parse().unwrap();
    set.data.insert(word.clone());
    assert_eq!(set.answer(), Some(&word));
  }

  #[test]
  fn answer_3() {
    let mut set = WordSet::new();
    let word1: Word = "hello".parse().unwrap();
    let word2: Word = "world".parse().unwrap();
    set.data.insert(word1);
    set.data.insert(word2);
    assert_eq!(set.answer(), None);
  }
}
