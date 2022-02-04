use wordle_solver::Solver;

fn main() {
  let stdin = std::io::stdin();
  let mut solver = Solver::new(true);
  let mut i = 0;

  println!("{}", solver.start());

  while i < 5 {
    let mut buf = String::new();
    if let Err(e) = stdin.read_line(&mut buf) {
      eprintln!("{e}");
      continue;
    };

    match solver.next(buf.trim()) {
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
