# 3470 - LANGV1-HAKO-PEEK-COMPAT-MATCH-ALIAS-001

## Status

Complete. Hako `peek` is a closed Compat2025 alias to the existing canonical
`Match` parser shape.

## Structural Scope

```text
Canonical peek -> stable reject before Peek JSON publication
Compat2025 peek -> existing Match parser body
raw normalized shape -> canonical EnumMatch implementation evidence
ParserPeekBox legacy JSON -> not published by this route
```

Match parsing remains one body and one public parser entry. The entry consumes
either the canonical five-character `match` keyword or the explicit Compat2025
four-character `peek` alias before entering the existing body. A separate
post-keyword helper was rejected because it expanded the Hako compiler call
graph and stalled the existing EnumMatch probe.

## Ordered Work

1. Add a Match parser entry that starts immediately after a recognized keyword.
2. Keep canonical `match` routed through the same body with its five-character
   keyword offset.
3. Canonical `peek` rejects with `parser/hako_peek_canonical_rejected` before
   legacy `Peek` JSON publication.
4. Compat2025 `peek` skips its four-character keyword and enters the shared
   Match parser body.
5. Reject shapes that cannot produce the canonical ordered Match-arm form with
   `parser/hako_peek_compat_not_normalizable`.
6. Preserve source order and one evaluation of the scrutinee and arm bodies.
7. Keep `ParserPeekBox` parked as legacy implementation evidence; it is not
   grammar authority and receives no new semantic route.
8. Extend the existing bounded adapter matrix and reusable grammar guard.

## Focused Fixtures

```text
Canonical peek -> hako_peek_canonical_rejected
Compat2025 normalizable peek -> same raw shape as canonical match
Compat2025 observe-only/legacy peek -> hako_peek_compat_not_normalizable
Canonical match -> unchanged
NYASH_FEATURES variants -> no profile selection
same source/profile twice -> deterministic adapter envelope
```

## Acceptance

```text
hako_peek_profile_seam = 1
hako_peek_compat_match_alias = 1
hako_peek_legacy_json_publication = 0
hako_match_parser_body_count = 1
hako_peek_mismatch_fail_fast = 1
hako_env_profile_authority = 0
hako_implicit_compat_retry = 0
```

## Non-Claims

```text
hako_parse_witness_conformance = 0
hako_from_migrated = 0
hako_from_transport_implemented = 0
parser_sharing = 0
runtime_backend_changes = 0
selfhost_claim = 0
```

## Verification

Verified on 2026-07-10:

```text
python3 -m unittest tools.language_v1.test_hako_adapter_health
Canonical peek -> parser/hako_peek_canonical_rejected
Compat2025 peek -> raw_program_digest 137c666d...ba0dc0
Compat2025 match -> raw_program_digest 137c666d...ba0dc0
Compat2025 observe-only peek -> parser/hako_peek_compat_not_normalizable
```

The reusable guard owns the quick structural checks and an opt-in sequential
full profile matrix. Full Hako probes are deliberately sequential because each
independent selfhost parse takes about 72-74 seconds on this host.

The work also closes an expression-boundary bug: local declarations now
propagate `[freeze:contract]` before constructing Program JSON. The legacy
behavior embedded the rejection token inside malformed JSON.

## Next

3471 is the scoped Hako `from` transport conformance design stop.
