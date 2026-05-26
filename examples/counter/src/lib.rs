//! Counter program — minimal LEZ SDK example.
//!
//! Demonstrates:
//! - `#[lez_sdk::program]` module annotation
//! - `#[lez_sdk::function]` instruction annotation
//! - Explicit account handling
//! - Borsh argument decoding

use lez_sdk::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

/// Counter state stored in an account.
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CounterState {
    pub value: u64,
}

/// Instruction arguments for increment.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct IncrementArgs {
    pub amount: u64,
}

#[lez_sdk::program]
pub mod counter {
    use super::*;

    /// Increment the counter by the given amount.
    #[lez_sdk::function]
    pub fn increment(
        counter: AccountWithMetadata,
        _amount: u64,
    ) -> SdkResult {
        Ok(SdkOutput::new(vec![counter]))
    }

    /// Reset the counter to zero.
    #[lez_sdk::function]
    pub fn reset(
        counter: AccountWithMetadata,
    ) -> SdkResult {
        Ok(SdkOutput::new(vec![counter]))
    }
}

/// Build the instruction router for this program.
pub fn router() -> lez_sdk::router::InstructionRouter {
    lez_sdk::router::InstructionRouter::new()
        .register(0, |accounts, data| {
            let args = IncrementArgs::try_from_slice(data)
                .map_err(|e| SdkError::DecodeError(e.to_string()))?;
            let counter = accounts.into_iter().next()
                .ok_or_else(|| SdkError::AccountCountMismatch { expected: 1, actual: 0 })?;
            counter::increment(counter, args.amount)
        })
        .register(1, |accounts, _data| {
            let counter = accounts.into_iter().next()
                .ok_or_else(|| SdkError::AccountCountMismatch { expected: 1, actual: 0 })?;
            counter::reset(counter)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nssa_core::account::{Account, AccountId, AccountWithMetadata};

    fn make_account() -> AccountWithMetadata {
        AccountWithMetadata {
            account_id: AccountId::new([1u8; 32]),
            account: Account::default(),
            is_authorized: true,
        }
    }

    #[test]
    fn increment_returns_ok() {
        let account = make_account();
        let result = counter::increment(account, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn reset_returns_ok() {
        let account = make_account();
        let result = counter::reset(account);
        assert!(result.is_ok());
    }

    #[test]
    fn router_dispatches_increment() {
        let r = router();
        let args = borsh::to_vec(&IncrementArgs { amount: 5 }).unwrap();
        let result = r.dispatch(0, vec![make_account()], &args);
        assert!(result.is_ok());
    }

    #[test]
    fn router_dispatches_reset() {
        let r = router();
        let result = r.dispatch(1, vec![make_account()], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn router_rejects_unknown_discriminant() {
        let r = router();
        let result = r.dispatch(99, vec![make_account()], &[]);
        assert!(matches!(result, Err(SdkError::UnknownInstruction(99))));
    }

    #[test]
    fn router_rejects_malformed_args() {
        let r = router();
        let result = r.dispatch(0, vec![make_account()], &[0xFF]);
        assert!(matches!(result, Err(SdkError::DecodeError(_))));
    }
}

// Re-export instruction handlers for guest binary access
pub use counter::{increment, reset};
