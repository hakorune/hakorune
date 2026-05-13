# 293x-256 D201 Language Feature Task Order SSOT

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

Preserve the planned low-level Hakorune language feature set and fix the implementation order without moving semantic ownership into Stage0.

This is a docs-only structure card.
It does not consume the current allocator blocker `M211 purge candidate policy inventory`.

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/language-minimal-surface-ssot.md` | Canonical minimal keyword and surface rule. |
| `docs/development/current/main/design/delegation-no-inheritance-ssot.md` | Canonical no-inheritance and explicit delegation rule. |
| `docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md` | Canonical Stage0 / Stage1 / Stage2-mainline ownership split. |
| `docs/development/current/main/design/language-feature-implementation-order-ssot.md` | Canonical feature inventory and Wave A/B/C task order. |
| `docs/reference/language/low-level-capabilities.md` | Shallow public index linking the future feature map to the design SSOTs. |

## Fixed decisions

Stage0 remains a thin capsule lane.

```text
Stage0 owns:
  parse / metadata / trivial desugar only

Stage0 does not own:
  semantic checking / verifier policy / lowering policy

Retire when:
  Stage1 parser and metadata transport emit the same shape
```

Stage1 owns language meaning:

```text
brand semantics
record layout and lowering
contract and invariant checks
state transition facts
Result / Option prelude behavior
PackedArray eligibility
capability policy
const evaluation
Span / view no-escape
interface conformance
module visibility
proof report object
```

Minimal surface decisions:

```text
while / for / repeat / until:
  not canonical

loop:
  sole repetition family

state:
  not MVP keyword; use enum values plus transition

interface / impl:
  deferred static conformance; prefer delegate field exposes first

inheritance:
  not canonical; legacy from/override/internal extends are quarantine surfaces

cap:
  deferred block syntax; use method-level uses first
```

## Preserved implementation order

| Wave | Meaning |
| --- | --- |
| Wave A | Stage0 thin syntax and metadata capsules: loop-only decision, LoopRange metadata, brand, type, record literal shape, contracts, transition, uses, generic annotation metadata, deferred module header. |
| Wave B | Stage1 semantic nucleus: brand checker, assert, invariant, requires/ensures, enum transition facts, page lifecycle verifier pilot, record literal, with-update, Result/Option, guard-let. |
| Wave C | Stage1 low-level/CorePlan: PackedArray gates, const fn/assert, uses capability checker, Span/view decision, delegation no-inheritance closeout, delegate lowering, deferred interface/impl, module visibility, check report. |

## Closeout note

No code behavior is changed by this card.
Future feature rows must link to the two design SSOTs above and must not duplicate the Stage0/Stage1 split in per-row prose.
