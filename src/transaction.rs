use std::str::FromStr;

use crate::accounts::Account;

#[derive(Debug)]
pub enum TransactionError {
    ReadError(String),
    DeserializationError(String),
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            TransactionError::ReadError(err) => {
                format!("could not read transactions: {err}")
            }
            TransactionError::DeserializationError(err) => {
                format!("could not deserialize transactions: {err}")
            }
        })
    }
}

impl std::error::Error for TransactionError {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Transaction {
    pub r#type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

pub struct History(pub Vec<Transaction>);

impl History {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self(transactions)
    }

    fn check_formatting(&self) -> Result<(), TransactionError> {
        todo!()
    }

    pub fn from_path(path: &str) -> Result<Self, TransactionError> {
        let history = Self(
            csv::ReaderBuilder::new()
                // flexible is on to enable undisclosed amount (dispute, resolve & chargeback).
                .flexible(true)
                .from_path(std::path::PathBuf::from_str(path).unwrap())
                .map_err(|err| TransactionError::ReadError(err.to_string()))?
                .deserialize()
                .map(|result| result)
                .collect::<Result<Vec<Transaction>, _>>()
                .map_err(|err| TransactionError::DeserializationError(err.to_string()))?,
        );

        // TODO: history.check_formatting()?;

        Ok(history)
    }
}

impl History {
    /// Get a transaction by it's id.
    fn transaction_by_id(&self, tx: u32) -> Option<&Transaction> {
        self.0.iter().find(|old| old.tx == tx)
    }

    /// update all given accounts following the internal history of transactions.
    pub fn update_accounts<'a>(&'_ self, clients: &mut std::collections::HashMap<u16, Account>) {
        for transaction in &self.0 {
            let account = clients.get_mut(&transaction.client).unwrap();

            match (&transaction.r#type, transaction.amount) {
                (TransactionType::Deposit, Some(amount)) => {
                    account.available += amount;
                    account.total += amount;
                }
                (TransactionType::Withdrawal, Some(amount)) => {
                    if account.available - amount >= 0.0 {
                        account.available -= amount;
                        account.total -= amount;
                    }
                }
                (TransactionType::Dispute, None) => {
                    // We can search from the beginning because for a
                    // dispute to occur their first need to be a regular
                    // transaction (withdrawal or deposit).
                    if let Some(disputed) = self.transaction_by_id(transaction.tx).and_then(|old| {
                        match (&old.r#type, old.amount) {
                            (TransactionType::Deposit, Some(_))
                            | (TransactionType::Withdrawal, Some(_)) => Some(old),
                            _ => None,
                        }
                    }) {
                        account.available -= disputed.amount.unwrap();
                        account.held += disputed.amount.unwrap();

                        account.disputed = Some((*disputed).clone());
                    }
                }
                (TransactionType::Resolve, None) => {
                    if let Some(disputed) = &account.disputed {
                        {
                            account.available += disputed.amount.unwrap();
                            account.held -= disputed.amount.unwrap();

                            account.disputed = None;
                        };
                    }
                }
                (TransactionType::Chargeback, None) => {
                    if let Some(disputed) = &account.disputed {
                        account.held -= disputed.amount.unwrap();
                        account.total -= disputed.amount.unwrap();

                        account.locked = true;
                        account.disputed = None;
                    }
                }
                _ => {}
            };
        }
    }
}
