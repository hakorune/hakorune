---
Status: SSOT
Scope: smoke profile taxonomy and discovery rules
Decision: accepted
Related:
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-29cq/README.md
- tools/smokes/v2/run.sh
- tools/checks/smoke_inventory_report.sh
- docs/tools/check-scripts-index.md
---

# Smoke taxonomy and discovery SSOT

## Goal

Keep smoke navigation human-readable while preserving the existing runner contract.

The structural target is:

- daily entry stays small
- blocker pins stay traceable
- support buckets do not become live profile members by accident
- suite manifests become the human-facing execution contract

## Current pressure snapshot

As of 2026-03-21, the smoke tree is heavily concentrated in a few leaves:

- `integration`: about `1200+` scripts
- `integration/apps`: about `160+` active scripts at the leaf root
- `integration/rc_gc_alignment`: `4` scripts, split out of `integration/apps` as the first live semantic family
- `integration/json`: `3` scripts, split out of `integration/apps` as the second live semantic family
- `integration/mir_shape`: `1` script, split out of `integration/apps` as the third live semantic family
- `integration/ring1_providers`: `4` scripts, split out of `integration/apps` as the fourth live semantic family
- `integration/phase29ck_boundary`: `16` scripts, split out of `integration/apps` as the fifth live semantic family
- `integration/vm_hako_caps`: `32` scripts, split out of `integration/apps` as the sixth live semantic family
- `integration/phase29cc_wsm/g3_canvas`: `11` scripts, split out of `integration/apps` as the seventh live semantic family
- `integration/phase29cc_wsm/g2_browser`: `2` scripts, split out of `integration/apps` as the eighth live semantic family
- `integration/phase29cc_wsm/g4`: `10` scripts, split out of `integration/apps` as the ninth live semantic family
- `integration/phase29cc_wsm/p10`: `11` scripts, split out of `integration/apps` as the tenth live semantic family
- `integration/phase29cc_wsm/p5`: `11` scripts, split out of `integration/apps` as the eleventh live semantic family
- `integration/phase29cc_wsm/p6`: `2` scripts, split out of `integration/apps` as the twelfth live semantic family
- `integration/phase29cc_wsm/p7`: `4` scripts, split out of `integration/apps` as the thirteenth live semantic family
- `integration/phase29cc_wsm/p8`: `1` script, split out of `integration/apps` as the fourteenth live semantic family
- `integration/phase29cc/plg_hm1`: `5` scripts, split out of `integration/apps` as the fifteenth live semantic family
- `integration/phase29x/vm_hako`: `4` scripts, split out of `integration/apps` as the sixteenth live semantic family
- `integration/phase29y/hako/emit_mir`: `7` scripts, split out of `integration/apps` as the seventeenth live semantic family
- `integration/phase21_5/perf/chip8`: `1` script, split out of `integration/apps` as the eighteenth live semantic family
- `integration/phase21_5/perf/kilo`: `7` scripts, split out of `integration/apps` as the nineteenth live semantic family
- `integration/phase21_5/perf/numeric`: `4` scripts, split out of `integration/apps` as the twentieth live semantic family
- `integration/apps/archive`: about `225` archived scripts
- `integration/joinir`: about `170` scripts
- `quick/core`: about `63` scripts

This is still too dense for casual human navigation, especially under `integration/apps`, but the first twenty-seven live splits have already been carved out as `integration/rc_gc_alignment`, `integration/json`, `integration/mir_shape`, `integration/ring1_providers`, `integration/phase29ck_boundary`, `integration/vm_hako_caps`, `integration/phase29cc_wsm/g3_canvas`, `integration/phase29cc_wsm/g2_browser`, `integration/phase29cc_wsm/g4`, `integration/phase29cc_wsm/p10`, `integration/phase29cc_wsm/p5`, `integration/phase29cc_wsm/p6`, `integration/phase29cc_wsm/p7`, `integration/phase29cc_wsm/p8`, `integration/phase29cc/plg_hm1`, `integration/phase29x/vm_hako`, `integration/phase29y/hako/emit_mir`, `integration/phase21_5/perf/chip8`, `integration/phase21_5/perf/kilo`, `integration/phase21_5/perf/numeric`, `integration/phase21_5/perf/apps/entry_mode`, `integration/phase21_5/perf/apps/mir_mode`, `integration/phase21_5/perf/apps/case_breakdown`, `integration/phase21_5/perf/apps/compile_run_split`, `integration/phase21_5/perf/apps/crosslang_bundle`, `integration/phase21_5/perf/apps/emit_mir_jsonfile_route`, and `integration/phase21_5/perf/apps/startup_subtract`.

