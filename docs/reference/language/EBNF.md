# Nyash Grammar (Stage‑2 EBNF)

Status: Fixed for Phase 15 Stage‑2. Parser implementations (Rust/Python/Nyash selfhost) should conform to this subset.

Design SSOT note (Scope Exit Semantics):
- `throw` is prohibited in surface language design.
- parser は `throw` を常時 reject する（`[freeze:contract][parser/throw_reserved]`）。
- DropScope surface (`fini {}` / `local ... fini {}`) is part of Stage‑3 parser syntax.
- Rune declaration metadata is active on both Rust and `.hako` parsers; canonical syntax is `@rune`, optimization families (`Hint` / `Contract` / `Lowering` / `Profile` / `IntrinsicCandidate`) are part of the same metadata lane, and legacy `@hint` / `@contract` / `@intrinsic_candidate` remain compat aliases. Program(JSON v0) is not widened for Rune metadata.
- SSOT:
  - `docs/development/current/main/design/rune-v0-contract-rollout-ssot.md`
  - `docs/development/current/main/design/rune-v1-metadata-unification-ssot.md`

program   := (static_const_table_decl | brand_decl | record_decl | stmt)* EOF

; M11b static const table syntax.
; Reads use the existing postfix index expression.
static_const_table_decl :=
             'static' 'const' IDENT ':' 'u16' '[' ']' '=' '[' const_int_list? ']'
const_int_list := const_int_expr (',' const_int_expr)* ','?
const_int_expr := INT
                | '-' const_int_expr
                | '(' const_int_expr ')'
                | const_int_expr ('+'|'-'|'*'|'/'|'%'|'<<'|'>>'|'&'|'|'|'^') const_int_expr

; BRAND-001 Stage0 capsule.
; `brand` is metadata transport only here. Distinct type checking,
; constructor/unwrap policy, and verifier facts are Stage1-owned.
brand_decl := 'brand' IDENT ':' TYPE_REF
           ; BRAND-002 Stage1 semantics use existing call syntax:
           ;   IDENT '(' expr ')'          ; explicit brand constructor when IDENT is a declared brand
           ;   IDENT '.unwrap' '(' expr ')' ; explicit brand unwrap when IDENT is a declared brand

stmt      := 'return' expr
           | local_stmt
           | fini_stmt
           | assign_stmt
           | guard_stmt
           | 'if' expr block ('else' block)?
           | loop_stmt
           | expr                         ; expression statement

loop_stmt := 'loop' loop_head? block
loop_head := loop_range_head | loop_condition_head
loop_condition_head := '('? expr ')'?
loop_range_head := IDENT 'in' expr '..' expr

local_stmt := 'local' IDENT local_tail
local_tail := '=' expr local_fini_opt
           | (',' IDENT)+
           | local_fini_opt
local_fini_opt := ('fini' block)?
fini_stmt  := 'fini' block

guard_stmt := 'guard' expr 'else' block
           ; C200: guard else is default early-exit sugar.
           ; It lowers to `if !(expr) block`.

assign_stmt := assign_target '=' expr
             | assign_target compound_assign_op expr
assign_target := assign_primary assign_tail*
assign_primary:= IDENT | 'me'
assign_tail   := '.' IDENT
               | '[' expr ']'
compound_assign_op := '+=' | '-=' | '*=' | '/='
                  ; C199: compound assignment is default surface sugar.
                  ; It lowers to ordinary assignment with the corresponding
                  ; binary operation. Plain assignment remains canonical.

; Semantic constraints:
; - local declarations with '=' are single-binding only (`local x = expr`).
; - `local ... fini` applies only to single-binding form (grammar-level).

block     := '{' stmt* '}'

record_decl := 'record' IDENT type_params? '{' record_field+ '}'
record_field:= IDENT ':' TYPE_REF ','?
           ; C202: record is the explicit identity-free aggregate surface.
           ; MVP fields must be typed and non-weak.

