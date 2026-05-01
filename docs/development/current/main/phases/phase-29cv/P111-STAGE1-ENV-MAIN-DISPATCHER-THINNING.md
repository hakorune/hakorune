---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: restore full Stage1 env Main to thin dispatcher form before adding new backend shape.
Related:
  - docs/development/current/main/phases/phase-29cv/P110-STAGE1-ENV-GLOBAL-PROGRAM-JSON-OWNER-LOCK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - lang/src/runner/stage1_cli_env.hako
---

# P111 Stage1 Env Main Dispatcher Thinning

## Goal

Remove the stale inline emit implementation from `Main.main()` in the full
Stage1 env source.

P110 classified the current pure-first stop:

```text
mir_call Global BuildBox.emit_program_json_v0/2
```

That stop happens because `Main.main()` still carries an older inline
`emit-program` / `emit-mir` implementation before the owner-local boxes in the
same file. The file already has the intended structure:

```text
Main
  -> Stage1ModeContractBox
  -> Stage1EmitMirDispatchBox
  -> Stage1SourceProgramAuthorityBox
  -> Stage1SourceMirAuthorityBox
  -> Stage1ProgramJsonCompatBox
```

The clean next step is to use that structure instead of adding any backend
matcher.

## Decision

- `Main.main()` becomes a thin dispatcher:
  - resolve mode via `Stage1ModeContractBox.resolve_mode()`
  - call the existing `Main._run_*` helper methods
  - return `97` for no/unknown mode
- The exact source-to-Program authority remains in
  `Stage1SourceProgramAuthorityBox`.
- The exact source-to-MIR authority remains in `Stage1SourceMirAuthorityBox`.
- The explicit Program(JSON) compat route remains quarantined in
  `Stage1ProgramJsonCompatBox`.
- No new entry file, shell route, environment variable, or ny-llvmc matcher is
  added in this card.

## Acceptance

The diagnostic MIR for the full Stage1 env source must no longer have a direct
`Main.main` call to `BuildBox.emit_program_json_v0/2`.

```bash
target/release/hakorune \
  --emit-mir-json /tmp/p111_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako

jq -e '
  [.functions[]
   | select(.name == "main")
   | .. | objects
   | select(.op? == "mir_call")
   | select(.mir_call.callee.type? == "Global")
   | select(.mir_call.callee.name? == "BuildBox.emit_program_json_v0/2")]
  | length == 0
' /tmp/p111_stage1_cli_env.mir.json

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The full env pure-first compile may still fail on a user/global-call owner
boundary. That is expected; this card removes the stale main-level
Program(JSON) authority leak only.

Observed after this change:

```text
main BuildBox.emit_program_json_v0/2 calls: 0
full-source BuildBox.emit_program_json_v0/2 calls: 2
next pure-first stop:
  mir_call Global Stage1ModeContractBox.resolve_mode/0
```

That next stop is the typed user/global-call family, not Program(JSON v0)
authority leakage.
