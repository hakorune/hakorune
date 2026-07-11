---
Status: Landed
Date: 2026-04-27
Scope: Prune CFG extractor helper from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/cfg_extractor.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-542: CFG Extractor Root Export Prune

## Goal

Keep `extract_cfg_info` on the `mir::cfg_extractor` owner module instead of
re-exporting it through the broad MIR root facade.

CFG extraction is a JSON/hako-check support helper, not core MIR model
vocabulary. Callers should name the owner module explicitly.

## Inventory

Removed root export:

- `extract_cfg_info`

Migrated consumer:

- `src/runner/mir_json_emit/root.rs`

## Cleaner Boundary

```text
mir::cfg_extractor
  owns CFG extraction for JSON / hako-check support

mir root
  does not re-export CFG extraction helper vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change CFG JSON contents.
- Do not change MIR JSON schema selection.
- Do not change hako-check behavior.

## Acceptance

- MIR root no longer re-exports `extract_cfg_info`.
- JSON emission imports the owner path.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed CFG extraction helper from the MIR root export surface.
- Preserved JSON emission behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
