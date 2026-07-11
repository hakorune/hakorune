---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: `tools/hakorune_emit_mir.sh` の helper-local legacy Program(JSON)->MIR CLI delegate を退役する。
Related:
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - docs/development/current/main/phases/archive/phase-29ci/README.md
  - tools/hakorune_emit_mir.sh
  - tools/smokes/v2/lib/emit_mir_route.sh
---

# P8 Helper Legacy Delegate Retire

## Goal

P7 の Candidate A を実施する。

`tools/hakorune_emit_mir.sh` の helper default route から、最後の raw
`--program-json-to-mir` fallback (`delegate-legacy`) だけを削る。

## Decision

- `hako-mainline` はこれまで通り selfhost-first + no-delegate + mainline-only の fail-fast route。
- `hako-helper` は selfhost-first を試し、必要なら provider fallback (`env.mirbuilder.emit`) までに留める。
- helper-local `delegate-legacy` は削除する。
- raw CLI `--program-json-to-mir` 自体は削除しない。P7 の他 bucket がまだ live。

## Why This Is Safe

- `hako-mainline` は既に `HAKO_SELFHOST_NO_DELEGATE=1` で fallback を禁止している。
- representative helper canaries は selfhost-first または provider route で緑。
- provider canary は `delegate-legacy` タグ不在を明示的に確認する。

## Out Of Scope

- `src/runner/pipe_io.rs` の `--program-json-to-mir` 実装削除。
- `tools/selfhost/lib/selfhost_build_exe.sh` / `tools/selfhost_exe_stageb.sh` の Stage-B delegate 移行。
- `phase29bq_hako_mirbuilder_*` の Program(JSON) fixture producer 移行。

## Acceptance

```bash
bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/smokes/v2/profiles/integration/core/phase2231/hakorune_emit_mir_return42_canary_vm.sh
bash tools/smokes/v2/profiles/integration/core/phase215/emit_provider_no_jsonfrag_canary.sh
bash tools/checks/current_state_pointer_guard.sh
```

Inventory checks:

```bash
rg -n -- 'try_legacy_program_json_delegate|delegate-legacy|provider\\+legacy' tools/hakorune_emit_mir.sh
rg -n -- '--program-json-to-mir' tools/hakorune_emit_mir.sh
```

Both inventory checks should return no live helper hit.
