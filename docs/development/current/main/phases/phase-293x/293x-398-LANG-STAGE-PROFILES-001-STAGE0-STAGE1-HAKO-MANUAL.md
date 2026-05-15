# 293x-398 LANG-STAGE-PROFILES-001 Stage0 / Stage1 .hako Manual

Status: landed
Date: 2026-05-15

## Decision

Stage0 and Stage1 must not become separate `.hako` language specifications.
The canonical grammar and topic semantics remain under `docs/reference/language/`.

This row adds a practical profile manual that answers:

```text
what may Stage0 carry today?
what may Stage1 code rely on today?
```

The profile manual is support/status documentation only. It does not accept new
syntax, widen Stage0 semantic ownership, or change backend behavior.

## Scope

- Add `docs/reference/language/stage-profiles.md` as the Stage0 / Stage1 usable
  surface manual.
- Keep `docs/reference/language/EBNF.md` as the canonical grammar owner.
- Update reference indexes so current readers do not follow the historical
  `LANGUAGE_REFERENCE_2025.md` snapshot as the latest spec.
- Deduplicate the broad Stage0 / Stage1 feature table out of
  `low-level-capabilities.md`; that page now points to the profile manual.
- Refresh stale LoopRange status wording after the carrier policy row landed.

## Stop Lines

- No language behavior change.
- No new source syntax.
- No Stage0 semantic checker.
- No Stage1 feature activation.
- No allocator-provider activation, hook, or process allocator replacement.
- Do not change the current blocker; `MIMAP-THREADSAFE-ABI-001` remains next.

## Implementation

- Added `docs/reference/language/stage-profiles.md`.
- Updated `docs/reference/language/README.md` and `docs/reference/README.md`.
- Updated `docs/reference/core-language/README.md` to point at current language
  references and mark `LANGUAGE_REFERENCE_2025.md` historical.
- Updated `docs/reference/language/low-level-capabilities.md` to link the new
  profile manual instead of carrying a duplicate feature matrix.
- Updated `docs/reference/language/quick-reference.md` and
  `docs/reference/language/EBNF.md` LoopRange wording.
- Added `enum_decl` to the top-level EBNF program inventory to match the
  current enum section and type-system reference.

## Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
