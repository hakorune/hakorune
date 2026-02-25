# Statement Separation and Semicolons

Status: Adopted for Phase 15.3+; parser implementation is staged.

Policy
- Newline as primary statement separator.
- Semicolons are optional and only needed when multiple statements appear on one physical line.
- Minimal ASI (auto semicolon insertion) rules to avoid surprises.

Rules (minimal and predictable)
- Newline ends a statement when:
  - Parenthesis/brace/bracket depth is 0, and
  - The line does not end with a continuation token (`+ - * / . ,` etc.).
- Newline does NOT end a statement when:
  - Inside any open grouping `(...)`, `[...]`, `{...}`; or
  - The previous token is a continuation token.
- `return/break/continue` end the statement at newline unless the value is on the same line or grouped via parentheses.
- `if/else` (and similar paired constructs): do not insert a semicolon between a block and a following `else`.
- One‑line multi‑statements are allowed with semicolons: `x = 1; y = 2; print(y)`.
- Method chains can break across lines after a dot: `obj\n  .method()` (newline treated as whitespace).

Style guidance
- Prefer newline separation (no semicolons) for readability.
- Use semicolons only when placing multiple statements on a single line.

Examples
```nyash
// Preferred (no semicolons)
local x = 5
x = x + 1
print(x)

// One line with multiple statements (use semicolons)
local x = 5; x = x + 1; print(x)

// Line continuation by operator
local n = 1 +
          2 +
          3

// Grouping across lines
return (
  1 + 2 + 3
)

// if / else on separate lines without inserting a semicolon
if cond {
  x = x - 1
}
else {
  print(x)
}

// Dot chain across lines
local v = obj
  .methodA()
  .methodB(42)

// Grouped assignment as expression (Stage‑3)
local y = (x = x + 1)    # (x = x + 1) が 1 を返す式として扱われる
if (x = next()) != null {
  print(x)
}
```

Notes（locals）
- `local x` は `local x = null`（= `Void`）の糖衣。初期化が不要でも binding を作る用途に使う。
- `null`/`Void` に対するメソッド呼び出し・フィールドアクセスは **TypeError（fail-fast）**（SSOT: `docs/reference/language/types.md`）。

Implementation notes (parser)
- Tokenizer keeps track of grouping depth.
- At newline, attempt ASI only when depth==0 and previous token is not a continuation.
- Error messages should suggest adding a continuation token or grouping when a newline unexpectedly ends a statement.

Parser dev notes (Stage‑1/2)
- return + newline: treat bare `return` as statement end. To return an expression on the next line, require grouping with parentheses.
- if/else: never insert a semicolon between a closed block and `else` (ASI禁止箇所)。
- Dot chains: treat `.` followed by newline as whitespace (line continuation)。
- One‑line multi‑statements: accept `;` as statement separator, but formatter should prefer newlines.
- Unary minus: disambiguate from binary minus; implement after Stage‑1（当面は括弧で回避）。

## Assignment Expressions（Stage‑3 追加仕様）

Stage‑3 では、制御構造や短絡評価との相性を良くするために「**括弧付き代入を式として扱う**」最小拡張を導入するよ。

Rules
- `x = expr` は従来通り **代入文（statement）** として扱う。
- `'(x = expr)'` のように **括弧で囲まれた代入** だけを、値を返す式（expression）として扱う。
  - 値と型は右辺 `expr` と同じになる（`(x = 1)` の値は `1`）。
- この拡張は Stage‑3 パーサーのみで有効（Rust: `NYASH_FEATURES=stage3` / selfhost: `--stage3`/`NYASH_NY_COMPILER_STAGE3=1`）。

Examples
```nyash
# 依然として「文」
x = x + 1

# こちらは「値を返す式」
local y = (x = x + 1)   # y と x の両方が 1 増える

if (x = next()) != null {
  print("got: " + x)
}

return (counter = counter + 1)
```

Notes（実装指針）
- EBNF 上は `assignment_expr := IDENT '=' expr` を定義し、`factor` に ` '(' assignment_expr ')'` を追加する形で表現する。
- lowering では「代入命令 + その結果値を表す SSA 値」を 1 セットで生成し、その ValueId を式の値として返す。
- 仕様を広げすぎないため、当面は「括弧付き代入のみ式扱い」とし、裸の `x = expr` を expression 文に自動昇格するような拡張は行わない。
