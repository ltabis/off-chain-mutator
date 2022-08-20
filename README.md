# Off Chain Mutator

A small program that take off-chain transactions and perform operations on a set of accounts.

# How to run

The program takes the path to your data input as a csv file.

```sh
$ cargo run -- input-file.csv
```

It outputs the accounts final values on stdout using the csv format.

# Library

This project also exposes a library than you can use to customize the program, allowing different types of inputs and outputs.

# Tests

Integration tests are available in the `tests` folder.
To run tests, use:
```sh
$ cargo test
```

Todo:
- [ ] add fuzzing.
- [ ] add integration tests.

# Other

I did not took time to make the app "beautiful" with nice colors, formatting and stuff because I think it's irrelevant for this project.

# Roadmap

- [ ] setup clippy.
- [ ] use internal-tagged enums for transactions (not yet supported by csv).
- [ ] add a log feature for data errors.