# Phase 288.1 P1: REPL Session Persistence + Auto-Display (instruction sheet)

## Objective

- Make REPL variables persist across input lines (`x = 1` then `print(x)` works).
- Add minimal expression auto-display and `_` last-value binding (REPL-only).
- Keep file mode semantics unchanged; quick smokes must remain green.

SSOT:
- `docs/reference/language/repl.md`

## Constraints (non-negotiable)

- No new env vars.
- No file-mode behavior changes.
- Keep REPL-specific state and logic isolated (Box-first).

## Design (recommended)

Do **not** persist `ValueId` across evaluations. Persist runtime values (VMValue/handle) in `ReplSessionBox`, and bridge them via a REPL-only rewrite step.

### Bridge approach: REPL-only AST rewrite

In REPL mode, rewrite variable reads/writes that are not declared in the current input line into calls against a REPL session object:

- Read `x` → `__repl.get("x")`
- Assignment `x = expr` → `__repl.set("x", expr)`
- `_` is treated as a normal name in the session (written by auto-display).

This keeps the parser unchanged and avoids contaminating the normal MIR builder scope model.

### Session API (REPL-only)

Provide a minimal host object `__repl` with:

- `get(name: String) -> Any` (Fail-Fast if missing)
- `set(name: String, value: Any) -> void`
- `reset() -> void`

Implementation options:

- Rust VM only (initially): inject `__repl` as a built-in box/object available only in `--repl`.
- LLVM parity is not required for Phase 288.1; REPL is a dev tool.

## Step-by-step

1) Add a REPL-only `__repl` host object and VM handlers for `get/set/reset`.
2) Implement AST rewrite in the REPL runner before compilation:
   - Collect “declared names” in the current line (`local name`, function params, etc.).
   - Rewrite remaining identifier reads/writes to `__repl.get/set`.
3) Implement auto-display:
   - If the input is a pure expression, `print(result)` (or equivalent) in REPL only.
   - Store result into session name `_` (skip if `void`).
4) Add conformance checks (manual is OK if no test harness exists yet):
   - `x = 1` (silent)
   - `print(x)` prints `1`
   - `y` (read) is NameError
   - `1 + 1` prints `2` and `_` becomes `2`
   - `.reset` clears variables (`print(x)` errors)
5) Run regression gate:
   - `./tools/smokes/v2/run.sh --profile quick` must stay `0 FAILED`.

