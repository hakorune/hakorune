# 3478 - LANGV1-GRAMMAR-REGISTRY-NORMALIZATION-AND-SURFACE-EXPANSION-001

## Status

Active implementation series after 3477 accepts all remaining grammar and
registry-structure decisions. Series A, the recursive witness schema, and the
Series C loop family are complete. Live recursive projection for both parsers
is next. The declaration conformance owner is accepted as a parser-owned,
non-semantic ProgramJSON evidence sidecar.

Decision: accepted by 3477.
Declaration sidecar decision: accepted by post-loop design consultation.

Implementation: in progress.

## Progress

### Series A - Complete

```text
Language v1 authority:
  grammar/language-v1-registry.toml

legacy root-build input:
  grammar/legacy/nyash-v1.1-codegen-input.toml

ambiguous grammar/unified-grammar.toml:
  removed

source schema:
  one spelling row with canonical + compat2025 required contracts

generated projections:
  preserve existing (row_id, profile) consumer API

parser support fields:
  removed from authority rows
```

Verification:

```text
Python registry/corpus tests = green
frontend grammar crate tests = green
root hakorune build = green
language_v1_grammar_contract_substrate_guard = green
```

### Series B Schema - Complete; Live Projection - In Progress

```text
authority support fields:
  rust_support / hako_support removed

recursive witness owner:
  NormalizedSyntaxNode { kind, value?, children }

fixture schema:
  normalized_form recursive inline tree

comparison:
  strict whole-tree equality; nested drift unit test green

support matrix:
  deferred to Series D execution output after both adapters cover the expanded rows
```

The schema and comparator are complete. The live projection foundation is now
active:

```text
Rust observation:
  AST JSON -> Rust-owned row projector -> recursive normalized_form

Hako observation:
  ProgramJSON -> Hako-owned row projector -> recursive normalized_form

Hako compile-once batch comparison:
  expected normalized_form == observed recursive projection
```

The old `ImplementationAccepted` leaf has been removed from the drift report.
Raw ProgramJSON equality remains optional Hako-internal evidence and is not
cross-parser proof. Full live recursive conformance is still a non-claim until
all current registered fixtures pass.

### Series C0 Live Recursive Witness Projection - In Progress

Implement two independent projections against the already-shared schema:

```text
Rust AST evidence
  -> Rust-owned recursive NormalizedSyntaxNode projection

Hako raw ProgramJSON evidence
  -> Hako-adapter-owned recursive NormalizedSyntaxNode projection

shared authority
  -> row/profile/expected recursive form from the corpus

strict gate
  -> expected == Rust projection == Hako projection
```

Rules:

```text
no shared parser/projection implementation
no fixture-id-specific result table
no copying expected normalized_form into observed output
no parser-internal node names in the public witness
no raw ProgramJSON equality as cross-parser proof
rejected rows carry no normalized form
missing projection kind fails parser/witness_missing
nested difference fails parser/witness_drift
identifier/type/value drift fails parser/witness_drift
```

First close the projection vocabulary for the current registered rows. Only
then add the remaining weak/record/literal/construction rows and their new
projection kinds. This keeps the Series D composition runner from amplifying a
shallow comparison.

Current focused evidence:

```text
optional normalized node value = implemented
independent Rust/Hako projector unit tests = green
strict Hako loop/map recursive batch = 7 green
loop block-head delimiter drift = fixed through one expression context owner
strict Hako postfix/try recursive batch = 6 green
legacy try normalization is explicit and does not constrain canonical postfix
```

Full inventory exposed pre-existing corpus/parser drift that the shallow gate
did not detect: 16 Hako and 27 Rust accept/tag mismatches across the current
non-transport fixtures. These are corrective inputs for C0, not reasons to
weaken or bypass recursive comparison.

### Series C Preparation - Complete

```text
shared corpus manifest:
  grammar/language-v1-grammar-contract-corpus.toml

foundation fragment:
  grammar/language-v1-grammar-contract-corpus/foundation.toml

loader behavior:
  Rust and Python merge ordered fragments into one logical fixture set

size boundary:
  foundation fragment remains below 800 lines
remaining-surface fixtures will live in a separate fragment
```

### Series C Loop Family - Complete

```text
canonical loop forms:
  LoopInfinite, LoopCondition, LoopRange, Break, Continue

Compat2025 alias:
  while condition block -> LoopCondition

rejected in both profiles:
  for, do-while, repeat, until

profile authority:
  Rust tokenizer no longer lets the Stage3 environment gate rewrite
  loop-profile spellings into identifiers

Hako evidence:
  one compile-once, 12-row canonical/Compat loop batch = green
  one compile-once, 8-row legacy-loop rejection batch = green
```

