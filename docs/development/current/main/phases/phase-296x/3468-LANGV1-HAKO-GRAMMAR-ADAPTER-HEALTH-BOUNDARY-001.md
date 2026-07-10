# 3468 - LANGV1-HAKO-GRAMMAR-ADAPTER-HEALTH-BOUNDARY-001

## Status

Active implementation card. Restore bounded deterministic Hako grammar-adapter
execution before any Hako grammar-profile or acceptance change.

## Structural Scope

```text
Hako parser output = raw implementation evidence
health envelope = adapter execution evidence
ParseWitness = not produced or claimed by this card
GrammarProfile = not activated by this card
```

Primary owners:

```text
tools/language_v1/grammar_contract_hako_adapter.hako
tools/language_v1/grammar_contract_drift_report.py
one reusable language-v1 grammar guard
focused health fixtures
```

Do not change `ParserBox.parse_program2` acceptance, statement `try`, `peek`,
or either `from` form while repairing the adapter boundary.

## Ordered Work

1. Separate adapter health mode from grammar observation mode.
2. Give the runner one named timeout configuration; no hidden timeout literal
   and no environment-selected grammar profile.
3. Make each health probe terminate within the configured bound.
4. Emit exactly one structured JSON health envelope on stdout.
5. Keep diagnostics off stdout; contaminated stdout is a hard failure.
6. Normalize timeout, process error, no output, malformed output, and
   nondeterministic output to stable fail-fast tags.
7. Run the same input twice and compare normalized envelopes.
8. Prove `NYASH_FEATURES` does not select a grammar profile or change the health
   envelope's profile fields.
9. Preserve raw Program(JSON) as non-authority evidence only.
10. Keep all new source files below 800 lines and extend the reusable grammar
    guard rather than adding a per-probe shell guard.

## Health Envelope

The exact serialization may follow existing tooling conventions, but its
semantic fields are fixed:

```text
schema = language-v1-hako-adapter-health-v0
adapter_kind = hako_grammar_contract_adapter
bounded = true
deterministic = true
raw_program_json_authority = false
parse_witness_conformance = false
```

The envelope is not ParseWitness and carries no Canonical/Compat2025 acceptance
claim.

## Stable Fail-Fast Tags

```text
parser/hako_adapter_timeout
parser/hako_adapter_process_error
parser/hako_adapter_no_output
parser/hako_adapter_malformed_output
parser/hako_adapter_stdout_contaminated
parser/hako_adapter_non_deterministic_output
parser/hako_adapter_probe_unknown
parser/hako_adapter_health_not_green
parser/hako_env_profile_forbidden
```

Timeout is an adapter-health failure, not grammar rejection or witness drift.

## Focused Fixture Matrix

```text
health ping -> one JSON envelope, bounded success
minimal source -> bounded current-behavior observation
malformed source -> bounded stable failure, never timeout
same source twice -> identical normalized health envelope
mixed log plus JSON -> stdout_contaminated
empty stdout -> no_output
invalid JSON -> malformed_output
nonzero process without structured failure -> process_error
configured deadline exceeded -> timeout
NYASH_FEATURES variants -> no grammar-profile selection
```

Test-only fault injection may be used at the runner boundary when it cannot
enter production parsing or become an environment-selected grammar profile.

## Acceptance

```text
hako_adapter_health_boundary_implemented = 1
hako_adapter_bounded_execution = 1
hako_adapter_deterministic_output = 1
hako_adapter_timeout_fail_fast = 1
hako_adapter_stdout_contract = one_json_envelope
hako_raw_program_json_non_authority = 1
hako_parser_acceptance_changed = 0
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Non-Claims

```text
hako_profile_activation = 0
hako_canonical_profile_activated = 0
hako_compat2025_activated = 0
hako_parse_witness_conformance = 0
hako_try_migrated = 0
hako_peek_migrated = 0
hako_from_migrated = 0
hako_from_transport_implemented = 0
hako_raw_json_as_canonical_authority = 0
parser_sharing = 0
environment_selected_profiles = 0
implicit_compat_fallback = 0
runtime_backend_changes = 0
selfhost_claim = 0
```

## Verification

Run the focused adapter-health tests, the reusable Language v1 grammar guard,
the grammar substrate guard, the current-state pointer guard, and
`git diff --check`. Record the named timeout used by the focused tests.

## Next

Only after this health matrix is green may the same grammar macro row open a
profile-bearing facade plus statement-try seam. `peek` follows after that.
Both Hako `from` forms remain missing evidence until a later accepted transport
decision.
