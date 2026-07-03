# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-03
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read the `latest_card_path` named in `CURRENT_STATE.toml`.
3. Read the workstream/task-order doc named by `latest_workstream_card`, when present.
4. If `current_blocker_token` names an explicit design-stop frontier, also
   read `docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md`
   before selecting any new family-specific task.
5. Check the worktree:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the current code slice is ready.

## Current Fields

Read these fields in `docs/development/current/main/CURRENT_STATE.toml`:

- `active_lane`
- `active_phase`
- `latest_workstream_card`
- `latest_card_path`
- `current_blocker_token`

If `current_blocker_token` names the explicit design-stop frontier, do not
invent a new executable owner from historical mirrors. Review the frontier
card first and keep the goal open until the frontier names a concrete next
owner.

Current implementation details, acceptance, parked items, and non-claims live in
the active card and task-order SSOT. Do not duplicate them here.

## Immediate Maintenance Slice

Status: closed for the Hakorune naming cleanup guard follow-up.

The completed guard follow-up is tracked in:

```text
docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md
```

Current state:

- all `HAKORUNE-*` naming cleanup slices in the task-order SSOT are landed;
- the only `active in this slice` entry is the always-on
  `NAMING-CHARTER-STAGE-TERM-DISAMBIGUATION-001` guardrail;
- broad package/env/ABI renames remain out of scope for maintenance slices;
- PHI / LocalSSA / variable-map internals remain out of scope for naming
  cleanup.

Stage-term existing-name migration has a classification inventory at:

```text
docs/development/current/main/design/hakorune-stage-term-existing-name-migration-inventory.md
```

The first classified stage-term migration slice is landed:

```text
STAGE-TERM-SYNTAX3-ALIAS-001
STAGE-TERM-SYNTAX3-DIAGNOSTIC-WORDING-001
STAGE-TERM-MODEB-COMPAT-ENV-WORDING-001
STAGE-TERM-MODEB-PROOF-ROUTE-WORDING-001
STAGE-TERM-MODEB-STAGE1-BRIDGE-WORDING-001
STAGE-TERM-MODEA-COMPAT-ROUTE-WORDING-001
STAGE-TERM-MODEB-HHAKO-ENTRY-WORDING-001
STAGE-TERM-MODEB-HHAKO-COMPAT-FIXTURE-WORDING-001
STAGE-TERM-HHAKO-COMPILER-ROUTE-WORDING-001
STAGE-TERM-MODEB-HHAKO-HELPER-COMMENT-WORDING-001
STAGE-TERM-MODEB-CAPTURE-CALLER-GUARD-WORDING-001
STAGE-TERM-PHASE1-PROGRAM-JSON-GUARD-WORDING-001
STAGE-TERM-STAGE0-SHAPE-GATE-LABEL-WORDING-001
STAGE-TERM-MODEB-HHAKO-FUNC-SCANNER-COMMENT-WORDING-001
STAGE-TERM-MODEB-K2-WIDE-GUARD-DIAGNOSTIC-WORDING-001
STAGE-TERM-SELFHOST-SMOKE-COMMENT-WORDING-001
STAGE-TERM-CHECK-SCRIPTS-INDEX-WORDING-001
STAGE-TERM-SYNTAX3-RUST-ENV-COMMENT-WORDING-001
STAGE-TERM-HHAKO-PARSER-BUILD-COMMENT-WORDING-001
STAGE-TERM-JSON-V0-BRIDGE-COMMENT-WORDING-001
```

`--syntax-3` is now the frontend syntax-level spelling; `--stage3` remains a
compatibility alias. Live MIR builder hints now say `syntax-3` and `mode-B`
compatibility routes. Live env docs/comments now describe `STAGEB` names as
mode-B compatibility aliases without renaming those compatibility surfaces.
The explicit proof-only selfhost route docs/diagnostics now also say mode-B
compatibility while keeping `--stage-b`, `stageb-delegate`, and script names as
compatibility surfaces. Stage-1 bridge module payload comments/docs also say
mode-B compatibility while keeping `HAKO_STAGEB_*` env names and bridge file
names unchanged. Rust selfhost compat route comments/diagnostics now say
mode-A compatibility while keeping `stage-a-compat` runtime-mode aliases and
`stage_a_*` file/function names unchanged. HHako compiler entry comments now
say mode-B compatibility while keeping `StageB*` Box names, trace strings,
file names, and `--stage-b` route tokens unchanged. HHako legacy fixture and
adapter comments that defer authority to BuildBox now also say mode-B
compatibility while keeping env names, trace strings, and Box names unchanged.
`compiler.hako` route comments and the string-indexing diagnostic now say
mode-A/mode-B compatibility while keeping `stage_b`, trace strings, Box names,
and route tokens unchanged. HHako helper comments for driver guard, trace,
main/body detection, Rune helper, and user-box declaration scanner now also say
mode-B/mode-A compatibility while keeping `StageB*` Box names, trace strings,
env names, file names, and route tokens unchanged. The active Program(JSON)
capture caller guard comments/diagnostics and quick gate label now say mode-B
compatibility while keeping compatibility script names and allowed caller
surfaces unchanged. Active Stage1 Program(JSON) guard comments/diagnostics and
quick gate labels now say phase-1 compatibility while keeping script names,
fixture names, and helper symbols unchanged. The Stage0-named shape inventory
script now appears in quick gate output as `GlobalCallTarget shape inventory
guard` while keeping script and inventory doc paths unchanged. FuncScanner
comments now say mode-B compatibility while keeping PHI / LocalSSA /
variable-map internals untouched. Two active K2-wide guard diagnostics now say
mode-B compatibility while keeping script names, StageB Box names, and guard
logic unchanged. Active selfhost smoke comments and human-facing diagnostics
now say mode-B compatibility, phase-1 compatibility, or syntax-3 while keeping
smoke file names, compatibility route tokens, exact expected stderr, and
StageB/Stage1 Box names unchanged. The check-scripts index rows for
already-migrated active guards now also use mode-B / phase-1 / GlobalCallTarget
wording while keeping guard script names unchanged. Rust env comments now say
syntax-3 for the parser surface while keeping `stage3` feature tokens and env
names as compatibility surfaces. HHako parser/build/MIR builder comments now
also use mode-A/mode-B compatibility or syntax-3 wording while keeping `stage3`
fields/functions, trace strings, file names, and PHI/LocalSSA/variable-map
internals untouched. Rust JSON v0 bridge comments and one local freeze
diagnostic now use mode-B compatibility / bootstrap wording while keeping
`try_lower_stageb_*` names, route symbols, and behavior unchanged. Next safe
naming work, if explicitly selected, must pick a different classified layer
from that inventory and keep compatibility aliases or replacement routes in the
same slice.

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
