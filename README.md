# lez-sdk

A minimal Rust SDK for writing LEZ (Logos Execution Zone) programs.

## Design Goals

- Remove entrypoint and routing boilerplate
- Explicit CPI construction — no hidden account fetching
- No IDL generation, no account schemas, no framework DSL
- Inspectable via `cargo expand`
- Close to idiomatic Rust — small surface area

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
lez-sdk = { git = "https://github.com/bristinWild/lez-sdk.git" }
```

## Example

```rust
use lez_sdk::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IncrementArgs { pub amount: u64 }

#[lez_sdk::program]
mod counter {
    use super::*;

    #[lez_sdk::function]
    pub fn increment(
        counter: AccountWithMetadata,
        _amount: u64,
    ) -> SdkResult {
        Ok(SdkOutput::new(vec![counter]))
    }
}

pub fn router() -> lez_sdk::router::InstructionRouter {
    lez_sdk::router::InstructionRouter::new()
        .register(0, |accounts, data| {
            let args = IncrementArgs::try_from_slice(data)
                .map_err(|e| SdkError::DecodeError(e.to_string()))?;
            let counter = accounts.into_iter().next()
                .ok_or_else(|| SdkError::AccountCountMismatch {
                    expected: 1, actual: 0
                })?;
            counter::increment(counter, args.amount)
        })
}
```

## Crate Structure
lez-sdk/                  — Core types: SdkOutput, SdkError, SdkResult
lez-sdk-macros/           — Proc macros: #[program], #[function]
examples/
counter/                — Hello-world counter program
cpi-example/            — Cross-program invocation (Program A calls Program B)

## Key Concepts

### Instruction Routing

Routing is explicit — you register handlers by discriminant:

```rust
let router = InstructionRouter::new()
    .register(0, |accounts, data| my_program::increment(accounts, data))
    .register(1, |accounts, data| my_program::reset(accounts, data));

router.dispatch(discriminant, accounts, data)?;
```

### CPI (Cross-Program Invocation)

CPI is explicit — accounts and instruction data are always passed directly:

```rust
let cpi_call = CpiCall::new(target_program_id)
    .with_accounts(vec![vault_account])
    .build(&VaultInstruction::Deposit { amount: 100 });

Ok(SdkOutput::empty().with_calls(vec![cpi_call]))
```

### What This SDK Does NOT Do

- No IDL generation
- No account schemas or derive systems
- No hidden reflection-based dispatch
- No implicit account fetching or mutation rules
- No framework DSL

These are explicit non-goals. Use [SPEL](https://github.com/logos-co/spel) if you need a full framework.

## Running Tests

```bash
cargo test --workspace
```

## License

Licensed under MIT or Apache-2.0.