expr      := logic
logic     := compare (('&&' | '||') compare)*
compare   := sum (( '==' | '!=' | '<' | '>' | '<=' | '>=' ) sum)?
sum       := term (('+' | '-') term)*
term      := unary (('*' | '/') unary)*
unary     := ( '-' | '!' | 'not' | '~' ) unary
           | weak_unary
           | factor

; Phase 285W-Syntax-0.1: `weak(<expr>)` is invalid. The operand must not be a grouped
; expression starting with `(`. (Write `weak x`, not `weak(x)`.)
weak_unary := 'weak' unary_no_group
unary_no_group := ( '-' | '!' | 'not' | '~' ) unary_no_group
                | INT
                | FLOAT
                | STRING
                | 'true'
                | 'false'
                | 'null'
                | 'void'
                | IDENT call_tail*
                | 'new' IDENT '(' args? ')'
                | '[' args? ']'           ; Array literal (Stage‑1 sugar, gated)
                | '%{' map_entries? '}'   ; Map literal (Stage‑2 sugar, gated)
                | match_expr              ; Pattern matching (replaces legacy peek)

factor    := INT
           | FLOAT
           | STRING
           | 'true'
           | 'false'
           | 'null'
           | 'void'
           | IDENT call_tail*
           | check_expr
           | '(' expr ')'
           | '(' assignment_expr ')'  ; Stage‑3: grouped assignment as expression
           | 'new' IDENT '(' args? ')'
           | '[' args? ']'           ; Array literal (Stage‑1 sugar, gated)
           | '%{' map_entries? '}'   ; Map literal (Stage‑2 sugar, gated)
           | match_expr              ; Pattern matching (replaces legacy peek)

check_expr := 'check' STRING? '{' check_item* '}'
check_item := STRING ':' expr
            | expr

match_expr := 'match' expr '{' match_arm+ default_arm? '}'
match_arm  := pattern guard? '=>' (expr | block) ','?
default_arm:= '_' '=>' (expr | block) ','?

pattern   := '_'
           | STRING | INT | FLOAT | 'true' | 'false' | 'null' | 'void'
            | IDENT '(' IDENT? ')'           ; Type pattern or known-enum single-payload shorthand
            | IDENT                          ; Known-enum unit shorthand, e.g. None
            | IDENT '{' IDENT (',' IDENT)* '}' ; Known-enum record shorthand, e.g. Ident { name }
            | '[' (IDENT (',' '..' IDENT)? )? ']'
            | '{' ( (STRING|IDENT) ':' IDENT (',' '..')? )? '}'
            | pattern '|' pattern            ; OR pattern (same arm)

guard     := 'if' expr

map_entries := STRING '=>' expr (',' STRING '=>' expr)* [',']

call_tail := '.' IDENT '(' args? ')'   ; method
           | '(' args? ')'             ; function call

args      := expr (',' expr)*

; Stage‑3: grouped assignment expression
; `(x = expr)` だけを式として認める。値と型は右辺 expr と同じ。
assignment_expr := IDENT '=' expr

Notes
- ASI: Newline is the primary statement separator. Do not insert a semicolon between a closed block and a following 'else'.
- Semicolon (optional): When `NYASH_PARSER_ALLOW_SEMICOLON=1` is set, `;` is accepted as an additional statement separator (equivalent to newline). It is not allowed between `}` and a following `else`.
- Do‑while: not supported by design. Prefer a single‑entry, pre‑condition loop normalized via sugar (e.g., `repeat N {}` / `until cond {}`) to a `loop` with clear break conditions.
- Short-circuit: '&&' and '||' must not evaluate the RHS when not needed.
- Proof checks: `check "name" { "label": expr }` is an eager proof-list
  expression. It must not be treated as an alias for short-circuit '&&' / '||'.