`break` and `continue` outside a loop remain context-verifier rules. They are
not fixture aliases on the grammar row contracts.

### Series C Declaration Conformance Boundary - Accepted

The Hako grammar adapter owns statement and expression parsing only. A direct
observation of `record User { id: i64 }` currently fails with
`parser/hako_record_fields_expected_canonical`: the source is routed through
the record-literal expression owner because no declaration parser owner exists.

This blocks strict Hako conformance for these accepted rows:

```text
record_declaration
weak_stored_field
weak_visibility_field
weak_legacy_init_field
```

Decision B is accepted:

```text
ProgramJSON.body
  = existing semantic parser evidence

ProgramJSON.parser_evidence.declarations
  = parser-owned grammar evidence only
  = external ParseWitness adapter input only
  = never a semantic/MIR/runtime/backend input
```

Structure:

```text
lang/src/compiler/parser/decl/README.md
lang/src/compiler/parser/decl/parser_declaration_box.hako
lang/src/compiler/parser/decl/parser_record_declaration_box.hako
lang/src/compiler/parser/decl/parser_box_weak_field_box.hako

ParserProgramBox
  -> declaration-head dispatch before statement/expression fallback
  -> append sidecar evidence from the same parse invocation
  -> do not append declarations to ProgramJSON.body
```

`ParserDeclarationBox` owns declaration dispatch and profile gating. Record
and box-weak-field parsing are separate subparsers. The Hako profile gate
consumes a generated projection of `language-v1-registry.toml`; do not create
another hand-maintained spelling/status table.

Minimum sidecar row:

```text
row_id
profile
accepted
normalized_form = { kind, value?, children }
stable_reject_tag
semantic_publication_allowed = false
mir_lowering_allowed = false
runtime_allowed = false
backend_allowed = false
```

The optional parser-neutral `value` is required where identifier, type, field,
visibility, or literal identity is part of conformance. It must not carry
spans, source paths, parser class/node names, MIR ids, ValueIds, handles, or
backend descriptors.

First declaration scope:

```text
record_declaration
weak_stored_field
weak_visibility_field
weak_legacy_init_field
```

Profile contracts:

```text
weak_stored_field = canonical in both profiles
weak_visibility_field = Canonical reject; Compat2025 alias
weak_legacy_init_field = Canonical reject; Compat2025 alias
```

Do not add adapter-local source scanning, a `CompatibilityTransport` AST node,
fixture-specific acceptance, or declaration entries in semantic body. Record
literals, record updates, weak unary expressions, primitive literals, arrays,
maps, and construction remain separate Series C surfaces.

### Remaining Ordered Checkpoints

Keep these checkpoints inside 3478; do not create spelling-specific cards:

```text
1. live recursive projection for the current registered corpus
2. Hako declaration sidecar structure + generated profile projection
3. remaining weak/record/literal/construction rows + both parser migrations
4. generated support matrix + canonical source migration report
5. bounded grammar-aware differential composition gate
6. LANGV1-GRAMMAR-001 closeout audit
```

Recommended commit boundaries are projection, remaining surfaces, and final
conformance/closeout. A declaration structure-only commit is allowed before
remaining surfaces when the accepted owner requires new modules.

## Objective

Close `LANGV1-GRAMMAR-001` through one structural series that:

```text
1. gives Language v1 one contradiction-free physical registry
2. makes profile completeness a typed source invariant
3. derives parser support from corpus evidence
4. registers every accepted remaining v1 surface
5. compares recursive parser-neutral forms from two independent parsers
6. proves deterministic bounded composition at the milestone gate
```

This is one Refactor Series Mode purpose. Do not mix type-contract,
failure-model, ownership, runtime, backend, or selfhost authority work into it.

## Series A - Physical And Typed Registry Owner

### Structure

```text
grammar/language-v1-registry.toml
  sole Language v1 grammar authority

grammar/legacy/nyash-v1.1-codegen-input.toml
  explicit non-authority input for the current root build.rs consumer

root build.rs
  reads the named legacy input only

Language v1 build/tool consumers
  read language-v1-registry.toml only
```

Do not leave `grammar/unified-grammar.toml` as a forwarding or duplicate
authority. Move every live consumer explicitly, then remove that ambiguous
path.

### Source Schema

One spelling is represented once. Common production/owner fields are not
duplicated. Each source row contains two required typed profile contracts:

