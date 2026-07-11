# P381GS Callsite Canonicalize Dead Null Sentinel Helper

Date: 2026-05-06
Scope: remove one unused callsite-canonicalize helper.

## Context

`cargo check --bin hakorune` reported
`callsite_canonicalize/helpers.rs::collect_const_null_sentinels` as dead code.

That helper was local to the callsite canonicalize pass and had no callers. The
active null-sentinel scan used by global-call routing lives separately in
`src/mir/global_call_route_plan.rs`.

## Change

- Removed the unused callsite-canonicalize `collect_const_null_sentinels`
  helper.
- Did not share or move the global-call route-plan scanner.

## Result

Observed `cargo check --bin hakorune` warning count:

```text
before: 33 warnings
after:  32 warnings
```

This is dead helper cleanup only. It does not change callsite canonicalization or
global-call route planning.

## Validation

```bash
cargo check --bin hakorune
cargo test -q callsite_canonicalize --lib
```
