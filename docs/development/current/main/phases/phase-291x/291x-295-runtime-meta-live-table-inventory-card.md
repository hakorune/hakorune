---
Status: Landed
Date: 2026-04-26
Scope: Inventory remaining `lang/src/runtime/meta` live exports after mir-call mirror-table retirement.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-294-mir-call-surface-policy-export-retirement-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/core_method_contract_box.hako
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - lang/src/runtime/meta/using_resolver.hako
  - lang/src/runtime/meta/using_decision.hako
  - lang/src/runtime/meta/json_shape_parser.hako
---

# 291x-295 Runtime/Meta Live Table Inventory Card

## Goal

Fix the remaining `lang/src/runtime/meta` ownership map after retiring the
unused `mir_call` route / need / surface `.hako` mirror tables.

This is a docs-only inventory. It does not change module exports or compiler
behavior.

## Current Exports

`lang/src/runtime/meta/hako_module.toml` currently exports:

```text
UsingResolver
UsingDecision
JsonShapeToMap
CoreMethodContract
```

## Classification

| Export | Current role | Evidence | Next action |
| --- | --- | --- | --- |
| `CoreMethodContract` | live compiler semantic contract owner | `tools/core_method_contract_manifest_codegen.py` derives `generated/core_method_contract_manifest.json` from `CoreMethodContractBox` and guards it | keep |
| `generated/core_method_contract_manifest.json` | generated consumer artifact | guarded by `core_method_contract_manifest_guard.sh`; not an owner | keep generated |
| `JsonShapeToMap` | support / JoinIR fixture utility | `src/mir/join_ir_vm_bridge_dispatch/targets.rs` has a bridge target for `JsonShapeToMap._read_value_from_pair/1` | audit separately before moving/deleting |
| `UsingResolver` | minimal meta support stub | no external `selfhost.meta.UsingResolver` user found outside `UsingDecision`; separate Stage1/Pipeline using resolvers are the real compiler paths | audit separately |
| `UsingDecision` | thin wrapper around `UsingResolver` shape | only depends on `selfhost.meta.UsingResolver`; no external user found in the current search | audit with `UsingResolver` |

## Decision

After `291x-294`, the only live compiler semantic contract table in
`runtime/meta` is `CoreMethodContractBox` plus its generated manifest.

The remaining non-CoreMethod exports are support / fixture-era modules, not
current `mir_call` semantic owners. They must be handled as a separate
BoxShape cleanup series and not mixed with CoreMethodContract row changes.

## Next

Open a focused owner audit for:

```text
UsingResolver / UsingDecision runtime/meta support exports
```

Do not delete them by analogy. Confirm whether any stage1/module loader path
imports `selfhost.meta.UsingResolver` or `selfhost.meta.UsingDecision` first.
Keep `JsonShapeToMap` separate because JoinIR bridge tests name its function
directly.

## Acceptance

```bash
rg -n "selfhost\\.meta\\.(UsingResolver|UsingDecision|JsonShapeToMap|CoreMethodContract)|using selfhost\\.meta|CoreMethodContractBox|JsonShapeToMap|UsingDecision|UsingResolver" lang/src src tools apps crates --glob '!target/**' --glob '!*.json'
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
