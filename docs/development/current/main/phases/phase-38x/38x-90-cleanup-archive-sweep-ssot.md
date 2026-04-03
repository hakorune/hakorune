Status: Active
Date: 2026-04-03
Scope: post-37x cleanup/archive sweep

# 38x-90 Cleanup / Archive Sweep SSOT

## Macro Reading

| Wave | Status | Read as |
| --- | --- | --- |
| `38xA legacy smoke archive` | active | legacy embedded smoke を top-level `tools/` から外す |
| `38xB delete-ready shim sweep` | landed | drained shim のうち current pointer drain 済みのものを delete する |
| `38xC archive-later queue freeze` | landed | current/historical docs drain が残る shim を archive-later に固定する |
| `38xD closeout` | active | cleanup queue と next source lane を current docs に戻す |

## Candidate Reading

| Path | State | Reading |
| --- | --- | --- |
| `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh` | archived-now | former `tools/stage1_smoke.sh`; current mainline route is `tools/selfhost/stage1_mainline_smoke.sh` |
| `tools/hako_check_deadblocks_smoke.sh` | deleted | canonical path already lives under `tools/hako_check/deadblocks_smoke.sh`; no live executable callers remained |
| `tools/bootstrap_selfhost_smoke.sh` | archive-later | top-level shim only, but current/historical docs drain remains |
| `tools/plugin_v2_smoke.sh` | archive-later | top-level shim only, but plugin lane historical/current docs still point here |
| `tools/hako_check_deadcode_smoke.sh` | archive-later | shim-only, but current/historical docs drain remains |

## Active Front

- active macro wave: `38xD closeout`
- active micro task: `38xD1 closeout and handoff`
- next queued micro task: `next source lane selection`
