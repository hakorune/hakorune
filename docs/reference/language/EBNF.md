# Hakorune Grammar Reference

Status: Living grammar/reference snapshot plus explicitly labeled
implementation evidence. The 2026-08-05 Result/exit and C′ lifecycle Decisions
supersede the exception/cleanup/fini target prose and productions still present
below. Those productions remain an unsynchronized migration inventory until
the implementation rows and mandatory EBNF/registry/parser reference closeout;
parser acceptance alone is not language authority. Practical bootstrap / phase-1 support status is tracked in
`docs/reference/language/stage-profiles.md`.

Selfhost tooling uses `--syntax-3` with compatibility alias `--stage3`.
These flags select an implementation surface; they do not independently define
canonical language semantics.

Design SSOT note (Scope Exit Semantics):
- `throw` is prohibited in surface language design.
- parser は `throw` を常時 reject する（`[freeze:contract][parser/throw_reserved]`）。
- source `try` is rejected in Canonical and Compat2025 language profiles.
- typed Result-only postfix `?` is the accepted unchanged-propagation target;
  its grammar/verified consumer remains production 0.
- source catch and `RecoverableFailure` are rejected targets.
- standalone `cleanup {}` is the sole lexical cleanup target; local/postfix
  cleanup and scope-position `fini` are retirement input.
- Box-member `fini {}` is the accepted non-callable terminal Home hook; direct
  `obj.fini()` is rejected. Its production remains 0.
- `release root` is the accepted explicit whole-root Home-end target. It is a
  statement-only contextual-keyword production with one identifier root and a
  dedicated source carrier; semantic authority remains in resolution/Home
  Flow, not parser or MIR name matching. `release(value)` stays an ordinary
  Call; bindings/callables named `release` and `Build.release` remain legal.
  `release` is not a globally reserved lexer token. `drop` and `unbox` have no
  Home alias production. The dedicated Rust/Hako syntax/source row is live;
  Home resolution and execution remain 0.
- The concrete productions below are not rewritten in this Decision-only
  slice. `LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0` and
  `OWN-HOME-REFERENCE-CLOSEOUT0-DOC0` must synchronize EBNF, registry, corpus,
  and both parsers after implementation.
- Rune declaration metadata is active on both Rust and `.hako` parsers; canonical syntax is `@rune`, optimization families (`Inline` / `Hint` / `Contract` / `Profile` / `IntrinsicCandidate`) are part of the same metadata lane, and legacy `@hint` / `@contract` / `@intrinsic_candidate` plus compat `Lowering(inline_required)` remain migration aliases. Program(JSON v0) is not widened for Rune metadata.
- `@rune CallableContract(query)` is the accepted non-repeatable whole-call
  query-contract target. It remains parser/resolver production 0 until the
  ordered Box-method inventory and Rust/`.hako` parity rows land. The generic
  `rune_attr` production below does not imply that its value is currently
  accepted. Types and arity remain in the method signature; physical ABI is
  not encoded in the rune value. See `callable-contracts.md`.
- SSOT:
  - `docs/development/current/main/design/rune-v0-contract-rollout-ssot.md`
  - `docs/development/current/main/design/rune-v1-metadata-unification-ssot.md`

Ownership grammar status (2026-08-04):

- `docs/reference/language/ownership.md` accepts the Home direction: ordinary
  use is a non-owning handle, a sealed destination demand transfers one Home,
  and only explicit `share` may add an independent owner.
- The bounded target surface accepts declaration-side contextual `take`,
  expression-side contextual `share` over one non-group postfix operand, and
  statement-side contextual `release root`. Contract result `from` remains a
  separate provisional row. Only the exact-root `release` parser/source row is
  live; `take` and `share` remain parser-inactive.
- `take`, `share`, and `release` remain `IDENT` spellings and are not global
  lexer keywords. Contextual recognition requires same-line lookahead.
  `share(...)` and `share (expr)` are permanently ordinary calls;
  `adopt(share expr)` is ordinary-call composition.
- The first release profile is whole-root owning local/parameter only.
  Generic/composite whole-root support under the same statement, fields,
  projections, containers, and unknown capability remain provisional or
  rejected; no parser acceptance may add a generic wrapper callable or widen
  them.
- Composite/generic classification, Shared representation, owning storage,
  callable boundaries, and CFG Home Flow must close before grammar activation.
