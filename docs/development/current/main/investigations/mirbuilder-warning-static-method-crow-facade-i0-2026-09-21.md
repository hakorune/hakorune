---
Status: closed__2026-09-21__StaticMethodCRowFacade__OwnerChainDeleted
Task: MIRBUILDER-WARNING-STATIC-METHOD-CROW-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i67-2026-09-21.md
Implementation permission: true for the bounded facade chain only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I68
---

# MirBuilder static method C-row facade I0

## Six-line brief

```text
Decision: remove one unused parent facade re-export after the I67 refresh.
Source authority + canonical issuer: the compiler published-backend-view owner.
Non-authority: the historical mir/function.rs facade and warning-count guesses.
Fail-fast boundary: any owner-local warning move, unresolved import, or changed
  C-frame consumer stops the slice.
Smallest next slice: delete only PublishedStaticMethodCallCRowV1 from the
  parent mir/function.rs re-export, then run the two fixed quick gates.
Non-claims: no C-frame change, no ABI change, no dead-code cleanup, no route or
  production switch, and no broad warning census.
```

## Evidence and acceptance

I67 refreshed the fixed baseline at lib **1,754** / lib-test **558**. A complete
`rg` census shows `PublishedStaticMethodCallCRowV1` is defined and consumed by
the compiler published-backend-view owner; `src/mir/function.rs` is the sole
unused facade occurrence. The child historical module imports the compiler
owner directly, so the deletion has no owner-local handoff.

The fast slice must run these commands sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

Commit `1e387d9f81` removed the unused C-row re-export from the compiler owner
facade, the historical `mir/function` facade, and the parent `mir/function.rs`
surface. The complete source census now finds the C-row only in its canonical
`c_transport` owner and its v2 consumer; no facade occurrence remains.

Both fixed gates passed sequentially on 2026-09-21: lib generated **1,754**
warnings and lib-test generated **558** warnings, matching the prior diagnostic
totals. The remaining C-frame import shares the former grouped diagnostic, so
the total count does not fall; the C-row item itself is gone and no replacement
C-row warning appears. `cargo fmt --check`, `git diff --check`, and the current
state pointer guard are green. This is a source deletion with stable warning
totals, not a claim of broad warning reduction.
