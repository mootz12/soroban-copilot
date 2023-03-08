use soroban_sdk::{Address, Env};

use crate::{errors::TokenError, storage};

pub fn verify_admin(e: &Env, user: &Address) -> Result<(), TokenError> {
    let admin = storage::read_admin(e);
    if admin != user.clone() {
        return Err(TokenError::UnauthorizedError);
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::{Address as _, BytesN as _},
        BytesN,
    };

    use super::*;

    #[test]
    fn test_verify_admin() {
        let e = Env::default();

        let token_id = BytesN::<32>::random(&e);

        let admin = Address::random(&e);
        let not_admin = Address::random(&e);

        e.as_contract(&token_id, || {
            storage::write_admin(&e, &admin);

            let is_admin_result = verify_admin(&e, &admin);
            assert_eq!(is_admin_result, Ok(()));

            let is_not_admin_result = verify_admin(&e, &not_admin);
            assert_eq!(is_not_admin_result, Err(TokenError::UnauthorizedError));
        });
    }
}