- Therefore the live EBNF below contains only the bounded Release syntax row;
  the other accepted target grammar remains recorded in `OWN-HOME-SYNTAX-D0`.
  Current parsers must reject inactive ownership spellings and former
  `move/view/shared` lookalikes. Accidental parsing is not support.
- The parked order is
  `docs/development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md`.
  Support status is reported by `stage-profiles.md`.

Accepted ownership grammar and implementation status:

```ebnf
target_take_param    := IDENT("take") HTRIVIA IDENT HTRIVIA ':' type_ref
target_share_expr    := IDENT("share") HTRIVIA non_group_postfix_expr
release_stmt         := IDENT("release") HSPACE IDENT stmt_end  (* parser/source live; semantic 0 *)
explicit_extern_call := IDENT("externcall") HTRIVIA STRING HTRIVIA '(' argument_list? ')'
```

`HTRIVIA` excludes line terminators. Release I0 narrows `HSPACE` to spaces and
tabs; comment trivia is not part of that row. `take` is recognized only at parameter
head, `share` only at expression-prefix position when the next token starts a
non-group primary, and `release` only at statement head. See
`docs/development/current/main/investigations/own-home-syntax-d0-design-task-2026-08-09.md`
for FIRST/precedence, ordinary-call preservation, and parser parity
requirements.

Function-exit semantic status:

- `docs/reference/language/function-exit-and-entry-result.md` owns accepted
  function, Main, Script-result, and process-entry semantics.
- The live grammar below admits `return expr`; bare `return` has accepted
  target Unit semantics but remains grammar-inactive until a separate registry
  row and both parser witnesses land.

program   := (cfg_item | static_const_table_decl | brand_decl | type_alias_decl | record_decl | enum_decl | box_decl | function_decl | stmt)* EOF

cfg_item  := 'gate' build_predicate '{' program_item* '}' ('else' cfg_else)?
cfg_else  := cfg_item | '{' program_item* '}'
program_item := static_const_table_decl | brand_decl | type_alias_decl | record_decl | enum_decl | box_decl | function_decl | stmt

; Build conditional predicates are not ordinary runtime expressions.
; LANG-CFG-001 owns parser transport and prune-before-resolution semantics.
build_predicate :=
             'Build' '.' ('test' | 'debug' | 'release')
           | 'Feature' '(' STRING ')'
           | 'Target' '.' ('os' | 'arch') '==' IDENT
           | 'Backend' '.' 'kind' '==' IDENT
           | 'not' '(' build_predicate ')'
           | 'all' '(' build_predicate (',' build_predicate)* ')'
           | 'any' '(' build_predicate (',' build_predicate)* ')'

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

; TYPE-001 Stage0 capsule.
; Alias diagnostics and expansion facts are Stage1-owned.
type_alias_decl := 'type' IDENT '=' TYPE_REF

stmt      := 'return' expr
           | release_stmt
           | local_stmt
           | cleanup_stmt
           | assign_stmt
           | guard_stmt
           | gate_stmt
           | fastmem_stmt
           | 'if' expr block ('else' block)?
           | loop_stmt
           | expr                         ; expression statement

; Nested If syntax is recursive at the parser boundary. The selected
; resolved one-level nested-If recipe is a narrower production profile:
; one outer + one inner explicit-else pure fallthrough over one binding.
; This profile note does not make deeper/effectful shapes parser errors.

loop_stmt := 'loop' loop_head? block
loop_head := loop_range_head | loop_condition_head
loop_condition_head := '('? expr ')'?
loop_range_head := IDENT 'in' expr '..' expr

local_stmt := 'local' IDENT local_type_opt local_tail
local_type_opt := (':' TYPE_REF)?
local_tail := '=' expr local_cleanup_opt
           | (',' IDENT)+
           | local_cleanup_opt
local_cleanup_opt := ('cleanup' block)?
cleanup_stmt  := 'cleanup' block
           ; accepted target grammar; registry/parser rows remain pending.
           ; Compat2025 `fini` aliases normalize to these shapes later.

guard_stmt := 'guard' expr 'else' block
           | 'guard' 'let' qualified_variant_pattern '=' expr 'else' block
           ; C200: guard else is default early-exit sugar.
           ; It lowers to `if !(expr) block`.
           ; GUARDLET-001: guard-let is narrow enum variant sugar.
           ; MVP form: guard let Type::Variant(binding) = expr else block.
           ; It rewrites through existing Local / If / EnumMatchExpr pieces.

gate_stmt  := 'gate' build_predicate block
           ('else' ('gate' build_predicate block | block))?
           ; statement-level build selection inside block/method bodies.
           ; inactive branches are parsed but pruned before MIR/lowering.

