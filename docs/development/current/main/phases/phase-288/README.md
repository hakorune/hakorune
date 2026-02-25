# Phase 288: REPL mode (design-first, file-mode unaffected)

Status: ✅ Phase 288.1 complete (2025-12-25)

Goal: Add an interactive REPL for Nyash while keeping file mode semantics unchanged and keeping the language SSOT stable.

SSOT:
- REPL spec: `docs/reference/language/repl.md`

## Scope

- CLI entry: `hakorune --repl` / `-i`
- REPL commands: `.help`, `.exit/.quit`, `.reset`
- Fail-Fast: undefined reads are errors
- File mode: no behavior changes (quick smokes remain green)

## Completed (P0–P3)

- ✅ **P0 (docs)**: REPL SSOT 確立 — file mode vs REPL mode 意味論、VMValue 永続化方針 (`docs/reference/language/repl.md` 336行)
- ✅ **P1 (CLI)**: `--repl` / `-i` 起動、.help/.exit/.reset コマンド、REPL ループ実装
- ✅ **P2 (state)**: `ReplSessionBox` (VMValue ベース) 実装、暗黙 local 許可（file mode 不変、ValueId 永続化回避）
- ✅ **P3 (UX)**: print() 出力表示、Main box wrapper（VM entry point 修正）、.reset 実装
- ✅ **Box化**: `src/runner/repl/` モジュール分離（ReplRunnerBox）、runner/mod.rs -118行 (commit: 3445ef7a7)

## Completed (Phase 288.1) - Session Persistence + Auto-Display

✅ **AST Rewriter** (~430 lines, `src/runner/repl/ast_rewriter.rs`):
- Transforms undeclared variables: `x` → `__repl.get("x")`
- Transforms assignments: `x = 42` → `__repl.set("x", 42)`
- Respects `local` declarations, reserved names (`me`, `true`, `false`, `null`)
- Special handling: skips nested scopes, Main wrapper

✅ **ExternCall Bridge**:
- MIR builder: `__repl.get/set` → `ExternCall` lowering (`src/mir/builder/calls/build.rs` +41 lines)
- VM handler: `ExternCall("__repl", "get"/"set")` implementation (`src/backend/mir_interpreter/handlers/externals.rs` +54 lines)
- Fail-fast undefined variable errors with hints

✅ **Session Management**:
- `Rc<RefCell<ReplSessionBox>>` pattern for proper persistence
- VM receives session via `set_repl_session()` (`src/backend/mir_interpreter/mod.rs` +12 lines)
- Session survives across REPL lines

✅ **Expression Auto-Display**:
- Detects pure expressions vs statements (wrapper AST check)
- Auto-displays non-Void expression results
- Stores displayed values in `_` variable

✅ **Testing**:
- All 154 smoke tests pass (no regressions)
- Variable persistence: `x = 42` then `print(x)` → `42`
- Expression display: `1 + 1` → `2`
- `_` variable: `10 * 2` → `20`, then `_` → `20`
- Fail-fast: undefined variables error with hints
- Session reset: `.reset` clears all variables

**Files Modified** (8 files, +592 lines):
- `src/runner/repl/ast_rewriter.rs` (NEW, +430 lines)
- `src/runner/repl/repl_runner.rs` (+84/-35 lines)
- `src/backend/mir_interpreter/handlers/externals.rs` (+54 lines)
- `src/mir/builder/calls/build.rs` (+41 lines)
- `src/backend/mir_interpreter/mod.rs` (+12 lines)
- `src/runner/repl/repl_session.rs` (+11/-9 lines)
- `src/runner/repl/mod.rs` (+2 lines)
- `src/runner/mod.rs` (+2/-1 lines)

**Confirmation Commands**:
```bash
# Variable persistence
echo -e 'x = 42\nprint(x)\n.exit' | ./target/release/hakorune --repl

# Expression auto-display + _
echo -e '1 + 1\n_\n.exit' | ./target/release/hakorune --repl

# Fail-fast undefined
echo -e 'print(y)\n.exit' | ./target/release/hakorune --repl

# Session reset
echo -e 'x = 1\n.reset\nprint(x)\n.exit' | ./target/release/hakorune --repl

# Regression test
./tools/smokes/v2/run.sh --profile quick  # 154/154 PASS
```

## Next

Further UX improvements (deferred):
- Multi-line input handling (continuations)
- Syntax highlighting
- Command history persistence
