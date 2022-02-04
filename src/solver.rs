use std::error::Error;
use std::fmt::Display;
use wasm_bindgen::prelude::*;
use super::*;

/// class that holds all states to solve.
#[wasm_bindgen]
pub struct Solver {
  available: WordSet,
  set: WordSet,
  queries: Vec<Word>,
  answer: Option<Word>,
}

#[derive(Debug)]
pub enum SolverError {
  ParseStatusError(status::ParseStatusError),
  NoWordsLeft,
}

use SolverError::*;

impl From<status::ParseStatusError> for SolverError {
  fn from(e: status::ParseStatusError) -> Self {
    ParseStatusError(e)
  }
}

impl Display for SolverError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParseStatusError(e) => write!(f, "{e}"),
      NoWordsLeft => write!(f, "no words left"),
    }
  }
}

impl Error for SolverError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      ParseStatusError(e) => Some(e),
      NoWordsLeft => None,
    }
  }
}

impl Into<JsValue> for SolverError {
  fn into(self) -> JsValue {
    JsValue::from_str(&self.to_string())
  }
}

#[wasm_bindgen]
impl Solver {
  /// create new instance.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let available = WordSet::queries();
    let set = WordSet::candidates();

    Self {
      available,
      set,
      queries: Vec::new(),
      answer: None,
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
  ///   * `_` (` `): Gray color (no match)
  ///
  /// * returns next word to be queried
  ///   * if not finished, the word that can provide good information
  ///   * if finished, answer
  ///
  /// Panics if this method is called before `start` has been.
  ///
  /// Errors if `status` was invalid.
  ///
  /// might take time up to 0.10 second.
  pub fn next(&mut self, status: &str) -> Result<String, SolverError> {
    let word = self.queries.last().expect("call start before next");

    if self.set.filter(word, &status.parse()?) == 0 {
      return Err(NoWordsLeft);
    };

    if let Some(answer) = self.set.answer() {
      self.answer = Some(answer.clone());
      return Ok(answer.to_string());
    }

    let query = self.set.suggest(&self.available);
    self.queries.push(query.clone());

    Ok(query.to_string())
  }

  /// returns if solver has answer.
  pub fn finished(&self) -> bool {
    self.answer.is_some()
  }

  /// returns answer if solver has it.
  pub fn answer(&self) -> Option<String> {
    self.answer.as_ref().map(|w| w.to_string())
  }
}
