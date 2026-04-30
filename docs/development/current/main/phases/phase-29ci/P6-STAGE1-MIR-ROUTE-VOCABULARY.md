---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: `--hako-emit-mir-json` を current Stage-1 MIR authority launcher として固定し、Program(JSON v0) public compat surface の削除順を再開する。
Related:
  - docs/development/current/main/phases/archive/phase-29ci/README.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/reference/environment-variables.md
  - src/cli/args.rs
---

# P6 Stage1 MIR Route Vocabulary

## Goal

`--hako-emit-mir-json` と Program(JSON v0) compat flags を同じ bucket に読まないよう、route vocabulary を先に固定する。

この slice は hard-delete resume の入口だが、authority migration ではない。

## Vocabulary

| Surface | Route class | Current action |
| --- | --- | --- |
| `stage1-env-mir-source` | current authority route id | keep; source -> MIR(JSON) の正本 |
| `--hako-emit-mir-json <out> <src>` | launcher CLI entry | keep; `stage1-env-mir-source` へ入る public/dev entry |
| `--emit-mir-json <out> <src>` | Rust direct debug/mainline emit | keep; selfhost authority ではない |
| raw `stage1-cli emit mir-json` / binary-only direct | monitor-only legacy | keep until evidence lanes are retired |
| `stage1-env-mir-program` | explicit Program(JSON) -> MIR compat | keep as exact probe/helper only |
| `--emit-program-json-v0` | raw Program(JSON v0) compat emit | keep until smoke/tool callers are migrated |
| `--program-json-to-mir` | raw Program(JSON v0) -> MIR compat convert | retired in P16 |
| `--hako-emit-program-json` | hako-prefixed Program(JSON) public alias | retired in this slice; it had no current tool/smoke caller bucket |

## Delete Order

1. Sync docs/help so `--hako-emit-mir-json` reads as the MIR authority launcher, not as a generic `json_v0_bridge` route.
2. Remove the hako-prefixed Program(JSON) public alias first: `--hako-emit-program-json`. Status: landed in this slice.
3. Keep raw `--emit-program-json-v0` until its shell/smoke callers are migrated or explicitly quarantined. `--program-json-to-mir` is retired in P16.
4. Only after raw compat caller inventory reaches zero, reopen `src/runner/stage1_bridge/program_json*` / `src/stage1/program_json_v0*` hard delete.

## Guardrails

- Do not remove `--hako-emit-mir-json`; it is the public/dev launcher for the current source -> MIR route.
- Do not mix raw compat flag deletion with `.hako` live/bootstrap caller cleanup.
- Do not delete `--emit-program-json-v0` while phase29bq mirbuilder smokes still pin Program(JSON) fixtures through it.
- Do not reintroduce `--program-json-to-mir`; explicit helper work must use
  `env.mirbuilder.emit` / `tools/selfhost/lib/program_json_mir_bridge.sh`.

## Acceptance

- `selfhost-bootstrap-route-ssot.md` owns the route vocabulary table.
- public docs no longer present `--hako-emit-program-json` as a usable example.
- CLI help for `--hako-emit-mir-json` names `stage1-env-mir-source`.
- `--hako-emit-program-json` is no longer a registered CLI flag or `CliConfig` / `EmitConfig` field.
- pointer guard stays green after current lane mirror sync.