- Unary minus has higher precedence than '*' and '/'.
- IDENT names consist of [A-Za-z_][A-Za-z0-9_]*
- Array literal is enabled when syntax sugar is on (NYASH_SYNTAX_SUGAR_LEVEL=basic|full) or when NYASH_ENABLE_ARRAY_LITERAL=1 is set.
- Map literal is enabled when syntax sugar is on (NYASH_SYNTAX_SUGAR_LEVEL=basic|full) or when NYASH_ENABLE_MAP_LITERAL=1 is set.
- Identifier keys in map literals are out of v1 scope (string keys only): use `%{"name" => v}`.
- Pattern matching: `match` replaces legacy `peek`. MVP supports wildcard `_`, literals, simple type patterns, fixed/variadic array heads `[hd, ..tl]`, simple map key extract `{ "k": v, .. }`, OR patterns, and guards `if`.
- Known-enum shorthand: `Some(v)` / `None` is accepted only when the arm set resolves to a known enum declaration in the current source inventory.
- Known-enum exhaustiveness: shorthand enum matches must name every variant explicitly; `_` does not satisfy exhaustiveness for that lane.
- `Option<T>` does not add new grammar in the first cut. It uses the existing enum declaration, qualified constructor, and known-enum match surface. Future `some` / `none` / `if some` sugar is reserved and must be implemented in both Rust and `.hako` parsers together.
- Static const tables: `static const NAME: u16[] = [...]` and `NAME[index]` reads are accepted for the narrow M11b row. Initializer elements may use side-effect-free integer const expressions; const fn is still reserved.

### C197 Logical Condition Surface

Decision: accepted.

Ordinary `&&` / `||` chains are the source-level surface for short-circuit
boolean control flow. Parenthesized multiline conditions are accepted for normal
`if` / `loop` / expression use, including leading logical operators on
continuation lines:

```hako
if (
    ready == 1
    && count < limit
    || force == 1
) {
    return 0
}
```

The RHS of `&&` / `||` keeps the short-circuit contract. This row does not add
proof-list behavior, variadic `all(...)`, or allocator-specific condition
syntax.

### C198 Check Block Surface

Decision: accepted.

`check "name" { "label": expr }` is the source-level surface for labeled proof
lists. It evaluates every item left-to-right, even after an earlier item fails,
and returns a scalar pass/fail value:

```hako
local ok = check "release seam" {
    "first fact": released == 1
    "second fact": ((observed = observed + 1) == 1)
}
```

The v0 result is an integer lane value: `1` when all items are truthy, `0`
otherwise. Labels are source-level proof metadata in this row; they are kept
for readable source and future diagnostics, but C198 does not add automatic
printing or a proof-report object.

Stop line:
`check` is not a macro, not variadic `all(...)`, not a short-circuit operator,
not an allocator-specific DSL, and not a backend route selector. Unsupported
backend behavior must fail explicitly rather than silently treating a VM-only
route as complete.

### C199 Compound Assignment Surface

Decision: accepted.

`+=`, `-=`, `*=`, and `/=` are accepted for ordinary assign targets:

```hako
x += 1
me.count += delta
array[0] += 2
```

They are pure surface sugar for the existing assignment form:

```hako
target += rhs
```

lowers as if the source had been written:

```hako
target = target + rhs
```

with the corresponding binary operator for `-=`, `*=`, and `/=`.

Stop line:
C199 does not add a new overflow policy, allocator-specific meaning, hidden
atomic read-modify-write behavior, or special backend route. The canonical AST
shape remains `Assignment { value: BinaryOp { ... } }`.

### C200 Guard Else Surface

Decision: accepted.

`guard expr else { ... }` is accepted as early-exit source sugar:

```hako
guard handle.isValid() else {
    return 0
}
```

It lowers as if the source had been written:

```hako
if !(handle.isValid()) {
    return 0
}
```

C200 does not add a new AST control-flow node, exception behavior, fallback
semantics, or backend route. The canonical AST shape remains an `If` whose
condition is `UnaryOp::Not` over the guard condition.

### C202 Record Surface And Semantics

Decision: accepted.

`record` is the source-level surface for identity-free aggregate values:

```hako
record HakoAllocAlignedSmallMeta {
    ptr: i64
    alignment: i64
    requested_size: i64
    usable_size: i64
}
```

