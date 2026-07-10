# 3477 - LANGV1-GRAMMAR-REMAINING-SURFACE-CONTRACT-DESIGN-STOP-001

## Status

Active design consultation stop after the 3476 closeout audit disproves
`LANGV1-GRAMMAR-001` completion.

Decision: consultation required before registry or parser changes.

## Closeout Audit

The current corpus and both in-scope parser adapters are green, but green is
not exhaustive coverage.

```text
registry rows = 22
profiles = Canonical + Compat2025
covered spellings = 11

covered families:
  guard
  exception
  match
  delegation

required but not registered:
  loops
  weak
  records
  current literal surfaces
```

The missing families are explicit deliverables in
`language-v1-convergence-current.md`. Therefore:

```text
language_v1_grammar_closeout = 0
current_corpus_green = 1
current_corpus_exhaustive = 0
```

## Source Evidence

### Loops

```text
canonical EBNF:
  loop { ... }
  loop condition { ... }
  loop i in start..end { ... }
  break / continue

explicitly non-canonical:
  while / for / do-while / repeat / until

implementation evidence:
  Rust still has a stage-gated while route into parse_loop
  Hako has parse_loop / parse_break / parse_continue
```

The canonical loop rows are mechanically classifiable. The treatment of the
live Rust `while` spelling under Compat2025 still needs an explicit decision.

### Weak

```text
live surfaces:
  weak expr
  weak field
  public/private weak field
  legacy init { weak field }

known contract:
  weak(expr) is rejected; canonical unary spelling is weak expr
  direct weak field is the preferred stored-field spelling
  init { weak field } is documented as backward compatibility
```

One `weak` row would collapse expression, field, visibility sugar, and legacy
member syntax. They need distinct spelling rows and a decision on whether the
visibility sugar is canonical or compatibility-only.

### Records

```text
accepted source surfaces:
  record declaration
  record literal
  record with-update

current restrictions:
  fixed typed non-weak fields
  scalar literal defaults only
  no record methods / fini / inheritance / interface implementation
```

These surfaces are accepted in the reference specification. The registry must
not turn their Stage1 semantic restrictions into parser acceptance authority.

### Literals

```text
minimum v1 literals:
  integer
  string
  bool
  null
  void

additional current surfaces:
  float
  typed integer suffixes on Rust front
  typed-context array literal
  map literal behind legacy environment/sugar gates
  new box expression
```

`current literal surfaces` is not a closed set today. In particular, map
literal acceptance is controlled by legacy environment switches, while the v1
grammar contract requires explicit profiles and forbids ambient grammar
authority.

## Authority

```text
language laws:
  docs/reference/language/semantic-contract-charter.md

v1 minimum surface and closeout law:
  docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  docs/development/current/main/workstreams/language-v1-convergence-current.md

canonical productions and accepted decisions:
  docs/reference/language/EBNF.md
  docs/reference/language/grammar-contract.md
  docs/reference/language/types.md

registry authority after decision:
  grammar/unified-grammar.toml

implementation evidence only:
  Rust parser
  Hako parser
```

## Non-Authority

```text
current parser acceptance alone
environment feature gates
legacy syntax comments alone
source path or test count
existing AST shape
current green corpus alone
row count or coverage percentage
runtime/backend behavior
```

## Structural Review Findings

The remaining-row expansion touches every registry consumer, so the registry
representation must be corrected before adding more rows.

### Finding 1 - Conflicting physical sources in one TOML

Confirmed. `grammar/unified-grammar.toml` currently contains both:

```text
legacy Nyash v1.1 keyword/training/codegen input:
  peek is preferred over match
  box/from has live inheritance meaning
  loop(condition) is the only documented loop form

Language v1 contract rows:
  match is canonical
  peek is Canonical-rejected / Compat2025 alias
  from forms are transport-only
```

The comment that declares the upper tables non-authority does not remove the
physical contradiction. However, root `build.rs` still consumes the legacy
keyword/operator/syntax tables, so deleting or passively archiving them would
break a live codegen input.

Required structural fix:

