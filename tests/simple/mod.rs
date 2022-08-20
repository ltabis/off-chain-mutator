use off_chain_mutator::{accounts::Account, transaction::History};

#[test]
fn test_simple_deposits() {
    let transactions = History::from_path("./tests/simple/simple-deposit.csv").unwrap();

    let mut clients = transactions
        .0
        .iter()
        .map(|transaction| (transaction.client, Account::new(transaction.client)))
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
                locked: false,
                disputed: None,
            },
            Account {
                client_id: 2,
                available: 11.0,
                total: 11.0,
                held: 0.0,
                locked: false,
                disputed: None,
            },
            Account {
                client_id: 10,
                available: 2.0,
                total: 2.0,
                held: 0.0,
                locked: false,
                disputed: None,
            }
        ]
    );
}

#[test]
fn test_simple_operations() {
    let transactions = History::from_path("./tests/simple/simple-operations.csv").unwrap();

    let mut clients = transactions
        .0
        .iter()
        .map(|transaction| (transaction.client, Account::new(transaction.client)))
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
                available: 39.90,
                total: 39.90,
                held: 0.0,
                locked: false,
                disputed: None,
            },
            Account {
                client_id: 2,
                available: 0.0,
                total: 0.0,
                held: 0.0,
                locked: true,
                disputed: None,
            },
        ]
    );
}
