use super::*;

pub struct Solver {
  available: WordSet,
  set: WordSet,
  queries: Vec<Word>,
}

impl Solver {
  pub fn new() -> Self {
    let available = WordSet::from_file("data/queries");
    let set = WordSet::from_file("data/candidates");

    Self {
      available,
      set,
      queries: Vec::new(),
    }
  }

  pub fn start(&mut self) -> String {
    let query = self.set.suggest(&self.available);
    self.queries.push(query.clone());

    query.to_string()
  }

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
