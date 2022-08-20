use std::str::FromStr;

use crate::accounts::Account;

#[derive(Debug)]
pub enum TransactionError {
    ReadError,
    DeserializationError,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TransactionError::ReadError => "could not read transactions",
            TransactionError::DeserializationError => "could not deserialize transactions",
        })
    }
}

impl std::error::Error for TransactionError {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub r#type: TransactionType,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub id: u32,
    // TODO: up to four places past the decimal.
    pub amount: f32,
}

pub struct History(pub Vec<Transaction>);

impl History {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self(transactions)
    }

    pub fn from_path(path: &str) -> Result<Self, TransactionError> {
        Ok(Self(
            csv::ReaderBuilder::new()
                .from_path(std::path::PathBuf::from_str(path).unwrap())
                .map_err(|_| TransactionError::ReadError)?
                .deserialize()
                .map(|result| result)
                .collect::<Result<Vec<Transaction>, _>>()
                .map_err(|_| TransactionError::DeserializationError)?,
        ))
    }
}

impl History {
    /// Get a transaction by it's id.
    pub fn transaction_by_id(&self, id: u32) -> Option<&Transaction> {
        self.0.iter().find(|old| old.id == id)
    }

    /// update all given accounts following the internal history of transactions.
    pub fn update_accounts<'a>(&'_ self, clients: &mut std::collections::HashMap<u16, Account>) {
        for transaction in &self.0 {
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
