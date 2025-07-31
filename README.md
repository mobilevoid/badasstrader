This repository contains a simple Rust-based trading program.

### Build and Run

```
cargo run -- data/sample.csv
```

The program reads a CSV file containing daily closing prices and emits a buy,
sell or hold signal based on a 5/20 day moving average crossover.
