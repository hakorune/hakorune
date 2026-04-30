Stage-B Parser - Loop JSON Canary (archived)

Purpose
- Guard against regressions where `loop(i<n){ i=i+1 }` is emitted as an empty body or `i<0` in Program(JSON v0).
- Keep Hakorune selfhost‑first line productive without Rust rebuilds. Fallback is conservative and only triggers on broken shapes.

What was added
- Parser fallback (dev‑only, defaults unchanged):
  - In `lang/src/compiler/parser/stmt/parser_control_box.hako::parse_loop`:
    - If loop body parses as `[]`, re‑parse block region directly to recover statements.
    - If `cond.rhs` regresses to `Int(0)`, extract identifier after '<' and reconstruct rhs as `Var(name)`.
  - These guards only activate on broken shapes and do not affect well‑formed inputs.

- Archived canary script:
  `tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh`
  - Builds a minimal loop program and asserts:
    - Loop node exists
    - `cond` is `Compare` with `<`
    - `lhs` is `Var i`
    - `rhs` is `Var` or `Int`
    - body contains `Local i = i + 1`

How to run
```
bash tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh
# [PASS] stageb_loop_json_canary
```

Notes
- This is historical stabilization evidence, not a current gate. The active
  Program(JSON v0) keeper set is tracked by the phase-29cv compat capsule docs.
- This was originally a temporary stabilization. The root cause (gpos/VM interaction in nested contexts) should be addressed separately; once fixed, the fallback can be removed.
- No defaults were changed; no additional logs are emitted unless instrumented locally. The fallback only modifies JSON when the previously observed broken forms are detected.
