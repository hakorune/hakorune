---
Status: Landed
Date: 2026-04-26
Scope: BuildBox parse-source narrowing SSOT handoff.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-281-buildbox-remaining-cleanup-order-card.md
  - lang/src/compiler/build/README.md
  - lang/src/compiler/build/build_box.hako
  - lang/src/compiler/entry/body_extraction_box.hako
  - lang/src/compiler/hako_module.toml
---

# 291x-282: BuildBox Parse-Source Narrowing SSOT

## Goal

Remove BuildBox's duplicate main-body scanner and delegate parse-source
narrowing to `BodyExtractionBox`.

The target shape is:

```text
BuildBox._resolve_parse_src(scan_src)
  -> BodyExtractionBox.extract_main_body(scan_src)
  -> fallback to scan_src when no body exists
```

`BuildBox` remains the source-to-Program(JSON v0) sequencing authority.
`BodyExtractionBox` owns string-aware `Main.main` body extraction.

## Non-Goals

- Do not change parser fallback semantics.
- Do not change bundle handling.
- Do not change defs/imports fragment injection.
- Do not reopen CoreMethodContract fallback rows.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_ok_vm.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

Additional gate:

```bash
tools/checks/dev_gate.sh quick
```

Result: PASS.
