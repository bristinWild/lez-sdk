#![feature(prelude_import)]
//! Counter program — minimal LEZ SDK example.
//!
//! Demonstrates:
//! - `#[lez_sdk::program]` module annotation
//! - `#[lez_sdk::function]` instruction annotation
//! - Explicit account handling
//! - Borsh argument decoding
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use lez_sdk::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};
/// Counter state stored in an account.
pub struct CounterState {
    pub value: u64,
}
#[automatically_derived]
impl borsh::ser::BorshSerialize for CounterState {
    fn serialize<__W: borsh::io::Write>(
        &self,
        writer: &mut __W,
    ) -> ::core::result::Result<(), borsh::io::Error> {
        borsh::BorshSerialize::serialize(&self.value, writer)?;
        Ok(())
    }
}
#[automatically_derived]
impl borsh::de::BorshDeserialize for CounterState {
    fn deserialize_reader<__R: borsh::io::Read>(
        reader: &mut __R,
    ) -> ::core::result::Result<Self, borsh::io::Error> {
        Ok(Self {
            value: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CounterState {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "CounterState",
            "value",
            &&self.value,
        )
    }
}
#[automatically_derived]
impl ::core::default::Default for CounterState {
    #[inline]
    fn default() -> CounterState {
        CounterState {
            value: ::core::default::Default::default(),
        }
    }
}
/// Instruction arguments for increment.
pub struct IncrementArgs {
    pub amount: u64,
}
#[automatically_derived]
impl borsh::ser::BorshSerialize for IncrementArgs {
    fn serialize<__W: borsh::io::Write>(
        &self,
        writer: &mut __W,
    ) -> ::core::result::Result<(), borsh::io::Error> {
        borsh::BorshSerialize::serialize(&self.amount, writer)?;
        Ok(())
    }
}
#[automatically_derived]
impl borsh::de::BorshDeserialize for IncrementArgs {
    fn deserialize_reader<__R: borsh::io::Read>(
        reader: &mut __R,
    ) -> ::core::result::Result<Self, borsh::io::Error> {
        Ok(Self {
            amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for IncrementArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "IncrementArgs",
            "amount",
            &&self.amount,
        )
    }
}
pub mod counter {
    use super::*;
    /// Increment the counter by the given amount.
    pub fn increment(counter: AccountWithMetadata, _amount: u64) -> SdkResult {
        Ok(
            SdkOutput::new(
                ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [counter],
                    ),
                ),
            ),
        )
    }
    /// Reset the counter to zero.
    pub fn reset(counter: AccountWithMetadata) -> SdkResult {
        Ok(
            SdkOutput::new(
                ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [counter],
                    ),
                ),
            ),
        )
    }
}
/// Build the instruction router for this program.
pub fn router() -> lez_sdk::router::InstructionRouter {
    lez_sdk::router::InstructionRouter::new()
        .register(
            0,
            |accounts, data| {
                let args = IncrementArgs::try_from_slice(data)
                    .map_err(|e| SdkError::DecodeError(e.to_string()))?;
                let counter = accounts
                    .into_iter()
                    .next()
                    .ok_or_else(|| SdkError::AccountCountMismatch {
                        expected: 1,
                        actual: 0,
                    })?;
                counter::increment(counter, args.amount)
            },
        )
        .register(
            1,
            |accounts, _data| {
                let counter = accounts
                    .into_iter()
                    .next()
                    .ok_or_else(|| SdkError::AccountCountMismatch {
                        expected: 1,
                        actual: 0,
                    })?;
                counter::reset(counter)
            },
        )
}