fastmem_stmt := 'fastmem' IDENT block
           ; FastMemory contract region. The IDENT is a required contract id
           ; such as PageMapV0. Contract-less `fastmem { ... }` is rejected.
           ; The parser transports the region boundary; verifier/lowering rows
           ; own MemOp legality and backend support.

qualified_variant_pattern := IDENT '::' IDENT '(' IDENT ')'

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

function_decl := 'function' IDENT '(' params? ')' ( ':' TYPE_REF )? signature_clause* block
               ; return annotation is optional. `: void` is an explicit
               ; no-value contract; omission is an unannotated result contract,
               ; not implicit void or source-level result inference.

signature_clause := uses_clause | contract_clause

uses_clause := 'uses' IDENT (',' IDENT)*
                 ; USES-001 Stage0 capsule. Carries capability metadata only.
                 ; Capability policy and backend gates are Stage1-owned.

contract_clause := ('requires' | 'ensures') expr
                 ; CONTRACT-002 Stage0 capsule. Carries metadata only.
                 ; Runtime insertion, invariant checking, and verifier facts are Stage1-owned.

invariant_member := 'invariant' expr
                 ; CONTRACT-002 Stage0 capsule for box/record declaration metadata.

transition_member := 'transition' TYPE_REF '::' IDENT '-' '>' TYPE_REF '::' IDENT 'by' IDENT
                 ; TRANS-001 Stage0 capsule for box-local lifecycle relation metadata.
                 ; Legality checks, enum/method lookup, and verifier facts are Stage1-owned.

record_decl := 'record' IDENT type_params? '{' record_member+ '}'
record_member:= record_field | invariant_member
record_field:= IDENT ':' TYPE_REF ('=' record_default_expr)? ','?
           ; C202: record is the explicit identity-free aggregate surface.
           ; ARG-DATA-003: scalar literal defaults are accepted for local
           ; record construction defaults. They are not runtime stored fields.
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
                | new_expr
                | '[' args? ']'           ; Canonical Array literal; explicit Array<T> context adds an element contract
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
           | new_expr
           | record_literal
           | record_update
           | '[' args? ']'           ; Canonical Array literal; explicit Array<T> context adds an element contract
           | '%{' map_entries? '}'   ; Map literal (Stage‑2 sugar, gated)
           | match_expr              ; Pattern matching (replaces legacy peek)

record_literal := IDENT '{' record_literal_field? ((',' | NEWLINE) record_literal_field)* ','? '}'
record_literal_field := IDENT (':' expr)?
              ; REC-001/ARG-DATA-003: `field` is shorthand for `field: field`.
              ; Omitted fields use record declaration defaults when present.
              ; Missing non-defaulted fields and extra fields fail-fast.
              ; Constructor IDENT is resolved in the type namespace.
              ; Value identifiers inside shorthand are resolved in the value namespace.

record_update := expr 'with' '{' record_update_field (',' record_update_field)* ','? '}'
record_update_field := IDENT (':' expr)?
              ; REC-003: `with` is contextual in expression-postfix position.
              ; It is identity-free record replacement, not mutation.
              ; The base expression must lower to a tracked local record value.
              ; Ordinary boxes do not support `with`.

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
- Do-while, repeat, until, while, and for are not canonical surface forms.
  Use `loop cond { ... }`, `loop i in start..end { ... }`, or `loop { ... }`.
- Short-circuit: '&&' and '||' must not evaluate the RHS when not needed.
- Proof checks: `check "name" { "label": expr }` is an eager proof-list
  expression. It must not be treated as an alias for short-circuit '&&' / '||'.
- Unary minus has higher precedence than '*' and '/'.
- IDENT names consist of [A-Za-z_][A-Za-z0-9_]*
- Array literal syntax is canonical. An explicit supported `Array<T>` context
  creates a Typed Array element contract; without one the value is an ordinary
  `AnyDefault` Array. Homogeneous element inference is representation evidence
  only and never creates a semantic contract.