## Suite-first contract

- `tools/smokes/v2/suites/<profile>/<suite>.txt` is the primary human-facing execution contract.
- `run.sh --profile <profile> --suite <suite>` is the preferred daily/presubmit entry for curated packs.
- `--profile` remains the compatibility floor and coarse lane selector.
- `--suite` is additive: it applies an allowlist intersection over the live profile set.
- recursive discovery remains as a compatibility mechanism for uncatalogued profile runs, not as the long-term organization model.
- role-first reading is preferred when a semantic lane is already explicit:
  - `llvm/exe` = product
  - `rust-vm` = engineering/bootstrap
  - `vm-hako` = reference/conformance
  - `wasm` = experimental

Current seeded suites:

- `integration/presubmit`
- `integration/collection-core`
- `integration/vm-hako-core`
- `integration/vm-hako-caps`
- `integration/selfhost-core`
- `integration/joinir-bq`

Smoke split work is parked while the kernel migration lane resumes; the suite list below remains the future-facing organization map for any later smoke wave.

Reference-lane note:

- `integration/vm-hako-core` and `integration/vm-hako-caps` are reference/conformance suites.
- They are not product-mainline packs and should not be used as evidence that
  `vm-hako` became a main lane.

Experimental-lane note:

- `integration/phase29cc_wsm/**` families are experimental smoke families.
- They are not product-mainline packs and should not be used as evidence that
  `wasm` became a co-main runtime.

## Discovery fallback contract

- `tools/smokes/v2/run.sh` auto-discovers `*.sh` under `profiles/$PROFILE`.
- Discovery is recursive, but it now prunes support buckets by directory name:
  - `archive`
  - `lib`
  - `tmp`
  - `fixtures`
- Scripts under those directories remain directly runnable by `bash ...`, but they are not live profile members in `run.sh`.
- suite manifests may only reference paths that survive this live discovery fallback.

## Manifest format and failure contract

- manifest format:
  - `#` comment allowed
  - one relative path per line
  - path is relative to `tools/smokes/v2/profiles/<profile>/`
- fail-fast cases:
  - missing manifest
  - duplicate manifest entry
  - manifest entry that is not part of live discovery for that profile

## Taxonomy rules

### Rule 1: top level stays by run tier

- `profiles/quick/`
- `profiles/integration/`
- `profiles/strict/`
- `profiles/plugins/`
- `profiles/archive/`
- `profiles/full/` is legacy compatibility vocabulary only; do not treat it as the current live root unless a dedicated tree is reintroduced

`strict` is the live narrow fail-fast gate tier. `quick` stays fast, `integration` stays curated, `plugins` stays plugin-only, and `archive` stays manual replay / retired pins.

### Rule 2: second level is semantic domain

Prefer:

- `core/`
- `collections/`
- `array/`
- `map/`
- `string/`
- `parser/`
- `joinir/`
- `selfhost/`
- `runtime/`
- `vm/`
- `analyze/`

Use `apps/` only for app-level end-to-end cases. Do not keep every feature probe in `apps/`.

### Rule 3: third level is intent

Inside a domain, prefer intent buckets over phase buckets:

- `smoke/`
- `contract/`
- `gate/`
- `canary/`
- `probe/`
- `parity/`
- `regression/`
- `inventory/`

Phase IDs may remain in filenames, but they should not be the primary folder key for new structure.

### Rule 4: support buckets are never daily discovery

