# 3460 - LANGV1-GRAMMAR-DESIGN-STOP-001

## Status

Design consultation stop. Do not change parser implementations, grammar
registry generation, compatibility acceptance, or backend behavior from this
card.

## Established Basis

`LANGV1-SEMANTIC-KERNEL-001` closes through 3459. Compound assignment now
uses an evaluated Place and has source-order, fail-fast, and VM-reference
evidence. The next macro row is canonical grammar and dual-parser conformance.

## Decision Required

Choose the grammar-contract basis before implementation:

```text
registry row schema
canonical / compatibility_only / reserved / rejected status
Canonical default and explicit Compat2025 boundary
ParseWitness fields and Rust/Hako comparison boundary
initial closed surface inventory and stable reject tags
```

The decision must resolve the current `guard`, `try`, `peek`, and `from`
document/parser drift without treating one parser's present behavior as
language authority.

## Drift Inventory

This inventory records implementation evidence, not language authority.

| Family | Reference contract | Rust parser evidence | Hako parser evidence | Current proof gap |
| --- | --- | --- | --- | --- |
| guard | `guard expr else` and narrow `guard let ... else` are canonical | accepts both by default; lowers to `If(Not(...))` or enum-match sugar | no guard statement route found under `lang/src/compiler/parser/**` | no shared positive/negative corpus or Hako witness |
| try | statement `try` is legacy-only; postfix `catch/cleanup` and `fini` are canonical | accepts `try` by default; `NYASH_FEATURES=no-try-compat` rejects with `[freeze:contract][parser/try_reserved]` | statement parser dispatches `try` without a Canonical/compat profile check | Canonical default currently accepts legacy syntax in both routes |
| match/peek | `match` is canonical; `peek` is legacy and replaced | tokenizer/parser accepts `match`; no `PEEK` token or live `peek` parser route | accepts both; `peek` emits a distinct `Peek` JSON shape | no decision whether Compat2025 normalizes `peek` or rejects it |
| delegation/from | `delegate field exposes { ... }` is canonical; `from`, `override`, `extends`, and `super` are legacy | accepts `box Child from Parent` and `from Parent.method()` legacy shapes as live AST | no equivalent `from` parser route found in the current parser boxes | no semantics-preserving normalization proof from inheritance-style `from` to explicit delegation |

Additional drift:

```text
Compat2025 profile implementation = absent
ParseWitness type = absent
shared Rust/Hako grammar corpus = absent
stable reject tag coverage = try only among the four families
parser implementations = independent, as required
```

Existing tests are implementation-specific. Rust has focused guard, match,
delegate, and Stage-3 try tests. Hako has parser boxes and selfhost fixtures,
but there is no shared profile-aware golden suite.

## Candidate Basis

Recommended first implementation slice after consultation:

```text
one registry schema
four closed families: guard, exception, match, delegation
Canonical and Compat2025 expectations in every row
one span-free ParseWitness schema
one shared positive/negative corpus
two independent parser adapters
```

This is a first slice, not the macro-row closeout. `LANGV1-GRAMMAR-001` closes
only after the registry exhaustively covers the accepted v1 surface listed by
the workstream. Do not create separate cards for each spelling or rerun.

Suggested registry fields:

```text
row_id
family
spelling_id
profile
status = canonical | compatibility_only | reserved | rejected
production
normalized_shape
semantic_owner
stable_reject_tag
rust_support
hako_support
positive_fixture_ids
negative_fixture_ids
```

Suggested ParseWitness fields:

```text
row_id
profile
accepted
normalized_kind
normalized_children
stable_reject_tag
```

Spans, source paths, parser-internal node names, and test counts are excluded
from the witness.

## Consultation Packet

Ask for one coherent decision covering all questions below.

```text
We are at LANGV1-GRAMMAR-DESIGN-STOP-001 in Hakorune.

Accepted laws:
- one canonical language contract
- Rust and Hako parsers remain independent
- compatibility is explicit opt-in
- compatibility aliases normalize immediately to canonical shape
- unsupported syntax fails before effects
- parser implementation behavior is evidence, not language authority

Observed drift:
1. guard expr else and guard let are canonical. Rust accepts both, but the
   current Hako parser has no guard statement route.
2. try statement is documented legacy-only. Rust accepts it by default and
   rejects only with no-try-compat; Hako dispatches try without a profile check.
3. match is canonical and peek is legacy. Rust has only match; Hako accepts
   both and emits a distinct Peek JSON shape.
4. delegate field exposes is canonical. Rust still accepts box/from inheritance
   and from Parent.method(); Hako has no equivalent current from route.
5. Compat2025, ParseWitness, and a shared profile-aware corpus do not exist.

Please decide:
A. Exact canonical/compatibility_only/reserved/rejected status for guard,
   guard-let, postfix catch, postfix cleanup, fini, try, match, peek, delegate,
   box-from inheritance, and from Parent.method().
B. Whether Canonical becomes the immediate default, with legacy acceptance
   available only through explicit Compat2025.
C. Whether peek can normalize to Match in Compat2025. If not always, define
   the closed normalizable subset and stable rejection boundary.
D. Whether either from form can normalize to explicit delegation without
   changing semantics. If not, choose compatibility-only transport or reject.
E. The minimum registry row schema and span-free ParseWitness fields.
F. Stable reject-tag families for canonical rejection and parser drift.
G. Whether the first implementation slice should cover the four closed drift
   families above, while the same macro row later expands to the exhaustive v1
   registry before closeout.

For the selected basis, provide:
- exact status table
- authority and non-authority sources
- fail-fast rules
- normalized witness examples
- allowed first implementation slice
- non-claims
- conditions for declaring the full grammar row complete

Do not authorize parser sharing, implicit fallback, broad parser rewrites,
type-contract activation, failure-model changes, or selfhost migration.
```

## Source Authority

```text
language laws = semantic-contract-charter.md
evaluation law = semantic-kernel.md
canonical grammar text = EBNF.md
current parser evidence = independent Rust and Hako implementations
```

## Non-Authority

```text
legacy parser acceptance alone
historical syntax notes
source path or use count
one parser's AST representation
compatibility fallback behavior
```

## Fail-Fast Boundary

```text
no implicit compatibility after Canonical rejection
no shared parser implementation
no parser rewrite before registry decision
missing registry row or witness drift -> fail-fast
```

## Non-Claims

```text
grammar_registry_implemented = 0
compat2025_activated = 0
parse_witness_conformance = 0
rust_hako_parser_behavior_changed = 0
type_contract_activation = 0
selfhost_claim = 0
```
