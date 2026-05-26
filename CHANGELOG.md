# Changelog

All notable changes to `lez-sdk` are documented here.

## [0.1.0] — 2026-05-26

### Added
- `lez-sdk` core crate with `SdkOutput`, `SdkError`, `SdkResult` types
- `lez-sdk-macros` crate with `#[lez_sdk::program]` and `#[lez_sdk::function]` proc macros
- `InstructionRouter` — explicit discriminant-based instruction dispatch
- `CpiCall` builder for cross-program invocation
- `counter` example program with 6 unit tests
- `cpi-example` with vault + token_gate programs demonstrating CPI (Program A calls Program B) — 7 unit tests
- `docs/design.md` — instruction encoding, routing, CPI construction, extension points, non-goals
- `docs/expand-counter.rs` — `cargo expand` output showing macro expansion
- CI workflow via GitHub Actions
- On-chain demo script (`demo.sh`) running against LEZ sequencer with `RISC0_DEV_MODE=0`
- Compile-time overhead measurement documented in design doc
