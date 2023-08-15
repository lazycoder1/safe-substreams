// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeMultiSigTransactions {
    #[prost(message, repeated, tag="1")]
    pub safe_multi_sig_tx: ::prost::alloc::vec::Vec<SafeMultiSigTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogUseGelato1Balances {
    #[prost(message, repeated, tag="1")]
    pub log_use_gelato_1_balance: ::prost::alloc::vec::Vec<LogUseGelato1Balance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeMultiSigTransaction {
    #[prost(string, tag="1")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub data: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub operation: u64,
    #[prost(string, tag="6")]
    pub safe_tx_gas: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub base_gas: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub gas_token: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub refund_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub signatures: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub additional_info: ::prost::alloc::string::String,
    #[prost(uint64, tag="13")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogUseGelato1Balance {
    #[prost(string, tag="1")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sponsor: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub fee_token: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub one_balance_chain_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub native_to_fee_token_x_rate_numerator: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub native_to_fee_token_x_rate_denominator: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub correlation_id: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
