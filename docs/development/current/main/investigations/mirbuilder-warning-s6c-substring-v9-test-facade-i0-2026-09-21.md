---
Status: closed__2026-09-21__WarningS6CSubstringV9TestFacade
Task: MIRBUILDER-WARNING-S6C-SUBSTRING-V9-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i43-2026-09-21.md
Implementation permission: true for two cfg(test) S6C Substring V9 imports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I44
---

# Warning cleanup: S6C Substring V9 test facade

## Six-line brief

```text
Decision: gate NonZeroU64 and EndAuthorizedTextV1 imports used only by the
  S6C Substring V9 test helpers.
Source authority + canonical issuer: s6c_substring_v9_issuer.rs; production
  lease reject/consume types remain unconditional.
Non-authority: cargo-fix, wildcard imports, lease redesign, V9 behavior, or
  warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test warning,
  or warning-count mismatch rejects the slice.
Smallest next slice: split the imports and run fixed gates once.
Non-claims: no V9 issuer redesign, runtime lease change, suppression, or cutover.
```

## Preconditions and acceptance

I43 records two lib-only unused imports at
`src/mir/builder/resolved_lowering/common_v2_session/s6c_substring_v9_issuer.rs:8,17`.
Both are referenced only inside the `#[cfg(test)]` helper module.

Acceptance requires lib warnings to drop from **1,788 to 1,786**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the two import annotations may change.

## Closeout evidence

The sole permitted source edit split the S6C V9 imports: `NonZeroU64` and
`EndAuthorizedTextV1` are both `#[cfg(test)]`; production lease reject/consume
types remain unconditional. The fixed gates completed sequentially with exit 0:

* lib: 1,786 warnings — `/tmp/hakorune-warning-i0-s6c-substring-v9-test-facade-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-s6c-substring-v9-test-facade-lib-test-20260921.log`

No V9 issuer validation, runtime lease behavior, or production route changed.
