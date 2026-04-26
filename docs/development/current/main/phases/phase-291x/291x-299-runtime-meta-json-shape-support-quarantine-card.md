---
Status: Landed
Date: 2026-04-26
Scope: runtime/meta JsonShapeToMap support quarantine
Related:
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/support/README.md
  - lang/src/runtime/meta/support/json_shape_parser.hako
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
  - src/tests/joinir_frontend_if_select.rs
---

# 291x-299: runtime/meta JsonShapeToMap Support Quarantine

## Goal

Keep the active `JsonShapeToMap` support export available while removing it
from the semantic-table root.

This is BoxShape cleanup only. It is an implementation-location quarantine, not
a behavior change.

## Change

Moved:

```text
lang/src/runtime/meta/json_shape_parser.hako
```

to:

```text
lang/src/runtime/meta/support/json_shape_parser.hako
```

The public export is preserved:

```toml
JsonShapeToMap = "support/json_shape_parser.hako"
```

The bridge-visible function name is preserved:

```text
JsonShapeToMap._read_value_from_pair/1
```

The targeted JoinIR frontend smoke path was also normalized from stale
`../../tests/...` paths to repo-root-relative `tests/...` paths so the smoke can
be run directly with `cargo test` from the package root.

## Result

`lang/src/runtime/meta/` now has a clearer split:

```text
core_method_contract_box.hako
generated/core_method_contract_manifest.json
support/json_shape_parser.hako
```

Only `CoreMethodContractBox` plus the generated manifest is a compiler semantic
contract table. `JsonShapeToMap` remains an audited active support utility.

## Non-Goals

- No deletion of `JsonShapeToMap`.
- No rename of `selfhost.meta.JsonShapeToMap`.
- No rename of `JsonShapeToMap._read_value_from_pair/1`.
- No JoinIR route change.
- No `.inc` classifier growth.

## Validation

```bash
bash tools/selfhost/refresh_stage1_module_env_snapshot.sh
bash tools/checks/module_registry_hygiene_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo test -q embedded_snapshot_matches_registry_doc
cargo test -q joinir_frontend_json_shape_read_value_ab_test
git diff --check
```