```text
row_id
family
spelling_id
production
semantic_owner

canonical:
  status
  normalization_mode
  normalized_shape
  stable_reject_tag
  positive_fixture_ids
  negative_fixture_ids

compat2025:
  same fixed fields
```

The TOML loader fails if either profile contract is missing. After load, a
typed source row cannot represent a missing profile. Rust/Python generated
views may expand each source row into the current `(row_id, profile)` API to
keep consumers stable during migration.

### Acceptance

```text
language_v1_physical_registry_count = 1
legacy_codegen_input_authority = 0
source_row_per_spelling = 1
required_profile_contract_count_per_row = 2
expanded_projection_behavior_preserved = 1
old_unified_grammar_path_remaining = 0
```

## Series B - Evidence Ownership And Recursive Witness

Remove handwritten `rust_support` and `hako_support` fields from authority
rows. Corpus execution produces a generated support matrix keyed by parser,
profile, and row. It is evidence output and cannot select acceptance.

Replace shallow `normalized_children: Vec<String>` with one recursive,
parser-neutral form:

```text
NormalizedSyntaxNode {
  kind
  value?
  children: [NormalizedSyntaxNode]
}
```

The form excludes spans, source paths, parser node names, test counts, and
runtime/backend data. Stable canonical serialization is generated from the
typed tree for corpus comparison and diagnostics. Both adapters remain
independent; parser implementations are never generated.

Transition rule:

```text
add typed recursive form
migrate corpus and both adapters
compare recursive form strictly
remove shallow child-list owner in the same series
```

## Series C - Remaining Surface Rows

### Loop Family

```text
canonical in both profiles:
  loop_infinite -> LoopInfinite
  loop_condition -> LoopCondition
  loop_range -> LoopRange
  break -> Break
  continue -> Continue

while_loop_condition:
  Canonical -> rejected parser/while_legacy_replaced_by_loop_condition
  Compat2025 -> compatibility alias LoopCondition

rejected in both profiles:
  for_loop -> parser/for_loop_noncanonical
  do_while_loop -> parser/do_while_noncanonical
  repeat_loop -> parser/repeat_loop_noncanonical
  until_loop -> parser/until_loop_noncanonical
```

Only a closed `while condition { body }` subset may normalize. Other forms
fail with `parser/while_compat_not_normalizable`. `break` and `continue`
outside a loop are context-verifier failures, not alternate grammar rows.

Migrate canonical-owned `.hako` source from live `while` statements to `loop`
or invoke an explicitly Compat2025 tool entry. Do not add an ambient profile
fallback.

### Weak Family

```text
weak_unary_expr:
  canonical in both profiles -> WeakExpr

weak_paren_expr:
  rejected in both -> parser/weak_paren_call_rejected

weak_stored_field:
  canonical in both -> WeakStoredField

weak_visibility_field:
  Canonical rejected -> parser/weak_visibility_sugar_requires_compat2025
  Compat2025 alias -> WeakStoredFieldWithVisibility

weak_legacy_init_field:
  Canonical rejected -> parser/weak_init_field_legacy
  Compat2025 alias -> WeakStoredField
```

Ownership, identity, upgrade, and finalization semantics remain outside this
grammar card.

### Record Family

```text
record_declaration -> RecordDeclaration
record_literal -> RecordLiteral
record_with_update -> RecordWithUpdate
```

All three are canonical in both profiles. Syntax-invalid fields are negative
fixtures on these rows. Fixed typed fields, default-value restrictions,
methods, weak fields, inheritance, interfaces, and `fini` restrictions belong
to Stage1 semantic verifier tags, not parser authority.

### Literal And Construction Families

Canonical in both profiles:

```text
literal_integer -> IntegerLiteral
literal_float -> FloatLiteral
literal_string -> StringLiteral
literal_bool -> BoolLiteral
literal_null -> NullLiteral
literal_void -> VoidLiteral
array_literal -> ArrayLiteral
map_literal_percent_brace -> MapLiteral
construction_new_box -> NewBoxExpression
```

Rejected in both profiles:

```text
typed_integer_suffix -> parser/typed_integer_suffix_rust_evidence_only
map_literal_legacy_brace_colon -> parser/map_literal_legacy_rejected
```

Array typed context is Stage1 authority. `%{"key" => value}` is the canonical
map spelling. Rust must stop using `NYASH_SYNTAX_SUGAR_LEVEL` or
`NYASH_ENABLE_MAP_LITERAL` as its acceptance owner. Existing environment
variables must be retired or narrowed away from grammar authority according to
the environment-variable SSOT. The one observed typed-suffix app fixture must
be migrated or explicitly parked; do not add a source-name exception.

