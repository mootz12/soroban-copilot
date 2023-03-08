/// Implemented in `contract-implementations/token`
mod contract {
    soroban_sdk::contractimport!(file = "./wasm/token.wasm");
}

pub use contract::{Contract as Token, WASM as TokenWASM, Client as TokenClient, TokenError};