- Map literal is enabled when syntax sugar is on (NYASH_SYNTAX_SUGAR_LEVEL=basic|full) or when NYASH_ENABLE_MAP_LITERAL=1 is set.
- Identifier keys in map literals are out of v1 scope (string keys only): use `%{"name" => v}`.
- Pattern matching: `match` replaces legacy `peek`. MVP supports wildcard `_`, literals, simple type patterns, fixed/variadic array heads `[hd, ..tl]`, simple map key extract `{ "k": v, .. }`, OR patterns, and guards `if`.
- Known-enum shorthand: `Some(v)` / `None` is accepted only when the arm set resolves to a known enum declaration in the current source inventory.
- Known-enum exhaustiveness: shorthand enum matches must name every variant explicitly; `_` does not satisfy exhaustiveness for that lane.
  RESULT-002C tags known-enum `_` exhaustiveness diagnostics.
- `Option<T>` and `Result<T,E>` are built-in enum prelude surfaces in
  RESULT-001. They use qualified constructors such as `Option::None` and
  `Result::Ok(value)`. Dot variants are rejected for known enum variants.
  RESULT-002A adds tagged prelude missing-arm diagnostics.
  RESULT-002B adds tagged prelude payload arity diagnostics.
  RESULT-002D adds tagged prelude expected-type diagnostics.
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

The C202 MVP accepts only fixed typed fields. ARG-DATA-003 additionally accepts
scalar literal field defaults for local record construction ergonomics. Record
declarations still reject weak fields, methods, `fini`, inheritance, and
interface implementation.

Explicit record literals construct identity-free record values:

```hako
local meta = HakoAllocAlignedSmallMeta {
    ptr: ptr_id
    alignment: 16
    requested_size: requested
    usable_size: usable
}
```

Record literals may omit fields that have declaration defaults:

```hako
record ReportFields {
    accepted: i64 = 0
    reason: i64 = 0
}

local fields = ReportFields {}
local rejected = ReportFields { reason: 2 }
```

`RecordName { field }` is shorthand for `RecordName { field: field }`. Missing
non-defaulted fields and extra fields are Stage1 errors. Lowered Program JSON
v0 carries declared field index/type metadata on construction fields, and
tracked local record reads lower as `RecordField` rather than ordinary box field
access.

Current executable record use is intentionally narrow. A local record value may
act as a compiler-local value carrier for:

```hako
local fields = SomeReportFields {
    reason: reason,
    count: count
}

local reason = fields.reason
return me.makeReport(fields)
```

For same-owner helper calls, the helper parameter must declare the exact record
type. The compiler may scalarize helper body field reads from that parameter
without materializing a runtime record object:

```hako
makeReport(fields: SomeReportFields): SomeReport {
    local result = new SomeReport()
    result.reason = fields.reason
    result.count = fields.count
    return result
}
```

This is a compiler-local scalarization contract, not a second MIR dialect.
The carrier must not escape as a returned value, ordinary call argument,
stored field, ArrayBox/MapBox element, or backend/runtime object. The current
SSOT is:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

new_expr := 'new' IDENT type_args? ( '(' args? ')' )? box_init_block?
box_init_block := '{' box_init_field (',' box_init_field)* ','? '}'
box_init_field := IDENT ':' expr
              ; BOX-INIT-001: explicit construction-site box field
              ; initializers only. This is sugar for NewBox followed by
              ; FieldSet in source order. It is not named constructor
              ; arguments, record materialization, wildcard copy, or shorthand
              ; field copy.

Examples:

```hako
local report = new Report { accepted: 1, reason: 0 }
local page = new Page(PageId(1)) { live: 1 }
```

Unmentioned fields keep declaration defaults / birth behavior. Duplicate field
entries and unknown user-defined box fields fail-fast.

This is a field-set contract surface, not a source-size shortcut. Use
same-owner `makeReport(fields)` helper scalarization to reduce repeated
call-site boilerplate; use `new Report { field: fields.field }` inside the
helper when grouping the initialization improves the boundary.

Record with-update replaces selected fields without mutating the original
record value:

```hako
local next = meta with {
    usable_size: new_usable
}
```

The base must be a tracked local record value. The update field names must exist
on that record type. `with` is not an ordinary box copy surface: `box_value with
{ field: expr }` is rejected rather than shallow-copying or calling `new`.
Construct boxes explicitly with `new Box { field: expr }`, or provide an
ordinary named copy/update method when box identity or resources are involved.
Array element field write-through such as `metas[i].usable_size = next` is not
part of this surface; use explicit get/update/set composition in later
container rows.

Stop line:
The current record surface does not add runtime record materialization, packed
`ArrayBox` storage, blanket ordinary-box flattening, reflection semantics,
record methods, cross-function record-local ABI, backend record lowering, or
allocator-specific syntax. Ordinary `box` declarations keep identity-capable
object semantics.

