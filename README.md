# RustyClueSolver
A program to help with deducing the facts from a Clue (or Cluedo in UK) game.

## Installation
Right now, the easiest way to install is to check the releases page and see if there is a binary for your system.

If there is not, then you have to clone the repository and run
```bash
cargo install --path .
```

## Usage
You can initialize a new game by running:
```bash
cluesolver init
```
then answering the questions.
This will create a new .json file in the current working directory that contains the state of the current game.


You can then use 
```
cluesolver accuse
```

to log an accusation by you or another player. Lastly, you can run 
```
cluesolver wins
```
to query whether the program has deduced any winning cards yet from the information given.
