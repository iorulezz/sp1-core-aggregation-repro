# SP1 v6 Core Aggregation Repro

This is a minimal standalone repro for the SP1 v6 CPU prover failure we are hitting when:

1. generating an inner proof in `Compressed` mode,
2. aggregating that proof inside a second zkVM program via `verify_sp1_proof`, and
3. generating the outer proof in `Core` mode.

The structure intentionally mirrors the SP1 example layout:

- `program/fibonacci`: inner guest program
- `program/aggregator`: outer guest program that verifies the inner proof
- `script`: host crate that builds both guests and runs the repro

## Prerequisites

- SP1 toolchain and build dependencies installed locally
- the `succinct` Rust toolchain available for building guest programs

## Run

```bash
RUST_LOG=info cargo run --release --manifest-path script/Cargo.toml
```

## Expected failure signature

The inner Fibonacci proof should succeed, then the outer `Core` proof should fail with SP1 logs similar to:

```text
ERROR Controller: task failed: Fatal(io error: unexpected end of file)
...
error=An unexpected error occurred: artifact not found
```

This repro is intentionally minimal and modeled on the same deferred-proof aggregation flow currently used in Delta.