- `archive/` is for retired pins and manual replay
- `lib/` is for shared helpers
- `tmp/` is for scratch or generated artifacts
- `fixtures/` is for reusable inputs

These names are reserved and should not contain live profile entries that must run through `run.sh`.

## Operating rules

- Daily entry uses `tools/checks/dev_gate.sh` or lane gate packs.
- runner-level suite entry is allowed for curated packs (`run.sh --profile ... --suite ...`), but `--profile` remains the compatibility floor.
- Single-purpose scripts are evidence pins or blocker probes.
- `1 blocker = 1 pin` remains valid, but pins should fold back into packs after the lane reaches stop line.
- Use `tools/checks/smoke_inventory_report.sh` for milestone inventory instead of manual ad-hoc pruning.
- Inventory reports are suite-aware and scoped to the target subtree; use a profile root for whole-profile coverage and a semantic subtree for domain coverage.

## First reorganization order

1. Fix discovery semantics so support buckets are not live.
2. Introduce suite manifests without changing `--profile` compatibility.
3. Prefer suite manifests for daily/presubmit entry before any semantic path split.
4. Keep inventory tooling aligned with the same prune contract.
5. Split `integration/apps` by semantic domain before any mass rename; the first live splits are `integration/rc_gc_alignment/`, `integration/json/`, `integration/mir_shape/`, `integration/ring1_providers/`, `integration/phase29ck_boundary/`, `integration/vm_hako_caps/`, `integration/phase29cc_wsm/g3_canvas/`, `integration/phase29cc_wsm/g2_browser/`, `integration/phase29cc_wsm/g4/`, `integration/phase29cc_wsm/p10/`, `integration/phase29cc_wsm/p5/`, `integration/phase29cc_wsm/p6/`, `integration/phase29cc_wsm/p7/`, `integration/phase29cc_wsm/p8/`, `integration/phase29cc/plg_hm1/`, `integration/phase29x/vm_hako/`, `integration/phase29y/hako/emit_mir/`, `integration/phase21_5/perf/chip8/`, `integration/phase21_5/perf/kilo/`, and `integration/phase21_5/perf/numeric/`; after that, keep new `integration/apps` growth under the semantic domain tree and do not add new live scripts to the bundle root.
6. Move historical residue to `archive/` buckets only after docs and packs stop pointing at the old path.

## First safe target

The first overloaded bucket to split is:

- `tools/smokes/v2/profiles/integration/apps/`

First live split already landed:

- `tools/smokes/v2/profiles/integration/rc_gc_alignment/`
- `tools/smokes/v2/profiles/integration/json/`
- `tools/smokes/v2/profiles/integration/mir_shape/`
- `tools/smokes/v2/profiles/integration/ring1_providers/`
- `tools/smokes/v2/profiles/integration/phase29ck_boundary/`
- `tools/smokes/v2/profiles/integration/vm_hako_caps/`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/g3_canvas/`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/p6/`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/p7/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/chip8/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/`
- `tools/smokes/v2/profiles/integration/phase29cc/plg_hm1/`
- `tools/smokes/v2/profiles/integration/phase29x/vm_hako/`
- `tools/smokes/v2/profiles/integration/phase29y/hako/emit_mir/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/numeric/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/entry_mode/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/mir_mode/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/case_breakdown/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/compile_run_split/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/crosslang_bundle/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/emit_mir_jsonfile_route/`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/apps/startup_subtract/`

Recommended next semantic groups for a future smoke wave:

- `phase29x/observability`
- `phase29x/optimization`
- `phase29x/runtime`
- `phase29x/cache`
- `phase29x/core`
- `phase29x/llvm`
- `phase29x/abi`
- `phase29cc/wsm02d`
- `phase29cc/wsm`
- `phase29y/binary_only`
- `phase29y/using_resolver_parity`
- `phase29y/lane`
- `phase29y/rc`
- `phase29y/core`
- `phase21_5_concat3_assoc_contract_vm.sh`

Do not mass-move all archived content in the same slice. Archive separation and active semantic split should remain separate commits.
