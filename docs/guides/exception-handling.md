Exception Handling — Postfix catch / cleanup (Stage‑3)

Summary
- Nyash adopts a flatter, postfix-first exception style:
  - There is no `try` statement in the language spec. Use postfix `catch` and `cleanup` instead.
  - `catch` = handle exceptions from the immediately preceding expression/call.
  - `cleanup` = always-run finalization (formerly finally), regardless of success or failure.
- This matches the language’s scope unification and keeps blocks shallow and readable.

Spec Clarifications (Stage‑3)
- Acceptance gates and profiles
  - Expression‑postfix: `NYASH_PARSER_STAGE3=1` enables `expr catch(...) {..} cleanup {..}` on calls/chains.
  - Block‑postfix: `NYASH_BLOCK_CATCH=1` or Stage‑3 enables `{ ... } catch(...) {..} cleanup {..}` (standalone block statement)。
  - Method‑postfix: `NYASH_METHOD_CATCH=1` or Stage‑3 enables method body postfix on the most recent method.
- Cardinality and order
  - Postfix (expr/block/method): at most one `catch` and at most one `cleanup` — in this order. A second `catch` after postfix is a parse error. Multiple `cleanup` are not allowed.
  - Legacy compatibility: some builds may still accept the historical `try { ... } catch ... cleanup ...` form, but it is not part of the language spec and will be disabled by default. Prefer postfix forms.
- Binding and chaining
  - Postfix binds to the immediately preceding expression (the last call in a chain) or to the just‑parsed block/method body. It does not extend to the entire statement unless parentheses are used.
  - After constructing the postfix `TryCatch`, further method chaining on that expression is not accepted.
- Semantics and control‑flow
  - `cleanup` (finally) always runs, regardless of success/failure of the try part.
  - `return` inside the try part is deferred until after `cleanup` executes. This is implemented by the MIR builder as a deferred return slot/jump to the `cleanup`/exit block.
  - `return` inside `cleanup` is disallowed by default; enable with `NYASH_CLEANUP_ALLOW_RETURN=1`.
  - `throw` inside `cleanup` is disallowed by default; enable with `NYASH_CLEANUP_ALLOW_THROW=1`.
  - `break/continue` inside `cleanup` are allowed (no special guard); use with care. Cleanup executes before the loop transfer takes effect.
  - Nested cleanup follows lexical unwinding order (inner cleanup runs before outer cleanup).
  - If no `catch` is present, thrown exceptions still trigger `cleanup`, then propagate outward.
- Diagnostics
  - Method‑postfix: duplicate postfix after a method body is a parse error: "duplicate postfix catch/cleanup after method".
  - Block‑postfix: a standalone postfix without a preceding block is a parse error: "catch/cleanup must follow a try block or standalone block".
  - Expression‑postfix: only one `catch` is accepted at expression level; a second `catch` triggers a parse error.

Status
- Phase 1: normalization sugar（既存）
  - `NYASH_CATCH_NEW=1` でコア正規化パスが有効化。
  - 後置フォームは内部 `TryCatch` AST に変換され、既存経路で降下。
  - 実行時コストはゼロ（意味論不変）。
- Phase 2（実装済み・Stage‑3ゲート）
  - パーサが式レベルの後置 `catch/cleanup` を直接受理。
  - ゲート: `NYASH_PARSER_STAGE3=1`
  - 糖衣正規化はそのまま併存（関数糖衣専用）。キーワード直受理と二重適用はしない設計。

Syntax (postfix)
- Expression-level postfix handlers (Stage‑3):
  - `expr catch(Type e) { /* handle */ }`
  - `expr catch { /* handle (no variable) */ }`
  - `expr cleanup { /* always-run */ }`
  - Combine: `expr catch(Type e){...} cleanup{...}`
- Method/function calls are just expressions, so postfix applies:
  - `call(arg1, arg2) catch(Error e) { log(e) }`
  - `obj.method(x) cleanup { obj.release() }`

Precedence and chaining
- Postfix `catch`/`cleanup` binds to the immediately preceding expression (call/chain result), not to the whole statement.
- For long chains, we recommend parentheses to make intent explicit:
  - `(obj.m1().m2()) catch { ... }`
  - `f(a, b) catch { ... } cleanup { ... }`
- Parser rule (Stage‑3): postfix attaches once at the end of a call/chain and stops further chaining on that expression.

Diagram (conceptual)
```
// before (parse)
obj.m1().m2() catch { H } cleanup { C }

// precedence (binding)
obj.m1().[ m2()  ↖ binds to this call ] catch { H } cleanup { C }

// normalization (conceptual AST)
TryCatch {
  try:   [ obj.m1().m2() ],
  catch: [ (type:Any, var:None) -> H ],
  finally: [ C ]
}
```

Normalization (Phase 1)
- With `NYASH_CATCH_NEW=1`, postfix sugar is transformed into legacy `TryCatch` AST:
  - `EXPR catch(T e){B}` → `TryCatch { try_body:[EXPR], catch:[(T,e,B)], finally:None }`
  - `EXPR cleanup {B}` → `TryCatch { try_body:[EXPR], catch:[], finally:Some(B) }`
  - Multiple `catch` are ordered top-to-bottom; first matching type handles the error.
  - Combined `catch ... cleanup ...` expands to a single `TryCatch` with both blocks.
- Lowering uses the existing builder (`cf_try_catch`) which already supports cleanup semantics.

Semantics
- catch handles exceptions from the immediately preceding expression only.
- cleanup is always executed regardless of success/failure (formerly finally).
- Multiple catch blocks match by type in order; the first match is taken.
- In loops, `break/continue` cooperate with cleanup: cleanup is run before leaving the scope.
 - Return deferral: A `return` in the try section defers until after cleanup. `return`/`throw` inside cleanup are disabled by default; see env toggles below.

Environment toggles
- `NYASH_PARSER_STAGE3=1`: Enable Stage‑3 syntax (postfix catch/cleanup for expressions; also gates others by default)
- `NYASH_BLOCK_CATCH=1`: Allow block‑postfix (independent of Stage‑3 if needed)
- `NYASH_METHOD_CATCH=1`: Allow method‑postfix (independent of Stage‑3 if needed)
- `NYASH_CLEANUP_ALLOW_RETURN=1`: Permit `return` inside cleanup (default: off)
- `NYASH_CLEANUP_ALLOW_THROW=1`: Permit `throw` inside cleanup (default: off)

Migration notes
- try is deprecated: prefer postfix `catch/cleanup`.
- Member-level handlers (computed/once/birth_once/method) keep allowing postfix `catch/cleanup` (Stage‑3), unchanged.
- Parser acceptance of postfix at expression level will land in Phase 2; until then use the gate for normalization.

Examples
```
// Postfix catch on a call
do_work() catch(Error e) { env.console.log("error: " + e) }

// Always-run cleanup
open_file(path) cleanup { env.console.log("closed") }

// Combined
connect(url)
  catch(NetworkError e) { env.console.warn(e) }
  cleanup { env.console.log("done") }

// Stage‑3 parser gate quick smoke (direct acceptance)
//   NYASH_PARSER_STAGE3=1 ./target/release/hakorune --backend vm \
//     apps/tests/macro/exception/expr_postfix_direct.hako
```
