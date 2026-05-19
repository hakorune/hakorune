# Hakorune .hako Stage Profiles

Status: Current reference
Scope: Stage0 / Stage1 usable `.hako` surface profiles. This document is a
support manual, not a second language specification.
Related:
- `docs/reference/language/README.md`
- `docs/reference/language/EBNF.md`
- `docs/development/current/main/design/language-minimal-surface-ssot.md`
- `docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md`
- `docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md`
- `docs/reference/language/low-level-capabilities.md`
- `docs/reference/concurrency/semantics.md`

## Decision

Hakorune has one canonical `.hako` language surface. Stage0 and Stage1 do not
define separate languages.

This profile manual answers a narrower question:

```text
Given a canonical .hako feature, what may Stage0 carry today, and what may
Stage1 code rely on today?
```

Use this split:

| Manual | Owns |
| --- | --- |
| `docs/reference/language/EBNF.md` | canonical grammar and accepted syntax rows |
| topic pages under `docs/reference/language/` | executable semantics by topic |
| `language-minimal-surface-ssot.md` | keyword/surface admission policy |
| `stage0-stage1-feature-responsibility-split-ssot.md` | durable Stage0 vs Stage1 ownership rule |
| this file | practical Stage0 / Stage1 support profile |

If this file conflicts with the canonical grammar or a topic semantics page,
fix this file. Do not create a Stage0-only or Stage1-only language spec.

## Profile Definitions

### Stage0 Bootstrap Profile

Stage0 is a bootstrap reader and transport lane.

Allowed:

```text
syntax acceptance
AST / JSON / metadata transport
trivial desugar into already-defined syntax
diagnostics and fail-fast plumbing
host substrate calls needed by the bootstrap route
```

Forbidden:

```text
type-system authority
brand checking
record layout policy
contract / invariant checking
transition legality checking
PackedArray planning
capability policy
allocator algorithm meaning
backend route selection from names
```

Stage0 support in the matrix means one of:

- `live`: the bootstrap route can parse and use the shape as ordinary code;
- `transport`: Stage0 may carry the shape as metadata only;
- `reject`: Stage0 must fail fast rather than inventing meaning.

### Stage1 Selfhost Profile

Stage1 owns language meaning and the compiler facts that make a feature useful.

Allowed:

```text
semantic lowering
diagnostics
verifier facts
CorePlan facts
backend capability fail-fast contracts
selfhost compiler / hako_alloc policy structure
```

Stage1 support in the matrix means one of:

- `live`: the feature is usable in the current narrow supported shape;
- `guarded`: usable only when the named guard/backend route accepts it;
- `metadata-only`: visible to compiler facts but not a user-visible runtime
  behavior yet;
- `pending`: syntax or plan exists, but the semantic row is not live;
- `deferred`: intentionally not a current profile feature.

## Usage Rules

- Writing canonical source: use `EBNF.md`, topic pages, and this profile.
- Writing Stage0 bootstrap code: use only `live` Stage0 rows as behavior.
  `transport` rows must not be treated as semantics.
- Writing Stage1 / selfhost / `hako_alloc` code: use `live` or explicitly
  `guarded` Stage1 rows. Open a task row before relying on `pending` behavior.
- Writing low-level allocator code: use capability modules and runtime
  substrate rows. Do not use concurrency surface syntax to model allocator
  worker/TLS/atomic substrate.
- Unsupported backends must fail fast. Silent fallback is not a profile.

## Support Matrix

