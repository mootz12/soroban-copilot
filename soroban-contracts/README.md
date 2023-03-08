# soroban-contracts
A collection of Soroban contract implementations and interface clients.

## Safety
This is **experimental software** and is provided on an "as is" and "as available" basis.

We do **not give any warranties** and **will not be liable for any loss** incurred through any use of this codebase.

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
soroban-contracts = "<desired version>"
```

### Using a Client
Contract clients are generated from a Rust trait using the [soroban-sdk](https://docs.rs/soroban-sdk/latest/soroban_sdk/attr.contractclient.html). They can be imported and used as follows:

```rust
use stellar_sdk::{BytesN, Env};
use soroban_contracts::token::{TokenClient};

let env = Env::default();
let usdc_address =  BytesN::from_array(&env, &[u8; 0]);
let usdc = TokenClient::new(&env, BytesN::from_array(&env, &usdc_address));

// perform an action against the usdc contract...
```

For each function defined in the trait like `balance`, the client contains a standard `balance` function that obeys the interface and a `try_balance` that wraps the returned value in a `Result` to allow the calling contract to gracefully handle errors if required.

### Implementing a Trait
Traits can be implemented for a contract through a Rust `impl` tag and a [contractimpl](https://docs.rs/soroban-sdk/latest/soroban_sdk/attr.contractimpl.html) attribute.

```rust
use soroban_sdk::{contractimpl};
use soroban_contracts::token::{Token};

pub struct MyContract;

#[contractimpl]
impl Token for MyContract {
    // implement the Token trait based on your contract's needs
}
```

### Deploying a Contact
Implemented contracts expose their optimized WASM bundle. This bundle can be used to deploy to networks or used in tests as shown below:

```rust
use soroban_sdk::{testutils::{BytesN as _}, BytesN, Env};
use soroban_contracts::token::{TokenWASM, TokenClient};

let e = Env::default();
let contract_id = BytesN::<32>::random(&e);
e.register_contract_wasm(&contract_id, TokenWASM);
let token_client = TokenClient::new(e, &contract_id);

// perform an action against the newly deployed token contract...
```

## Supported Contracts
This library only supports contracts that have been agreed upon by the community via a SEP or CAP.

Current:
* token - A standardized token defined by [CAP-0046-06](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046-06.md)

If there are any missing contracts - please file an issue.

## Acknowledgements
This library was inspired by or directly modified from many sources, primary:
- [OpenZeppelin](https://github.com/OpenZeppelin/openzeppelin-contracts)

## WASM
The WASM target `wasm32-unknown-unknown` is supported.

## Contributions
Contributions are welcome. Please check out the contribution guide (TODO)!

## License
This library is released under the [MIT License](../LICENSE).
