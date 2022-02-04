use super::*;

/// class that holds all states to solve.
pub struct Solver {
  available: WordSet,
  set: WordSet,
  queries: Vec<Word>,
}

impl Solver {
  /// create new instance.
  pub fn new() -> Self {
    let available = WordSet::from_file("data/queries");
    let set = WordSet::from_file("data/candidates");

    Self {
      available,
      set,
      queries: Vec::new(),
    }
  }

  /// start solving.
  /// 
  /// returns first query.
  /// 
  /// might take time up to 1.0 second.
  pub fn start(&mut self) -> String {
    let query = self.set.suggest(&self.available);
    self.queries.push(query.clone());

    query.to_string()
  }

  /// receives status from last query.
  /// * has to be called only after `start` was called.
  /// * `status` should be text of 5 characters chosen from:
  ///   * `G` (`g`): Green color (match on exact position)
  ///   * `Y` (`y`): Yellow color (match on other position)
  ///   * `_`: Gray color (no match)
  /// 
  /// * returns:
  ///   * 0: next word to be queried
  ///   * 1: if the word is the answer
  /// 
  /// Panics if this method is called before `start` has been.
  /// 
  /// Errors if `status` was invalid.
  /// 
  /// might take time up to 0.10 second.
  pub fn next(&mut self, status: &str) -> Result<(String, bool), String> {
    let word = self.queries.last().expect("call start before next");
    self.set.filter(word, &status.parse()?);

    if let Some(answer) = self.set.answer() {
      return Ok((answer.to_string(), true));
    }

    let query = self.set.suggest(&self.available);
    self.queries.push(query.clone());

    Ok((query.to_string(), false))
  }
}
