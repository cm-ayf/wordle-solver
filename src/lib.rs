//! # wordle-solver
//!
//! [wordle](https://www.powerlanguage.co.uk/wordle/) solver API.
//!
//! ## usage
//!
//! ```
//! let mut solver = Solver::new();
//!
//! assert!(!solver.finished());
//! assert_eq!(solver.answer(), None);
//!
//! let word = solver.start();
//! assert_eq!(&word, "AIERY");
//! let word = solver.next("__YY_")?;
//! assert_eq!(&word, "WEROS");
//! let word = solver.next("_GY__")?;
//! assert_eq!(&word, "TURFY");
//! let word = solver.next("YYY__")?;
//! assert_eq!(&word, "ZYMIC");
//! let word = solver.next("_____")?;
//! assert_eq!(&word, "REBUT");
//!
//! assert!(solver.finished());
//! assert_eq!(solver.answer(), Some("REBUT".to_string()));
//! ```

mod solver;
mod status;
mod word;
mod word_set;

pub use solver::Solver;
use status::Status;
use word::Word;
use word_set::WordSet;
