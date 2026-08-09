---
Status: parked — blocked by `CALLABLE-BODY-SOURCE-AUTHORITY-D0`
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-declared-query-home-aggregate-i0-implementation-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-CONFORMANCE-CATALOG-D0

This parked row will define the separate body-conformance closure for the
declared callable catalog. It must verify complete coverage of declared
contracts without inferring or replacing public Query/Home meaning from body
facts, MIR `EffectMask`, or physical signatures.

The prerequisite source/body authority is not present yet. The parser handoff
is intentionally AST-free and carries declaration/signature/typed-rune rows,
while the Box method declaration catalog does not own a body or function-owner
link. `VerifiedResolvedFunctionV1` is a separate function-body authority and
cannot be paired by name, ordinal, or equal-looking owner. See
`own-home-callable-body-source-d0-design-task-2026-08-09.md`.

It opens only after the declared Query/Home aggregate I0 lands with its
same-brand/site coverage tests **and** a complete branded body-source/body-
facts path exists. It must remain separate from resolver target,
Recipe/CallSlot, Builder/MIR, runtime, provider, fallback, and production
selection until its own source/body authority is designed.
