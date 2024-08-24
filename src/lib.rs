//! # StreamingFast's Rust-compiled protocol buffers.
//!
//! This module provides access to Rust implementations of StreamingFast's protocol buffer definitions,
//! enabling the encoding and decoding of data for Ethereum blockchain and bstream services.

use thiserror::Error;

/// Module for Ethereum-related data structures and operations.
/// Currently contains the `.proto` defined [here](https://github.com/streamingfast/firehose-ethereum/blob/d9ec696423c2288db640f00026ae29a6cc4c2121/proto/sf/ethereum/type/v2/type.proto#L9)    
pub mod ethereum {
    pub mod r#type {
        pub mod v2 {
            use alloy_primitives::{Address, Bloom, FixedBytes, Uint};
            use ethportal_api::types::execution::header::Header;
            use reth_primitives::TxType;
            use transaction_trace::Type;

            use crate::StreamingFastProtosError;

            tonic::include_proto!("sf.ethereum.r#type.v2");

            impl TryFrom<&Block> for Header {
                type Error = StreamingFastProtosError;

                fn try_from(block: &Block) -> Result<Self, Self::Error> {
                    let block_header = block
                        .header
                        .as_ref()
                        .ok_or(StreamingFastProtosError::BlockConversionError)?;

                    let parent_hash = FixedBytes::from_slice(block_header.parent_hash.as_slice());
                    let uncles_hash = FixedBytes::from_slice(block_header.uncle_hash.as_slice());
                    let author = Address::from_slice(block_header.coinbase.as_slice());
                    let state_root = FixedBytes::from_slice(block_header.state_root.as_slice());
                    let transactions_root =
                        FixedBytes::from_slice(block_header.transactions_root.as_slice());
                    let receipts_root =
                        FixedBytes::from_slice(block_header.receipt_root.as_slice());
                    let logs_bloom = Bloom::from_slice(block_header.logs_bloom.as_slice());
                    let difficulty = Uint::from_be_slice(
                        block_header
                            .difficulty
                            .as_ref()
                            .ok_or(StreamingFastProtosError::BlockConversionError)?
                            .bytes
                            .as_slice(),
                    );
                    let number = block_header.number;
                    let gas_limit = Uint::from(block_header.gas_limit);
                    let gas_used = Uint::from(block_header.gas_used);
                    let timestamp = block_header
                        .timestamp
                        .as_ref()
                        .ok_or(StreamingFastProtosError::BlockConversionError)?
                        .seconds as u64;
                    let extra_data = block_header.extra_data.clone();
                    let mix_hash = Some(FixedBytes::from_slice(block_header.mix_hash.as_slice()));
                    let nonce = Some(FixedBytes::from_slice(&block_header.nonce.to_be_bytes()));

                    Ok(Header {
                        parent_hash,
                        uncles_hash,
                        author,
                        state_root,
                        transactions_root,
                        receipts_root,
                        logs_bloom,
                        difficulty,
                        number,
                        gas_limit,
                        gas_used,
                        timestamp,
                        extra_data,
                        mix_hash,
                        nonce,
                        base_fee_per_gas: None,
                        withdrawals_root: None,
                        blob_gas_used: None,
                        excess_blob_gas: None,
                        parent_beacon_block_root: None,
                    })
                }
            }

            impl From<Type> for TxType {
                fn from(tx_type: Type) -> Self {
                    use TxType::*;
                    use Type::*;

                    match tx_type {
                        TrxTypeLegacy => Legacy,
                        TrxTypeAccessList => Eip2930,
                        TrxTypeDynamicFee => Eip1559,
                        TrxTypeBlob => Eip4844,
                        TrxTypeArbitrumDeposit => unimplemented!(),
                        TrxTypeArbitrumUnsigned => unimplemented!(),
                        TrxTypeArbitrumContract => unimplemented!(),
                        TrxTypeArbitrumRetry => unimplemented!(),
                        TrxTypeArbitrumSubmitRetryable => unimplemented!(),
                        TrxTypeArbitrumInternal => unimplemented!(),
                        TrxTypeArbitrumLegacy => unimplemented!(),
                        TrxTypeOptimismDeposit => unimplemented!(),
                    }
                }
            }
        }
    }
}
pub mod bstream {
    pub mod v1 {
        tonic::include_proto!("sf.bstream.v1");
    }
}

pub mod firehose {
    pub mod v2 {
        use prost::Message;

        use crate::{ethereum::r#type::v2::Block, StreamingFastProtosError};

        tonic::include_proto!("sf.firehose.v2");

        impl TryFrom<SingleBlockResponse> for Block {
            type Error = StreamingFastProtosError;

            fn try_from(response: SingleBlockResponse) -> Result<Self, Self::Error> {
                let any = response.block.ok_or(StreamingFastProtosError::NullBlock)?;
                let block = Block::decode(any.value.as_ref())?;
                Ok(block)
            }
        }

        impl TryFrom<Response> for Block {
            type Error = StreamingFastProtosError;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                let any = response.block.ok_or(StreamingFastProtosError::NullBlock)?;
                let block = Block::decode(any.value.as_ref())?;
                Ok(block)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum StreamingFastProtosError {
    #[error("Block conversion error")]
    BlockConversionError,

    #[error("Error in decoding block: {0}")]
    DecodeError(#[from] prost::DecodeError),

    #[error("Null block field in block response")]
    NullBlock,
}
