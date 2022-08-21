# Off Chain Mutator

A small program that take an off-chain transaction history and update a set of accounts from those transactions.

# How to run

The program takes the path to a csv file containing transaction data.

```sh
$ cargo run -- input-file.csv
```

The format of the data must contain the following headers as the first row: `type`, `client`, `tx`, `amount`.

- `type`: the type of the transaction, can be: `deposit`, `withdrawal`, `dispute`, `resolve` or `chargeback`.
- `client`: the id of the client account to update.
- `tx`: the id of the transaction, must be unique in the history, except for the `dispute`, `resolve` and `chargeback`.
      `dispute`, `resolve` and `chargeback` use the `tx` field to point to the influenced transaction.
- `amount`: the amount to update the account with, can be omitted for types `dispute`, `resolve` and `chargeback`.

It outputs the final values of the processed accounts on stdout using the csv format:
`client`, `available`, `held`, `total`, `locked`.

# Library

This project also exposes a library than you can use to customize the program, allowing different types of inputs and outputs.

# Tests

Integration tests are available in the `tests` folder.

```sh
$ cargo test
```

Todo:
- [ ] add fuzzing.

# Other

I did not took time to make the app "beautiful" with nice colors, formatting and stuff because I think it's irrelevant for this project.

# Roadmap

- [ ] setup clippy.
- [ ] use internal-tagged enums for transactions (not yet supported by csv).
- [ ] add a log feature for data errors.
- [ ] use streams to accept input.
- [ ] setup a library crate & publish it.
- [ ] add other data format support.
- [ ] make clean error handling for the binary.