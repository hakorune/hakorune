---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: `phase-33x shared helper family recut` の concrete queue と evidence command をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-33x/README.md
  - docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md
---

# 33x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `33xA helper family inventory` | landed | exact keep/rehome/shim-only reading |
| 2 | `33xB hako_check family path truth` | landed | family-local smoke helpers move first and keep reason is fixed |
| 3 | `33xC emit_mir thin wrapper path truth` | active | thin wrappers stay truthful route-preset shims before broad helper keep |
| 4 | `33xD closeout/docs cleanup` | active | landed helper-family truth is reflected in current/public docs |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `33xA1` | landed | helper family caller inventory |
| 2 | `33xB1` | landed | `hako_check_deadblocks_smoke` family-home rehome |
| 3 | `33xB2` | landed | `hako_check.sh` top-level keep gate |
| 4 | `33xC1` | landed | `emit_mir` thin wrapper caller inventory |
| 5 | `33xC2` | landed | `emit_mir` thin wrapper route-preset lock |
| 6 | `33xC3` | landed | `hakorune_emit_mir.sh` top-level keep gate |
| 7 | `33xD1` | active | closeout/docs cleanup |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
git diff --check
rg -n "hako_check_deadblocks_smoke\\.sh|hako_check/deadblocks_smoke\\.sh|hakorune_emit_mir_mainline\\.sh|hakorune_emit_mir_compat\\.sh|hakorune_emit_mir\\.sh" \
  CURRENT_TASK.md \
  docs/development/current/main \
  docs/tools/README.md \
  tools/selfhost/README.md \
  tools/hako_check/README.md \
  tools/hako_check_deadblocks_smoke.sh \
  tools/hako_check \
  tools/hakorune_emit_mir.sh \
  tools/hakorune_emit_mir_mainline.sh \
  tools/hakorune_emit_mir_compat.sh \
  tools/smokes/v2/lib/emit_mir_route.sh
bash -n \
  tools/hako_check_deadblocks_smoke.sh \
  tools/hako_check/deadblocks_smoke.sh \
  tools/hakorune_emit_mir_mainline.sh \
  tools/hakorune_emit_mir_compat.sh
```

## Current Result

- `33xA1` landed:
  - `tools/hako_check.sh` is broad top-level keep
  - `tools/hakorune_emit_mir.sh` is broad top-level keep
  - `hako_check` deadcode/deadblocks helpers and `emit_mir` thin wrappers are the low-blast family-home targets
- `33xB1` landed:
  - canonical deadblocks home is `tools/hako_check/deadblocks_smoke.sh`
  - old top-level path is shim-only
  - current/live policy docs now point at the family home
- `33xB2` landed:
  - `tools/hako_check.sh` remains top-level keep because it is still the canonical analyzer entry, plus one family helper and one analyze smoke still call it
  - drain condition stays `family analyzer entry + doc/script-index repoint`, then revisit rehome later
- `33xC1` landed:
  - thin wrapper live callers are current-docs/proof oriented and low blast
- `33xC2` landed:
  - thin wrappers stay as top-level route-preset compatibility wrappers
  - operational routing truth stays in `tools/smokes/v2/lib/emit_mir_route.sh`
- `33xC3` landed:
  - `tools/hakorune_emit_mir.sh` remains top-level keep because route-owner, perf, check/debug, proof, and current-doc pressure are still broad
  - drain condition stays `route-selecting callers -> emit_mir_route`, then revisit the helper later
- current front:
  - `33xD1 closeout/docs cleanup`
