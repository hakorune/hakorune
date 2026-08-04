# Ny JSON IR v0 — Minimal Spec (Stage‑2)

Status: experimental but stable for Phase‑15 Stage‑2. Input to `--ny-parser-pipe`.

Version and root
- `version`: 0
- `kind`: "Program"
- `body`: array of statements

Statements (`StmtV0`)
- `Return { expr }`
- `Extern { iface, method, args[] }` (optional; passes through to `ExternCall`)
- `Expr { expr }` (expression statement; side effects only)
- `Local { name, expr }` (Stage‑2)
- `If { cond, then: Stmt[], else?: Stmt[] }` (Stage‑2)
- `Loop { cond, body: Stmt[] }` (Stage‑2; while(cond) body)
- `Break` (Stage‑3; exits current loop)
- `Continue` (Stage‑3; jumps to loop head)
- `Try { try: Stmt[], catches?: Catch[], finally?: Stmt[] }` (Stage‑3 skeleton; surface syntax uses `cleanup`, but the v0 field name remains `finally` for compatibility; currently lowered as sequential `try` body only when runtime support is absent)

Expressions (`ExprV0`)
- `Int { value }` where `value` is JSON number or digit string
- `Str { value: string }`
- `Bool { value: bool }`
- `Binary { op: "+"|"-"|"*"|"/", lhs, rhs }`
- `Compare { op: "=="|"!="|"<"|"<="|">"|">=", lhs, rhs }`
- `Logical { op: "&&"|"||", lhs, rhs }` (short‑circuit)
- `Call { name: string, args[] }` (function by name)
- `Method { recv: Expr, method: string, args[] }` (box method)
- `New { class: string, args[] }` (construct Box)
- `Var { name: string }`
- `Throw { expr }` (Stage‑3; currently degrades to expression statement when runtime semantics are disabled)

CFG conventions (lowered by the bridge)
- If: create `then_bb`, `else_bb`, `merge_bb`. Both branches jump to merge if unterminated.
- Loop: `preheader -> cond_bb -> (body_bb or exit_bb)`, body jumps back to cond.
- Short‑circuit Logical: create `rhs_bb`, `fall_bb`, `merge_bb` with constants on fall path.
- All blocks end with a terminator (branch/jump/return).

PHI merging（legacy JSON-v0 bridge）
- These bullets describe the experimental JSON-v0 bridge and its historical
  MIR13/edge-copy compatibility behavior. They are not the authority for the
  resolved canonical Recipe route.
- The resolved canonical nested-If profile emits PHIs in the existing MIR
  canonical lowerer: one outer and one inner explicit-else merge, with the
  inner merge predecessor/value feeding the outer then edge. No second
  JSON/Recipe PHI owner is implied.
- ループは既存 CFG（preheader→cond→{body|exit}; body→cond）の検出により、ヘッダ BB で搬送値の PHI を構築。
- 将来（LoopForm= MIR18）では LoopForm 占位命令から逆 Lowering で PHI を自動化予定。
 - PHI‑off 運用（Builder 側の規約）: merge 内に copy を置かず、then/else の pred へ edge_copy のみを挿入（self‑copy は No‑Op）。use‑before‑def と重複 copy を原理的に回避する。

Type meta (emitter/LLVM harness cooperation)
- `+` with any string operand → string concat path（handle固定）。
- `==/!=` with both strings → string compare path。

Special notes
- `Var("me")`: Bridge 既定では未定義エラー。デバッグ用に `NYASH_BRIDGE_ME_DUMMY=1` でダミー `NewBox{class}` を注入可（`NYASH_BRIDGE_ME_CLASS` 省略時は `Main`）。
- `--ny-parser-pipe` は stdin の JSON v0 を受け取り、MIR→MIR‑Interp 経由で実行する。

Box members
- Accepted target: stored/weak fields become slots and behavior remains an
  ordinary method. Program JSON v0 gains no Property node.
- Current production compatibility still lowers computed/once/birth_once
  source forms into synthetic methods/hidden state. That route is scheduled for
  atomic retirement by
  `BOX-MEMBER-PROPERTY-RETIRE0-I0-R0-G0`; it is not a permanent JSON contract.
- After cutover, `__get_*` is an ordinary method spelling and must not recover
  Property meaning. Stale artifacts that require magic-name recovery reject.
- Stored initializer expressions remain construction-path work in declaration
  order. Current method postfix `catch/cleanup` transport is migration
  evidence only; C′ rejects the handler surface and replaces standalone
  cleanup with one verified exit owner without widening Program JSON v0 here.
- Design/task order:
  [box-member-field-method-surface-ssot.md](../../development/current/main/design/box-member-field-method-surface-ssot.md).

CLI/Env cheatsheet
- Pipe: `echo '{...}' | target/release/nyash --ny-parser-pipe`
- File (compat intake): `target/release/nyash --json-file sample.json`
- Verbose MIR dump: `NYASH_CLI_VERBOSE=1`
- me dummy: `NYASH_BRIDGE_ME_DUMMY=1 NYASH_BRIDGE_ME_CLASS=ConsoleBox`

Examples

Arithmetic
```json
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{
    "type":"Binary","op":"+",
    "lhs":{"type":"Int","value":1},
    "rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}
  }}
]}
```

If with local + PHI merge
```json
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"x","expr":{"type":"Int","value":1}},
  {"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}},
    "then":[{"type":"Local","name":"x","expr":{"type":"Int","value":10}}],
    "else":[{"type":"Local","name":"x","expr":{"type":"Int","value":20}}]
  },
  {"type":"Return","expr":{"type":"Var","name":"x"}}
]}
```
- `Break` / `Continue` are emitted when Stage‑3 gate is enabled. When the bridge is compiled without Stage‑3 lowering, frontends may degrade them into `Expr(Int(0))` as a safety fallback.
- `Try` nodes include optional `catches` entries of the form `{ param?: string, typeHint?: string, body: Stmt[] }`. Until runtime exception semantics land, downstream lowers only the `try` body and ignores handlers/`finally`.
