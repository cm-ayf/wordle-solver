use std::collections::HashMap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct WordSet {
  data: Vec<Word>,
}

impl WordSet {
  fn from_vec(vec: Vec<&'static str>) -> Self {
    let data = vec.iter().map(|str| str.parse().unwrap()).collect();
    Self { data }
  }

  pub fn candidates() -> Self {
    Self::from_vec(wordle_solver_data::candidates())
  }

  pub fn queries() -> Self {
    Self::from_vec([wordle_solver_data::queries(), wordle_solver_data::candidates()].concat())
  }

  pub fn filter(&mut self, word: &Word, status: &Status) -> usize {
    let mut to_be_removed = Vec::new();
    for (u, w) in self.data.iter().enumerate() {
      if &word.status(&w) != status {
        to_be_removed.push(u);
      }
    }

    to_be_removed.reverse();

    for u in to_be_removed {
      self.data.remove(u);
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
        Some(w) => {
          if let Some(u) = self.matches_less_than(w, max) {
            word = w;
            max = u;
          }
        }
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
      Self { data: Vec::new() }
    }
  }

  #[test]
  fn full() {
    let set = WordSet::candidates();
    assert_eq!(set.data.len(), 2315);
  }

  #[test]
  fn filter_1() {
    let mut set = WordSet::candidates();
    let mut ans = Vec::new();
    let word = "hello".parse().unwrap();

    assert_eq!(set.filter(&word, &"ggggg".parse().unwrap()), 1);
    ans.push(word);

    let ans = WordSet { data: ans };

    assert_eq!(set, ans);
  }

  #[test]
  fn filter_2() {
    let mut set = WordSet::candidates();
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
    set.data.push(word.clone());
    assert_eq!(set.answer(), Some(&word));
  }

  #[test]
  fn answer_3() {
    let mut set = WordSet::new();
    let word1: Word = "hello".parse().unwrap();
    let word2: Word = "world".parse().unwrap();
    set.data.push(word1);
    set.data.push(word2);
    assert_eq!(set.answer(), None);
  }
}
