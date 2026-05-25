//! Cross-Program Invocation (CPI) helpers.
//!
//! Provides explicit CPI construction — accounts and arguments
//! are always passed explicitly. No implicit account fetching.
//!
//! # Example
//!
//! ```rust,ignore
//! #[derive(Serialize)]
//! enum TokenInstruction { Transfer { amount: u64 } }
//!
//! let call = CpiCall::new(token_program_id)
//!     .with_accounts(vec![from, to])
//!     .build(&TokenInstruction::Transfer { amount: 100 });
//!
//! SdkOutput::empty().with_calls(vec![call])
//! ```

use nssa_core::account::AccountWithMetadata;
use nssa_core::program::{ChainedCall, ProgramId};
use serde::Serialize;

/// Builder for a CPI call to another program.
pub struct CpiCall {
    program_id: ProgramId,
    accounts: Vec<AccountWithMetadata>,
}

impl CpiCall {
    /// Create a new CPI call targeting the given program.
    pub fn new(program_id: ProgramId) -> Self {
        Self {
            program_id,
            accounts: vec![],
        }
    }

    /// Set the accounts to pass to the target program.
    pub fn with_accounts(mut self, accounts: Vec<AccountWithMetadata>) -> Self {
        self.accounts = accounts;
        self
    }

    /// Build the ChainedCall with the given serializable instruction.
    pub fn build<I: Serialize>(self, instruction: &I) -> ChainedCall {
        ChainedCall::new(self.program_id, self.accounts, instruction)
    }
}