### CONTRACT-002 Contract Metadata Surface

Decision: accepted.

`requires`, `ensures`, and `invariant` are Stage0 metadata-only syntax:

```hako
releaseLocal(block_id: BlockId): Result<void, ReleaseError>
    requires block_id >= 0
    ensures block_id >= 0
{
    return Ok(void)
}

box HakoAllocPageModel {
    used: usize
    capacity: usize

    invariant used <= capacity
}
```

The parser preserves these clauses as metadata and leaves the body unchanged.
`requires`, `ensures`, and `invariant` are contextual in their syntax slots and
are not reserved as general identifiers.

Stop line:
CONTRACT-002 does not add `assert`, runtime contract insertion, invariant
boundary policy, verifier facts, or static discharge. Those are Stage1-owned.

### TRANS-001 Transition Metadata Surface

Decision: accepted.

`transition Enum::Value -> Enum::Value by method` is Stage0 metadata-only syntax
for box-local lifecycle relations:

```hako
enum PageState {
    Active
    Retired
}

box HakoAllocPageModel {
    state: PageState

    transition PageState::Active -> PageState::Retired by retire
}
```

The parser preserves the source state, target state, and method name as
metadata. `transition` and `by` are contextual in this box-member syntax slot
and are not reserved as general identifiers.

Stop line:
TRANS-001 does not add a `state` keyword, enum/variant lookup, method existence
checking, transition legality checking, runtime lowering, or lifecycle verifier
facts. Those are Stage1-owned.

### USES-001 Capability Metadata Surface

Decision: accepted.

`uses capability` is Stage0 metadata-only syntax for declaration-level
capability requirements:

```hako
freshPage(size: Bytes): Result<Page, Error>
    uses osvm
{
    return OsVm.reserve(size)
}
```

Multiple capability names can be listed with commas:

```hako
copyRaw(dst: RawBuf, src: RawBuf, len: Bytes): i64
    uses rawbuf, atomic
{
    return len
}
```

The parser preserves the capability names as metadata and leaves the body
unchanged. `uses` is contextual in this declaration-header syntax slot and is
not reserved as a general identifier.

Stop line:
USES-001 does not add `unsafe`, `cap` blocks, capability checking, backend
route selection, runtime lowering, provider activation, allocator hooks, or
`#[global_allocator]` coupling. Those are later Stage1/substrate rows.

### GEN-001 Generic Type Annotation Metadata Surface

Decision: accepted.

Generic type references are Stage0 metadata in declaration type positions:

```hako
type PageList = Array<PageId>

record MetaStore<T> {
    metas: PackedArray<T>
}

box Store {
    metas: PackedArray<Meta<PageId>>
    weak view: Span<PageId>
}
```

The parser preserves type-reference text in the AST and Program JSON v0. This
includes params, returns, fields, aliases, brands, enum payloads, and
box/record/enum type parameters.

Stop line:
GEN-001 only added generic type annotation transport. GEN-002 now owns known
generic arity checking. ARRAY-001 owns typed-context `Array<T>` literals.
PACKED-001 owns source-level `PackedArray<T>` declaration eligibility.
ARRAY-002A owns typed `Array<T>` method-name and arity diagnostics.
Constraint solving, `where` clauses, `Array<T>` element/type semantics,
PackedArray auto-use/backend lowering, `Span<T>` no-escape semantics, and
backend fallback policy remain later Stage1/CorePlan rows.

### GEN-002 Generic Arity Checker

Decision: accepted.

Stage1 checks generic type argument counts for known generic type names in
declaration metadata. The checker covers built-in/prelude generic surfaces and
same-program `box` / `record` / `enum` declarations:

```text
Array<T>       expects 1 argument
PackedArray<T> expects 1 argument
Span<T>        expects 1 argument
Option<T>      expects 1 argument
Result<T,E>    expects 2 arguments
```

Same-program declarations use their declared type parameter count:

```hako
record Meta<T> {
    value: T
}

box Store {
    ok: PackedArray<Meta<PageId>>
    // reject: Meta expects 1 argument
    bad: PackedArray<Meta<PageId, BlockId>>
}
```

Fail-fast tag:

```text
[generic/arity]
```

Stop line:
GEN-002 does not add type existence checking for unknown names, constraint
solving, `where` clauses, type substitution, monomorphization, `Array<T>`
element semantics, `PackedArray<T>` eligibility, or `Span<T>` no-escape
semantics. Those remain later Stage1/CorePlan rows.

