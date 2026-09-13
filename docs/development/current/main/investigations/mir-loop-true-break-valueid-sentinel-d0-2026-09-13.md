---
Status: Implementation complete — focused quick evidence green
Date: 2026-09-13
Decision: MIR-LOOP-TRUE-BREAK-VALUEID-SENTINEL-D0
Parent: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
ProductionCaller: normalized-shadow LoopTrueBreakOnceBuilderBox (dev-only)
ReplacementCell: strict propagation diagnostic lookup
---

# MIR-LOOP-TRUE-BREAK-VALUEID-SENTINEL-D0

## Six-line brief

Decision: replace one strict-check dummy `ValueId(0)` with an explicit checked lookup.
Source authority + canonical issuer: `EnvLayout.env_fields()` and `NormalizedHelperBox::collect_env_args` own the ordered environment argument relation; no new semantic issuer is needed.
Non-authority: the diagnostic fallback, numeric `ValueId(0)`, target names, runtime metadata, and the emitted `Call.args` vector.
Fail-fast boundary: after successful environment collection and before the strict propagation comparison; a missing positional entry becomes the existing typed freeze instead of a fabricated value.
Smallest next slice: `MIR-LOOP-TRUE-BREAK-VALUEID-SENTINEL-I0` — replace `unwrap_or(ValueId(0))`, retain the existing positive test, and add one reusable missing-environment negative.
Non-claims: no source-shape expansion, Call-schema migration, production-route repair, normalized-shadow promotion, or whole M7/R7 completion.

## Finite scope and state table

Census boundary: `LoopTrueBreakOnceBuilderBox::lower` StepTree admission ->
`loop_body` environment collection -> strict propagation check -> `k_exit`
Call emission. Includes only the one `loop_body_args.get(idx)` check; excludes
the separate void-return dummy in `common/return_value_lowerer_box.rs`, all
other normalized-shadow routes, and production backends.

| State | Authority and condition | Terminal behavior |
| --- | --- | --- |
| `RouteDeclined` | StepTree is outside the loop(true)-break-once shape | existing `Ok(None)` before environment construction |
| `CollectEnvError` | ordered field is absent from the loop environment | existing `phase131/loop_true/env_missing` error before `Call` emission |
| `StrictDisabled` | strict environment check is off | retain current route behavior; no diagnostic comparison |
| `Checked` | every changed field has one positional `loop_body_args` entry | compare the real `ValueId`; emit the unchanged `k_exit` Call args |
| `IndexMissing` | the positional relation is broken after collection | typed `phase131/loop_true/env_missing` freeze; no fabricated `ValueId(0)` and no `Call` |
| `EnvNotPropagated` | checked entry equals the pre-body value after an update | existing `phase131/env_not_propagated` error before `Call` emission |

## Source evidence

- `loop_true_break_once.rs:138` snapshots `env_fields` once from `EnvLayout`.
- `loop_true_break_once.rs:278-285` calls `collect_env_args(&env_fields, &env_loop_body)?` before the check.
- `common/normalized_helpers.rs:47-59` emits one argument per field and returns an error when a field is missing.
- `loop_true_break_once.rs:308-317` derives `idx` from the same immutable `env_fields` sequence and already freezes if the field is absent.
- `loop_true_break_once.rs:319` uses `unwrap_or(ValueId(0))` only for the strict comparison; `loop_body_args` at `:333` is passed through unchanged.
- Existing `loop_true_break_once/tests.rs::test_loop_true_break_once_passes_updated_env_to_k_exit` proves the updated value reaches `k_exit`; `NormalizedHelperBox` already owns the reusable ordering contract.

The `get(idx)` miss is unreachable while the two collection invariants hold.
The present defect is therefore a hidden diagnostic fallback, not an observed
production miscompile. If a future length invariant breaks, the fabricated zero
could hide the mismatch or report an unrelated old-value match.

## I0 task and closeout

1. Delete only the `unwrap_or(ValueId(0))` edge at `loop_true_break_once.rs:319`.
2. Use the existing `error_tags::freeze_with_hint` style and the same
   `phase131/loop_true/env_missing` boundary for a missing positional entry.
3. Keep the existing updated-environment positive test and add a focused
   `collect_env_args` missing-field negative; do not add a production hook to
   manufacture an unreachable `IndexMissing` case.
4. Update the normalized-shadow module README and this card in the same I0 if
   the error wording or ownership contract changes.
5. Run the selected focused Rust test, pointer guard, and `git diff --check`
   under the one-Cargo-process/quick-profile rule; classify unrelated warning
   debt as baseline only after the parent command reproduces it.

### Evidence (2026-09-13)

- `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normalized_shadow::`
  passed 83/83 tests, including the updated-env `k_exit` positive and the
  missing-field helper negative.
- The command generated 533 existing compiler warnings and no test failures;
  these remain known baseline warning debt and were not suppressed or rewritten.
- `loop_true_break_once.rs` is 626 lines and
  `common/normalized_helpers.rs` is 193 lines, both below the 760-line design
  threshold.
- `current_state_pointer_guard.sh` and `git diff --check` pass. The source
  search contains no `unwrap_or(ValueId(0))` in this route.

The I0 is a BoxShape/fail-fast cleanup selected by `CURRENT_STATE.toml`. It
must not be combined with the separate
`ValueId(0)` void-return contract, PHI alias sealing, Call schema retirement,
or backend parity work.

## Reopen and non-claims

Reopen this D0 if the check becomes source-reachable with a different argument
owner, if `collect_env_args` no longer proves one argument per field, or if a
production caller depends on the dummy value. Such a change requires a new
authority decision. This card does not claim that all `ValueId(0)` uses are
retired; the M7 list remains an aggregate caller-zero requirement.
