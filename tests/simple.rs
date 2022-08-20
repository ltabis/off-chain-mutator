#[cfg(test)]
mod test {
    const DATA_PATH: &str = "./tests/data/";

    use off_chain_mutator::{
        accounts::Account,
        transaction::{History, Transaction},
    };

    #[test]
    fn test_simple_deposits() {
        let transactions = History::new(
            csv::ReaderBuilder::new()
                .from_path(std::path::PathBuf::from_iter([
                    DATA_PATH,
                    "simple-deposit.csv",
                ]))
                .expect("could not open csv database")
                .deserialize()
                .map(|result| result.expect("could not deserialize csv"))
                .collect::<Vec<Transaction>>(),
        );

        let mut clients = transactions
            .0
            .iter()
            .map(|transaction| (transaction.client_id, Account::new(transaction.client_id)))
            .collect::<std::collections::HashMap<_, _>>();

        transactions.update_accounts(&mut clients);

        let mut accounts = clients
            .into_iter()
            .map(|(_, account)| account)
            .collect::<Vec<_>>();

        accounts.sort_unstable_by_key(|account| account.client_id);

        assert_eq!(
            accounts,
            vec![
                Account {
                    client_id: 1,
                    available: 1.0,
                    total: 1.0,
                    held: 0.0,
                    locked: false
                },
                Account {
                    client_id: 2,
                    available: 11.0,
                    total: 11.0,
                    held: 0.0,
                    locked: false
                },
                Account {
                    client_id: 10,
                    available: 2.0,
                    total: 2.0,
                    held: 0.0,
                    locked: false
                }
            ]
        );
    }
}