### Array / PackedArray / Result / Option Canonical Surface

Decision: accepted.

Canonical collection and failure surfaces:

```hako
local ids: Array<PageId> = []
local metas: PackedArray<Meta> = []
local r: Result<Handle, AllocError> = Result::Err(AllocError::ZeroSize)
```

Rules:

- `Array<T>` is the ordinary typed collection spelling.
- `PackedArray<T>` requests packed residence and must fail-fast if unsupported.
- `Option<T>` and `Result<T,E>` are enum surfaces, not exception/null sugar.
- `[]` requires typed context in canonical code.
- `Array<T>` locals support the canonical methods `push(value)`, `get(index)`,
  `set(index, value)`, and `length()`; name/arity diagnostics are Stage1-owned.
- Direct literal / `push` / `set` element values are checked for typed
  `Array<T>` locals when the direct expression type is known.
- `T[]` is compatibility / low-level static-table spelling, not the canonical
  ordinary collection spelling.
- `Type::Variant` is the canonical enum variant spelling.
- `.` remains object field / method access, so `Result.Ok(...)` is not
  canonical.
- `Option<T>` and `Result<T,E>` are built-in enum prelude surfaces as of
  RESULT-001. `Option::Some(null)` / `Option::Some(void)` fail-fast.
- Prelude `Option<T>` / `Result<T,E>` local constructors require explicit typed
  context when generic parameters would otherwise be ambiguous; RESULT-002D
  tags those diagnostics with `[enum/expected-type][prelude]`.
- Known enum variants written with dot syntax, such as `Result.Ok(...)`, are
  rejected with enum-variant diagnostics.
- PACKED-002 emits metadata-only source PackedArray auto-use pilot rows for
  eligible `PackedArray<Record>` field declarations. Public record
  materialization, backend lowering, and boxed fallback stay disabled.

Canonical enum variants:

```hako
Option::Some(value)
Option::None
Result::Ok(handle)
Result::Err(reason)
PageState::Active
AllocError::ZeroSize
```

Stop line:
ARRAY-001's historical Stage1 typed-context restriction is superseded by the
Language v1 split: unannotated literals produce ordinary `AnyDefault` Arrays,
while supported explicit `Array<T>` annotations attach a semantic element
contract. `PackedArray<T> = []` still fail-fasts; there is no silent fallback
to ordinary `Array<T>` / `ArrayBox`. ARRAY-002A implements typed `Array<T>`
method-name and arity diagnostics.
ARRAY-002B implements direct typed `Array<T>` element diagnostics.
ARRAY-002C keeps unsupported `Array<T>` inference fail-fast.
ARRAY-002D fixes the JSON v0 / ArrayBox guard for ordinary `Array<T>` and PackedArray no-fallback.
RESULT-001 implements `Option<T>` / `Result<T,E>` as built-in enum prelude surfaces.
Known enum variants must use `Type::Variant`; dot variant spelling fail-fasts.
Match exhaustiveness expansion and PackedArray runtime/backend lowering remain
separate rows.

## Box Members

Decision: accepted target; production cutover pending.

The canonical Box member surface separates storage from behavior:

```text
obj.x    = stored field access
obj.x()  = method call
```

Source-level computed/once/birth_once Property forms are accepted only by the
current production compatibility implementation. They retire through
`BOX-MEMBER-PROPERTY-RETIRE0-I0-R0-G0`; they are not part of the final grammar.
The ordered retirement contract is
[`box-member-field-method-surface-ssot.md`](../../development/current/main/design/box-member-field-method-surface-ssot.md).

### Accepted target grammar

