# SSA-RC-A1c llvm_py Ownership Materialization Evidence

Status: Closed

Date: 2026-07-14

Decision authority:
`mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md`

## Closed boundary

The llvmlite object lane now consumes one exact transported
`VerifiedOwnershipSsaV1` witness before function-local LLVM effects begin.

```text
CopyOwned:
  strict BoxRef i64 handle
  -> nyrt_handle_retain_h
  -> fresh destination ValueId

DestroyOwned:
  strict BoxRef i64 handle
  -> exact nyrt_handle_release_h

provider:
  nyash_kernel only
```

The Rust emitter transports the sealed owner, closed ownership kinds, and the
exact block/instruction operation inventory. The Python function-scoped
session claims each operation exactly once and rejects foreign, duplicate, or
missing sites.

The ABI and operation inventory are stored inside `VerifiedOwnershipSsaV1`;
the JSON emitter does not reconstruct them from mutable MIR. Backend preflight
reruns the same verifier and requires the full kind/disposition/operation/ABI
product to equal the sealed witness. Post-seal CFG, Phi, Return, or ownership-op
mutation is rejected as a stale witness.

## Fail-fast boundary

The shared Rust backend preflight accepts ownership operations only for
`llvmlite-obj` with an installed sealed witness. Wasm, Wasm V2, PyVM, native
llvmc, and other unproved lanes reject
`owned-value-lifecycle-v1` before backend effects.

The llvm_py PyVM also rejects `copy_owned` and `destroy_owned` explicitly. Its
historical unknown-op skip cannot turn ownership into a silent no-op.

The transported witness pins provider `nyash_kernel`; the root compatibility
shim is not an accepted ownership provider.

## Authority counters

```text
Rust ownership witness transport emitters = 1
production witness installers             = 0
canonical ownership-op callers            = 0
exact BoxRef source producers              = 0
llvm_py ownership handlers                 = 2
PyVM ownership silent skips                = 0
legacy ReleaseStrong semantic changes      = 0
```

This closes backend materialization capability only. It does not activate
canonical source Ownership SSA, change accepted grammar, or make direct JSON
an ownership authority.

## Verification

```text
python llvm_py ownership fixtures                 4/4 green
Rust ownership backend preflight fixtures         3/3 green
Rust MIR JSON sealed-witness transport fixture    1/1 green
backend core-op contract fixtures                21/21 green
nyash_kernel handle lifecycle fixtures            3/3 green
resolved region-flow authority guard              green
cargo build --release --bin hakorune               green
dev_gate quick                                    66/66 green
git diff --check                                  green
all touched source/check files                    <= 672 lines
```

Commands:

```bash
PYTHONPATH=src/llvm_py \
  python3 -m unittest -v src.llvm_py.tests.test_ownership_lowering
cargo test -q --lib ownership_backend_capability -- --nocapture
cargo test -q --lib ownership_transport_round_trips_with_exact_boxref_witness -- --nocapture
cargo test -q --lib backend_core_ops -- --nocapture
cargo test -q -p nyash_kernel handle_lifecycle -- --nocapture
bash tools/checks/resolved_region_flow_authority_guard.sh
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```

## Next row

`SSA-RC-RET-P0` inventories every legacy `ReleaseStrong` producer, consumer,
transport, pass, fixture, and document without changing its meaning. Physical
retirement and canonical ownership activation remain later rows.
