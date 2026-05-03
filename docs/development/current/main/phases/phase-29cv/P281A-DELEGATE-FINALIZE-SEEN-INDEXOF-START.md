---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P281a, delegate finalize seen-indexOf explicit start
Related:
  - docs/development/current/main/phases/phase-29cv/P280A-COUNT-PARAM-FINAL-EMIT-TEXT-MATERIALIZATION.md
  - lang/src/mir/builder/internal/delegate_finalize_box.hako
---

# P281a: Delegate Finalize Seen IndexOf Explicit Start

## Problem

After P280a, the source-execution probe advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderDelegateFinalizeBox._build_user_box_decls_from_program_json/1
```

The body already routes the main string scans with explicit start positions:

```text
s.indexOf("\"box\":\"", pos)
s.indexOf("\"", name_start)
```

but the duplicate-name guard still uses the one-argument form on a loop-PHI
`seen` receiver:

```text
seen.indexOf(marker)
```

The Stage0 route should not grow generic one-argument `String.indexOf`
semantics or infer this PHI receiver shape just to pass this owner-local
sentinel scan.

## Decision

Make the search receiver and start explicit at the `.hako` owner boundary:

```text
local seen_text = "" + seen
seen_text.indexOf(marker, 0)
```

This keeps the body on the already-supported `StringIndexOf` route form and
does not add any new C shim or classifier behavior.

## Non-Goals

- no generic `String.indexOf/1` widening
- no loop-PHI receiver inference widening
- no new accepted body shape
- no C shim/body-specific emitter change
- no user box declaration behavior change

## Acceptance

- `BuilderDelegateFinalizeBox._build_user_box_decls_from_program_json/1` no
  longer stops because of the owner-local `seen.indexOf(marker)` call shape.
- The source-execution probe advances to the next blocker.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

The duplicate-name scan now has an explicit `StringIndexOf` plan entry:

```text
b12719.i13	string_indexof	StringIndexOf	2	nyash.string.indexOf_hh	DirectAbi
```

Fresh source-execution advanced past
`BuilderDelegateFinalizeBox._build_user_box_decls_from_program_json/1` and now
stops at the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1
```
