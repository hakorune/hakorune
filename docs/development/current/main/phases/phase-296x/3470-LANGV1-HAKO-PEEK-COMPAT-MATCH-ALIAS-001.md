# 3470 - LANGV1-HAKO-PEEK-COMPAT-MATCH-ALIAS-001

## Status

Active implementation card. Migrate Hako `peek` as a closed Compat2025 alias
to the existing canonical `Match` parser shape.

## Structural Scope

```text
Canonical peek -> stable reject before Peek JSON publication
Compat2025 peek -> existing Match parser body
raw normalized shape -> canonical EnumMatch implementation evidence
ParserPeekBox legacy JSON -> not published by this route
```

Do not duplicate Match parsing. Split the existing Match entry into a thin
keyword wrapper plus one shared post-keyword parser body, then route Compat2025
`peek` to that body.

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

Run focused match/peek fixtures through the bounded adapter, then run the
adapter-health tests, reusable Language v1 grammar guard, current-state pointer
guard, and `git diff --check`. Keep every new source file below 800 lines.

## Next

After Hako peek is green, rerun scoped Rust/Hako witness inventory and decide
whether grammar conformance can formally exclude the still-missing Hako `from`
transport evidence or requires a dedicated transport decision.
