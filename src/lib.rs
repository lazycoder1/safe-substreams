mod abi;
mod pb;

use std::u64;

use abi::gelato_relay::events::LogUseGelato1Balance as LogUseGelato1BalanceEvent;
use abi::gnosis_safe;
use abi::gnosis_safe::events::SafeMultiSigTransaction as SafeMultiSigTransactionEvent;
use pb::gtms::safe_multi_sig;
use pb::gtms::safe_multi_sig::v1::{
    LogUseGelato1Balance, LogUseGelato1Balances, SafeMultiSigTransaction, SafeMultiSigTransactions,
};
use prost::Message;
use substreams::errors::Error;
use substreams::{log, Hex};
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
pub fn map_gelato_relay(block: eth::v2::Block) -> Result<LogUseGelato1Balances, Error> {
    let mut log_use_gelato_1_balance = vec![];

    for log in block.logs() {
        let transaction_hash = Hex(&log.receipt.transaction.hash).to_string();
        if log.receipt.transaction.status != 1 {
            continue;
        }

        if !LogUseGelato1BalanceEvent::match_log(log.log) {
            continue;
        }
        let event = LogUseGelato1BalanceEvent::decode(log.log).unwrap();

        log_use_gelato_1_balance.push(LogUseGelato1Balance {
            transaction_hash,
            sponsor: Hex(event.sponsor).to_string(),
            target: Hex(event.target).to_string(),
            fee_token: Hex(event.fee_token).to_string(),
            one_balance_chain_id: event.one_balance_chain_id.to_string(),
            native_to_fee_token_x_rate_numerator: event
                .native_to_fee_token_x_rate_numerator
                .to_string(),
            native_to_fee_token_x_rate_denominator: event
                .native_to_fee_token_x_rate_denominator
                .to_string(),
            correlation_id: Hex(event.correlation_id).to_string(),
        });
    }

    Ok(LogUseGelato1Balances {
        log_use_gelato_1_balance,
    })
}

#[substreams::handlers::map]
pub fn graph_out(safe_multi_sig_txns: SafeMultiSigTransactions, log_use_gelato_1_balances: LogUseGelato1Balances) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for safe_multi_sig_tx in safe_multi_sig_txns.safe_multi_sig_tx {
        log::info!(
            "New safe multi-sig transaction detected {}",
            &safe_multi_sig_tx.transaction_hash
        );

        tables
            .create_row(
                "SafeMultiSigTransaction",
                &safe_multi_sig_tx.transaction_hash,
            )
            .set(
                "transaction_hash",
                append_0x(safe_multi_sig_tx.transaction_hash),
            )
            .set("to", append_0x(safe_multi_sig_tx.to))
            .set("from", append_0x(safe_multi_sig_tx.from))
            .set("data", append_0x(safe_multi_sig_tx.data))
            .set("operation", safe_multi_sig_tx.operation)
            .set(
                "safe_tx_gas",
                string_to_bigint(safe_multi_sig_tx.safe_tx_gas),
            )
            .set("base_gas", string_to_bigint(safe_multi_sig_tx.base_gas))
            .set("gas_price", string_to_bigint(safe_multi_sig_tx.gas_price))
            .set(
                "refund_receiver",
                append_0x(safe_multi_sig_tx.refund_receiver),
            )
            .set("signatures", append_0x(safe_multi_sig_tx.signatures))
            .set(
                "additional_info",
                append_0x(safe_multi_sig_tx.additional_info),
            )
            .set("timestamp", safe_multi_sig_tx.timestamp);
    }

    for gelato_1_balance in log_use_gelato_1_balances.log_use_gelato_1_balance {
        log::info!(
            "New log use gelato 1 balance {}",
            &gelato_1_balance.transaction_hash
        );

        tables
            .create_row("LogUseGelato1Balance", &gelato_1_balance.transaction_hash)
            .set(
                "transaction_hash",
                append_0x(gelato_1_balance.transaction_hash),
            )
            .set("sponsor", append_0x(gelato_1_balance.sponsor))
            .set("target", append_0x(gelato_1_balance.target))
            .set("fee_token", append_0x(gelato_1_balance.fee_token))
            .set("one_balance_chain_id", string_to_bigint(gelato_1_balance.one_balance_chain_id))
            .set("native_to_fee_token_x_rate_numerator", string_to_bigint(gelato_1_balance.native_to_fee_token_x_rate_numerator))
            .set("native_to_fee_token_x_rate_denominator", string_to_bigint(gelato_1_balance.native_to_fee_token_x_rate_denominator))
            .set("correlation_id", append_0x(gelato_1_balance.correlation_id));
    }

    Ok(tables.to_entity_changes())
}

fn string_to_bigint(str: String) -> substreams::scalar::BigInt {
    str.parse::<substreams::scalar::BigInt>()
        .expect("failed to parse str")
}

fn append_0x(str: String) -> String {
    format!("{}{}", "0x", str)
}
