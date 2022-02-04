use std::error::Error;

use wordle_solver::Solver;

fn main() {
  let stdin = std::io::stdin();
  let mut solver = Solver::new();
  let mut i = 0;

  println!("{}", solver.start());

  while i < 5 {
    match (|| {
      let mut buf = String::new();
      stdin.read_line(&mut buf)?;
      let next = solver.next(buf.trim())?;
      Result::<_, Box<dyn Error>>::Ok(next)
    })() {
      Ok(w) => {
        println!("{w}");
        if solver.finished() {
          return;
        } else {
          i += 1;
        }
      }
      Err(e) => eprintln!("{e}"),
    }
  }
}
