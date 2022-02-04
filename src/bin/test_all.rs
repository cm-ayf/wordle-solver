use std::time::Instant;

use wordle_solver::*;

fn main() {
  let start = Instant::now();
  let answers = wordle_solver_data::candidates();
  let mut solver = Solver::new(true);
  let start_word: Word = solver.start().parse().unwrap();

  for answer in answers {
    let answer = answer.parse().unwrap();
    let mut solver = solver.clone();
    let mut word = start_word.clone();

    loop {
      let status = word.status(&answer);

      if &status.to_string() == "ggggg" {
        break;
      }

      match solver.next(&status.to_string()) {
        Ok(w) => {
          let new_word = w.parse().unwrap();
          if new_word != word {
            word = new_word;
          } else {
            eprintln!("{word}: loop!");
            break;
          }
        },
        Err(e) => {
          eprintln!("{e}");
          break;
        }
      }
    }
  }

  println!("{} ms", start.elapsed().as_millis());
}