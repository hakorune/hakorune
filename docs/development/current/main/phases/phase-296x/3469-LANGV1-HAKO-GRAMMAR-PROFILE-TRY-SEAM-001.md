# 3469 - LANGV1-HAKO-GRAMMAR-PROFILE-TRY-SEAM-001

## Status

Complete. The accepted explicit per-invocation Hako grammar-profile facade now
owns statement `try` migration.

## Structural Scope

```text
caller -> HakoGrammarProfileFacade(source, profile) -> ParserBox
profile owner = facade invocation
default profile = Canonical
Compat2025 = explicit only
raw Program(JSON) = non-authority evidence
ParseWitness projection = external and deferred
```

Do not store profile authority in `NYASH_FEATURES`, process-global environment,
or independent checks inside statement/parser modules. The facade passes one
closed profile value into one parser invocation.

## Ordered Work

1. Add one small profile facade module with a closed
   `Canonical | Compat2025` representation.
2. Default source-only adapter entry to Canonical.
3. Add an explicit profile-bearing adapter entry; missing/unknown values fail
   before parser execution.
4. Configure the parser context once per invocation; tokenizer/parser decisions
   must not infer profile independently.
5. Canonical statement `try` rejects with `parser/hako_try_reserved`.
6. Compat2025 accepts only the current closed normalizable statement-try subset.
7. Non-normalizable Compat2025 statement `try` rejects with
   `parser/hako_try_compat_not_normalizable`.
8. Canonical postfix `catch`, `cleanup`, and `fini` remain unchanged.
9. No Canonical rejection retries Compat2025.
10. Extend the existing adapter envelope and reusable grammar guard; do not add
    a spelling-specific rerun card or shell guard.

## Stable Fail-Fast Tags

```text
parser/hako_profile_missing
parser/hako_profile_unknown
parser/hako_profile_mismatch
parser/hako_env_profile_forbidden
parser/hako_implicit_compat_retry_forbidden
parser/hako_profile_required_for_compat
parser/hako_try_reserved
parser/hako_try_compat_not_normalizable
```

## Focused Fixtures

```text
unspecified profile + statement try -> hako_try_reserved
Canonical + statement try -> hako_try_reserved
Compat2025 + closed statement try -> raw accepted evidence
Compat2025 + typed/non-closed catch -> hako_try_compat_not_normalizable
Canonical postfix catch/cleanup/fini -> unchanged
unknown profile -> hako_profile_unknown before parser execution
NYASH_FEATURES variants -> no profile selection
Canonical rejection -> no Compat2025 retry
same source/profile twice -> deterministic adapter envelope
```

## Acceptance

```text
hako_grammar_profile_facade_implemented = 1
hako_profile_per_parse_invocation = 1
hako_canonical_default = 1
hako_compat2025_explicit_only = 1
hako_statement_try_profile_seam = 1
hako_try_mismatch_fail_fast = 1
hako_implicit_compat_retry = 0
hako_env_profile_authority = 0
hako_parser_implementation_shared_with_rust = 0
```

## Non-Claims

```text
hako_parse_witness_conformance = 0
hako_peek_migrated = 0
hako_from_migrated = 0
hako_from_transport_implemented = 0
hako_raw_json_as_canonical_authority = 0
runtime_backend_changes = 0
selfhost_claim = 0
```

## Verification

Completed:

```text
python3 -m unittest tools.language_v1.test_hako_adapter_health
bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
LANGV1_HAKO_PROFILE_FULL=1 bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
bash tools/checks/language_v1_rust_grammar_profile_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The full matrix proves Canonical rejection, explicit Compat2025 closed-set
acceptance, typed-catch rejection, and `NYASH_FEATURES=no-try-compat` having no
profile authority. Each full Hako observation completed within the named
90-second bound. Unconditional nested-statement diagnostics were restored to
the existing `NYASH_PARSER_TRACE=1` gate so stdout remains one envelope.

All new source files are below 800 lines. `parser_box.hako` was already above
the cap before this card; this slice adds only the profile mode field and three
small accessors and does not mix a broad BoxShape split into grammar behavior.

## Closeout

```text
hako_grammar_profile_facade_implemented = 1
hako_profile_per_parse_invocation = 1
hako_canonical_default = 1
hako_compat2025_explicit_only = 1
hako_statement_try_profile_seam = 1
hako_try_mismatch_fail_fast = 1
hako_implicit_compat_retry = 0
hako_env_profile_authority = 0
hako_parse_witness_conformance = 0
```

## Next

3470 migrates Hako `peek` as the closed Compat2025 alias to canonical `Match`.
Keep Hako `from` evidence deferred.
