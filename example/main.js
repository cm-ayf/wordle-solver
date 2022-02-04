const { createInterface } = require('readline');
const { Solver } = require('../pkg/wordle_solver'); // '@cm-ayf/wordle-solver'

const solver = new Solver(true);
const reader = createInterface({
  input: process.stdin,
});

console.log(solver.start());

let i = 0;

reader.on("line", (input) => {
  try {
    const word = solver.next(input);
    console.log(word);
    i++;
    if (solver.finished() || i >= 5) reader.close();
  } catch (e) {
    console.error(e);
  }
});