The C202 MVP accepts only fixed typed fields. It rejects weak fields,
initializers, methods, `fini`, inheritance, and interface implementation in
record declarations.

Stop line:
C202 does not add local scalar replacement, packed `ArrayBox` storage, blanket
ordinary-box flattening, reflection semantics, or allocator-specific syntax.
Ordinary `box` declarations keep identity-capable object semantics.

## Box Members (Phase‑15, env gate: NYASH_ENABLE_UNIFIED_MEMBERS; default ON)

This section adds a minimal grammar for Box members (a unified member model) without changing JSON v0/MIR. Parsing is controlled by env `NYASH_ENABLE_UNIFIED_MEMBERS` (default ON; set `0/false/off` to disable).

```
box_decl       := 'box' IDENT '{' member* '}'

member         := visibility_block
                | weak_stored
                | stored
                | computed
                | once_decl
                | birth_once_decl
                | delegate_decl
                | method_decl
                | block_as_role      ; nyash-mode (block-first) equivalent

visibility_block := ( 'public' | 'private' ) '{' member* '}'
                  ; member visibility grouping (Phase 285A1.3). `weak` is allowed inside.

weak_stored    := 'weak' IDENT ( ':' TYPE )?
                  ; weak field declaration (Phase 285A1.2). Enforces WeakRef type at compile-time.

visibility_weak_sugar := ('public'|'private') 'weak' IDENT ( ':' TYPE )?
                  ; sugar syntax (Phase 285A1.4). Equivalent to visibility block form.
                  ; e.g., `public weak parent` ≡ `public { weak parent }`

stored         := IDENT ( '=' expr )?
                | IDENT ':' TYPE ( '=' expr )?
                  ; stored property (read/write). `IDENT` alone is the simple
                  ; untyped stored field form. `IDENT ':' TYPE` carries
                  ; declared-type metadata for tooling / typed-object planning;
                  ; it is not a general runtime type check.
                  ; `= expr` emits a construction prologue assignment before
                  ; the user `birth` body.

delegate_decl  := 'delegate' IDENT 'exposes' '{' delegate_expose+ '}'
delegate_expose:= IDENT ( 'as' IDENT )? ','?
                  ; DEL-002 Stage0 capsule. Carries explicit method exposure
                  ; metadata only. No forwarding/collision/interface semantics.

computed       := get_computed | legacy_computed

get_computed  := 'get' IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
                  ; canonical computed property syntax. `get` is contextual
                  ; only in Box member head position.

legacy_computed:= IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
                  ; compatibility shorthand for computed properties. Accepted,
                  ; but canonical docs should prefer `get IDENT`.

once_decl      := 'once' IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
                  ; lazy once. First read computes and caches; later reads return cached value.

birth_once_decl:= 'birth_once' IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
                  ; eager once. Computed during construction (before user birth), in declaration order.

method_decl    := IDENT '(' params? ')' ( ':' TYPE )? block handler_tail?

params         := param (',' param)*
param          := IDENT (':' TYPE_REF)?
TYPE_REF       := IDENT ('.' IDENT)* ('<' TYPE_REF (',' TYPE_REF)* '>')? ('[' ']')*
                  ; parameter list (Phase 285A1.5+)
                  ; Type annotations are preserved as AST metadata.
                  ; `params` remains the canonical names-only compatibility surface.
                  ; Numeric substrate names such as i64/u64/usize are IDENT
                  ; names here. Literal suffix grammar is not live.

; nyash-mode (block-first) variant — gated with NYASH_ENABLE_UNIFIED_MEMBERS=1
block_as_role  := block 'as' ( 'once' | 'birth_once' )? IDENT ':' TYPE

handler_tail   := ( catch_block )? ( cleanup_block )?
catch_block    := 'catch' ( '(' ( IDENT IDENT | IDENT )? ')' )? block
cleanup_block  := 'cleanup' block

; Stage‑3 (Phase 1 via normalization gate NYASH_CATCH_NEW=1)
; Postfix handlers for expressions and calls (cleanup may appear without catch)
postfix_catch      := primary_expr 'catch' ( '(' ( IDENT IDENT | IDENT )? ')' )? block
postfix_cleanup    := primary_expr 'cleanup' block
```

