# wordle-solver

wordle solver.

npm package (via wasm): [`@cm-ayf/wordle-solver`](https://www.npmjs.com/package/@cm-ayf/wordle-solver)

## status (hint) format

five-chars `&str` (`string` in JS/TS) of:

* `G` (`g`): Green color (match on exact position)
* `Y` (`y`): Yellow color (match on other position)
* `_` (` `): Gray color (no match)

## install

* Rust

add following to Cargo.toml, below `[dependencies]`:

```toml
wordle-solver = { git = "https://github.com/cm-ayf/wordle-solver" }
```

* JS / TS

```sh
npm i @cm-ayf/wordle-solver
```

## usage

* Rust

```rs
let mut solver = Solver::new();

assert!(!solver.finished());
assert_eq!(solver.answer(), None);
let word = solver.start();
assert_eq!(&word, "AIERY");
let word = solver.next("__YY_")?;
assert_eq!(&word, "WEROS");
let word = solver.next("_GY__")?;
assert_eq!(&word, "TURFY");
let word = solver.next("YYY__")?;
assert_eq!(&word, "ZYMIC");
let word = solver.next("_____")?;
assert_eq!(&word, "REBUT");

assert!(solver.finished());
assert_eq!(solver.answer(), Some("REBUT".to_string()));
```

* TypeScript / JavaScript

```ts
import { Solver } from 'wordle-solver';
// const { Solver } = require('wordle-solver'); // JavaScript

const solver = new Solver();
console.log(solver.finished()); // false
console.log(solver.answer()); // undefined

let word = solver.start();
console.log(word); // AIERY
word = solver.next('__YY_');
console.log(word); // WEROS
word = solver.next('_GY__');
console.log(word); // TURFY
word = solver.next('YYY__');
console.log(word); // ZYMIC
word = solver.next('_____');
console.log(word); // REBUT

console.log(solver.finished()); // true
console.log(solver.answer()); // REBUT
```

## reference

### struct (class) `Solver`

* `Solver(.prototype).start`
  * start solving.
  * returns first query.
  * might take a few seconds.
* `Solver(.prototype).next`
  * `status: &str / string`: text describing the hint from last query.
  * returns next query.
  * might take a few sub second.
* `Solver(.prototype).finished`
  * returns boolean; if solver has the answer, true.
* `Solver(.prototype).answer`
  * returns answer if solver has it.
