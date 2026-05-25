//! CPI Example — Program A calls Program B.
//!
//! Demonstrates cross-program invocation using lez-sdk's CpiCall builder.
//!
//! ## Programs
//!
//! - `vault` (Program B) — stores a u64 balance, exposes `deposit` instruction
//! - `token_gate` (Program A) — validates a condition then calls vault::deposit via CPI

use borsh::{BorshDeserialize, BorshSerialize};
use lez_sdk::cpi::CpiCall;
use lez_sdk::prelude::*;
use serde::Serialize;

// Program B: vault

/// Vault state stored in an account.
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct VaultState {
    pub balance: u64,
}

/// Vault instruction enum for CPI serialization.
#[derive(Serialize)]
pub enum VaultInstruction {
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

#[lez_sdk::program]
mod vault {
    use super::*;

    /// Deposit funds into the vault.
    #[lez_sdk::function]
    pub fn deposit(vault_account: AccountWithMetadata, _amount: u64) -> SdkResult {
        Ok(SdkOutput::new(vec![vault_account]))
    }

    /// Withdraw funds from the vault.
    #[lez_sdk::function]
    pub fn withdraw(vault_account: AccountWithMetadata, _amount: u64) -> SdkResult {
        Ok(SdkOutput::new(vec![vault_account]))
    }
}

pub fn vault_router() -> lez_sdk::router::InstructionRouter {
    lez_sdk::router::InstructionRouter::new()
        .register(0, |accounts, data| {
            #[derive(BorshDeserialize)]
            struct Args {
                amount: u64,
            }
            let args =
                Args::try_from_slice(data).map_err(|e| SdkError::DecodeError(e.to_string()))?;
            let vault_account =
                accounts
                    .into_iter()
                    .next()
                    .ok_or_else(|| SdkError::AccountCountMismatch {
                        expected: 1,
                        actual: 0,
                    })?;
            vault::deposit(vault_account, args.amount)
        })
        .register(1, |accounts, data| {
            #[derive(BorshDeserialize)]
            struct Args {
                amount: u64,
            }
            let args =
                Args::try_from_slice(data).map_err(|e| SdkError::DecodeError(e.to_string()))?;
            let vault_account =
                accounts
                    .into_iter()
                    .next()
                    .ok_or_else(|| SdkError::AccountCountMismatch {
                        expected: 1,
                        actual: 0,
                    })?;
            vault::withdraw(vault_account, args.amount)
        })
}

// Program A: token_gate

/// token_gate instruction: validates caller then CPI calls vault::deposit.
#[lez_sdk::program]
mod token_gate {
    use super::*;

    /// Gate a deposit — checks authorization then calls vault via CPI.
    #[lez_sdk::function]
    pub fn gated_deposit(
        caller: AccountWithMetadata,
        vault_account: AccountWithMetadata,
        vault_program_id: ProgramId,
        amount: u64,
    ) -> SdkResult {
        // Authorization check — explicit, no hidden magic
        if !caller.is_authorized {
            return Err(SdkError::Unauthorized(
                "caller is not authorized".to_string(),
            ));
        }

        // Build CPI call to vault::deposit — explicit account and data passing
        let cpi_instruction = VaultInstruction::Deposit { amount };
        let cpi_call = CpiCall::new(vault_program_id)
            .with_accounts(vec![vault_account])
            .build(&cpi_instruction);

        Ok(SdkOutput::new(vec![caller]).with_calls(vec![cpi_call]))
    }
}

pub fn token_gate_router(vault_program_id: ProgramId) -> lez_sdk::router::InstructionRouter {
    lez_sdk::router::InstructionRouter::new().register(0, move |accounts, data| {
        #[derive(BorshDeserialize)]
        struct Args {
            vault_program_id: [u8; 32],
            amount: u64,
        }
        let args = Args::try_from_slice(data).map_err(|e| SdkError::DecodeError(e.to_string()))?;
        let _ = args.vault_program_id; // program id passed via closure
        let mut iter = accounts.into_iter();
        let caller = iter.next().ok_or_else(|| SdkError::AccountCountMismatch {
            expected: 2,
            actual: 0,
        })?;
        let vault_account = iter.next().ok_or_else(|| SdkError::AccountCountMismatch {
            expected: 2,
            actual: 1,
        })?;
        token_gate::gated_deposit(caller, vault_account, vault_program_id, args.amount)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nssa_core::account::{Account, AccountId, AccountWithMetadata};

    fn make_account(id: u8, authorized: bool) -> AccountWithMetadata {
        AccountWithMetadata {
            account_id: AccountId::new([id; 32]),
            account: Account::default(),
            is_authorized: authorized,
        }
    }

    fn vault_program_id() -> ProgramId {
        [1u32; 8]
    }

    #[test]
    fn vault_deposit_works() {
        let vault = make_account(1, false);
        let result = vault::deposit(vault, 100);
        assert!(result.is_ok());
    }

    #[test]
    fn vault_withdraw_works() {
        let vault = make_account(1, false);
        let result = vault::withdraw(vault, 50);
        assert!(result.is_ok());
    }

    #[test]
    fn gated_deposit_authorized_caller_succeeds() {
        let caller = make_account(1, true);
        let vault = make_account(2, false);
        let result = token_gate::gated_deposit(caller, vault, vault_program_id(), 100);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.chained_calls.len(), 1);
    }

    #[test]
    fn gated_deposit_unauthorized_caller_rejected() {
        let caller = make_account(1, false);
        let vault = make_account(2, false);
        let result = token_gate::gated_deposit(caller, vault, vault_program_id(), 100);
        assert!(matches!(result, Err(SdkError::Unauthorized(_))));
    }

    #[test]
    fn gated_deposit_produces_cpi_call() {
        let caller = make_account(1, true);
        let vault = make_account(2, false);
        let result = token_gate::gated_deposit(caller, vault, vault_program_id(), 250);
        let output = result.unwrap();
        assert_eq!(
            output.chained_calls.len(),
            1,
            "should produce exactly one CPI call to vault"
        );
    }

    #[test]
    fn vault_router_dispatches_deposit() {
        let r = vault_router();
        let args = borsh::to_vec(&(100u64,)).unwrap();
        let result = r.dispatch(0, vec![make_account(1, false)], &args);
        assert!(result.is_ok());
    }

    #[test]
    fn vault_router_rejects_unknown() {
        let r = vault_router();
        let result = r.dispatch(99, vec![make_account(1, false)], &[]);
        assert!(matches!(result, Err(SdkError::UnknownInstruction(99))));
    }
}
