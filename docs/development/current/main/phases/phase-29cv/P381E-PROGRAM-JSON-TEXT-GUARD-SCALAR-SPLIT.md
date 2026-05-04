---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381E, Stage1 Program(JSON) text guard cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381D-STAGE1-EMIT-MIR-PROGRAM-JSON-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - lang/src/runner/stage1_cli_env.hako
---

# P381E: Program(JSON) Text Guard Scalar Split

## Problem

P381D moved the explicit Program(JSON)-to-MIR call to an extern route and exposed
the next source-execution blocker:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=Stage1ProgramJsonTextGuardBox.coerce_text_checked/3
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`Stage1ProgramJsonTextGuardBox.coerce_text_checked/3` mixed three responsibilities
in one helper:

```text
presence check -> failure logging -> String/null return
```

That created a shared `String|Void` helper. After splitting it, the same shape
remained in thin compat wrappers above the checked handoff leaf. Pulling those
wrappers into Stage0 as another body shape would widen the generic string
classifier and violate the P207A size guard.

## Decision

Retire the shared `String|Void` guard/wrapper helpers and split the contract into
existing small pieces:

```text
Stage1ProgramJsonTextGuardBox.input_present/1      -> ScalarI64
Stage1ProgramJsonTextGuardBox._coerce_text_compat/1 -> String
caller-local missing branch                         -> null
Stage1ProgramJsonMirCallerBox._emit_mir_from_program_json_text_checked/2
                                                    -> checked handoff leaf
Stage1EmitMirDispatchBox.run_emit_mir_program_json_compat_mode/1
                                                    -> compat orchestration
```

The two callers keep their own fail-fast message and null return. The shared box
still owns the presence predicate and text coercion, and the MIR caller box still
owns the exact `MirBuilderBox.emit_from_program_json_v0(...)` handoff. Neither
box re-exports a `String|Void` wrapper across an extra global helper boundary.
The explicit compat entry performs guard, coercion, handoff, null-to-exit-code,
and final validation in one i64-return orchestration function.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic `String|Void` classifier widening
- no C shim body-specific emitter
- no change to the external compat contract
- no VM fallback

## Acceptance

```bash
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond
`Stage1ProgramJsonTextGuardBox.coerce_text_checked/3`.

## Result

Accepted.

The phase29cg replay moved beyond
`Stage1ProgramJsonTextGuardBox.coerce_text_checked/3` and the follow-on thin
compat wrappers:

```text
Stage1ProgramJsonMirCallerBox.emit_mir_from_program_json_checked/2
Stage1ProgramJsonCompatBox.emit_mir_from_text/2
```

After the split, `Main._run_emit_mir_program_json_compat_mode/1` and
`Stage1EmitMirDispatchBox.run_emit_mir_program_json_compat_mode/1` are both
classified as direct `generic_i64_body` targets. The next blocker is a separate
LLVM IR type issue in `Stage1InputContractBox._env_flag_enabled/1`:

```text
opt: /tmp/p381e_bad.ll:257:19: error: '%r8' defined with type 'i1' but expected 'i64'
  %r9 = phi i64 [ %r8, %bb1 ], [ 0, %bb2 ]
                  ^
```
