---
Status: completed prototype; not accepted as the successor lifecycle authority
Date: 2026-08-16
Work mode: fast
Parent: TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-I0

This row records a caller-zero proof-only prototype without changing runtime
residence, MIR, DraftSeal Return placement, or backend lowering. It is retained
as historical evidence, not as the accepted successor lifecycle authority:
the copied per-exit rows must be replaced by a stamp-only finish capability.

## Six-line brief

```text
Decision: retain the prototype only as evidence and replace its copied rows with one private non-Clone PreparedTextFormalExitFinishSetV1 carrying a same-function/plan/frame/residence/session stamp; retain runtime TextFormalCallResidenceV1 and the physical Return writer in their current owners.
Source authority + canonical issuer: VerifiedFunctionCompletionV1 and PreparedFunctionExitSetV1 remain the sole site/block/value ledger; the successor issuer co-seals only the same-owner stamp and issues no runtime or semantic meaning.
Non-authority: copied block/value/site rows, independent exit counts, return ordinals, MIR/JSON metadata, raw slot/generation/token, Completion copies, semantic cleanup, superseded Stage0 prose, and any backend or runtime fallback.
Fail-fast boundary: missing active no-unwind capability, foreign owner/stamp, unsupported implicit/unit exit, detached residence, missing/duplicate/foreign canonical coverage, or attempted token/Return ownership rejects before any Builder effect.
Smallest next slice: design the private stamp-only capability and DraftSeal handoff beside the existing exit set; leave entry acquire, runtime finish, DraftSeal materialization, PinnedTextOp, and GEP/load unopened.
Non-claims: no live frame, pin/rollback/finish execution, lifecycle CFG, Canonical session adoption, typed leaf lowering, production caller, route, performance, fallback/retry, or main integration.
```

## BoxShape

The prototype obligation was created only from a validated existing exit set and
carried private facts equivalent to:

```text
function owner/stamp
plan/frame stamp
explicit-value exit cardinality: 1 or 2
site-keyed exit coverage in Completion order (prototype only)
```

It has no public parts iterator and is not `Clone` or `Copy`. A later lifecycle
materializer must not consume these copied rows. It must consume the corrected
stamp-only capability together with the move-only runtime residence, while
`PreparedFunctionExitSetV1::try_for_each_exit` remains the sole exit iteration;
no block scan, source ordinal inference, or JSON reconstruction is allowed.

## Acceptance

```text
positive:
  explicit-value Single, explicit-value ExactTwo, source-order preservation,
  repeated caller aliases with distinct exit claims

negative:
  implicit/unit, zero claims, three claims, duplicate site, foreign owner,
  foreign plan/frame stamp, missing/foreign exit claim, unit witness,
  token/runtime field request, Clone/Copy escape, second issuer

guard:
  existing Completion/DraftSeal tests remain green, source stays below 800
  lines, and no MIR/JSON/runtime/C consumer changes occur
```

## Non-claims and successor

The prototype does not establish that runtime entry happened, that a root
pointer is live, or that every Return has a physical finish. Those belong to
the later residence materializer, which must preserve the order
`operand -> finish -> Return` under an explicitly issued backend no-unwind
capability.
Failure keeps the parent lifecycle bridge as `NoSafeSlice` and never retries
through a legacy route.

## Implementation receipt

The private obligation was implemented in
`src/mir/builder/resolved_lowering/draft_seal/text_residence_exit.rs`. It
co-seals the existing `ReadyFunctionCompletionV1`,
`PreparedFunctionExitSetV1`, `PinnedTextAccessPlanTableV1` stamp, and scoped
backend-frame revision. The four focused tests cover explicit-value Single,
reverse-ordered ExactTwo site matching, owner/plan/exit-kind rejection, and
duplicate-site/implicit-unit rejection. The obligation has no token, pointer,
JSON, MIR, or Return writer and is not `Clone`/`Copy`.

Evidence: `cargo test --lib mir::builder::resolved_lowering::draft_seal::text_residence_exit`
passed 4/4; `cargo check -q`, `cargo fmt --all`, and `git diff --check` are
green. The next row is a design-only lifecycle materializer decision.