Semantics (summary)
- stored: O(1) slot read; write via assignment. Bare stored fields are dynamic/untyped. Typed stored fields keep declared-type metadata for optimizers/verifiers and typed-object planning, but ordinary field writes are not type-enforced by this syntax.
- stored initializers: `name = expr` and `name: Type = expr` are accepted and lower to constructor prologue assignments equivalent to `me.name = expr`. The prologue runs before the user `birth` body, in field declaration order. Initializer expressions are evaluated for each construction, so `field: ArrayBox = new ArrayBox()` creates a per-instance value rather than a shared static default.
- computed/get: read‑only; each read evaluates the block; assignment is an error unless a setter is explicitly defined.
- once: first read evaluates the block and caches the value; subsequent reads return the cached value. On exception without a `catch`, the property becomes poisoned and rethrows on later reads (no retries).
- birth_once: evaluated before the user `birth` body, in declaration order; exceptions without a `catch` abort construction; cycles between `birth_once` members are an error.
- handlers: `catch/cleanup` are permitted for computed/once/birth_once/method blocks (Stage‑3), not for stored.

Lowering (no JSON v0 change)
- stored → slot; declared type, when present, is copied into field-declaration metadata
- computed/get → synthesize `__get_name():T { try body; catch; finally }`; reads of `obj.name` become `obj.__get_name()`
- once → add `__name: Option<T>` and emit `__get_name()` with first‑read initialization; on uncaught exception mark poisoned and rethrow on subsequent reads
- birth_once → add `__name: T` and insert initialization just before user `birth` in declaration order; handlers apply to each initializer
- method → existing method forms; optional postfix handlers lower to try/catch/finally

## Legacy: `init { ... }` field list (compatibility)

Some docs and older code use an `init { a, b, c }` list inside a `box` body. This is a legacy compatibility form to declare stored slots.

Semantics (SSOT):
- `init { a, b, c }` declares **untyped stored slots** named `a`, `b`, `c` (equivalent to writing `a` / `b` / `c` as stored members without type).
- `init { weak x, weak y }` declares **weak fields** (equivalent to writing `weak x` / `weak y` as members).
- It does not execute code. Initialization logic belongs in `birth(...) { ... }` and assignments.
- **New code** should prefer the direct syntax: `field_name` for simple dynamic slots, `field_name: Type` for declared-type metadata, `weak field_name` for weak fields, or the rest of the unified member model (`get`/`once`/`birth_once`).
- Legacy `init { weak field }` syntax still works for backward compatibility but is superseded by `weak field`.

## Enum Declarations (Phase-163x parser surface)

```ebnf
enum_decl        := 'enum' IDENT ('<' IDENT (',' IDENT)* '>')? '{' enum_variant* '}'
enum_variant     := IDENT
                  | IDENT '(' TYPE_REF ')'
                  | IDENT '{' enum_record_field (',' enum_record_field)* '}'
enum_record_field:= IDENT ':' TYPE_REF
qualified_ctor   := IDENT '::' IDENT '(' args? ')'
                  | IDENT '::' IDENT '{' enum_record_init (',' enum_record_init)* '}'
enum_record_init := IDENT ':' expr
```

Notes:
- current executable surface includes unit variants, single-payload tuple variants, and a narrow named-record variant cut
- known-enum shorthand includes `Some(v)` / `None` and narrow record patterns like `Ident { name }`
- record constructors / patterns must mention the declared field set exactly
- multi-payload variants and block-bodied record shorthand arms are not part of this cut yet
- `qualified_ctor` is the narrow constructor surface used by enum values; this does not imply a general `::` static-method migration

## Stage‑3 (Gated) Additions

