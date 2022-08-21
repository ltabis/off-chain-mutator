// TODO:
//  - handle formatting errors.
//  - think about big datasets.

use off_chain_mutator::{accounts::Account, transaction::History};

fn main() {
    let mut args = std::env::args();

    args.next();

    if let Some(path) = args.next() {
        // FIXME: error handling: does all files have headers ?
        let transactions = match History::from_path(&path) {
            Ok(transactions) => transactions,
            Err(err) => {
                eprintln!("failed to read transactions: {}", err);
                std::process::exit(1);
            }
        };

        // not extracting accounts directly in the history enable the user
        // to create it's own list of accounts.
        let mut clients = transactions
            .0
            .iter()
            .map(|transaction| (transaction.client, Account::new(transaction.client)))
            .collect::<std::collections::HashMap<_, _>>();

        transactions.update_accounts(&mut clients);

        let mut output = csv::Writer::from_writer(std::io::stdout());

        for record in clients
            .iter()
            .map(|(_, account)| account)
            .collect::<Vec<_>>()
        {
            if let Err(err) = output.serialize(record) {
                eprintln!("could not display clients accounts: {}", err);
                std::process::exit(1);
            }
        }

        output.flush().unwrap();
    } else {
        eprintln!("expected path to csv database as argument");
        std::process::exit(1);
    }
}