```
box_decl       := 'box' IDENT '{' member* '}'

member         := visibility_block
                | weak_stored
                | stored
                | delegate_decl
                | transition_member
                | invariant_member
                | method_decl
                | gate_member

visibility_block := ( 'public' | 'private' ) '{' member* '}'
                  ; member visibility grouping. `weak` is allowed inside.

weak_stored    := 'weak' IDENT ( ':' TYPE )?
                  ; non-owning stored field relation

visibility_weak_sugar := ('public'|'private') 'weak' IDENT ( ':' TYPE )?
                  ; equivalent to visibility block form
                  ; e.g., `public weak parent` ≡ `public { weak parent }`

stored         := IDENT ( '=' expr )?
                | IDENT ':' TYPE ( '=' expr )?
                  ; stored field (read/write). `IDENT` alone is the simple
                  ; untyped stored field form. `IDENT ':' TYPE` carries
                  ; declared-type metadata for tooling / typed-object planning;
                  ; it is not a general runtime type check.
                  ; `= expr` emits a construction prologue assignment before
                  ; the user `birth` body.

delegate_decl  := 'delegate' IDENT 'exposes' '{' delegate_expose+ '}'
delegate_expose:= IDENT ( 'as' IDENT )? ','?
                  ; DEL-002 Stage0 capsule. Carries explicit method exposure
                  ; metadata only. No forwarding/collision/interface semantics.

method_decl    := IDENT '(' params? ')' ( ':' TYPE_REF )? signature_clause* block handler_tail?
                  ; return annotation is optional. `: void` is an explicit
                  ; no-value contract; omission is an unannotated result contract,
                  ; not implicit void or source-level result inference. `birth`
                  ; is a constructor hook governed by the construction SSOT.
                  ; handler_tail is unsynchronized migration inventory and is
                  ; rejected by the accepted C′ target.

gate_member    := 'gate' build_predicate '{' member* '}' ('else' ('gate' build_predicate '{' member* '}' | '{' member* '}'))?
                  ; member-level build selection. Branches must preserve the
                  ; same public signature; public ABI/layout drift fails-fast.

params         := param (',' param)*
param          := IDENT (':' TYPE_REF)?
TYPE_REF       := ('void' | IDENT ('.' IDENT)*) ('<' TYPE_REF (',' TYPE_REF)* '>')? ('[' ']')*
                  ; parameter list
                  ; Type annotations are preserved as AST metadata.
                  ; `params` remains the canonical names-only compatibility surface.
                  ; Numeric substrate names such as i64/u64/usize are IDENT
                  ; names here. Literal suffix grammar is not live.

handler_tail   := ( catch_block )? ( cleanup_block )?
catch_block    := 'catch' ( '(' ( IDENT IDENT | IDENT )? ')' )? block
                ; historical parser/transport evidence; rejected C′ target.
cleanup_block  := 'cleanup' block
                ; historical handler-tail cleanup evidence; canonical C′ uses
                ; one standalone cleanup statement.

; Historical parser/transport evidence. Both postfix forms are rejected by C′.
postfix_catch      := primary_expr 'catch' ( '(' ( IDENT IDENT | IDENT )? ')' )? block
postfix_cleanup    := primary_expr 'cleanup' block
```

Semantics (summary)
- stored: O(1) slot read; write via assignment. Bare stored fields are dynamic/untyped. Typed stored fields keep declared-type metadata for optimizers/verifiers and typed-object planning, but ordinary field writes are not type-enforced by this syntax.
- stored initializers: `name = expr` and `name: Type = expr` are accepted and lower to constructor prologue assignments equivalent to `me.name = expr`. The prologue runs before the user `birth` body, in field declaration order. Initializer expressions are evaluated for each construction, so `field: ArrayBox = new ArrayBox()` creates a per-instance value rather than a shared static default.
- weak: a stored, non-owning relation; it is not a Property kind.
- field declaration syntax admits neither `=>` nor a block body. Computation is
  an ordinary method and therefore requires `()` at the call site.
- method handlers remain migration evidence only. C′ rejects catch and
  handler-tail/postfix cleanup; `LANGUAGE-RESULT-EXIT-C-PRIME0-R0` retires the
  carriers and `DOC0` rewrites this production after implementation.

Current migration lowering boundary (no JSON v0 change in this docs slice)
- stored → slot; declared type, when present, is copied into field-declaration metadata
- methods → ordinary callable lowering and generated callable Home ABI
- legacy method handlers still use the current TryCatch bridge and must not be
  widened. C′ I0 replaces cleanup with the verified exit owner; C′ R0 retires
  catch and the handler carrier.
- unsupported routes reject before protected-body effects. JSON v0 is unchanged.

### Current production compatibility grammar

Until the retirement row lands, the Rust parser still accepts these legacy
Property heads behind `NYASH_ENABLE_UNIFIED_MEMBERS` (default ON):

```ebnf
get_computed    := 'get' IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
legacy_computed := IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
once_decl       := 'once' IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
birth_once_decl := 'birth_once' IDENT ':' TYPE ( '=>' expr | block ) handler_tail?
block_as_role   := block 'as' ( 'once' | 'birth_once' )? IDENT ':' TYPE
```