| Feature family | Stage0 bootstrap profile | Stage1 selfhost profile | Notes |
| --- | --- | --- | --- |
| `box`, `static box`, methods, functions, fields, `birth` | live for ordinary bootstrap code | live | Stored field type annotations are metadata unless a specific verifier row owns them. |
| `local`, assignment, compound assignment, field/method calls, literals | live | live | `+=` / `-=` / `*=` / `/=` are surface sugar over assignment plus binary op. |
| `if`, `guard expr else`, `loop cond`, `loop {}`, `break`, `continue`, `return` | live | live | `while`, `for`, `repeat`, and `until` are not canonical source spellings. |
| `loop i in start..end` | transport / narrow live parse | live, guarded by LoopRange facts | End-exclusive, step `1`, entry-bound capture, read-only index. Fresh body-local writes are accepted; loop-carried writes remain fail-fast. Stage0 must not desugar this to local/increment form. |
| `check "name" { ... }` | live scalar proof-list expression | live scalar proof-list expression | It is eager and returns an integer pass/fail value. It is not a macro, short-circuit operator, or report object. |
| `static const NAME: u16[] = [...]` and `NAME[index]` | live first-row table syntax | live for `u16[]` readonly data | Const fn, const references, and extra element types remain future rows. |
| `using` | live current import surface | live current import surface | Module/package visibility semantics are later Stage1 rows. |
| `brand Name: Type` | transport | live constructor / unwrap / mismatch checker | Stage0 must not treat brands as type-checker truth. |
| `type Alias = Type` | transport | pending richer diagnostics | Alias metadata is preserved; broad alias equality semantics are not live. |
| `record`, explicit record literal, `with` update | transport for declaration/literal/update shape | live construction/read/update in the accepted shape; narrow local record helper-argument scalarization is live for same-owner helper calls | Records are identity-free aggregates. Record-local scalarization is compiler-local only: no runtime record object, no cross-function record ABI, no backend record route, and no ordinary-box auto-recordification. |
| `enum`, `Type::Variant`, `Option<T>`, `Result<T,E>` | parse / transport inventory; no Stage0 special-case meaning | live narrow enum/prelude surface and diagnostics | Dot variants such as `Result.Ok(...)` are rejected for known enum variants. |
| `guard let Type::Variant(binding) = expr else { ... }` | parser sugar only | live narrow enum guard sugar | No null sugar, `try`, `throw`, or `?` family. |
| `Array<T>` and `[]` literals | type annotation and literal-shape transport | live typed-context arrays, method contract, direct element checks | `local xs = []` and unresolved generic contexts fail fast. |
| `PackedArray<T>` | type metadata transport | guarded CorePlan facts and no-fallback contract | Source `PackedArray<Record>` rows are still narrow/metadata-first; no silent fallback to ordinary `ArrayBox`. |
| fixed-width names `i8..u64`, `isize`, `usize` | annotation text transport and exact const metadata | guarded metadata/verifier rows; runtime remains current dynamic `Integer(i64)` lane | Backends must not infer exact unsigned runtime behavior from the spelling alone. |
| `@rune ...` declaration metadata | live parser metadata transport | live for current verifier/inline/profile fact rows | Backend route selection must not read profile names directly. |
| `uses capability` | transport | pending broad capability checker; some substrate route guards are live | `uses` is the MVP capability declaration surface. `cap` blocks are deferred. |
| `requires`, `ensures`, `invariant` | transport | pending runtime-check insertion / verifier discharge | Stage0 must not insert or discharge contracts. |
| `transition Enum::A -> Enum::B by method` | transport | pending transition legality checker | `state` keyword is not canonical MVP syntax. |
| `delegate field exposes { ... }` | transport | live exposes lowering in accepted shape | Legacy `from`, `override`, `extends`, and `super` are historical/compat, not canonical new code. |
| `externcall` / low-level capability module calls | live where the bootstrap route provides the host substrate | guarded by runtime substrate and lowering routes | Prefer capability modules / `hako_alloc` owners over by-name backend shortcuts. |
| `nowait` / `await` | supported by the Rust/parser concurrency route; use only as documented by the concurrency manual | concurrency-profile feature; not a general Stage1 selfhost prerequisite | These do not imply worker-local allocator cache, TLS, or true thread semantics. Treat Stage1 selfhost use as profile-gated unless the active selfhost route says otherwise. |
| `Channel`, `task_scope`, `lock<T>`, `scoped`, source `worker_local` | reject/deferred for language-core bootstrap use | design/scaffold/deferred unless the concurrency manual says otherwise | Mimalloc needs internal worker/TLS/atomic substrate, not source-level `worker_local`. |
| `Span<T>`, `view`, `interface`, `impl`, `where`, `const fn`, `comptime`, `const assert`, `cap {}` | reject/deferred | deferred | Open a Stage1 row before using these as real source features. |

## Reserved And Legacy Surface

Do not use these as canonical Stage0 or Stage1 source in new code:

```text
while
for
repeat
until
do
try
throw
?
class
extends
super
origin
unsafe
state keyword
cap block syntax
implicit null-based Option/Result sugar
ordinary box as record
PackedArray boxed fallback
```

Legacy compatibility may still exist in parser or historical docs. Compatibility
does not make a spelling canonical.

## When Adding A New Row

Every new profile row must state:

```text
Decision:
Canonical syntax:
Stage0 owns:
Stage0 does not own:
Stage1 owns:
Unsupported backend behavior:
Retire condition for Stage0 capsules:
Guard / fixture:
```

If a Stage0 task needs semantic checks, stop the row and split it into a Stage1
task. If a Stage1 task needs backend support, define the fail-fast boundary
before accepting source code that depends on it.
