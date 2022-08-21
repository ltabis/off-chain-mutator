use off_chain_mutator::{
    accounts::Account,
    transaction::{History, Transaction, TransactionType},
};

#[test]
fn test_example() {
    let transactions = History::from_path("./tests/simple/example.csv").unwrap();

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
                available: 1.5,
                total: 1.5,
                held: 0.0,
                locked: false,
                disputed: vec![]
            },
            Account {
                client_id: 2,
                available: 2.0,
                total: 2.0,
                held: 0.0,
                locked: false,
                disputed: vec![]
            },
        ]
    );
}

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
                disputed: vec![],
            },
            Account {
                client_id: 2,
                available: 11.0,
                total: 11.0,
                held: 0.0,
                locked: false,
                disputed: vec![],
            },
            Account {
                client_id: 10,
                available: 2.0,
                total: 2.0,
                held: 0.0,
                locked: false,
                disputed: vec![],
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
                available: 89.9,
                total: 89.9,
                held: 0.0,
                locked: false,
                disputed: vec![],
            },
            Account {
                client_id: 2,
                available: 0.0,
                total: 0.0,
                held: 0.0,
                locked: true,
                disputed: vec![],
            },
            Account {
                client_id: 3,
                available: 5.0,
                total: 10.0,
                held: 5.0,
                locked: false,
                disputed: vec![Transaction {
                    r#type: TransactionType::Withdrawal,
                    client: 3,
                    tx: 8,
                    amount: Some(5.0),
                }],
            },
        ]
    );
}

#[test]
fn test_multiple_disputes() {
    let transactions = History::from_path("./tests/simple/multiple-disputes.csv").unwrap();

    let mut clients = transactions
        .0
        .iter()
        .map(|transaction| (transaction.client, Account::new(transaction.client)))
        .collect::<std::collections::HashMap<_, _>>();

    transactions.update_accounts(&mut clients);

    let accounts = clients
        .into_iter()
        .map(|(_, account)| account)
        .collect::<Vec<_>>();

    assert_eq!(
        accounts,
        vec![Account {
            client_id: 1,
            available: 80.0,
            total: 90.0,
            held: 10.0,
            locked: true,
            disputed: vec![Transaction {
                r#type: TransactionType::Withdrawal,
                client: 1,
                tx: 2,
                amount: Some(10.0),
            }],
        },]
    );
}
