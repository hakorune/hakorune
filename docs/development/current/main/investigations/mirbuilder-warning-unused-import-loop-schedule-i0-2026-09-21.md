---
Status: closed__2026-09-21__WarningUnusedImportLoopSchedule
Task: MIRBUILDER-WARNING-UNUSED-IMPORT-LOOP-SCHEDULE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i1-2026-09-21.md
Implementation permission: true for one private import deletion only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I2
---

# Warning cleanup: raw loop child schedule import

## Six-line brief

```text
Decision: remove the raw-loop production import of
  VerifiedCallableSemanticLoopBindingScheduleV1; the test module already
  imports the type explicitly.
Source authority + canonical issuer: Rust module resolution and the parent
  two-surface warning inventory.
Non-authority: wildcard imports, cargo-fix, warning counts, dead-code owner,
  or any loop route/Recipe meaning.
Fail-fast boundary: any production reference, compile error, new warning, or
  changed failure name rejects the deletion.
Smallest next slice: delete only the production import and rerun both fixed
  quick-profile checks plus fmt/pointer guards.
Non-claims: no loop behavior change, semantic cleanup, suppression, or route
  retirement.
```

## Precondition and acceptance

The parent inventory reports this import only on the lib surface. The test
module uses the same type, so the cleanup moves that test dependency to its
canonical `normal_callable_loop_handoff` path instead of relying on the parent
module's wildcard/re-export surface. A source search finds no production use
in the parent file. Acceptance requires lib and lib-test commands to finish
with no new warning/failure name, with the lib warning count reduced by one,
plus fmt, diff, and pointer guards.

## Execution evidence

The production import was removed and the test module now imports the type
directly from `normal_callable_loop_handoff`. `cargo check --profile quick
--lib -j4` completed with lib warnings 1,843 (down from 1,844), and
`cargo test --profile quick --lib --no-run -j4` completed with lib-test warnings
563. Both exit 0; fmt, diff, and pointer guards are green. No loop behavior or
failure inventory changed.