## Series D - Corpus, Differential Gate, And Closeout

Generate positive/negative fixture indexes and the parser support matrix from
the registry/corpus pipeline. Extend both adapters and compare:

```text
row presence
both profile contracts
accept/reject result
stable reject tag
recursive normalized form
transport exclusion/ownership
```

Add one deterministic grammar-aware composition runner:

```text
seed source = accepted registry fixtures
seed = fixed and reported
depth and case count = explicit finite bounds
Hako execution = one compile-once batch
comparison = recursive Rust/Hako witness
failure = reproducible source + seed + stable drift tag
default quick gate = unchanged
milestone/full grammar gate = composition enabled
```

Do not use wall-clock as semantic success. Record adapter cost separately so a
future performance card can optimize the measured owner.

## Canonical Source Migration Evidence

Initial inventory is migration evidence, not grammar authority:

```text
lang/src percent-brace map literals = 513 occurrences / 65 files
lang/src actual while statements = 13
repository typed integer suffixes in .hako = 1
```

Replace hand-maintained counts with a registry-derived migration report during
implementation. New Canonical-rejected spellings in canonical-owned source
roots fail the full gate. Comments and archived/Compat2025 fixtures are
classified explicitly rather than counted as executable source.

## Fail-Fast Tags

```text
parser/registry_row_missing
parser/profile_mismatch
parser/implicit_compat_retry_forbidden
parser/environment_profile_forbidden
parser/witness_missing
parser/witness_drift
parser/witness_internal_shape_forbidden
parser/literal_surface_unclassified
parser/map_literal_env_gate_forbidden
parser/map_literal_legacy_rejected
parser/typed_integer_suffix_rust_evidence_only
parser/while_legacy_replaced_by_loop_condition
parser/while_compat_not_normalizable
parser/weak_paren_call_rejected
parser/hako_declaration_sidecar_missing
parser/hako_declaration_sidecar_malformed
parser/hako_declaration_sidecar_in_semantic_body_forbidden
parser/hako_declaration_head_fallback_forbidden
parser/hako_record_declaration_misrouted_to_literal
parser/hako_declaration_evidence_to_mir_forbidden
parser/hako_declaration_evidence_to_runtime_forbidden
parser/hako_declaration_evidence_to_backend_forbidden
```

## Forbidden Designs

```text
parallel or forwarding grammar authority
handwritten parser-support truth in registry rows
duplicated common fields across profile twins
one parser AST/JSON used as normalized authority
parser implementation generation or sharing
implicit Canonical-to-Compat retry
environment-selected grammar acceptance
source-name or fixture-specific parser branch
source slicing or reparse fallback
AST rewrite for normalization
warn-only witness drift
runtime/backend fallback
```

## Verification

Each series commit must build. The final commit runs:

```text
registry schema/generation unit tests
Rust grammar adapter and profile tests
Hako compile-once full corpus
strict recursive witness comparator tests
registry-derived canonical source migration report
bounded differential milestone gate
current-state pointer guard
git diff --check
source files remain below 800 lines
```

## Claims After Green Closeout Only

```text
grammar_registry_implemented = 1
remaining_registry_rows_implemented = 1
profile_source_deduplicated = 1
parser_support_evidence_generated = 1
recursive_parse_witness = 1
bounded_differential_composition_gate = 1
current_corpus_exhaustive = 1
language_v1_grammar_closeout = 1
```

## Current Non-Claims

```text
grammar_registry_implemented = 0
remaining_registry_rows_implemented = 0
remaining_surface_parser_behavior_changed = 0
declaration_sidecar_implemented = 0
hako_declaration_conformance = 0
recursive_parse_witness = 0
bounded_differential_composition_gate = 0
language_v1_grammar_closeout = 0
semantic_body_record_declaration = 0
semantic_body_box_declaration = 0
declaration_sidecar_to_mir = 0
declaration_sidecar_to_runtime = 0
declaration_sidecar_to_backend = 0
type_contract_activation = 0
failure_model_change = 0
ownership_policy_change = 0
runtime_backend_changes = 0
selfhost_claim = 0
```

## Next

Implement Series C0 live recursive projection for the current registered
corpus. Keep Rust and Hako projection implementations independent, add
optional node values to the typed/corpus schema, and replace
`ImplementationAccepted`/raw-ProgramJSON equivalence with strict recursive
witness comparison before adding declaration or other remaining surface rows.
