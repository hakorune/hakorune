---
Status: selected cfg(test)-only source witness; production remains stopped
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-selection-disposition-matrix-d3-s1-design-2026-08-05.md
Decision: accepted child task — one parsed V1-only local shape
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-V1ONLY-LOCAL0-D3-S1-S1`
---

# V1-only local source matrix row

## Scope

Add exactly one `cfg(test)` source witness for the natural V1-only local
shape. This is coverage evidence for the D3 disposition matrix; it does not
create a neutral issuer, eligibility capability, selector arm, or production
handoff.

```hako
function generic_v1_only_local(i) {
    loop(i < 3) {
        local tmp = 0
        i = i + 1
    }
    return i
}
```

## Exact source contract

The parsed source must prove, through the existing
`VerifiedResolvedSourceUnitV1` / `FunctionSourceViewV1` path:

```text
function body       = [Loop, Return]
loop body           = [Local, Assignment]
condition           = i < 3
local               = tmp initialized from integer 0
assignment          = i = i + 1
return value        = i
forest               = one root, no children
source kind          = DeclaredFunction
```

The assignment target and post-loop return read must be the same resolver-
issued `BindingRefV1`, with the same function owner/source/frame identity.
The `tmp` local is a lexical body-local binding and must not be treated as a
recursive carrier. It is not the router's `has_body_local` flag: that flag is
reserved for `LoopBreakBodyLocalFacts` TrimSeg/DigitPos break-guard shapes.
No nested loop, `If`, effect call, compound assignment, index write,
extra statement, or hand-built AST source is allowed in this row.

## Facts and mode seal

The same invocation must co-seal:

```text
V0 facts             = false
V1 facts             = true
carrier observation  = CompleteNoRecursiveCarrier
has_body_local       = false
Release raw          = [GenericLoopV1]
Strict raw           = [GenericLoopV1]
planner_required     = false
```

The mode and `has_body_local` flags must come from the returned preflight frame
or equivalent actual frame receipt, not caller booleans, route labels, or a
second environment read. Fresh repeats must issue distinct function owners
while preserving source origin, source kind, frame key, BindingRef slots,
facts booleans, carrier, flags, and raw schedule.

## Typed disposition

The only result for the exact positive witness is:

```text
evidence status       = Observed
selection disposition = UnresolvedStop(V1OnlyNonRecursive)
```

It is not `ResolvedCandidate`, `LegacyPreserveExistingSchedule`,
`ProvenOutsideTarget`, eligibility, winner, or V0 suppression. Facts absent,
V0 present, V1 absent, raw `[]`, Both, simple-while, planner-required, shape
drift, mode drift, or identity drift must be a typed premise reject or
`NoStandaloneRow` and return to the D3-S1 design card. Do not force a V1-only
label from a missing facts product.

## Non-authority and implementation boundary

Carrier strings, route IDs, registry order, plan digests, legacy receipts,
synthetic helper bodies, and runtime results are corroboration only. The test
must not import or modify `registry/selection.rs`, router frame issuance,
`loop_structural_facts` production APIs, Generic composers, Recipe/JoinSig/
PHI/physicalizer, Builder/MIR/backend, Retry, or fallback behavior.

The implementation is one test sibling plus one `cfg(test)` registration at
most. All touched source/check files remain below 800 lines. No production
caller/import may be introduced.

## Acceptance and closeout

Required evidence:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib v1_only_local -- --nocapture
RUSTFLAGS='-Awarnings' cargo test --lib generic_resolved_carrier_ -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check --strict
```

The implementation closeout must update this task, the D3-S1 design card,
parent D3 handoff, Generic post-effect SSOT, stage-matrix reference, Generic
and resolved-semantics READMEs, `CURRENT_STATE.toml`, `10-Now.md`, the active
workstream, affected reference indexes, and the artifact manifest in the same
commit. The implementation-after-reference update is mandatory. The
workstream remains exactly 1000 lines.

## Stop

If the parser/resolver cannot produce the exact `[Loop, Return]` /
`[Local, Assignment]` shape, if V0 facts are present, if raw is not exactly
`[GenericLoopV1]`, if `has_body_local` is not co-sealed as `false`, or if any production
effect appears, stop with the typed premise result and return to D3-S1. Do
not add a new classifier arm, fallback, selector policy, or source workaround.
