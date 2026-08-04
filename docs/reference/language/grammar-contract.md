# Hakorune Language v1 Grammar Contract

Status: SSOT
Decision: accepted
Date: 2026-07-10
Decision token: `LANGV1-GRAMMAR-CONTRACT-BASIS-001`

Related:

- `docs/reference/language/semantic-contract-charter.md`
- `docs/reference/language/semantic-kernel.md`
- `docs/reference/language/EBNF.md`
- `grammar/language-v1-registry.toml`
- `grammar/legacy/nyash-v1.1-codegen-input.toml`
- `docs/development/current/main/workstreams/language-v1-convergence-current.md`

## Contract Basis

```text
default profile = Canonical
legacy profile = explicit Compat2025 only
implicit compatibility retry = forbidden
grammar authority = registry row + profile
parser behavior = implementation evidence
parser conformance boundary = span-free ParseWitness
parser implementation count = 2 independent implementations
```

Canonical is the target default immediately. Until both parser migrations land,
any different runtime default is an explicit implementation gap, not a second
language contract.

## Nested If production-profile boundary

The language grammar admits an `if` statement whose bodies contain further
statements, including another `if`. The resolved canonical nested-If recipe
currently proved in production is intentionally narrower: exactly one outer
and one inner explicit-`else` If, one shared trivial binding, pure fallthrough,
and no calls/effects/returns. The recipe/JoinSig/PHI contract does not claim
recursive-depth support beyond that profile; unsupported shapes must remain a
typed pre-effect rejection or use a separately selected route.

## Four-Family Status

The table is the accepted target contract. The registry, generated parser
projections, and both parser witnesses are still physically unsynchronized for
the Result/exception/cleanup/finalization rows after the accepted 2026-08-05 C′
Decision. That drift is tracked in `status-index.md` and is not permission to
treat a target row as live. EBNF/registry/corpus mutation belongs to the
post-implementation `LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0` and Home closeout.

| Spelling | Canonical | Compat2025 | Normalization |
| --- | --- | --- | --- |
| `guard expr else { ... }` | canonical | canonical | `GuardElse` |
| `guard let PAT = EXPR else { ... }` | canonical | canonical | `GuardLetElse`; else requires `NoFallthrough` |
| postfix Result `?` | canonical target / pending sync | canonical target / pending sync | typed `Result<T,E>` inside `Result<U,E>` with exact `E`; no Option/custom Try/implicit conversion |
| postfix `catch` | rejected target / registry retirement pending | rejected target / migration transport only | no `RecoverableFailure` language Outcome |
| standalone `cleanup {}` | canonical target / pending sync | canonical target / pending sync | `CleanupRegistrationV1` into one verified exit transaction |
| local/postfix `cleanup` | rejected target / registry retirement pending | rejected target / migration transport only | migrate to standalone cleanup |
| scope `fini` / `local ... fini` | rejected target / registry retirement pending | rejected target / migration transport only | not the Box-member hook |
| Box-member `fini {}` | canonical target / pending sync | canonical target / pending sync | non-callable terminal Home hook; direct `obj.fini()` rejected |
| statement `try` | reserved and rejected | reserved and rejected | none; historical migration reading is outside language grammar |
| `match` | canonical | canonical | `Match` |
| `peek` | rejected | compatibility_only | lossless closed subset aliases to `Match` |
| `delegate field exposes { ... }` | canonical | canonical | `DelegateExposes` |
| `box Child from Parent` | rejected | compatibility_only | migration transport only |
| `from Parent.method()` | rejected | compatibility_only | migration transport only |

Compatibility transport is not language execution acceptance. It may produce a
migration witness, but it has no semantic owner and cannot enter canonical MIR,
runtime, or backend lowering. Attempted semantic entry fails fast.

Historical source `try` is not a Compat2025 language row. If a migration tool
preserves it, the tool owns a non-semantic record outside both grammar profiles;
it cannot use a parser retry or a compatibility alias to reach canonical AST,
MIR, runtime, or backend lowering.

## Normalization Modes

```text
canonical_shape:
  canonical spelling and canonical semantic shape

compatibility_alias:
  explicit Compat2025 spelling that losslessly normalizes immediately

compatibility_transport:
  migration-tool syntax transport; canonical semantic entry forbidden

none:
  rejected or reserved spelling
```

