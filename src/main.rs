// TODO:
//  - handle formatting errors.
//  - think about big datasets.

use off_chain_mutator::{accounts::Account, transaction::History};

fn main() {
    let mut args = std::env::args();

    args.next();

    if let Some(path) = args.next() {
        // FIXME: error handling: does all files have headers ?
        // FIXME: error handling: does all row have the same nb of columns ?
        let transactions = History::from_path(&path).expect("failed to read transactions");

        // not extracting accounts directly in the history enable the user
        // to create it's own list of accounts.
        let mut clients = transactions
            .0
            .iter()
            .map(|transaction| (transaction.client_id, Account::new(transaction.client_id)))
            .collect::<std::collections::HashMap<_, _>>();

        transactions.update_accounts(&mut clients);

        let mut output = csv::Writer::from_writer(std::io::stdout());

        for record in clients
            .iter()
            .map(|(_, account)| account)
            .collect::<Vec<_>>()
        {
            output
                .serialize(record)
                .expect("could not display clients accounts");
        }

        output.flush().unwrap();
    } else {
        panic!("expected path to csv database as argument");
    }
}
