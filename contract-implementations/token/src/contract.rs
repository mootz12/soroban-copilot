use crate::{admin, allowance, balance, errors::TokenError, events, interface::CAP4606, storage};
use soroban_sdk::{contractimpl, panic_with_error, Address, Bytes, Env};

pub struct Token;

#[contractimpl]
impl CAP4606 for Token {
    fn initialize(e: Env, admin: Address, decimal: u32, name: Bytes, symbol: Bytes) {
        if storage::has_admin(&e) {
            panic_with_error!(&e, TokenError::AlreadyInitializedError)
        }
        storage::write_admin(&e, &admin);

        storage::write_decimals(&e, &decimal);
        storage::write_name(&e, &name);
        storage::write_symbol(&e, &symbol);
    }

    // --------------------------------------------------------------------------------
    // Admin interface – privileged functions.
    // --------------------------------------------------------------------------------

    fn clawback(e: Env, admin: Address, from: Address, amount: i128) {
        admin::verify_admin(&e, &admin).unwrap();
        admin.require_auth();

        verify_nonnegative(&e, amount);
        balance::spend_balance_no_authorization_check(&e, &from, &amount).unwrap();

        events::clawback(&e, admin, from, amount);
    }

    fn mint(e: Env, admin: Address, to: Address, amount: i128) {
        admin::verify_admin(&e, &admin).unwrap();
        admin.require_auth();

        verify_nonnegative(&e, amount);
        balance::receive_balance(&e, &to, &amount).unwrap();

        events::mint(&e, admin, to, amount);
    }

    fn set_admin(e: Env, admin: Address, new_admin: Address) {
        admin::verify_admin(&e, &admin).unwrap();
        admin.require_auth();

        storage::write_admin(&e, &new_admin);

        events::set_admin(&e, admin, new_admin);
    }

    fn set_auth(e: Env, admin: Address, id: Address, authorize: bool) {
        admin::verify_admin(&e, &admin).unwrap();
        admin.require_auth();

        balance::update_balance_authorization(&e, &id, authorize).unwrap();

        events::set_auth(&e, admin, id, authorize);
    }

    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------

    fn incr_allow(e: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        verify_nonnegative(&e, amount);
        allowance::increase_allowance(&e, &from, &spender, &amount).unwrap();

        events::incr_allow(&e, from, spender, amount);
    }

    fn decr_allow(e: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        verify_nonnegative(&e, amount);
        allowance::decrease_allowance(&e, &from, &spender, &amount).unwrap();

        events::decr_allow(&e, from, spender, amount);
    }

    fn xfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        verify_nonnegative(&e, amount);
        balance::spend_balance(&e, &from, &amount).unwrap();
        balance::receive_balance(&e, &to, &amount).unwrap();

        events::transfer(&e, from, to, amount);
    }

    fn xfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        from.require_auth();

        verify_nonnegative(&e, amount);
        allowance::spend_allowance(&e, &from, &spender, &amount).unwrap();
        balance::spend_balance(&e, &from, &amount).unwrap();
        balance::receive_balance(&e, &to, &amount).unwrap();

        events::transfer(&e, from, to, amount);
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        verify_nonnegative(&e, amount);
        balance::spend_balance(&e, &from, &amount).unwrap();

        events::burn(&e, from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        from.require_auth();

        verify_nonnegative(&e, amount);
        allowance::spend_allowance(&e, &from, &spender, &amount).unwrap();
        balance::spend_balance(&e, &from, &amount).unwrap();

        events::burn(&e, from, amount);
    }

    // --------------------------------------------------------------------------------
    // Read-only Token interface
    // --------------------------------------------------------------------------------

    fn balance(e: Env, id: Address) -> i128 {
        storage::read_balance(&e, &id).amount
    }

    fn spendable(e: Env, id: Address) -> i128 {
        storage::read_balance(&e, &id).amount
    }

    fn authorized(e: Env, id: Address) -> bool {
        let authorization_res = storage::read_balance(&e, &id).verify_authorization();
        match authorization_res {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        storage::read_allowance(&e, &from, &spender)
    }

    // --------------------------------------------------------------------------------
    // Descriptive Interface
    // --------------------------------------------------------------------------------

    fn decimals(e: Env) -> u32 {
        storage::read_decimals(&e)
    }

    fn name(e: Env) -> Bytes {
        storage::read_name(&e)
    }

    fn symbol(e: Env) -> Bytes {
        storage::read_symbol(&e)
    }
}

fn verify_nonnegative(e: &Env, amount: i128) {
    if amount.is_negative() {
        panic_with_error!(e, TokenError::NegativeAmountError);
    }
}
