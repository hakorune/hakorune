# DYNAMIC-CARRIER-REBIND-TRANSACTION-D0

Status: revised Decision closed; implementation remains `NoSafeSlice`
Decision: commit-before-end accepted; ingress/current disposition required first
Date: 2026-08-10

## Evidence correction

The complete operator-lifecycle program proves the Normal-only V17 result,
exact I16 `WriteBinding(B0,V17)`, and exact JoinSig Backedge `B0=V17`. It does
not prove that every prior-current B0 carries an end obligation.

The displaced carrier is never V15. I13/V15 is only a borrowed
`ReadBinding(B0)` result used by I15. The prior current is the symbolic current
instance of L0/B0: V1 on ingress, then a prior iteration's forwarded V17.

The first instance is especially important:

```text
plain parameter pos
  -> ordinary Handle demand by the language target
local i = pos
  -> Recipe carrier C0 / B0 / entry V1
```

No live canonical issuer currently binds that normal-callable parameter demand
to the V1/B0 ingress. Recipe `Dynamic`, C0, JoinSig, runtime tags, `MirType`,
ValueId, or the old `ReleaseStrong` cannot manufacture the missing lifecycle
truth. Rebind I0 therefore remains `NoSafeSlice` until the ingress disposition
is sealed.

## Accepted transaction law

The transaction consumes a previously verified current disposition; it does
not guess one:

```text
Current(B0, BorrowedIngressNoEnd)
or
Current(B0, OwnedCarrier(EndExactlyOnceUnlessForwarded))
```

The only accepted chronology is:

```text
1. keep prior-current B0 live
2. evaluate I15 using a borrowed read

I15 Fault:
  V17 absent
  I16 absent
  displaced receipt absent
  B0 unchanged
  original Fault remains primary

I15 Normal:
  V17 is one pending live carrier
  preflight all fallible semantic/physical relations
  atomically and infallibly commit I16:
    current B0 := V17
    V17 lifecycle := Forwarded
    previous current leaves B0 exactly once
    one non-Clone displaced disposition is returned

3. discharge the displaced disposition
   BorrowedIngressNoEnd -> owner-neutral completion; carrier End 0
   OwnedCarrier         -> consume one exact End obligation
4. only successful discharge authorizes the actual Backedge
```

Ending the old current before commit is rejected. A fallible end would leave
B0 naming an ended or partially ended carrier while V17 remained uninstalled.
The atomic commit instead behaves like `mem::replace`: install new and move old
out through one indivisible ownership boundary.

## Cleanup Fault after commit

The commit is never rolled back.

```text
displaced cleanup Fault:
  Backedge = 0
  B0 still names V17
  V17 enters remaining exit teardown
  if no earlier Fault, cleanup Fault becomes primary
  later teardown Faults are suppressed
```

If I15 itself Faults, no commit or displaced disposition exists. Prior-current
B0 remains for ordinary Fault-exit teardown. Primary-Fault selection and
best-effort draining belong to the later exit/cleanup coordinator, not this
rebind relation.

## Atomic type boundary

After ingress lifecycle is available, semantic I0 consumes exactly one whole
input:

```text
VerifiedDynamicOperatorCarrierLifecycleProgramV1
  -> VerifiedDynamicCarrierRebindTransactionProgramV1
```

The private co-seal additionally derives:

```text
sole root carrier C0 / L0 / B0 / Dynamic / entry V1
exact ingress current disposition
I13 ReadBinding(B0) -> V15 and exact StepReadI source
I15 DynamicAdd(left=V15, right=V16) -> V17
I15 Fault cutpoint and canonical operator contract
I16 WriteBinding(B0,V17) and exact source BindingRef/sites
JoinSig Backedge(B0=V17)
```

It seals the state-machine law only. It does not execute a rebind, create a
cleanup token, select an end instruction, or mutate Binding SSA.

The later physical owner may borrow canonical Binding SSA/CFG/PhiTxn services:

```text
fallible prepare, Builder effect 0
  -> private infallible replace commit
  -> move-only displaced disposition
  -> cleanup owner
  -> Backedge authorization or terminal Fault
```

Separate `install_new()` and `take_old()` APIs are forbidden.

## Failure taxonomy

```text
semantic reject / physical prepare failure:
  no mutation; compiler failure or whole unpublished-session discard

I15 source Fault:
  no V17, no I16, no displaced disposition, B0 unchanged

post-commit displaced cleanup Fault:
  no Backedge, no rollback, V17 remains current for exit teardown

later teardown Fault after an existing primary:
  suppressed diagnostic; best-effort teardown continues
```

Compiler/session failure must never be represented as a source Dynamic Fault.

## Required tests

Positive:

- first-iteration borrowed ingress and later owned replacement are distinct;
- I15 Fault leaves both dispositions unchanged and issues no replacement;
- Normal commit forwards V17 and moves the prior current exactly once;
- borrowed ingress displacement emits no carrier End;
- owned displacement emits exactly one end obligation;
- Backedge is authorized only after successful discharge;
- fresh-session repetition produces the same semantic chronology.

Negative/guards:

- missing/foreign/duplicate ingress disposition;
- treating V15 or ValueKey last use as the displaced carrier;
- wrong I13/I15/I16/binding/source/Backedge/Fault relation;
- end-before-commit, split install/take-old APIs, double install/end/consume;
- V17 publication or displaced disposition on I15 Fault;
- V17 both forwarded and ended;
- Backedge before discharge or after cleanup Fault;
- rollback/retry/fallback after cleanup Fault;
- caller-supplied key/site/contract/Completion/ValueId;
- `ReleaseStrong`, `DestroyOwned`, Home, runtime tag, provider, or `MirType`
  inference in the semantic module.

## Owner and retirement table

| Meaning | Owner |
| --- | --- |
| V17 result/Fault/lifecycle | canonical Dynamic operator contract |
| V17 -> I16 -> B0/Backedge | complete semantic-program co-seal |
| initial V1/B0 current disposition | next ingress lifecycle issuer |
| per-iteration current/displaced transition | carrier rebind transaction/flow |
| source Binding assignment | canonical Binding SSA |
| primary Fault and cleanup drain | later exit/cleanup coordinator |
| physical opaque-carrier end | later physical lifecycle projection |

The old Dynamic origin map, route-specific BinOp/rebind/PHI wrapper, and
pre-rebind `ReleaseStrong` are migration evidence only. Canonical Binding SSA,
CFG, and PhiTxn remain reusable physical services; the route-local current map
must not become a second semantic owner.

## Corrected task order

```text
1. DYNAMIC-CARRIER-INGRESS-LIFECYCLE-D0
2. DYNAMIC-CARRIER-INGRESS-LIFECYCLE-I0
3. DYNAMIC-CARRIER-REBIND-TRANSACTION-I0
4. DYNAMIC-CARRIER-FLOW-D0/I0
5. DYNAMIC-EXIT-CLEANUP-PLAN-I0
6. cleanup-capable Completion / exit transaction
7. physical prepare/commit/end
8. route-specific legacy retirement
```

## File split target

```text
semantic_program/carrier_ingress/
  mod.rs model.rs issuer.rs tests/

semantic_program/carrier_rebind/
  mod.rs model.rs issuer.rs tests/

resolved_lowering/dynamic_carrier_rebind/
  state.rs prepare.rs commit.rs tests/
```

Split at 650-700 lines, stop adding at 760, and keep every source below 800.

## Nonclaims

No rebind I0, carrier flow, end operation, Home, cleanup execution, Completion,
CFG/MIR/PHI, runtime/provider route, retry, fallback, or production activation
is opened by this Decision.