```text
grammar/language-v1-registry.toml
  sole Language v1 grammar authority

grammar/legacy/nyash-v1.1-codegen-input.toml
  explicit non-authority legacy input while its root build.rs consumer lives

root build.rs
  reads only the named legacy input until that consumer retires

Language v1 generators/adapters/guards
  read only language-v1-registry.toml
```

This is a move with explicit consumers and retirement conditions, not a second
grammar authority.

### Finding 2 - Profile twins duplicate common truth

Confirmed in direction, with one correction. The current 22 entries duplicate
11 spellings across two profiles. Common production/shape/owner fields can
drift independently.

The source row should own both profile contracts:

```text
row identity and common fields
canonical contract
compat2025 contract
```

Profile-specific status, normalization, reject tag, and fixture lists remain
distinct. A raw TOML document can still omit a profile field, so the loader
must validate both fixed profile contracts. After loading, the typed source
model must make a missing profile impossible. The generator may continue to
expand one source row into `(row_id, profile)` Rust/Python projections so
existing parser APIs do not need a simultaneous rewrite.

### Finding 3 - Parser support is evidence, not registry authority

Confirmed. `rust_support` and `hako_support` are handwritten into authority
rows and already describe historical implementation states. They can drift
independently from the corpus.

Required boundary:

```text
language-v1-registry.toml:
  no rust_support / hako_support fields

corpus execution:
  produces implementation evidence per parser/profile/row

generated support matrix:
  derived artifact from corpus evidence
  never an input to grammar acceptance
```

### Finding 4 - ParseWitness observes only one child level

Confirmed as a coverage gap. `normalized_children: Vec<String>` can prove only
the immediate child-kind list. It cannot detect disagreement inside a nested
guard, Match arm, record literal, or loop body.

Do not expose either parser's AST directly and do not use an untyped parser
dump as authority. Prefer one parser-neutral recursive normal form:

```text
NormalizedSyntaxNode {
  kind
  children: [NormalizedSyntaxNode]
}
```

Its serialized canonical representation may be JSON or an S-expression, but
the in-memory owner should be typed and span/source-path/internal-name free.
The consultation must decide whether recursive witness migration is required
for `LANGV1-GRAMMAR-001` closeout or is the immediately following hardening
slice.

### Finding 5 - Enumerated corpus lacks composition coverage

Confirmed as residual risk. Current fixtures are intentionally enumerated and
mostly shallow. A differential generator can recover the value of two
independent parsers outside the handwritten list.

The safe form is not an unbounded random fuzzer in the default gate:

```text
registry positive fixtures as seeds
fixed seed and bounded depth/case count
grammar-aware composition only
one compile-once Hako batch
Rust/Hako recursive witness comparison
failure prints a minimized reproducible source and seed
wall-clock budget is measured separately from semantic pass/fail
```

Because current Hako corpus compilation is material, the consultation must set
whether this is a quick gate, milestone gate, or post-closeout hardening gate.

## Required Decisions

### A - Loop compatibility boundary

Choose whether `while` is:

```text
A1. Canonical rejected and Compat2025 compatibility alias to loop condition
A2. rejected in both profiles
```

`for`, `do-while`, `repeat`, and `until` remain rejected unless a separate
closed lossless normalization is proven. Fix rows for loop infinite,
loop-condition, loop-range, break, and continue.

### B - Weak spelling decomposition

Fix separate rows and statuses for:

```text
weak unary expression
weak(expr) rejected spelling
direct weak stored field
visibility weak sugar
legacy init { weak field }
```

Do not combine ownership/identity semantics with parser conformance. This card
only decides spelling, profile, normalized shape, and stable reject tags.

### C - Record grammar boundary

Confirm whether these are three canonical rows under both profiles:

```text
record declaration
record literal
record with-update
```

Specify whether invalid field forms are represented by one production row plus
negative fixtures, rather than separate language rows. Keep Stage1 type/field
set restrictions outside parser authority.

### D - Closed literal set

Define the exact v1 registry set. At minimum decide independently:

```text
primitive literals: integer / float / string / bool / null / void
typed integer suffixes: canonical, Compat2025-only, or Rust evidence only
array literal: canonical syntax with Stage1 typed-context restriction
map literal: canonical, Compat2025-only, or rejected
new box expression: literal family or construction family
```