These spellings are implementation evidence, not canonical alternatives. The
cutover deletes their parser edges, synthetic getter/cache/cycle machinery,
magic-name recovery, field-read reroute, environment gate, and old-only docs.
Removed syntax receives an exact method/field migration diagnostic. Ordinary
members named `get`, `once`, or `birth_once` remain legal because those words
are contextual rather than global hard keywords.

## Legacy: `init { ... }` field list (compatibility)

Some docs and older code use an `init { a, b, c }` list inside a `box` body. This is a legacy compatibility form to declare stored slots.

Semantics (SSOT):
- `init { a, b, c }` declares **untyped stored slots** named `a`, `b`, `c` (equivalent to writing `a` / `b` / `c` as stored members without type).
- `init { weak x, weak y }` declares **weak fields** (equivalent to writing `weak x` / `weak y` as members).
- It does not execute code. Initialization logic belongs in `birth(...) { ... }` and assignments.
- **New code** should use direct stored fields (`field_name`,
  `field_name: Type`, and their `= expr` initializer forms), `weak field_name`
  for weak storage, and ordinary methods for computation. Do not introduce new
  `get`/`once`/`birth_once` Property declarations.
- Legacy `init { weak field }` syntax still works for backward compatibility but is superseded by `weak field`.

## Enum Declarations (Phase-163x parser surface)

```ebnf
enum_decl        := 'enum' IDENT ('<' IDENT (',' IDENT)* '>')? '{' enum_variant* '}'
enum_variant     := IDENT
                  | IDENT '(' TYPE_REF ')'
                  | IDENT '{' enum_record_field (',' enum_record_field)* '}'
enum_record_field:= IDENT ':' TYPE_REF
qualified_ctor   := IDENT '::' IDENT
                  | IDENT '::' IDENT '(' args? ')'
                  | IDENT '::' IDENT '{' enum_record_init (',' enum_record_init)* '}'
enum_record_init := IDENT ':' expr
```

Notes:
- current executable surface includes unit variants, single-payload tuple variants, and a narrow named-record variant cut
- known-enum shorthand includes `Some(v)` / `None` and narrow record patterns like `Ident { name }`
- record constructors / patterns must mention the declared field set exactly
- multi-payload variants and block-bodied record shorthand arms are not part of this cut yet
- `qualified_ctor` is the narrow constructor surface used by enum values; this does not imply a general `::` static-method migration

## Historical Parser-Gate Evidence (not grammar authority)

Existing Rust/Hako parsers and compatibility fixtures still contain Stage-3 and
environment-gated handler spellings, including Compat source `try`, ambient
`NYASH_*` gates, and `TryCatch`-shaped transport. They are implementation
evidence for the migration inventory only.

The accepted C′ target is instead:

```text
source try / throw / catch          = rejected in both language profiles
RecoverableFailure Outcome          = rejected
typed Result-only postfix ?         = accepted target; production 0
canonical lexical spelling          = standalone cleanup { ... }
handler-tail/postfix cleanup        = retirement input
scope fini                          = retirement input, not a retained alias
Box-member fini { ... }             = terminal-Home hook; production 0
unsupported parser/backend route    = fail before Builder effects
```

No environment variable is a language-semantic profile owner. A later explicit
`GrammarProfileV1` row owns parser selection; the current gates must be retired
under `LANGUAGE-STAGE3-ENV-GATES-SUNSET-001`. The legacy parser evidence does
not authorize source `try`, generic Catch/Throw lowering, source-to-`try`
rewriting, or a backend no-op degradation.

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
- canonical inline request surface is `Inline(prefer|avoid|required)`;
  `Hint(inline/noinline)` and `Lowering(inline_required)` are compat spellings.
- dotted rune identifiers such as `allocator.fast` are accepted as a single
  metadata argument; profile names are still expanded to primitive MIR facts and
  must not become backend-readable route selectors.
- declaration-leading legacy aliases normalize to declaration-local `attrs.runes`.
- declaration metadata is allowed only on declaration targets.
- active grammar requires Rust parser / `.hako` parser parity.
- Rune metadata is declaration-local on AST/direct MIR; do not widen Program(JSON v0).
- body-position legacy aliases remain compat/noop during the current migration window.

## LoopRange implementation note (2026-05-15)

`loop i in start..end { ... }` is the canonical range-loop surface. Stage1 JSON
v0 lowering supports the current route: `start` and `end` are captured once at
entry, the index advances by fixed step `1`, `continue` routes to the step path,
and `break` routes to exit. The index is read-only. Fresh body-local writes are
accepted; loop-carried writes remain fail-fast under the current carrier policy.
