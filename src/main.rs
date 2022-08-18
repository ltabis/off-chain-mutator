enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

struct Transaction {
    r#type: TransactionType,
    client_id: u16,
    transaction_id: u32,
    // TODO: up to four places past the decimal.
    amount: f32,
}

struct Account {
    client_id: u16,
    // TODO: up to four places past the decimal.
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

fn main() {
    println!("Hello, world!");
}
