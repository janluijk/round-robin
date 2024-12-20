# Round-Robin Tournament Generator
## Description
This project implements an algorithm to generate a round-robin tournament that satsfies the following criteria
- `n` teams participate in the tournament.
- There are `n` games in total.
- Each team plays each game twice
- Each team faces every other opponent exactly once

The algorithm ensures that the constraints are met for different values of *n*, starting at `n = 4` though some values are more challenging to solve than others.
For example, solutions have been found for `n = 5` and `n = 6` in only a couple of seconds, `n = 4` remains undiscovered.


## Running the project
1. Clone the repo:
```bash
git clone https://github.com/your-username/round-robin-tournament.git
cd round-robin-tournament
```

2. Build the project:
```bash
cargo build
```

3. Run the project:
```bash
cargo run
```

## Results
`n = 5`
```
[ 1 -  2] [ 3 -  4] [ 5 -  6] [ 7 -  8] [ 9 - 10] 
[ 3 -  5] [ 1 -  6] [ 7 -  9] [ 2 - 10] [ 4 -  8] 
[ 4 -  7] [ 2 -  9] [ 8 - 10] [ 1 -  5] [ 3 -  6] 
[ 6 - 10] [ 5 -  8] [ 2 -  4] [ 3 -  9] [ 1 -  7] 
[ 8 -  9] [ 7 - 10] [ 1 -  3] [ 4 -  6] [ 2 -  5] 
[ 1 -  4] [ 2 -  7] [ 3 -  8] [ 5 - 10] [ 6 -  9] 
[ 2 -  3] [ 4 -  9] [ 1 - 10] [ 6 -  8] [ 5 -  7] 
[ 5 -  9] [ 1 -  8] [ 2 -  6] [ 3 -  7] [ 4 - 10] 
[ 6 -  7] [ 3 - 10] [ 4 -  5] [ 1 -  9] [ 2 -  8] 
```

`n = 6`
```
[ 1 -  2] [ 3 -  4] [ 5 -  6] [ 7 -  8] [ 9 - 10] [11 - 12] 
[ 3 -  5] [ 1 -  6] [ 2 -  4] [ 9 - 11] [ 7 - 12] [ 8 - 10] 
[ 4 -  6] [ 7 - 10] [ 8 -  9] [ 1 - 12] [ 3 - 11] [ 2 -  5] 
[ 7 -  9] [ 8 - 12] [10 - 11] [ 4 -  5] [ 2 -  6] [ 1 -  3] 
[ 8 - 11] [ 2 -  9] [ 3 - 12] [ 6 - 10] [ 1 -  5] [ 4 -  7] 
[10 - 12] [ 5 - 11] [ 1 -  7] [ 2 -  3] [ 4 -  8] [ 6 -  9] 
[ 1 -  4] [ 2 -  7] [ 3 -  8] [ 5 - 10] [ 6 - 11] [ 9 - 12] 
[ 2 -  8] [ 3 -  6] [ 7 - 11] [ 1 -  9] [ 5 - 12] [ 4 - 10] 
[ 3 - 10] [ 5 -  9] [ 2 - 12] [ 4 - 11] [ 1 -  8] [ 6 -  7] 
[ 5 -  7] [ 4 - 12] [ 1 - 10] [ 6 -  8] [ 3 -  9] [ 2 - 11] 
[ 6 - 12] [ 1 - 11] [ 4 -  9] [ 3 -  7] [ 2 - 10] [ 5 -  8]
```
