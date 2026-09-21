---
Status: closed__2026-09-21__WarningBuildGateBrand__DeletedAndVerified
Task: MIRBUILDER-WARNING-BUILD-GATE-BRAND-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i110-2026-09-21.md
Implementation permission: true for the production-zero redundant BuildGate brand projection
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I112
---

# Warning cleanup: redundant BuildGate decision-set brand projection

## Six-line brief

```text
Decision: remove PreparedBuildGateDecisionSetV1::brand and its stored outer
brand; every issued decision row already carries the validated parser brand.
Source authority + canonical issuer: src/parser/build_cfg/decision_set.rs;
the parser decision issuer validates the brand before issuing each row.
Non-authority: warning suppression, cargo-fix, row-brand removal, projection
changes, or a new parser identity source.
Fail-fast boundary: any production caller, focused parser red, or warning-count
mismatch rejects the row.
Smallest next slice: delete the redundant field/accessor and change one unit
assertion to inspect the row-level brand, then run the parser focused gate.
Non-claims: no BuildGate selection change, AST rewrite, source-seal change,
fallback, route switch, or LegacyCallV0 retirement.
```

## Census boundary

The bounded inventory is the `PreparedBuildGateDecisionSetV1` constructor,
its `brand` field/accessor, the BuildGate projection consumer, and the one
unit-test assertion that calls the accessor. It excludes row-level brand
validation, `ProjectedProgramItemSlotSetV1`, and all unrelated parser brands.

## Caller census and acceptance

`rg` shows no production read of the outer field or accessor. The only caller
is `src/parser/build_cfg/decision_set_tests.rs`; it will assert that the first
issued row retains the expected parser brand. The focused gate is the parser
BuildGate decision/projection test family, followed by:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib parser::build_cfg
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Expected warning refresh: lib **1,699 → 1,697** (field plus accessor) and
lib-test **551** unchanged. No `#[allow]`, row-brand deletion, or unrelated
parser change is allowed.

## Closeout evidence

The redundant outer field and accessor were deleted. The focused BuildGate
parser family passed **10/10**, and the row-level brand assertion passed.
Sequential validation passed:

```text
cargo check --profile quick --lib -j4          -> lib warnings 1,697
cargo test --profile quick --lib --no-run -j4  -> lib-test warnings 551
cargo fmt --all -- --check                     -> pass
git diff --check                                -> pass
bash tools/checks/current_state_pointer_guard.sh -> pass
```

The decision rows still retain the parser brand used by the projection
consumer. No BuildGate selection, source-seal, fallback, or route behavior
changed. The next action is a baseline refresh before selecting another cohort.
