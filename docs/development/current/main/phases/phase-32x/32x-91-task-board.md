---
Status: Active
Decision: provisional
Date: 2026-04-02
Scope: `phase-32x product / engineering split` の concrete queue と evidence command をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-32x/README.md
  - docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md
---

# 32x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `32xA mixed-owner inventory` | landed | exact mixed-owner surfaces are locked |
| 2 | `32xB build.rs split plan` | landed | split source owner first |
| 3 | `32xC phase2100 role split plan` | landed | split thick smoke aggregator by role |
| 4 | `32xD top-level orchestrator rehome prep` | landed | drain callers before moving top-level keeps |
| 5 | `32xE direct-route takeover prep` | active | reduce shell-based `--backend vm` residues carefully |
| 6 | `32xF shared helper follow-up gate` | queued | defer helper-family recut to a dedicated lane |
| 7 | `32xG raw default/token gate` | deferred | decide backend default only after owner split |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `32xA1` | landed | `build.rs` mixed ownership inventory |
| 2 | `32xA2` | landed | `phase2100` mixed aggregator inventory |
| 3 | `32xB1` | landed | `build.rs` split target lock |
| 4 | `32xB2` | landed | `build.rs` implementation slice order |
| 5 | `32xC1` | landed | `phase2100` role bucket lock |
| 6 | `32xC2` | landed | `phase2100` thin meta-runner plan |
| 7 | `32xD1` | landed | `bootstrap_selfhost_smoke` caller drain map |
| 8 | `32xD2` | landed | `plugin_v2_smoke` caller drain map |
| 9 | `32xE1` | active | `child.rs` / `stage1_cli` direct-route gap inventory |
| 10 | `32xE2` | queued | `core_executor` takeover seam lock |
| 11 | `32xF1` | queued | shared helper follow-up gate |
| 12 | `32xG1` | deferred | raw backend default/token remains last |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
git diff --check
sed -n '1,260p' src/runner/build.rs
sed -n '1,260p' tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh
ls -1 tools/smokes/v2/profiles/integration/core/phase2100
rg -n -- '--backend vm|--backend llvm|cranelift|ny-llvmc|llvmlite|phase2100' \
  src/runner/build.rs \
  tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh \
  tools/bootstrap_selfhost_smoke.sh \
  tools/plugin_v2_smoke.sh \
  tools/selfhost/bootstrap_selfhost_smoke.sh \
  tools/plugins/plugin_v2_smoke.sh \
  src/runner/modes/common_util/selfhost/child.rs \
  lang/src/runner/stage1_cli/core.hako
```

## 32xA Result

- `build.rs` is confirmed as the first source split target.
- `phase2100/run_all.sh` is confirmed as the first thick smoke-aggregator split target.
- `32xB1` fixed the first owner split as:
  - shared prelude/link
  - product llvm build+emit
  - engineering vm/cranelift build+emit
- `32xB2` landed helper-first extraction inside `src/runner/build.rs` without changing owner behavior.
- `32xC1` fixed the exact role buckets:
  - `engineering-selfhost`
  - `probe-llvmlite`
  - `product-crate-exe`
  - `experimental-native`
  - `always-on/shared`
- `32xC2` landed the thin meta-runner shape:
  - `run_engineering_selfhost.sh`
  - `run_probe_llvmlite.sh`
  - `run_product_crate_exe.sh`
  - `run_experimental_native.sh`
  - `run_always_on_shared.sh`
  - `run_all.sh` now stays as the stable public entry
- `32xD1` landed:
  - canonical home is `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `Makefile`, `selfhost-pilot`, and `dev/selfhosting/README.md` now point there
  - old top-level path is shim-only
- `32xD2` landed:
  - canonical home is `tools/plugins/plugin_v2_smoke.sh`
  - `plugin_guard.rs` now points there
  - old top-level path is shim-only
- current front:
  - `32xE1 child.rs / stage1_cli direct-route gap inventory`
