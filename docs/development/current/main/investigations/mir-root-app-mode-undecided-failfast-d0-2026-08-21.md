---
Status: open design stop — complete root-mode state/caller audit
Date: 2026-08-21
Decision: MIR-ROOT-APP-MODE-UNDECIDED-FAILFAST-D0
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: existing non-Main static-Box lifecycle only; no new caller
ReplacementCell: `root_is_app_mode.unwrap_or(false)` must not authorize a lifecycle
Classification: BoxShape candidate; no accepted source shape or mode meaning is added
Execution row: MIR-ROOT-APP-MODE-UNDECIDED-P0 (only after this audit is accepted)
---

# MIR-ROOT-APP-MODE-UNDECIDED-FAILFAST-D0

## Six-line brief

Decision: Enumerate the root-mode state before the non-Main static-Box lifecycle
can register a user Box or open its transaction; an undecided mode must never be
silently treated as non-App.

Source authority + canonical issuer: `VerifiedRawRootExpansionV1::is_app_mode`
and the root lifecycle preparation that records it in `MirBuilder` are the only
mode authority. The lifecycle consumer may read that prepared state but cannot
infer it from AST names, Builder defaults, or the current module.

Non-authority: `unwrap_or(false)`, `Option::None` as an implicit compatibility
mode, `register_user_box`, transaction success, AST box spelling, test-builder
setup, `ValueId`/`MirType`, and backend output cannot select App vs non-App.

Fail-fast boundary: after the selected root lifecycle has prepared its mode and
before `register_user_box`, `ActiveRawStaticBoxCompilationStateV1::begin`, or
any method-body descent. A missing/foreign/conflicting mode must produce one
stable freeze error and no partial registration.

Smallest next slice: `MIR-ROOT-APP-MODE-UNDECIDED-P0` only if the caller census
proves `None` is a lifecycle contract violation (or gives it an explicit typed
compatibility disposition); otherwise open a separate source/lifecycle admission
D0. No mode issuer or production switch is created in this design stop.

Non-claims: no App-mode semantic change, root admission redesign, compatibility
callable retirement, Script Deferred repair, static-Box method migration, ABI or
backend change, fallback/retry policy, or performance claim.

## Classification-completeness receipt

The final audit must map every entry to exactly one state before effects:

| state | authority/condition | before effects | allowed terminal | fallback |
|---|---|---|---|---|
| `NotSelected` | this lifecycle is not the selected root statement | no lifecycle work | outer root dispatcher owner | existing sibling statement route only |
| `AppMode` | prepared root mode is `Some(true)` | no user-Box registration or method descent | existing Void terminal | no non-App retry |
| `NonAppMode` | prepared root mode is `Some(false)` | register Box, then open existing transaction | existing static-Box batch completion | no App retry |
| `ModeUndecided` | root mode is `None` at lifecycle entry | typed reject, zero registration/descent | freeze error only | no `false` default or compatibility fallthrough |
| `ModeConflict` | repeated preparation cannot prove one stable source mode | typed reject before lifecycle effects | freeze error only | no overwrite/rebind |
| `SourceDrift` | caller supplies a non-static/non-Main shape to this owner | typed reject before lifecycle effects | freeze error only | no AST-name fallback |

`ModeConflict` and `SourceDrift` are audit obligations: if the current source
authority proves they are impossible, the guard must say so explicitly rather
than silently collapsing them into `AppMode` or `NonAppMode`.

## Audit obligations before P0

- Census every production and test caller of
  `PreparedRawNonMainStaticBoxLifecycleV1::{lower_with_port_v1,lower_normal_with_port_v1}`.
- Prove which caller owns `VerifiedRawRootExpansionV1::is_app_mode` and whether
  raw compatibility callers intentionally lack that source product.
- Check whether root-mode preparation can be repeated with a different value;
  do not preserve an overwrite as an implicit `ModeConflict` policy.
- Add the stable error token and no-partial-registration assertion only after
  the source/lifecycle owner is accepted.
- Keep the owner and any new test child below the 760-line split trigger and
  800-line hard stop.

## Candidate acceptance for the later P0

- `Some(true)` preserves the current Void/no-registration behavior.
- `Some(false)` preserves registration, transaction, method order, and failure
  discard behavior.
- `None` has zero registration, zero transaction, and zero method-body effects.
- a foreign/conflicting mode has the same zero-effect rejection if the audit
  proves that state reachable.
- the old `unwrap_or(false)` expression is absent and no new `Option::None`
  fallback appears.

The row remains `NoSafeSlice` if a production compatibility caller requires an
unset mode but no source-owned compatibility disposition can be named. A local
green test or an AST-derived default cannot open the implementation row.
