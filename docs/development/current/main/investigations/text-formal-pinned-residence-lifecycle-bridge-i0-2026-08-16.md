---
Status: completed caller-zero proof-only implementation row
Date: 2026-08-16
Work mode: fast
Parent: TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-I0

This row proves the exit-side lifecycle relation without changing runtime
residence, MIR, DraftSeal Return placement, or backend lowering. It is the
smallest implementation slice after the accepted lifecycle BoxShape.

## Six-line brief

```text
Decision: issue one private non-Clone PinnedTextResidenceExitObligationV1 for the existing explicit-value Single/ExactTwo exit set; retain the runtime TextFormalCallResidenceV1 token and physical Return writer in their current owners.
Source authority + canonical issuer: VerifiedFunctionCompletionV1 and PreparedFunctionExitSetV1 provide the site-keyed claims; the new proof-only issuer validates one function/plan/frame stamp and exact exit coverage, but issues no runtime or semantic meaning.
Non-authority: block numbers, return ordinals, MIR/JSON metadata, raw slot/generation/token, Completion copies, semantic cleanup, Stage0 policy reimplementation, and any backend or runtime fallback.
Fail-fast boundary: foreign owner/stamp, unsupported implicit/unit exit, empty or duplicate site, missing/foreign exit, mismatched Single/ExactTwo cardinality, non-value claim, or attempted token/Return ownership rejects before any Builder effect.
Smallest next slice: add the private obligation and focused positive/negative tests beside the existing draft-seal exit vocabulary; leave entry acquire, runtime finish, DraftSeal materialization, PinnedTextOp, and GEP/load unopened.
Non-claims: no live frame, pin/rollback/finish execution, lifecycle CFG, Canonical session adoption, typed leaf lowering, production caller, route, performance, fallback/retry, or main integration.
```

## BoxShape

The obligation is created only from a validated existing exit set and carries
private facts equivalent to:

```text
function owner/stamp
plan/frame stamp
explicit-value exit cardinality: 1 or 2
site-keyed exit coverage in Completion order
```

It has no public parts iterator and is not `Clone` or `Copy`. A later lifecycle
materializer may consume it together with the move-only runtime residence, but
this I0 exposes no method that can finish a token or write a Return. The
`PreparedFunctionExitSetV1::try_for_each_exit` order is reused directly; no
block scan, source ordinal inference, or JSON reconstruction is allowed.

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

The proof does not establish that runtime entry happened, that a root pointer
is live, or that every Return has a physical finish. Those belong to the later
residence materializer, which must preserve the order
`operand -> finish -> Return` under the accepted Stage0 no-unwind policy.
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
