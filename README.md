This is a little script that evaluates various Wordle solvers. Its output for each solver looks like:

```
BasicSolver       1 guess       7      #
avg 703us         2 guesses     673    ###
7729/7750         3 guesses     3083   ###########
99% success       4 guesses     2935   ##########
                  5 guesses     901    ####
                  6 guesses     130    #
                  unsolved      21     #
```

On the left:

- The solver's name
- The average time taken for a solve
- The solve success rate
- The solve success rate, again

On the right is a distribution of how often each number of guesses was taken.

The program accepts an optional command-line parameter of the number of iterations to run. On each iteration, the solver is ran over every possible true target word.

## Adding a solver

To add a solver:

- Create a new .rs file in [src/solvers/](https://github.com/Astronomy487/wordlesolvers/tree/master/src/solvers)
- Make some type that implements the `Solver` trait
- Add it at the top of [src/solvers/mod.rs](https://github.com/Astronomy487/wordlesolvers/blob/master/src/solvers/mod.rs)
- Add it at the bottom of [src/main.rs](https://github.com/Astronomy487/wordlesolvers/blob/master/src/main.rs)

If I were better at writing macros, I would streamline this process.

The traits and types I provide in the `crate::` scope are listed and documented in [src/solvers/mod.rs](https://github.com/Astronomy487/wordlesolvers/blob/master/src/solvers/mod.rs). Read them before implementing a solver. Look to [src/solvers/dumbsolver.rs](https://github.com/Astronomy487/wordlesolvers/blob/master/src/solvers/dumbsolver.rs) and [src/solvers/basicsolver.rs](https://github.com/Astronomy487/wordlesolvers/blob/master/src/solvers/basicsolver.rs) for inspiration.

It uses [this wordlist](https://github.com/Astronomy487/wordlesolvers/blob/master/src/wordlist.txt), which I think is the original Wordle list. Let me know if it's not, I guess.