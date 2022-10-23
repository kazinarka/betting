pub mod consts;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("8mv1b8uoqYW3T7tVJKb3BJnVuCZCCEHcpoa3c7Grq26c");

pub type Timestamp = u64;
