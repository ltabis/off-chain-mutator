use off_chain_mutator::transaction::{History, TransactionError};

#[test]
fn test_duplicate_transaction_ids() {
    assert_eq!(
        History::from_path("./tests/error_handling/duplicate.csv").err(),
        Some(TransactionError::DataFormatError(
            "duplicate transactions found: 2".to_string()
        ))
    );
}
