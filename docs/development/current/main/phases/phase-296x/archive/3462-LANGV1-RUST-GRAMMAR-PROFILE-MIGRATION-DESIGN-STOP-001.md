# 3462 - LANGV1-RUST-GRAMMAR-PROFILE-MIGRATION-DESIGN-STOP-001

## Status

Decision accepted. 3461 is complete and pushed as `88b9601d5c`. Implementation
is authorized only by 3463.

## Accepted Decision

```text
grammar profile owner = ParserBuildConfig
profile transport = public parser API plus CLI
default profile = Canonical
Compat2025 = explicit opt-in only
NYASH_FEATURES=no-try-compat = no grammar authority
legacy env retention = temporary explicit-profile conflict detection only
first implementation slice = profile plumbing plus statement try
```

The tokenizer and parser receive the same typed `GrammarProfile`. Neither
layer reads environment state or infers a profile independently. Canonical
rejection never retries Compat2025.

An explicit profile combined with legacy `NYASH_FEATURES=no-try-compat` fails
with `parser/profile_legacy_env_conflict`. Without an explicit profile,
Canonical remains the default and the legacy feature does not select or alter
the profile.

Allowed decision claims:

```text
grammar_profile_owner_parser_build_config = 1
compat2025_public_parser_api_entry_required = 1
compat2025_cli_entry_required = 1
legacy_nyash_features_no_try_compat_not_profile_authority = 1
legacy_no_try_compat_conflict_boundary_required = 1
first_code_slice_try_only = 1
peek_from_migration_deferred = 1
hako_migration_deferred = 1
```

## Established State

3461 landed one physical registry, a generated typed projection, span-free
`ParseWitness`, shared corpus, independent Rust/Hako evidence adapters, a
strict comparator, and a deterministic drift report. The report preserves
current disagreement; it does not activate Canonical or Compat2025.

The accepted grammar contract requires Canonical as the target default and
Compat2025 as explicit opt-in only. The next rollout step is Rust migration,
before independent Hako migration and live strict dual-parser conformance.

## Inventory

Rust currently has no single grammar-profile owner.

```text
tokenizer:
  read_keyword_or_identifier classifies try/from/delegate unconditionally

parser:
  try is enabled by default; only NYASH_FEATURES=no-try-compat rejects it
  from statement and box inheritance routes are live without a profile check
  match is live; legacy peek has no Rust token/parser route

existing config:
  ParserBuildConfig reaches NyashParser after tokenization
  tokenizer does not receive ParserBuildConfig
```

This prevents a safe implementation of the accepted law:

```text
Canonical reject must not implicitly retry Compat2025.
Compat2025 must be selected explicitly at the parser entry boundary.
```

## Decision Required

Choose the Rust grammar-profile owner and entry transport.

### A. ParserBuildConfig profile, passed into tokenizer (recommended)

Add `GrammarProfile` to `ParserBuildConfig`; make every Rust parser entry
construct the tokenizer with that same config. Default config is Canonical.
CLI profile selection is an explicit flag that constructs `ParserBuildConfig`.

```text
source -> ParserBuildConfig { grammar_profile } -> tokenizer -> parser
```

This gives one typed, embedding-safe authority boundary. Existing `NYASH_*`
feature switches remain unrelated legacy controls and cannot silently select
Compat2025.

### B. Process-global environment profile

Use a new environment variable inside tokenizer/parser dispatch.

Rejected unless a consultation supplies a compelling migration constraint:
ambient process state conflicts with explicit opt-in, leaks across tests, and
cannot represent distinct embedded parser calls safely.

### C. Independent tokenizer/parser profile checks

Pass separate profile choices to each layer or infer them from existing feature
flags.

Rejected: it recreates the current drift and makes one parser layer able to
accept a spelling the other cannot classify consistently.

## Recommended First Code Slice After Acceptance

Only profile plumbing and the `try` migration seam:

```text
1. Add typed GrammarProfile to ParserBuildConfig.
2. Pass it to the Rust tokenizer and parser without using environment fallback.
3. Canonical statement try -> parser/try_reserved before effects.
4. Explicit Compat2025 statement try -> existing parser route only for the
   closed normalizable corpus subset.
5. Reject unknown profile/row, profile mismatch, and implicit fallback.
6. Do not change peek/from acceptance in the same slice.
```

This is one profile-owner BoxShape change plus one `try` acceptance seam. It
does not authorize parser sharing or a broad syntax rewrite.

## Consultation Packet

```text
We are at LANGV1-RUST-GRAMMAR-PROFILE-MIGRATION-DESIGN-STOP-001.

Accepted language contract:
- Canonical is the target default.
- Compat2025 is explicit opt-in only.
- registry row plus profile is grammar authority.
- no implicit Canonical -> Compat retry.
- Rust and Hako parsers remain independent.

Observed Rust seam:
- ParserBuildConfig is available only after tokenization.
- tokenizer unconditionally classifies try/from/delegate.
- try defaults to legacy acceptance and only an inverse NYASH_FEATURES switch
  rejects it.
- from routes have no profile gate.

Choose the owner and transport for the Rust grammar profile.

Recommended A:
  GrammarProfile in ParserBuildConfig, passed once to tokenizer and parser;
  Canonical default; CLI constructs an explicit Compat2025 config.

Please decide:
1. Accept or reject A.
2. Whether Compat2025 entry is CLI-only, public parser API plus CLI, or both.
3. Whether existing NYASH_FEATURES=no-try-compat is retired immediately,
   translated only at a named compatibility boundary, or retained temporarily
   as a fail-fast conflict with explicit profile selection.
4. Whether the first code slice may migrate try only while peek/from remain
   closed under the same typed profile boundary.

Return exact claims, non-claims, fail-fast tags, focused fixture matrix, and
the condition for advancing to the next Rust legacy spelling.

Do not authorize Hako changes, parser sharing, implicit environment fallback,
peek normalization, from semantic lowering, runtime/backend changes, type
contract activation, or selfhost migration.
```

## Source Authority

```text
grammar contract = docs/reference/language/grammar-contract.md
registry = grammar/unified-grammar.toml
typed profile-capable entry = ParserBuildConfig
current parser evidence = Rust tokenizer/parser source plus 3461 drift report
```

## Non-Authority

```text
NYASH_FEATURES legacy spelling alone
current Rust acceptance alone
current Hako timeout or acceptance alone
source path, test count, or parser-internal AST shape
```

## Non-Claims

```text
rust_canonical_profile_activated = 0
compat2025_activated = 0
rust_try_migrated = 0
rust_peek_migrated = 0
rust_from_migrated = 0
hako_parser_behavior_changed = 0
live_parse_witness_conformance = 0
parser_sharing = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Next

3463 implements one Rust profile plumbing and `try` seam card. Do not create
spelling-specific rerun cards.
