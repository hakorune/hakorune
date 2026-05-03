---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P380E, generated Stage1 CLI env EXE emit-mir smoke
Related:
  - docs/development/current/main/phases/phase-29cv/P380D-STAGE1-CLI-ENV-PURE-FIRST-EXE-CONFIRM.md
  - docs/development/current/main/phases/phase-29cv/P335A-RETURN-STR-TEXT-EMIT.md
  - tools/selfhost/lib/stage1_contract.sh
---

# P380E: Stage1 CLI Env emit-mir Smoke

## Problem

P380D confirmed that the full `stage1_cli_env.hako` artifact can be compiled to
an EXE through the pure-first route. The next boundary is execution: the
generated EXE must still satisfy the current methodize-on `emit-mir` contract.

## Decision

Use the P380D artifact and run the existing small methodize-on `emit-mir`
canaries with plugins disabled:

```bash
NYASH_DISABLE_PLUGINS=1 \
HAKO_MIR_BUILDER_METHODIZE=1 \
NYASH_STAGE1_MODE=emit-mir \
STAGE1_SOURCE_TEXT='...' \
/tmp/p380c_clean_stage1_cli_env.exe
```

The plugin-disabled environment is intentional. The generated compiler artifact
does not need dynamic plugins for these source-to-MIR canaries, and disabling
plugins avoids failing on an unrelated local missing
`libnyash_integer_plugin.so`.

## Non-Goals

- no source changes
- no Stage0 route/classifier widening
- no plugin build or plugin-path change
- no broad regression pack in this card

## Acceptance

Run these canaries:

```bash
NYASH_DISABLE_PLUGINS=1 HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "hello" } }' \
  /tmp/p380c_clean_stage1_cli_env.exe

NYASH_DISABLE_PLUGINS=1 HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return "he\\\"llo" } }' \
  /tmp/p380c_clean_stage1_cli_env.exe

NYASH_DISABLE_PLUGINS=1 HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return true } }' \
  /tmp/p380c_clean_stage1_cli_env.exe

NYASH_DISABLE_PLUGINS=1 HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { local x = 5 return x } }' \
  /tmp/p380c_clean_stage1_cli_env.exe

NYASH_DISABLE_PLUGINS=1 HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/p380c_clean_stage1_cli_env.exe

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: each generated EXE run returns `Result: 0` and prints MIR JSON, not a
MirBuilder unsupported message.

## Result

All five canaries pass with `Result: 0`:

- `return "hello"` emits a string const and ret.
- escaped string payload emits `he\\\"llo` in the MIR JSON value.
- `return true` emits i64 value `1`.
- `local x = 5 return x` emits a const and ret.
- `return 1 + 2` emits two consts, a binop, and ret.

This confirms that the current generated Stage1 CLI env EXE is usable for the
small methodize-on `emit-mir` smoke surface.
