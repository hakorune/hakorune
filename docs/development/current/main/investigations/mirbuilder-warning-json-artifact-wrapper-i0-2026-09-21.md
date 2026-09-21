---
Status: closed__2026-09-22__WarningJsonArtifactWrapper__DeletedAndVerified
Task: MIRBUILDER-WARNING-JSON-ARTIFACT-WRAPPER-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i118-2026-09-21.md
Implementation permission: true for the production-zero json-artifact wrapper
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I120
---

# Warning cleanup: obsolete JSON-artifact loader wrapper

## Six-line brief

```text
Decision: delete runner/json_artifact::load_mir_json_to_module and its
duplicate module-level test; retain mir_loader as the canonical loader owner.
Source authority + canonical issuer: runner/json_artifact/mir_loader.rs;
load_json_artifact_to_module already calls that module directly.
Non-authority: warning suppression, cargo-fix, artifact schema changes, or a
new JSON classifier.
Fail-fast boundary: any production caller, loader focused red, or warning
count mismatch rejects the row.
Smallest next slice: remove the dead wrapper and duplicate test, then run the
json-artifact loader and direct parse focused gates.
Non-claims: no v0/v1 classification change, direct parse change, fallback
repair, or LegacyCallV0 retirement.
```

## Census boundary

The bounded inventory is `load_mir_json_to_module` in
`src/runner/json_artifact/mod.rs:49-51` and its one module-level test. `rg`
found no production caller. `load_json_artifact_to_module` calls
`mir_loader::load_mir_json_to_module` directly, and the canonical loader
module retains its own equivalent Program-v0 and MIR-v0 tests plus direct
schema rejection guards. `parse_direct_mir_json_text`, the v0/v1 classifier,
and all artifact schema behavior remain in scope only as guards.

## Acceptance

```text
cargo fmt --all -- --check
cargo test --profile quick --lib runner::json_artifact
cargo test --profile quick --lib runner::json_artifact::mir_loader
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout evidence

The obsolete wrapper and its duplicate module-level test were deleted. The
canonical `mir_loader` owner remains unchanged and still covers the Program-v0
boundary plus direct MIR schema guards. No artifact schema, classifier, or
route behavior changed.

```text
cargo test --profile quick --lib runner::json_artifact       11/11 passed
cargo test --profile quick --lib runner::json_artifact::mir_loader  7/7 passed
cargo check --profile quick --lib -j4                       passed; lib 1,693
cargo test --profile quick --lib --no-run -j4               passed; lib-test 550
cargo fmt --all -- --check                                  passed
git diff --check                                             passed
bash tools/checks/current_state_pointer_guard.sh              passed
```

The warning baseline therefore moved from lib **1,694** to **1,693**, while
lib-test remained **550**. The old-edge retirement lane remains
`NoSafeSlice`; this deletion only removes a production-zero wrapper.
