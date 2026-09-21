---
Status: closed__2026-09-21__WarningCommonV2SessionTestFacade
Task: MIRBUILDER-WARNING-COMMON-V2-SESSION-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i42-2026-09-21.md
Implementation permission: true for two cfg(test) common-V2 session imports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I43
---

# Warning cleanup: common-V2 session test facade

## Six-line brief

```text
Decision: gate MirBuilder and ReadyFunctionDraftSealV1 imports used only by
  the common-V2 session test close seam.
Source authority + canonical issuer: common_v2_session/mod.rs; production methods
  retain fully qualified MirBuilder and existing session owners.
Non-authority: cargo-fix, wildcard imports, draft-seal redesign, session
  ownership, or warning guesses.
Fail-fast boundary: any production unqualified consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the two imports and run fixed gates once.
Non-claims: no session redesign, physical lowering change, suppression, or cutover.
```

## Preconditions and acceptance

I42 records two lib-only unused imports at
`src/mir/builder/resolved_lowering/common_v2_session/mod.rs:15,24`. Both are
used only by the `#[cfg(test)]` finish seam.

Acceptance requires lib warnings to drop from **1,790 to 1,788**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the two import annotations may change.

## Closeout evidence

The sole permitted source edit gated the two common-V2 session imports:
`MirBuilder` and `ReadyFunctionDraftSealV1` are both `#[cfg(test)]`; production
methods retain their existing fully qualified builder references. The fixed gates
completed sequentially with exit 0:

* lib: 1,788 warnings — `/tmp/hakorune-warning-i0-common-v2-session-test-facade-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-common-v2-session-test-facade-lib-test-20260921.log`

No session ownership, draft-seal behavior, or physical route changed.
