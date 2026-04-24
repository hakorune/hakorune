---
Status: Landed
Date: 2026-04-24
Scope: Add the first CoreMethodContract seed owner under `lang/src/runtime/meta/`.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-131-hotline-core-method-contract-task-plan.md
  - lang/src/runtime/meta/core_method_contract_box.hako
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/README.md
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-132 CoreMethodContract Seed Card

## Goal

Land HCM-1: create the first `.hako` contract owner for Array/String/Map method
surfaces without changing backend lowering or runtime behavior.

This is a BoxShape implementation card. It does not add a CoreBox row, parser
rule, `.inc` lowering branch, generated metadata table, environment variable,
runtime helper, or hot inline lowering.

## Implementation

- Added `CoreMethodContractBox` under `lang/src/runtime/meta/`.
- Exported it as `selfhost.meta.CoreMethodContract`.
- Refreshed the Stage-1 embedded module snapshot with the existing refresh
  script.
- Updated `lang/src/runtime/meta/README.md`.

The new box exposes:

- `schema_fields()`
- `rows()`
- `count()`
- `find(box_name, method_name, arity)`
- `core_op_for(box_name, method_name, arity)`
- `effect_for(box_name, method_name, arity)`

## Seed Rows

Initial rows are limited to the HCM SSOT seed scope:

| Family | Rows |
| --- | --- |
| `ArrayBox` | `length/len/size`, `get`, `set`, `push` |
| `MapBox` | `get`, `set`, `has`, `size/len/length` |
| `StringBox` | `length/len/size`, `substring/substr`, `indexOf/find` |

Each row carries:

- `box`
- `canonical`
- `aliases`
- `arity`
- `effect`
- `core_op`
- `hot_lowering`
- `cold_lowering`
- `runtime_owner`
- `status`
- `guards`

## Boundary

The seed owner is intentionally inert:

- `.inc` still uses existing guarded mirror paths.
- MIR does not consume `CoreMethodContractBox` yet.
- `hot_lowering` is metadata only.
- Rust remains the cold/slow/storage substrate.

## Focused Probe

Temporary import probe:

```text
using selfhost.meta.CoreMethodContract as CoreMethodContract

CoreMethodContract.schema_fields().length() -> 11
CoreMethodContract.count() -> 11
CoreMethodContract.core_op_for("ArrayBox", "len", 0) -> ArrayLen
CoreMethodContract.core_op_for("MapBox", "length", 0) -> MapLen
CoreMethodContract.core_op_for("StringBox", "find", 2) -> StringIndexOf
```

Observed output:

```text
fields=11,count=11,ops=ArrayLen/MapLen/StringIndexOf
```

## Proof

```bash
bash tools/selfhost/refresh_stage1_module_env_snapshot.sh
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
tools/checks/dev_gate.sh quick
git diff --check
```

## Next

- HCM-2: add generated metadata or a checked generated-table path.
- HCM-3: add the `.inc` method-name-classifier no-growth guard.
- Keep `.inc` consumer migration as a later, one-family card.