Enabled when `NYASH_PARSER_STAGE3=1` for the Rust parser (and via `--stage3`/`NYASH_NY_COMPILER_STAGE3=1` for the selfhost parser):

- Legacy (compat): `try` statement (deprecated)
  - Surface SSOT is postfix `catch/cleanup` + DropScope `fini`. `try_stmt` is legacy only; avoid in new code.
  - Some parser paths may still accept `try_stmt` for compatibility.
  - To enforce no-`try` surface in parser, use `NYASH_FEATURES=no-try-compat` (fail-fast tag: `[freeze:contract][parser/try_reserved]`).
  - Legacy syntax (if accepted): `try_stmt := 'try' block ('catch' '(' (IDENT IDENT | IDENT | ε) ')' block)? ('cleanup' block)?`

- Block‑postfix catch/cleanup（Phase 15.5）
  - `block_catch := '{' stmt* '}' ('catch' '(' (IDENT IDENT | IDENT | ε) ')' block)? ('cleanup' block)?`
  - Applies to standalone block statements. Do not attach to `if/else/loop` structural blocks (wrap with a standalone block when needed).
  - Gate: `NYASH_BLOCK_CATCH=1` (or `NYASH_PARSER_STAGE3=1`).
- throw（design target）
  - prohibited in surface language SSOT and parser rejects it unconditionally.
  - freeze tag: `[freeze:contract][parser/throw_reserved]`

- Method‑level postfix catch/cleanup（Phase 15.6, gated）
  - `method_decl := IDENT '(' params? ')' ( ':' TYPE )? block ('catch' '(' (IDENT IDENT | IDENT | ε) ')' block)? ('cleanup' block)?`
  - Gate: `NYASH_METHOD_CATCH=1`（または `NYASH_PARSER_STAGE3=1` と同梱）

- DropScope / fini（Stage‑3）
  - `fini_stmt := 'fini' block`
  - `local_fini_stmt := 'local' IDENT ( '=' expr )? 'fini' block`
  - `local_fini_stmt` は単一束縛のみ（`,` 併用禁止）。
  - `fini` ブロック内の `return` / `break` / `continue` / `throw` は禁止（Fail-Fast）。
  - 同一スコープの複数 `fini` は scope exit で LIFO 実行。

- Member‑level postfix catch/cleanup（Phase 15.6, gated）
  - Applies to computed/once/birth_once in the unified member model: see “Box Members”.
  - Gate: `NYASH_PARSER_STAGE3=1` (shared). Stored members do not accept handlers.

These constructs remain experimental; behaviour may degrade to no‑op in some backends until runtime support lands, as tracked in CURRENT_TASK.md.

## Rune Declaration Metadata (docs-locked)

The following fragment is docs-locked only. It does not mean current default grammar accepts metadata without the parser gate.

```
metadata_attr      := rune_attr | legacy_opt_attr
rune_attr          := '@' 'rune' IDENT rune_arg_list?
legacy_opt_attr    := '@' ('hint' | 'contract' | 'intrinsic_candidate') '(' rune_arg (',' rune_arg)* ')'
rune_arg_list      := '(' rune_arg (',' rune_arg)* ')'
rune_arg           := STRING | rune_ident
rune_ident         := IDENT ('.' IDENT)*

; abstract target set for v0
; concrete declaration grammar remains owned by the relevant parser lane
metadata_target    := box_decl
                    | method_decl
                    | function_decl
                    | extern_decl
```

Notes
- canonical docs surface is `@rune`.
- dotted rune identifiers such as `allocator.fast` are accepted as a single
  metadata argument; profile names are still expanded to primitive MIR facts and
  must not become backend-readable route selectors.
- declaration-leading legacy aliases normalize to declaration-local `attrs.runes`.
- declaration metadata is allowed only on declaration targets.
- active grammar requires Rust parser / `.hako` parser parity.
- Rune metadata is declaration-local on AST/direct MIR; do not widen Program(JSON v0).
- body-position legacy aliases remain compat/noop during the current migration window.
