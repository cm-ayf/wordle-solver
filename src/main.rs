use std::error::Error;

use wordle_solver::Solver;

fn main() {
  let stdin = std::io::stdin();
  let mut solver = Solver::new();
  let mut i = 0;

  println!("{}", solver.start());

  while i < 6 {
    match (|| {
      let mut buf = String::new();
      stdin.read_line(&mut buf)?;
      Result::<_, Box<dyn Error>>::Ok(solver.next(buf.trim())?)
    })() {
      Ok((w, ended)) => {
        println!("{w}");
        if ended { return; } else { i += 1; }
      },
      Err(e) => eprintln!("{e}")
    }
  }
}
