use std::collections::HashSet;
use wordle_solver::WordSet;

fn main() {
  let full_set = HashSet::full();
  let mut set = full_set.clone();

  let stdin = std::io::stdin();

  for _ in 0..5 {
    if let Some(word) = set.answer() {
      println!("{}", word);
      return;
    }

    let word = set.suggest(&full_set);
    println!("{}", word);
    loop {
      let mut buf = String::new();
      match (stdin.read_line(&mut buf), buf.trim().parse()) {
        (Ok(_), Ok(status)) => {
          set.filter(word, &status);
          break;
        },
        _ => eprintln!("error try again"),
      }
    }
  }

  match set.answer() {
    Some(word) => println!("{}", word),
    None => eprintln!("failed"),
  };
}
