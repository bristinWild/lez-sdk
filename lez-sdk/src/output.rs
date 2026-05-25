//! Instruction output types.

use nssa_core::account::AccountWithMetadata;
use nssa_core::program::{AccountPostState, ChainedCall};

/// Output from a LEZ program instruction handler.
///
/// Contains the post-states of all accounts modified by the instruction,
/// plus any chained CPI calls.
#[derive(Debug)]
pub struct SdkOutput {
    pub post_states: Vec<AccountPostState>,
    pub chained_calls: Vec<ChainedCall>,
}

impl SdkOutput {
    /// Create output from a list of accounts.
    /// Accounts are converted to post-states without claims.
    pub fn new(accounts: Vec<AccountWithMetadata>) -> Self {
        Self {
            post_states: accounts
                .into_iter()
                .map(|a| AccountPostState::new(a.account))
                .collect(),
            chained_calls: vec![],
        }
    }

    /// Create output with chained CPI calls.
    pub fn with_calls(mut self, calls: Vec<ChainedCall>) -> Self {
        self.chained_calls = calls;
        self
    }

    /// Create empty output (no account mutations).
    pub fn empty() -> Self {
        Self {
            post_states: vec![],
            chained_calls: vec![],
        }
    }
}
