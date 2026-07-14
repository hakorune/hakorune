# SSA-RC-A1b Rust Owned Forwarding Evidence

Status: Closed
Date: 2026-07-14
Decision: consume the sealed Ownership SSA witness only through an explicit
Rust interpreter function session; keep production canonical callers at zero.

## Boundary

The ordinary interpreter path remains unchanged. The disconnected A1b entry
requires an expected function owner plus `VerifiedOwnershipSsaV1`, rejects a
foreign owner before frame effects, installs the witness for one closure-scoped
function frame, and restores the caller witness on success or error.

```text
parameter/result transport:
  move

Owned Phi:
  select exact predecessor inputs
  reject duplicate selected sources
  take every source register
  publish every destination only after collection succeeds

Owned Return:
  take exact result register

ordinary None/Borrowed Phi and Return:
  unchanged legacy interpreter behavior
```

The first profile remains BoxRef-only. No canonical lowerer, llvm_py, Wasm,
Hako interpreter, or legacy retry path is activated by this row.

## Evidence

```text
cargo test -q --features vm-reference ownership_forwarding_tests -- --nocapture
  3 passed

cargo test -q --features vm-reference frame_transaction_tests -- --nocapture
  6 passed

bash tools/checks/lib/resolved_binding_ssa_contract.sh
  green
```

Focused fixtures cover both diamond predecessors, foreign-owner rejection, and
error-path witness restoration. Every modified source/check file remains below
800 lines.

## Non-claims

```text
canonical ownership production activation
llvm_py or nyash_kernel lifecycle materialization
all backend ownership support
owned calls or edge arguments
String/Array/Future/Opaque ownership
legacy ReleaseStrong retirement
```
