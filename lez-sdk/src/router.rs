//! Explicit instruction routing.
//!
//! Programs use `InstructionRouter` to map instruction discriminants
//! to handler functions. The router is constructed explicitly — no
//! hidden reflection or magic dispatch.
//!
//! # Example
//!
//! ```rust,ignore
//! let router = InstructionRouter::new()
//!     .register(0, |accounts, data| counter::increment(accounts, data))
//!     .register(1, |accounts, data| counter::decrement(accounts, data));
//!
//! router.dispatch(discriminant, accounts, data)?;
//! ```

use crate::error::{SdkError, SdkResult};
use nssa_core::account::AccountWithMetadata;

/// Type alias for instruction handler functions.
pub type HandlerFn = Box<dyn Fn(Vec<AccountWithMetadata>, &[u8]) -> SdkResult>;

/// Explicit instruction router.
///
/// Maps u32 discriminants to handler functions.
/// All routing is visible and inspectable — no hidden dispatch.
pub struct InstructionRouter {
    handlers: Vec<(u32, HandlerFn)>,
}

impl InstructionRouter {
    /// Create a new empty router.
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }

    /// Register a handler for a given discriminant.
    pub fn register<F>(mut self, discriminant: u32, handler: F) -> Self
    where
        F: Fn(Vec<AccountWithMetadata>, &[u8]) -> SdkResult + 'static,
    {
        self.handlers.push((discriminant, Box::new(handler)));
        self
    }

    /// Dispatch an instruction to the registered handler.
    ///
    /// Returns `SdkError::UnknownInstruction` if no handler
    /// is registered for the given discriminant.
    pub fn dispatch(
        &self,
        discriminant: u32,
        accounts: Vec<AccountWithMetadata>,
        data: &[u8],
    ) -> SdkResult {
        for (d, handler) in &self.handlers {
            if *d == discriminant {
                return handler(accounts, data);
            }
        }
        Err(SdkError::UnknownInstruction(discriminant))
    }
}

impl Default for InstructionRouter {
    fn default() -> Self {
        Self::new()
    }
}
