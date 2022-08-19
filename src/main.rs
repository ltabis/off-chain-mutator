#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Transaction {
    r#type: TransactionType,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    id: u32,
    // TODO: up to four places past the decimal.
    amount: f32,
}

struct History {
    inner: Vec<Transaction>,
}

impl History {
    fn transaction_by_id(&self, id: u32) -> Option<&Transaction> {
        self.inner.iter().find(|old| old.id == id)
    }

    fn update_accounts<'a>(&'_ self, clients: &mut std::collections::HashMap<u16, Account>) {
        for transaction in &self.inner {
            let account = clients.get_mut(&transaction.client_id).unwrap();

            match transaction.r#type {
                TransactionType::Deposit => {
                    account.available += transaction.amount;
                    account.total += transaction.amount;
                }
                TransactionType::Withdrawal => {
                    if account.available - transaction.amount >= 0.0 {
                        account.available -= transaction.amount;
                        account.total -= transaction.amount;
                    }
                }
                TransactionType::Dispute => {
                    if let Some(disputed_amount) =
                        self.transaction_by_id(transaction.id).map(|old| old.amount)
                    {
                        account.available -= disputed_amount;
                        account.held += disputed_amount;
                    }
                }
                TransactionType::Resolve => {
                    match self.transaction_by_id(transaction.id) {
                        Some(disputed_transaction)
                            if matches!(disputed_transaction.r#type, TransactionType::Dispute) =>
                        {
                            account.available += disputed_transaction.amount;
                            account.held -= disputed_transaction.amount;
                        }
                        _ => {}
                    };
                }
                TransactionType::Chargeback => {
                    // NOTE: chargeback can make total & held values negative.
                    match self.transaction_by_id(transaction.id) {
                        Some(disputed_transaction)
                            if matches!(disputed_transaction.r#type, TransactionType::Dispute) =>
                        {
                            account.held -= disputed_transaction.amount;
                            account.total -= disputed_transaction.amount;
                            account.locked = true;
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct Account {
    #[serde(rename = "client")]
    client_id: u16,
    // TODO: up to four places past the decimal.
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

impl Account {
    fn new(client_id: u16) -> Self {
        Self {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}

fn main() {
    let mut args = std::env::args();

    args.next();

    if let Some(path) = args.next() {
        // TODO: does all files have headers ?
        let mut rdr = csv::ReaderBuilder::new()
            .from_path(path)
            .expect("could not open csv database");

        let transactions = History {
            inner: rdr
                .deserialize()
                .map(|result| result.expect("could not deserialize csv"))
                .collect::<Vec<Transaction>>(),
        };

        let mut clients = transactions
            .inner
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
