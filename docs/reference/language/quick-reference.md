# Nyash Quick Reference (MVP)

Purpose
- One‑page practical summary for writing and implementing Nyash.
- Keep grammar minimal; clarify rules that often cause confusion.

Keywords (reserved)
- control: `if`, `else`, `loop`, `match`, `case`, `break`, `continue`, `return`
- decl: `static`, `box`, `local`, `using`, `as`
- lit: `true`, `false`, `null`, `void`

Expressions and Calls
- Function call: `f(a, b)`
- Method call: `obj.m(a, b)` — internally normalized to function form: `Class.m(me: obj, a, b)`
  - Default‑ON（P4）: Known 受信者かつ関数が一意に存在する場合に正規化（userbox 限定）。
  - それ以外（Unknown/core/user‑instance）は安全に BoxCall へフォールバック（挙動不変）。
  - 環境で無効化: `NYASH_REWRITE_KNOWN_DEFAULT=0`（開発時の切替用）。
  - バックエンド（VM/LLVM/Ny）は統一形状の呼び出しを受け取る。
- Member: `obj.field` or `obj.m`

Display & Conversion
- Human‑readable display: `x.toString()`（推奨）
  - ユーザーBoxで表示をカスタムしたい場合は `str()` または `stringify()`（互換）を実装する。VMは `toString()` を `str()/stringify()` に再ルーティングする。
- Debug表示（構造的・安定）: `repr(x)`（将来導入、devのみ）
- JSONシリアライズ: `toJson(x)`（文字列）/ `toJsonNode(x)`（構造）

Operators (precedence high→low)
- Unary: `! ~ - weak` (`weak <expr>` produces a `WeakRef`; `weak(expr)` is invalid)
- Multiplicative: `* / %`
- Additive: `+ -`
- Compare: `== != < <= > >=`
- Logical: `&& ||` (short‑circuit, side‑effect aware)

Semicolons and ASI (Automatic Semicolon Insertion)
- Allowed to omit semicolon at:
  - End of line, before `}` or at EOF, when the statement is syntactically complete.
- Not allowed:
  - Line break immediately after a binary operator (e.g., `1 +\n2`)
  - Ambiguous continuations; parser must Fail‑Fast with a clear message.

Truthiness (boolean context)
- SSOT: `reference/language/types.md`（runtime truthiness）
- 実行上は `Bool/Integer/Float/String/Void` が中心。`BoxRef` は一部のコアBoxのみ許可され、その他は `TypeError`（Fail-Fast）。
- `null` は `void` の別名（構文糖衣）。どちらも boolean context では `TypeError`。

Equality and Comparison
- SSOT: `reference/language/types.md`（`==`/`!=` と `< <= > >=` の runtime 仕様）
- `==` の cross-kind は **`Integer↔Float` のみ**（精密ルール）。それ以外の mixed kinds は `false`（エラーではない）。
- `< <= > >=` は `Integer/Float/String` の **同型同士**のみ（異型は `TypeError`）。

String and Numeric `+`
- SSOT: `reference/language/types.md`（runtime `+` 仕様）
- `Integer+Integer` は加算、`Float+Float` は加算、`Integer↔Float` は `Float` に昇格して加算。
- 文字列連結は **`String + String` のみ**。`"a"+1` / `1+"a"` は `TypeError`（暗黙 stringify なし）。
- 文字列化して連結したい場合は明示的に `x.toString()` を使う。

Blocks and Control
- `if (cond) { ... } [else { ... }]`
- `loop (cond) { ... }` — minimal loop form
- `match (expr) { case ... }` — MVP (literals and simple type patterns)

Using / SSOT
- Dev/CI: file‑based `using` allowed for convenience.
- Prod: `nyash.toml` only. Duplicate imports or alias rebinding is an error.

Errors (format)
- Always: `Error at line X, column Y: <message>`
- For tokenizer errors, add the reason and show one nearby line if possible.

Dev/Prod toggles (indicative)
- `NYASH_DEV=1` — developer defaults (diagnostics, tracing; behavior unchanged)
- `NYASH_ENABLE_USING=1` — enable using resolver
- `NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1` — allow `main` as top‑level entry

Notes
- Keep the language small. Prefer explicit conversions (`x.toInteger()`, `x.toString()`, `x.toBool()`) over implicit coercions.
- Builder rewrites method calls to keep runtime dispatch simple and consistent across backends.
