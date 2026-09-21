---
Status: closed__2026-09-21__WarningResolvedLoweringFacadeTestExports
Task: MIRBUILDER-WARNING-RESOLVED-LOWERING-FACADE-TEST-EXPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i46-2026-09-21.md
Implementation permission: true for four cfg(test) resolved-lowering facade groups only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I47
---

# Warning cleanup: resolved-lowering facade test exports

## Six-line brief

```text
Decision: gate four resolved-lowering parent re-export groups and the dependent
  scalar child re-export used only by tests; keep child issuers, session owners,
  and capability implementations unchanged.
Source authority + canonical issuer: each child module under resolved_lowering;
  these parent/child re-exports are only test facades for the names.
Non-authority: cargo-fix, wildcard imports, issuer redesign, physical session
  behavior, capability semantics, or warning guesses.
Fail-fast boundary: any production facade consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the four groups and run fixed gates.
Non-claims: no lowering behavior change, suppression, or production cutover.
```

## Preconditions and acceptance

I46 records four lib-only unused-import diagnostics in
`src/mir/builder/resolved_lowering/mod.rs:67,71,78,92`. The scalar chain also
requires gating `common_v2_session/mod.rs:67`, which becomes unused when its
parent facade is test-only. Selected names have test-only observed consumers;
production paths use child modules directly.

Acceptance requires lib warnings to drop from **1,782 to 1,778**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only these facade cfg annotations/splits may change.

## Closeout evidence

The permitted source edits gated the four resolved-lowering parent facade groups
and `common_v2_session/mod.rs:67` scalar child re-export under `#[cfg(test)]`.
The fixed gates completed sequentially with exit 0:

* lib: 1,778 warnings — `/tmp/hakorune-warning-i0-resolved-lowering-facade-test-exports-lib-20260921.log`
* lib test: 561 warnings — `/tmp/hakorune-warning-i0-resolved-lowering-facade-test-exports-lib-test-20260921.log`

No child issuer, session owner, capability semantic, or production lowering path
changed.