`peek` is a compatibility alias only when its scrutinee, ordered arms, patterns,
guards, default behavior, binding behavior, evaluation order, and evaluation
count are observationally equivalent to `Match`. Other `peek` forms reject.

Neither accepted `from` form currently has a semantics-preserving proof to
explicit delegation. Both remain transport-only in Compat2025.

## Registry Row

The physical Language v1 source is `grammar/language-v1-registry.toml`. The
legacy codegen input is a separate non-authority file at
`grammar/legacy/nyash-v1.1-codegen-input.toml`; it must not carry v1 contract
rows. A source row owns both fixed profile contracts:

```text
row_id
family
spelling_id
production

canonical:
  status = canonical | compatibility_only | reserved | rejected
  normalization_mode
  normalized_shape
  semantic_owner
  stable_reject_tag
  positive_fixture_ids
  negative_fixture_ids

compat2025:
  same required fields
```

The loader rejects a source row missing either profile contract. Generated
Rust/Python projections expand the source row into `(row_id, profile)` entries
for consumer compatibility. Parser support is corpus evidence generated after
execution, never a field in the authority source.

## ParseWitness

```text
row_id
profile
accepted
normalized_form = recursive parser-neutral syntax tree
stable_reject_tag
```

Witnesses exclude spans, source paths, parser-internal node names, and test
counts. The typed form recursively carries only normalized node kinds and
children; its canonical serialization is comparison evidence, not an AST dump.
`accepted = true` on a compatibility transport row means syntactic migration
transport only; `normalized_form.kind = CompatibilityTransport` prevents
semantic admission.

Rust and Hako adapters are independent projections into this witness. Neither
adapter nor its source AST/JSON is grammar authority.

### Hako corpus execution

The Hako adapter may execute multiple shared-corpus rows in one process so the
merged parser module is compiled once. Batch rows carry explicit profile,
source, and inventory context by index and reuse the same single-row observation
function. The batch runner may select rows, but it must not rewrite source,
translate reject tags, or own grammar expectations; those remain in the shared
corpus. Batched raw ProgramJSON remains non-authority evidence.

## Stable Reject Tags

```text
parser/compat_profile_required
parser/try_reserved
parser/throw_reserved
parser/try_compat_not_normalizable
parser/peek_legacy_replaced_by_match
parser/peek_compat_not_normalizable
parser/match_expected_canonical
parser/hako_record_fields_expected_canonical
parser/enum_match_duplicate_variant
parser/enum_match_non_exhaustive
parser/enum_match_unit_binding
parser/from_inheritance_legacy
parser/from_call_legacy
parser/from_compat_transport_only
parser/guard_expected_canonical
parser/guard_let_no_fallthrough_required
parser/registry_row_missing
parser/witness_missing
parser/witness_drift
parser/stable_reject_tag_missing
parser/profile_mismatch
```

Missing registry rows, witnesses, reject tags, profile agreement, or normalized
shape agreement fail fast. Warn-only drift and Canonical-to-Compat retry are
forbidden.

## Authority Boundary

Authority:

```text
cross-cutting law = semantic-contract-charter.md
evaluation law = semantic-kernel.md
grammar contract = this document
structured grammar rows = grammar/language-v1-registry.toml after row admission
canonical production view = EBNF.md
```

Non-authority:

```text
Rust acceptance alone
Hako acceptance alone
both parsers agreeing without a registry row
legacy docs or training hints
source paths, use counts, route existence, or test counts
parser AST/JSON representation
```

## Rollout Order

1. Land the registry schema, four-family rows, ParseWitness schema, shared
   corpus, two adapters, and strict comparator without changing parser behavior.
2. Migrate Rust Canonical/Compat2025 acceptance to the accepted rows.
3. Migrate Hako Canonical/Compat2025 acceptance independently.
4. Enable strict live dual-parser conformance.
5. Expand the same registry exhaustively across the accepted v1 surface.
6. Generate reference/support views and close `LANGV1-GRAMMAR-001`.

Do not create one card per spelling, fixture, adapter, or rerun.

## Current Non-Claims

```text
grammar_registry_substrate = 1
ownership_grammar_rows = 0
canonical_default_activated = 0
compat2025_activated = 0
live_parse_witness_conformance = 0
parser_behavior_changed = 0
parser_sharing = 0
implicit_compat_fallback = 0
from_to_delegation_normalization = 0
runtime_backend_fallback = 0
type_contract_activation = 0
failure_model_change = 0
selfhost_claim = 0
```
