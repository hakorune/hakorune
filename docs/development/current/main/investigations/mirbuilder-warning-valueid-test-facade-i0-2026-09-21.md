---
Status: closed__2026-09-21__WarningValueIdTestFacade
Task: MIRBUILDER-WARNING-VALUEID-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i40-2026-09-21.md
Implementation permission: true for one cfg(test) ValueId import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I41
---

# Warning cleanup: ValueId test facade

## Six-line brief

```text
Decision: split the common-V2 textref bridge import and gate ValueId under
  cfg(test); keep BasicBlockId and all bridge owners unconditional.
Source authority + canonical issuer: common_v2_s6c_textref_entry_bridge.rs;
  the test helper is the only ValueId consumer in this module.
Non-authority: cargo-fix, wildcard imports, test removal, bridge semantics,
  physical lane ownership, or warning guesses.
Fail-fast boundary: any production ValueId consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: split the import, run the fixed quick-profile gates once.
Non-claims: no bridge redesign, suppression, semantic refactor, or cutover.
```

## Preconditions and acceptance

I40 records a single lib-only unused import at
`src/mir/builder/resolved_lowering/common_v2_s6c_textref_entry_bridge.rs:11`.
The only `ValueId` uses are in the `#[cfg(test)]` module; production retains
`BasicBlockId`.

Acceptance requires lib warnings to drop from **1,792 to 1,791**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the import split and cfg annotation may change.

## Closeout evidence

The sole permitted source edit split the import in
`src/mir/builder/resolved_lowering/common_v2_s6c_textref_entry_bridge.rs`:
`BasicBlockId` remains unconditional and `ValueId` is gated by `#[cfg(test)]`.
The fixed gates completed sequentially with exit 0:

* lib: 1,791 warnings — `/tmp/hakorune-warning-i0-valueid-test-facade-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-valueid-test-facade-lib-test-20260921.log`

No bridge semantics, physical lane owner, or production route changed.
