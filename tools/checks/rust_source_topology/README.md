# Rust Source Topology Check

Check-only Rust source parser for neutral, single-file syntax topology facts.

```text
one Rust source file
  -> syn parser
  -> items + ExprCall + ExprMethodCall
  -> neutral RustSourceTopologyV1 JSON
```

## Boundary

This crate owns syntax observation only. It does not own:

```text
FINALIZE0 entry families
semantic-operation classification
route or boundary policy
Cargo module inclusion
cfg evaluation
Rust name/type resolution
runtime observation
compiler/runtime behavior
```

Every call is therefore emitted with a typed unresolved reason in S0a.
Resolution, Cargo topology, and cfg profiles are later rows. Unsupported call
shapes must remain explicit; spelling heuristics are forbidden. Macro bodies,
`include!`, and external modules are not expanded; each remains a typed opaque
site so an unparsed surface cannot become a false zero-call result.

The tool is a standalone workspace so its parser dependencies do not enter the
root compiler workspace.

## Usage

```bash
cargo run --manifest-path tools/checks/rust_source_topology/Cargo.toml -- \
  single-file src/mir/compiler/mod.rs \
  --module-syntax-path hakorune::mir::compiler
```

Output is written to stdout. The syntax path, half-open byte range, source
slice, FNV-1a diagnostic digest, and callee syntax are neutral observations.
They are not semantic identity or resolution authority. Source reorder may
change report-local IDs and ranges.

## S0a guarantees

```text
parser-backed single-file item inventory
ExprCall / ExprMethodCall distinction
enclosing item syntax path/id
half-open byte range + source-slice digest
enclosing item and call-local cfg/cfg_attr syntax
typed unresolved projection for every observed call
typed opaque rows for macros, include!, and external modules
deterministic source-order JSON
```

## Disconnected project profile layer

`project` now owns the first disconnected S0b prerequisite:

```text
explicit profile request JSON
  -> structurally validated, deterministically ordered profile inputs
  -> pure three-valued cfg / cfg_attr decisions
```

The six initial inputs distinguish host dev, VM-reference, LLVM-harness,
wasm32 dev, unit-test library, and host release. Their expected activated
features are input assertions only. CARGO0 must compare them with Cargo's
actual feature closure before they become compile-unit evidence.

The cfg decision consumes an explicit environment and never reads Cargo,
rustc, process environment, source filenames, or FINALIZE0 policy. Unsupported
custom flags and unsealed target features return `Unknown`; malformed syntax
returns a typed error. The target matcher currently uses cfg-expr's built-in
target database. CARGO0 must add exact rustc/config fingerprints before any
repository inclusion claim.

## Stop lines

```text
no FINALIZE0 names or policy in this crate
no filename-based cfg classification
no alias/method/type inference
no macro expansion
no guessed resolved def-path
no claim that syntax paths are Rust semantic def-paths
no active/excluded cfg or production classification
no Cargo/profile CLI consumer before CARGO0
no root workspace dependency change
no source/check file at or above 800 lines
```
