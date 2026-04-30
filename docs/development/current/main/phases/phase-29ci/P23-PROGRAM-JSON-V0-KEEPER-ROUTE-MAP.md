---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: compact route map for the remaining Program(JSON v0) keeper buckets after P22.
Related:
  - docs/development/current/main/phases/phase-29ci/P22-SELFHOST-BUILD-MIR-ONLY-DIRECT-ROUTE.md
  - docs/development/current/main/phases/phase-29ci/P18-PROGRAM-JSON-V0-SHELL-EMIT-SSOT.md
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
---

# P23 Program JSON v0 Keeper Route Map

## Goal

Keep the active cleanup map short. Older P0/P3/P7/P12 ledgers remain useful
history, but new work should use this card as the current route map.

After P22, MIR-only selfhost build output is no longer a Program(JSON v0)
consumer. The remaining keepers are structural compatibility routes, not
simple dead shelf.

## Current Buckets

| Bucket | Owner | Current reason | Cleanup posture |
| --- | --- | --- | --- |
| shell emit spelling | `tools/lib/program_json_v0_compat.sh` | only current shell spelling of `--emit-program-json-v0` | keep until all shell callers leave |
| selfhost Stage-B artifact | `tools/selfhost/lib/selfhost_build_stageb.sh` | `--run`, `--keep-tmp`, and raw snapshot routes still need a Stage-B artifact | keep `--run`; normal `--exe` moved to direct source->MIR(JSON)->ny-llvmc in P26 |
| stage1 contract emit-program | `tools/selfhost/lib/stage1_contract.sh` | exact compat/probe helper for Program(JSON) materialization | keep; prune aliases only if zero caller |
| joinir/mirbuilder fixtures | `tools/smokes/v2/lib/stageb_helpers.sh` + `phase29bq_hako_mirbuilder_*` | `.hako` MirBuilder consumes Program(JSON v0) as its fixture input | keep unless a case is not testing MirBuilder-from-ProgramJSON |
| Rust bridge public flag | `src/runner/stage1_bridge/program_json_entry/**`, `src/runner/stage1_bridge/program_json/**`, `src/stage1/program_json_v0*` | `--emit-program-json-v0` public compat/deprecation route | delete last after shell/tool caller count reaches zero |
| retired hako alias guard | `src/cli/args.rs` test only | prevents `--hako-emit-program-json` reintroduction | keep as negative regression guard |

## Cleanup Order

1. Do not expand the historical ledgers unless a durable policy changes.
   Add compact cards like this one for active slices.
2. `selfhost_build.sh --exe` is no longer a Program(JSON v0) keeper for the
   normal non-diagnostic route. P26 fixed the direct `main(args)` entry birth
   shape in `ny-llvmc(boundary pure-first)` and moved the route to
   source->MIR(JSON)->EXE.
3. Keep `selfhost_build.sh --run` on Program(JSON v0) until direct
   `--mir-json-file` execution is green for the same reduced fixtures.
4. Keep joinir/mirbuilder fixture producers unless a caller only needs direct
   MIR or execution, because most of that bucket is explicitly testing
   Program(JSON v0)->`.hako` MirBuilder lowering.
5. Only after shell/tool callers are gone, remove the Rust bridge public flag
   and the runtime deprecation text.

## Acceptance

```bash
rg -n -g '!tools/historical/**' -g '!target/**' -- '--emit-program-json-v0|--program-json-to-mir|--hako-emit-program-json' src tools
rg -n -g '!target/**' 'program_json_v0_compat_emit_to_file|stageb_emit_program_json_v0_fixture' tools src lang
bash tools/checks/current_state_pointer_guard.sh
```
