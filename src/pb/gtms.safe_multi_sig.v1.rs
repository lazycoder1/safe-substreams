// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeMultiSigTransactions {
    #[prost(message, repeated, tag="1")]
    pub safe_multi_sig_tx: ::prost::alloc::vec::Vec<SafeMultiSigTransaction>,
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
// @@protoc_insertion_point(module)
