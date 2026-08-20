---
Status: Design stop — P0 selected; no implementation yet
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CLAIM-INGRESS-FAILFAST-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-parser-callable-source-handoff-i0-2026-08-20.md
ProductionCaller: none; selected-normal bridge remains disconnected from production
ReplacementCell: distinguish unavailable transport from source-location loss
Classification: BoxShape
Execution row: SCRIPT-DIRECT-STATIC-CALL-CLAIM-INGRESS-FAILFAST-P0
---

# SCRIPT-DIRECT-STATIC-CALL-CLAIM-INGRESS-FAILFAST-D0

## Six-line brief

Decision: Keep the landed selected-normal Script direct-static bridge, but make
claim ingress exhaustive: only an exact ScriptRoot site with no row is
`Absent`; a claimed row is physical; ledger-backed source loss or foreign
lineage is a pre-descent error, never an ordinary-route fallback.

Source authority + canonical issuer: the existing resolver-issued
`ScriptDirectStaticClaimLedgerV1` and the active
`RawInvocationSourceContextV1` transport remain the authorities. This row adds
no source product and does not re-resolve, reparse, or infer a row.

Non-authority: AST names/spans/ordinals, `UnlocatedCompatibility`, missing
context, `ValueId`, `MirType`, pending-row state, the ordinary static handler,
or a successful fallback cannot issue `Absent`, `NonBrand`, or a new target.

Fail-fast boundary: immediately after the already-selected
`MemberCallRoutePlan::StaticReceiver` and before receiver descent, argument
descent, or MIR effects. The route must distinguish `Absent`, `Claimed`, and
`Unavailable`; ledger-backed unlocated/foreign context is an error before any
child effect.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-CLAIM-INGRESS-FAILFAST-P0`.
Use one checked ingress result and an exhaustive member-route match, validate
the observed target/sites before the ledger state transition, and add focused
negative/effect-order tests plus one reusable structural guard.

Non-claims: canonical source-only A, canonical Script transport, source
admission, completion/Return integration, raw/compat retirement, production
switch, ABI/backend changes, and performance evidence.

## Audit evidence

The landed bridge is correctly placed at `MemberCallRoutePlan::StaticReceiver`
and uses the existing ordered argument driver and generic Call receipt. The
remaining gap is in the ingress boundary:

```text
normal_script_direct_static_claim_transport.rs
  ledger absent                         -> Unavailable
  ScriptRoot + exact context            -> Available
  Unlocated/foreign context             -> Unavailable

calls/member_route.rs
  if Claimed -> bridge
  everything else -> existing static route
```

The `script_direct_static_claim_ingress_v1` capability is currently only a
transport query; the route consumes `take_script_direct_static_claim_v1`
without an exhaustive ingress decision. A ledger-backed
`UnlocatedCompatibility` can therefore reach the ordinary static handler,
which may lower receiver/arguments before the pending row is noticed at scope
finish. This is weaker than the accepted fail-fast contract even if the outer
candidate is eventually discarded.

The current claim helper also removes a row into `in_flight` before checking
target, receiver site, argument sites, arity, and `ExactI64`. P0 must use one
checked operation (or borrow-and-validate followed by take) so a rejected
observation is never represented as a claimed physical row. Rollback and
reinsert APIs remain forbidden.

Type-op and reserved routes remain outside this row. The existing
`TYPEOP-DISJOINT-I0` is the prerequisite that the selected direct-static
domain reaches `StaticReceiver` only after effect-free route classification;
P0 does not broaden that domain or alter source admission.

## Required outcome vocabulary

```text
Unavailable
  no Script semantic ledger is installed on this compatibility/test port;
  this row is outside the claim-enabled path and preserves old behavior.

Absent
  an exact ScriptRoot source context is installed, the site is validated, and
  the ledger has no candidate row; only this state may enter the old handler.

Claimed
  an exact row was fully checked and atomically moved to in-flight; it may
  enter the physical bridge and cannot return to the ordinary route.

Error
  a ledger exists but source context is missing, unlocated, foreign, stale,
  or the observed target/site/arity/representation drifts; stop before
  receiver/argument effects and discard the candidate without retry.
```

`Unavailable` is transport capability vocabulary, not a permissive result
from a ledger-enabled port. `Absent` is not synthesized from missing context.
The member route must use an exhaustive `match`; an `if let Claimed` followed
by a catch-all ordinary path is not an accepted implementation.

## Acceptance

Positive:

- exact ScriptRoot + no matching row yields `Absent` and preserves existing
  static-route behavior with no ledger mutation;
- exact ScriptRoot + matching row yields one `Claimed` token and the already
  landed bridge path;
- no-ledger compatibility/test port reports `Unavailable` and remains outside
  this claim path;
- all target/name/arity/receiver/argument-site/representation checks pass
  before the row enters `in_flight`;
- duplicate claim, pending finish, and in-flight finish retain the existing
  linear failure contract;
- focused effects prove that a claimed row emits no ordinary-route receiver or
  argument effects before the claim decision.

Negative:

- ledger + `UnlocatedCompatibility` is a stable freeze error before any child
  effect, not `Unavailable`, `Absent`, or ordinary fallback;
- ledger + missing active context is the same pre-descent error;
- ledger + located non-ScriptRoot/foreign lineage is a pre-descent error;
- source site, owner, target, receiver site, argument cardinality/order, or
  `ExactI64` drift fails before effects and does not reinsert the row;
- argument lowering, Call receipt, or publication failure has no ordinary/raw
  retry and leaves the candidate unpublished/discarded;
- `Unavailable` cannot be returned by the claim ledger itself, and the route
  contains no `if let Claimed` catch-all fallback;
- no second AST matcher, source issuer, target resolver, or physical emitter
  is introduced.

## Structural guard and limits

The reusable Script direct-static guard should assert:

```text
ledger-backed Unlocated/foreign context -> explicit freeze error       = 1
exact ScriptRoot no-row -> ordinary route only                          = 1
member-route Claimed/Absent/Unavailable/Error exhaustive match           = 1
ordinary fallback after Claimed/Error                                    = 0
claim rollback/reinsert/Clone API                                        = 0
second AST MethodCall matcher                                            = 0
new source/Facts/Recipe/Join issuer                                      = 0
new Call emitter or Script publication owner                             = 0
source/check files >= 800 lines                                          = 0
semantic growth of the 760-line transport owner                          = 0
```

P0 may touch only the claim ingress child, the thin member-route match, a
checked-ledger helper, focused tests, and the reusable guard. It must not
change parser/source admission, canonical `compile_script`, the detached
Script recipe path, `ScriptPhysicalExitCommitV1`, raw retirement, or the
production caller set.

## Stop line and next order

Stop at `NoSafeSlice` if an installed ledger cannot distinguish source loss
from no candidate, if a source error can reach argument descent, if checked
validation requires rollback, or if compatibility behavior needs a guessed
non-candidate row. After P0 is green, reopen the separately parked
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0` as a source-authority
design only; do not connect the canonical caller or claim production parity.
