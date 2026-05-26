//! # lez-sdk
//!
//! A minimal Rust SDK for writing LEZ programs.
//!
//! ## Design goals
//!
//! - Remove entrypoint and routing boilerplate
//! - Explicit CPI construction — no hidden account fetching
//! - No IDL generation, no account schemas, no framework DSL
//! - Inspectable via `cargo expand`
//!
//! ## Example
//!
//! ```rust,ignore
//! use lez_sdk::prelude::*;
//!
//! #[lez_sdk::program]
//! mod counter {
//!     #[lez_sdk::function]
//!     pub fn increment(
//!         account: AccountWithMetadata,
//!         amount: u64,
//!     ) -> SdkResult {
//!         Ok(SdkOutput::new(vec![account]))
//!     }
//! }
//! ```

pub use lez_sdk_macros::{function, program};

pub mod cpi;
pub mod error;
pub mod output;
pub mod router;

pub mod prelude {
    pub use crate::error::{SdkError, SdkResult};
    pub use crate::output::SdkOutput;
    pub use crate::router::InstructionRouter;
    pub use crate::{function, program};
    pub use borsh::{BorshDeserialize, BorshSerialize};
    pub use nssa_core::account::{Account, AccountId, AccountWithMetadata};
    pub use nssa_core::program::{AccountPostState, ProgramId};
}
