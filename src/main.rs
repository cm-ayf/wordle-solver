use std::time::Instant;

use wordle_solver::WordSet;

fn main() {
  let available = WordSet::from_file("data/queries");
  let mut set = WordSet::from_file("data/candidates");

  let stdin = std::io::stdin();

  for _ in 0..5 {
    if let Some(word) = set.answer() {
      println!("{}", word);
      return;
    }

    let start = Instant::now();
    let word = set.suggest(&available);
    println!("{}", start.elapsed().as_millis());

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
