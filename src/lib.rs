mod abi;
mod pb;

use std::u64;

use abi::gnosis_safe;
use abi::gnosis_safe::events::SafeMultiSigTransaction as SafeMultiSigTransactionEvent;
use pb::gtms::safe_multi_sig;
use pb::gtms::safe_multi_sig::v1::{SafeMultiSigTransaction, SafeMultiSigTransactions};
use prost::Message;
use substreams::errors::Error;
use substreams::{Hex, log};
use substreams_database_change::change::AsString;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth;
use substreams_ethereum::Event;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_safe_multi_sig_transactions(
    block: eth::v2::Block,
) -> Result<SafeMultiSigTransactions, Error> {
    // let block_time = block.timestamp();
    let mut safe_multi_sig_tx = vec![];
    let block_time = block.timestamp_seconds();

    for log in block.logs() {
        if log.receipt.transaction.status != 1 {
            continue;
        }

        if !SafeMultiSigTransactionEvent::match_log(log.log) {
            continue;
        }
        let sender = Hex(log.address()).to_string();
        let event = SafeMultiSigTransactionEvent::decode(log.log).unwrap();

        safe_multi_sig_tx.push(SafeMultiSigTransaction {
            transaction_hash: Hex(&log.receipt.transaction.hash).to_string(),
            to: Hex(event.to).to_string(),
            from: sender,
            data: Hex(event.data).to_string(),
            operation: event.operation.to_u64(),
            safe_tx_gas: event.safe_tx_gas.to_string(),
            base_gas: event.base_gas.to_string(),
            gas_price: event.gas_price.to_string(),
            gas_token: Hex(event.gas_token).to_string(),
            refund_receiver: Hex(event.refund_receiver).to_string(),
            signatures: Hex(event.signatures).to_string(),
            additional_info: Hex(event.additional_info).to_string(),
            timestamp: block_time,
        })
    }

    Ok(SafeMultiSigTransactions { safe_multi_sig_tx })
}


#[substreams::handlers::map]
pub fn graph_out(safe_multi_sig_txns: SafeMultiSigTransactions) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for safe_multi_sig_tx in safe_multi_sig_txns.safe_multi_sig_tx {
        log::info!("New safe multi-sig transaction detected {}", &safe_multi_sig_tx.transaction_hash);

        tables
            .create_row("SafeMultiSigTransaction", &safe_multi_sig_tx.transaction_hash)
            .set("transaction_hash", safe_multi_sig_tx.transaction_hash)
            .set("to", safe_multi_sig_tx.to)
            .set("from", safe_multi_sig_tx.from)
            .set("data", safe_multi_sig_tx.data)
            .set("operation", safe_multi_sig_tx.operation)
            .set("safe_tx_gas", string_to_bigint(safe_multi_sig_tx.safe_tx_gas))
            .set("base_gas", string_to_bigint(safe_multi_sig_tx.base_gas))
            .set("gas_price", string_to_bigint(safe_multi_sig_tx.gas_price))
            .set("refund_receiver", safe_multi_sig_tx.refund_receiver)
            .set("signatures", safe_multi_sig_tx.signatures)
            .set("additional_info", safe_multi_sig_tx.additional_info)
            .set("timestamp", safe_multi_sig_tx.timestamp);
    }
    // log::info!("Items in table values:{} keys:{}", tables.tables.values().len(), tables.tables.keys().len());

    Ok(tables.to_entity_changes())
}

fn string_to_bigint(str: String) -> substreams::scalar::BigInt {
    str.parse::<substreams::scalar::BigInt>().expect("failed to parse str")
}