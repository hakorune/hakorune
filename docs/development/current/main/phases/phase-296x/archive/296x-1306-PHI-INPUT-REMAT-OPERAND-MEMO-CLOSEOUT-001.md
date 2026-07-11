# 296x-1306 PHI-INPUT-REMAT-OPERAND-MEMO-CLOSEOUT-001

Status: closed  
Date: 2026-06-19  
Output contract: `phi-input-remat-operand-memo-closeout-v0`

## Decision

`PHI-INPUT-REMAT-OPERAND-MEMO-001` is closed.

PHI input rematerialization now uses predecessor-local memoization:

- same predecessor + same original `ValueId` reuses the same materialized
  `ValueId`.
- receiver-prefixed substring rematerialization keeps callee receiver and
  `args[0]` aligned without requiring a later string recognizer workaround.
- cycle handling remains fail-closed through the existing visiting set.
- accepted rematerialization shapes were not expanded.

## Evidence

```bash
cargo test -q phi_input_materializer
cargo test -q string_corridor_sink
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

All focused commands were green.

Known note:

```text
cargo test -q phi
```

is intentionally not the focused gate for this row because it matches unrelated
Ring0 / global-route tests outside the PHI input materializer ownership seam.

## Next

Open:

```text
STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-001
```

Scope:

- inventory `optimization_hints` string parsing used as planning/correctness
  evidence.
- replace the active evidence path with typed string-corridor relation / plan
  data where coverage already exists.
- do not remove diagnostic output-only hints unless the typed evidence path is
  proven sufficient.

## Stop Line

- do not add new string corridor route families.
- do not make benchmark names or helper names selection owners.
- do not widen PHI rematerialization shapes in this next row.
- do not reopen fastpath optimization without a fresh measured owner.

summary=ok
