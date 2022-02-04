//! # wordle-solver
//!
//! [wordle](https://www.powerlanguage.co.uk/wordle/) solver API.
//!
//! ## usage
//!
//! ```
//! let mut solver = Solver::new();
//!
//! let word = solver.start()
//! println!("{}", word); //AIERY
//! let word = solver.next("__YY_")?;
//! println!("{}", word); //WEROS
//! let word = solver.next("_GY__")?;
//! println!("{}", word); //TURFY
//! let word = solver.next("YYY__")?;
//! println!("{}", word); //ZYMIC
//! let word = solver.next("_____")?;
//! println!("{}", word); //REBUT
//! assert!(solver.finished());
//! ```

mod solver;
mod status;
mod word;
mod word_set;

pub use solver::Solver;
use status::Status;
use word::Word;
use word_set::WordSet;