No environment variable may select a grammar profile or silently admit a
literal spelling.

### E - Registry representation migration

Confirm the following structural migration before row expansion:

```text
1. split the Language v1 authority from the live legacy codegen input
2. represent one spelling once with two fixed profile contracts
3. validate both profile contracts at load time
4. expand to existing per-profile typed projections at generation time
5. remove handwritten Rust/Hako support fields from authority rows
6. generate support evidence from corpus execution
```

Specify the legacy root-build consumer retirement condition. Do not keep
`unified-grammar.toml` as a second forwarding authority after migration.

### F - Witness depth and generated composition gate

Decide:

```text
F1. recursive parser-neutral witness is required before grammar closeout
F2. shallow witness may close grammar, recursive witness is immediate hardening

and

G1. bounded differential composition runs in the quick gate
G2. bounded differential composition runs only at milestone/full gate
G3. defer it until measured adapter cost is reduced
```

Any selected generator must be deterministic, bounded, reproducible, and must
not generate parser implementations or infer grammar authority from parser
agreement.

## Recommended Task Shape

After decisions A-F, implement one substantive refactor/expansion series under
one active card rather than one card per spelling:

```text
LANGV1-GRAMMAR-REGISTRY-NORMALIZATION-AND-SURFACE-EXPANSION-001

commit A - behavior-preserving physical/schema migration:
  split v1 authority from named legacy build input
  move to one source row with two validated profile contracts
  preserve expanded typed projection APIs

commit B - evidence ownership:
  remove handwritten parser support fields
  generate support evidence from corpus results

commit C - remaining accepted surface:
  add all decided loop/weak/record/literal rows
  add positive/negative fixtures under both profiles
  extend both independent adapters

commit D - witness/conformance closeout:
  implement the selected witness depth
  enforce missing/extra/status/tag/normalized-form drift
  generate support/fixture/reference indexes
  run the selected bounded composition gate when authorized
  rerun the full corpus once
  evaluate LANGV1-GRAMMAR-001 closeout in the same card
```

Do not split inventory, fixture addition, and rerun into separate numbered
cards. If a parser lacks a decided canonical row, fix that parser in the same
implementation card or fail fast with an explicit unsupported tag.

## Fail-Fast Boundary

```text
missing required row -> parser/registry_row_missing
missing profile twin -> parser/profile_mismatch
implicit Canonical-to-Compat retry -> parser/implicit_compat_retry_forbidden
environment-selected grammar acceptance -> parser/environment_profile_forbidden
Rust/Hako witness mismatch -> parser/witness_drift
parser-internal shape in witness -> parser/witness_internal_shape_forbidden
unclassified current literal spelling -> parser/literal_surface_unclassified
```

No warn-only drift, parser sharing, source slicing/reparse fallback, AST
rewrite, runtime fallback, or backend fallback is authorized.

## Claims

```text
remaining_grammar_surface_inventory = 1
language_v1_grammar_closeout_disproved = 1
remaining_surface_decision_required = 1
single_followup_implementation_card_required = 1
registry_physical_authority_split_required = 1
profile_source_deduplication_required = 1
parser_support_evidence_generation_required = 1
recursive_witness_decision_required = 1
bounded_differential_gate_decision_required = 1
```

## Non-Claims

```text
remaining_registry_rows_implemented = 0
loop_compatibility_decided = 0
weak_spelling_contract_decided = 0
record_registry_contract_decided = 0
literal_registry_contract_decided = 0
registry_representation_migrated = 0
parser_support_fields_removed_from_authority = 0
recursive_parse_witness = 0
bounded_differential_composition_gate = 0
parser_behavior_changed = 0
compat2025_acceptance_changed = 0
language_v1_grammar_closeout = 0
runtime_backend_changes = 0
selfhost_claim = 0
```

## Stop Rule

Do not edit the registry or parser acceptance until A-F are answered together.
The next step after consultation must be the single code-facing implementation
card above, not another basis/inventory/rerun card.
