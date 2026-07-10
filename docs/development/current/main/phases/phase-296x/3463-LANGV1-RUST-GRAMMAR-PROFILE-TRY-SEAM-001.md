# 3463 - LANGV1-RUST-GRAMMAR-PROFILE-TRY-SEAM-001

## Status

Complete. `27f9cf6458` applies the accepted 3462 profile-owner decision to the
Rust parser and migrates statement `try` only.

## Structural Scope

```text
GrammarProfile owner = ParserBuildConfig
transport = public parser API plus CLI
default = Canonical
Compat2025 = explicit opt-in only
first migrated legacy spelling = statement try
```

Pass one typed profile through every Rust tokenize-and-parse entry. Tokenizer
and parser must observe the same value. Do not add a process-global profile or
an environment fallback.

## Ordered Work

1. Add `GrammarProfile::{Canonical, Compat2025}` to `ParserBuildConfig` with a
   Canonical default.
2. Add a tokenizer constructor that receives the profile and preserve the
   existing constructor as a Canonical entry.
3. Ensure all public Rust parse-with-config paths tokenize and parse using the
   same `ParserBuildConfig` profile.
4. Add `--grammar-profile canonical|compat2025` to the CLI and construct the
   same typed config used by the public API.
5. Canonical statement `try` rejects with `parser/try_reserved`.
6. Compat2025 accepts only the closed normalizable statement-try corpus subset;
   other statement-try forms reject with `parser/try_compat_not_normalizable`.
7. Explicit CLI profile plus `NYASH_FEATURES=no-try-compat` fails before parsing
   with `parser/profile_legacy_env_conflict`.
8. Keep `peek`, both `from` forms, Hako parser behavior, runtime, backend, and
   selfhost behavior unchanged.

## Public API Contract

```text
ParserBuildConfig::default().grammar_profile = Canonical
parse_from_string(...) = Canonical
parse_from_string_with_build_config(..., Compat2025) = explicit compatibility
```

No API may silently retry with another profile after rejection.

## Focused Fixture Matrix

```text
default API + statement try -> parser/try_reserved
Canonical API + statement try -> parser/try_reserved
Compat2025 API + normalizable try -> accepted
Compat2025 API + non-normalizable try -> parser/try_compat_not_normalizable
Canonical CLI + statement try -> parser/try_reserved
Compat2025 CLI + normalizable try -> accepted
unknown CLI profile -> parser/profile_unknown
explicit CLI profile + no-try-compat -> parser/profile_legacy_env_conflict
unspecified profile + no-try-compat + try -> parser/try_reserved
Canonical postfix catch/cleanup/fini -> unchanged
peek/from focused baseline -> unchanged
```

## Fail-Fast Tags

```text
parser/profile_unknown
parser/profile_mismatch
parser/profile_required_for_compat
parser/profile_legacy_env_conflict
parser/try_reserved
parser/try_compat_not_normalizable
parser/implicit_compat_retry_forbidden
parser/registry_row_missing
parser/witness_missing
parser/witness_drift
```

## Acceptance

```text
grammar_profile_owner_parser_build_config = 1
rust_grammar_profile_plumbing_implemented = 1
default_profile_canonical = 1
compat2025_public_parser_api_entry = 1
compat2025_cli_entry = 1
rust_try_profile_seam_implemented = 1
legacy_no_try_compat_not_profile_authority = 1
legacy_profile_conflict_fail_fast = 1
implicit_compat_retry = 0
peek_from_behavior_changed = 0
hako_parser_behavior_changed = 0
```

## Non-Claims

```text
rust_peek_migrated = 0
rust_from_migrated = 0
hako_parser_behavior_changed = 0
live_parse_witness_conformance = 0
parser_sharing = 0
peek_normalization = 0
from_semantic_lowering = 0
broad_parser_rewrite = 0
runtime_backend_fallback = 0
type_contract_activation = 0
selfhost_claim = 0
```

## Verification

```bash
bash tools/checks/language_v1_rust_grammar_profile_guard.sh
cargo check
bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

Keep every new source file below 800 lines.

## Next

All acceptance rows are green. 3464 uses the same Rust migration family for
the `peek` compatibility alias. No try-specific rerun card is needed.
