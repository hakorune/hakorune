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
  docs/development/current/main/design/semantic-contract-charter.md

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

## Recommended Task Shape

After decisions A-D, implement one substantive expansion card rather than one
card per spelling:

```text
LANGV1-GRAMMAR-REMAINING-SURFACE-REGISTRY-IMPLEMENTATION-001

1. add all accepted rows for the four remaining families
2. add generated typed projections from the same registry
3. add positive and negative corpus fixtures under both profiles
4. extend both independent ParseWitness adapters
5. enforce missing/extra/status/tag/normalized-shape drift
6. generate support/fixture indexes from the registry
7. rerun the full corpus once
8. evaluate LANGV1-GRAMMAR-001 closeout in the same card
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
```

## Non-Claims

```text
remaining_registry_rows_implemented = 0
loop_compatibility_decided = 0
weak_spelling_contract_decided = 0
record_registry_contract_decided = 0
literal_registry_contract_decided = 0
parser_behavior_changed = 0
compat2025_acceptance_changed = 0
language_v1_grammar_closeout = 0
runtime_backend_changes = 0
selfhost_claim = 0
```

## Stop Rule

Do not edit the registry or parser acceptance until A-D are answered together.
The next step after consultation must be the single code-facing implementation
card above, not another basis/inventory/rerun card.
