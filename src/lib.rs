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
//! println!("{}", word); // AIERY
//! 
//! let (word, is_answer) = solver.next("__YY_")?;
//! println!("{}", word); // VROUS
//! let (word, is_answer) = solver.next("_Y_G_")?;
//! println!("{}", word); // DURRA
//! let (word, is_answer) = solver.next("_YY__")?;
//! println!("{}", word); // FLOCS
//! let (word, is_answer) = solver.next("_____")?;
//! println!("{}", word); // REBUT
//! assert!(is_answer);
//! ```

mod word;
mod status;
mod word_set;
mod solver;

use word::Word;
use status::Status;
use word_set::WordSet;
pub use solver::Solver;
