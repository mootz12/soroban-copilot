use soroban_sdk::{contracttype, Address, Bytes, Env};

use crate::errors::TokenError;

/********** Storage Types **********/

#[derive(Clone)]
#[contracttype]
pub struct Balance {
    pub amount: i128,
    pub authorized: bool,
}

impl Balance {
    pub fn verify_authorization(&self) -> Result<(), TokenError> {
        match self.authorized {
            true => Ok(()),
            _ => Err(TokenError::BalanceDeauthorizedError),
        }
    }
}

/********** Storage Key Types **********/

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum TokenDataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Admin,
    Decimals,
    Name,
    Symbol,
}

/********** Storage Helpers **********/

/***** Allowance *****/

pub fn read_allowance(e: &Env, from: &Address, spender: &Address) -> i128 {
    let key = TokenDataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });
    e.storage()
        .get::<TokenDataKey, i128>(&key)
        .unwrap_or(Ok(0))
        .unwrap()
}

pub fn write_allowance(e: &Env, from: &Address, spender: &Address, amount: &i128) {
    let key = TokenDataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });
    e.storage().set::<TokenDataKey, i128>(&key, amount)
}

/***** Balance *****/

pub fn read_balance(e: &Env, user: &Address) -> Balance {
    let key = TokenDataKey::Balance(user.clone());
    // addresses are authorized by default
    e.storage()
        .get::<TokenDataKey, Balance>(&key)
        .unwrap_or(Ok(Balance {
            amount: 0,
            authorized: true,
        }))
        .unwrap()
}

pub fn write_balance(e: &Env, user: &Address, balance: &Balance) {
    let key = TokenDataKey::Balance(user.clone());
    e.storage().set::<TokenDataKey, Balance>(&key, balance)
}

/***** Admin *****/

pub fn read_admin(e: &Env) -> Address {
    e.storage()
        .get_unchecked::<TokenDataKey, Address>(&TokenDataKey::Admin)
        .unwrap()
}

pub fn has_admin(e: &Env) -> bool {
    e.storage().has::<TokenDataKey>(&TokenDataKey::Admin)
}

pub fn write_admin(e: &Env, admin: &Address) {
    e.storage()
        .set::<TokenDataKey, Address>(&TokenDataKey::Admin, admin)
}

/***** Decimals *****/

pub fn read_decimals(e: &Env) -> u32 {
    e.storage()
        .get_unchecked::<TokenDataKey, u32>(&TokenDataKey::Decimals)
        .unwrap()
}

pub fn write_decimals(e: &Env, decimals: &u32) {
    e.storage()
        .set::<TokenDataKey, u32>(&TokenDataKey::Decimals, decimals)
}

/***** Name *****/

pub fn read_name(e: &Env) -> Bytes {
    e.storage()
        .get_unchecked::<TokenDataKey, Bytes>(&TokenDataKey::Name)
        .unwrap()
}

pub fn write_name(e: &Env, name: &Bytes) {
    e.storage()
        .set::<TokenDataKey, Bytes>(&TokenDataKey::Name, name)
}

/***** Symbol *****/

pub fn read_symbol(e: &Env) -> Bytes {
    e.storage()
        .get_unchecked::<TokenDataKey, Bytes>(&TokenDataKey::Symbol)
        .unwrap()
}

pub fn write_symbol(e: &Env, symbol: &Bytes) {
    e.storage()
        .set::<TokenDataKey, Bytes>(&TokenDataKey::Symbol, symbol)
}
