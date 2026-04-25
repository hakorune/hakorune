---
Status: Landed
Date: 2026-04-26
Scope: BuildBox facade closeout docs/checks.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-281-buildbox-remaining-cleanup-order-card.md
  - lang/src/compiler/build/README.md
  - lang/src/compiler/build/build_box.hako
---

# 291x-285: BuildBox Facade Closeout

## Goal

Close the BuildBox thinning series by confirming the live BuildBox is a thin
source-to-Program(JSON v0) facade.

Target shape:

```text
BuildBox
  -> BuildBundleInputBox / BuildBundleResolverBox for scan_src preparation
  -> ParserBox for Program(JSON v0) parsing
  -> BodyExtractionBox for parse_src narrowing
  -> BuildProgramFragmentBox for defs/imports enrichment
```

## Boundary

- Keep `BuildBox` as the only public source-to-Program(JSON v0) entry.
- Keep helper boxes as the owners of scanner/parser-adjacent details.
- Remove any remaining BuildBox-local wrappers that only obscure an owner
  handoff.

## Non-Goals

- Do not change bundle behavior.
- Do not change parser fallback semantics.
- Do not change Program(JSON v0) shape.
- Do not touch CoreMethodContract fallback rows.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_shape_canary_vm.sh
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
