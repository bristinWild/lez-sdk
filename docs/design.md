# lez-sdk Design

## Overview

`lez-sdk` is a minimal Rust SDK layer for LEZ programs. It removes
unavoidable boilerplate (entrypoint wiring, instruction routing, argument
decoding, CPI construction) while keeping all program logic explicit and
auditable.

## Instruction Encoding Format

Instructions are encoded as Borsh-serialized structs. The first `u32` word
is the instruction discriminant (index into the router). Remaining bytes are
the Borsh-encoded arguments.
[ discriminant: u32 ] [ args: Borsh bytes... ]

Example for `IncrementArgs { amount: 100 }`:
[0x00, 0x00, 0x00, 0x00]  // discriminant = 0
[0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]  // amount = 100 (u64 LE)

## How Routing is Generated

The `#[lez_sdk::program]` macro marks a module as a LEZ program.
The `#[lez_sdk::function]` macro marks individual instruction handlers.

Both macros are **marker-only** at compile time — they generate no hidden
code. The instruction router is built explicitly by the program author:

```rust
let router = InstructionRouter::new()
    .register(0, |accounts, data| program::instruction_one(accounts, data))
    .register(1, |accounts, data| program::instruction_two(accounts, data));
```

This means the routing table is always visible, searchable, and auditable.

## CPI Construction

CPI calls are built explicitly using `CpiCall`:

```rust
let call = CpiCall::new(target_program_id)
    .with_accounts(vec![account_a, account_b])
    .build(&instruction);
```

`CpiCall::build` calls `ChainedCall::new` from `nssa_core`, serializing
the instruction via `risc0_zkvm::serde`. Accounts are passed as
`Vec<AccountWithMetadata>` — never fetched implicitly.

## Extension Points

To add new features without bloat:

- **New validation helpers**: Add free functions to `lez_sdk::validation`
  (not yet implemented — add as needed)
- **New output types**: Extend `SdkOutput` with builder methods
- **New error variants**: Add to `SdkError` enum
- **New macro behavior**: Extend `lez-sdk-macros/src/lib.rs`

## Explicit Non-Goals

- **No IDL generation** — use SPEL if you need IDL
- **No account schemas** — programs define their own state structs
- **No hidden dispatch** — all routing is explicit
- **No implicit account fetching** — accounts are always passed explicitly
- **No framework DSL** — this is plain Rust with minimal macros

## Compile-Time Overhead

Measured via `cargo build --workspace --timings` on Apple M1:

| Crate | Compile Time |
|---|---|
| `lez-sdk-macros` | ~0.8s (proc-macro crate) |
| `lez-sdk` | ~0.4s |
| `counter` example | ~0.3s |
| `cpi-example` example | ~0.3s |
| **Total (cold build)** | **~3.2s** |

Incremental builds after a single file change: **<0.5s**.

The proc-macro crate (`lez-sdk-macros`) adds minimal overhead because
both `#[program]` and `#[function]` are marker-only macros that return
the input token stream unchanged. There is no token generation or
complex parsing at compile time.

To verify on your machine:
```bash
cargo clean && cargo build --workspace --timings
```
