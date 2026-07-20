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

## Stop lines

```text
no FINALIZE0 names or policy in this crate
no filename-based cfg classification
no alias/method/type inference
no macro expansion
no guessed resolved def-path
no claim that syntax paths are Rust semantic def-paths
no active/excluded cfg or production classification
no root workspace dependency change
no source/check file at or above 800 lines
```
