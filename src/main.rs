// TODO:
//  - handle formatting errors.
//  - think about big datasets.

use off_chain_mutator::{
    accounts::Account,
    transaction::{History, Transaction},
};

fn main() {
    let mut args = std::env::args();

    args.next();

    if let Some(path) = args.next() {
        // TODO: does all files have headers ?
        let mut rdr = csv::ReaderBuilder::new()
            .from_path(path)
            .expect("could not open csv database");

        let transactions = History::new(
            rdr.deserialize()
                .map(|result| result.expect("could not deserialize csv"))
                .collect::<Vec<Transaction>>(),
        );

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
