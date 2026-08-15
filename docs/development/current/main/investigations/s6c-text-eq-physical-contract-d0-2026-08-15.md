---
Status: accepted common TextEq design; current bounded row is ingress-surface R0
Date: 2026-08-15
Work mode: fast
Classification: closed T2 BoxShape followed by one T0 BoxShape
---

# LOOP-TEXT-EQ-PHYSICAL-CONTRACT-D0

The filename records the S6C discovery point.  The accepted contract is not
S6C-owned: `TextEq` remains one leaf of the common recursive Loop Recipe
algebra, and S6C is only its first source adapter.

## Current six-line brief

```text
Decision: split the landed S6C ingress owner and lend one exact TextEq/source plus Completion view; preserve behavior and add no physical target.
Source authority + canonical issuer: VerifiedS6CPrephysicalIngressV2 retains the same Facts/Recipe/Join/Completion product; its existing issuer remains sole owner.
Non-authority: the R0 may not issue TextEq policy, target, ABI, Home, effect, execution plan, ReadyEntry, MIR, or a new semantic receipt.
Fail-fast boundary: a later binder must obtain exact Equal(Text,Text)->Bool source sites, Recipe item/block/value parity, If condition, and retained exits without raw Facts/Recipe/Join escape.
Smallest next slice: LOOP-TEXT-EQ-INGRESS-SURFACE-R0, a behavior-neutral file split plus narrow HRTB views and focused parity tests.
Non-claims: no runtime symbol, performance keeper, Builder/session, CFG/SSA/PHI, production caller, fallback, retry, or legacy retirement.
```

## Accepted D0 decision

The old stop was valid: the repository had logical `Equal(Text,Text)->Bool`
and Recipe `TextEq`, but no neutral physical owner.  The stop is lifted by
accepting two distinct layers and one site-local selection terminal:

```text
normative String equality
  -> VerifiedLoopTextEqOperationContractV1
  -> PreparedLoopTextEqExecutionPlanV1 (exactly one route per site)
  -> common recursive Loop physical session

S6C retained ingress
  -> VerifiedS6CTextEqSourceBindingV1 (temporary first-consumer adapter)
  -> the same common contract and execution-plan terminal
```

`VerifiedLoopTextEqOperationContractV1` owns no symbol and no backend route.
It fixes the already accepted equality law, typed lanes, ownership, semantic
outcome, and the distinction between semantic effects and physical memory
behavior.  `PreparedLoopTextEqExecutionPlanV1` is the sole pre-Builder owner
of the selected implementation route.  One site receives exactly one plan;
runtime fallback, failure-time retry, and target reselection are forbidden.

No new S6C Recipe, physical demand family, physicalizer, CFG writer, SSA/PHI
writer, or Return writer is introduced.  Common physical APIs must not take
an S6C type.  The adapter retains the whole non-Clone ingress and lends a
neutral leaf view to the common physical owner.

## Equality meaning authority

The normative owner is `docs/reference/language/types.md` together with the
String/Text model in `docs/reference/language/strings.md`.  The accepted law
is:

```text
Text x Text -> Bool
exact Unicode-scalar sequence equality
case-sensitive
normalization-free
locale/collation-free
```

For valid UTF-8 Text, byte equality and scalar-sequence equality agree, but
the language-level name is scalar-sequence equality.  The future
`TextEqualityPolicyV1::ExactUnicodeScalarSequence` is a typed projection of
this existing law, not a new semantic choice.  `eq_vm`, `StringBox::equals`,
runtime helper behavior, and a symbol name are evidence only and may not
issue the policy.

## Operation contract

The common contract fixes these externally observable properties:

```text
lhs/rhs: borrowed canonical Text values in their existing Homes
result: immediate canonical Bool
ownership: no move, retain, release, store, rehome, or lease consumption
semantic state: read-only
language suspension/control: none
language Fault lane: none
invalid/stale/non-Text input or malformed outcome: invariant Trap
```

It deliberately does not claim LLVM `readonly`, physical no-allocation, or
lock/cache immutability.  Resolver BodyEffect, CoreMethod `PureRead`, V2
`NonFaulting`, runtime memory effects, and LLVM attributes remain separate
authorities.  In particular, the S6C Facts prove only the exact source
relation/effect closure, while V2 proves the logical `NonFaulting` class.

## Execution routes and performance boundary

The first correctness route is a strict versioned scalar ABI:

```c
int64_t nyrt_text_eq_v1(uint64_t lhs_handle, uint64_t rhs_handle);
```

Its one-word outcome is:

```text
0  = false
1  = true
-1 = invariant violation diagnostic
all values other than 0/1 are rejected by the consumer as canonical Trap
```

The implementation must use strict Text resolution and exact equality.  It
must not call a mutable dispatch hook, read a compatibility route selector,
map invalid handles to empty Text, retry, or fall back.  Panic/unwind may not
cross the C boundary.  Both handles are borrowed for the call only; a live
substring lease remains owned and consumed by its existing End authority.

This ABI is not declared the permanent hot-loop implementation.  The current
AOT pipeline links generated native object code with `libnyash_kernel.a`
without cross-object LTO, so `#[inline(always)]` cannot remove a per-iteration
C call or HostHandle registry lookup.  Contract and HRTB layers cost nothing
at runtime, but the selected route does.

After the caller-zero scalar ABI lands, one evidence row measures it before
S6C production selection:

```text
RuntimeScalarAbiV1
  -> keep only if exact/meso evidence makes call+registry cost non-owning

otherwise
  -> design BorrowedSpanInlineV1 from a new residence/lifetime capability
  -> select it once before Builder/session
```

`BorrowedSpanInlineV1` may remove the steady-state C call, registry lock, and
temporary substring publication, but it is not yet accepted: no residence or
lifetime capability exists.  A later whole-program fusion may consume the
sealed `Substring -> TextEq` cohort, but is parked and may not be inferred by
pattern matching.  There is never a runtime fast-path failure followed by the
scalar ABI; that would be fallback.

## Non-authority census

The following cannot issue the common contract or select its route:

```text
nyash.string.eq_hh
  - mutable forwarding hook
  - compatibility fallback / hook-miss sentinel
  - lossy invalid-handle-to-empty behavior

generic MIR CompareOp::Eq / JoinIR eval_compare
StringBox or CoreMethod names
TextScan contracts
Recipe item ordinal or source spelling
llvmlite / Dynamic evidence
runtime implementation observation
LTO availability
```

The generic Boundary string comparison route is a legacy consumer, not the
new authority.  It is retired only after a later production caller census.

## Required owned products and private views

The bounded design permits these products only:

```text
VerifiedLoopTextEqOperationContractV1
  - common, profile-neutral, no symbol/backend ID

PreparedLoopTextEqExecutionPlanV1
  - consumes the common contract for one site
  - owns exactly one admitted route and its ABI/wire evidence

VerifiedS6CTextEqSourceBindingV1
  - consumes VerifiedS6CPrephysicalIngressV2 by value
  - retains Facts/Recipe/Join/Completion through that ingress
  - binds only source/typed/placement identity to the common contract
```

The S6C adapter may not issue target, Home, ABI, effect, or session meaning.
It has no `Clone`, `into_parts`, `take_ingress`, or raw Facts/Recipe/Join/
Completion getter.  Its HRTB view lends the exact source and common leaf view
only.  When the common source-program issuer can emit this leaf directly and
the adapter caller census reaches zero, the adapter is deleted; the common
contract remains.

## Current R0 contract

`LOOP-TEXT-EQ-INGRESS-SURFACE-R0` is a T0 behavior-neutral BoxShape:

1. Split `s6c_prephysical_ingress.rs` before adding another responsibility:

   ```text
   s6c_prephysical_ingress.rs          facade/model
   s6c_prephysical_ingress/issuer.rs   existing validation/census
   ```

2. Add fixed narrow views for:

   ```text
   exact TextEq source site/lhs/rhs/placement/classes
   Recipe item/block/left/right/result
   TextEq result == exact If condition
   V2 logical NonFaulting classification
   exact Completion target, Loop Return, Tail site/value, cleanup-empty
   existing input/carrier relation needed by the later session
   ```

3. Keep raw output/Facts/Recipe/Join/Completion private.  Move any retained
   source borrow type to its source owner instead of keeping a backward
   logical-output -> ingress dependency.  Narrow crate-wide re-exports.
4. Do not touch the 794-line common `operation_emitter.rs`, the 757-line typed
   V2 schema, or the 753-line output-row file except for minimal module wiring.
5. Positive tests prove the old ingress identity and exact new views.  Negative
   tests cover swapped source operands, wrong placement/result/If condition,
   foreign Completion target, missing Tail value, and raw-escape absence.
6. Extend the existing Loop transfer/pre-cutover guards; new top-level guards
   are zero.  Production callers remain zero.

## Remaining bounded DAG

```text
[closed] LOOP-TEXT-EQ-PHYSICAL-CONTRACT-D0       T2 BoxShape
    -> [current] LOOP-TEXT-EQ-INGRESS-SURFACE-R0 T0 BoxShape
    -> LOOP-TEXT-EQ-OPERATION-CONTRACT-I0         T2 BoxCount
       - named existing semantic policy projection
       - one common non-Clone contract; caller-zero
    -> LOOP-TEXT-EQ-RUNTIME-SCALAR-ABI-I0         T2 BoxCount
       - header/Rust fact/strict Rust export/ABI parity
       - one-word result; hook/env/fallback/retry/out-param = 0
       - physical caller = 0
    -> LOOP-TEXT-EQ-SCALAR-PERF-EVIDENCE-R0       evidence only
       - exact + S6C meso + asm/call/lock/allocation counters
       - choose one route; do not add runtime fallback
    -> if scalar is non-owning: LOOP-S6C-TEXT-EQ-SOURCE-BINDING-I0
       else: LOOP-TEXT-EQ-BORROWED-SPAN-PLAN-D0/I0
             -> LOOP-S6C-TEXT-EQ-SOURCE-BINDING-I0
    -> S6C-PHYSICAL-SESSION-D0
       - one session owner only; no ReadyEntry/topology/op/Tail card family
       - implementation slices: entry/topology, exact cursor, After/Tail/
         Completion/DraftSeal
    -> parity/canary
    -> bounded selector/caller cutover
    -> exact-HEAD integration
    -> legacy caller retirement
```

The scalar ABI I0 is a correctness baseline, not a speed keeper or production
selection.  The performance row records same-handle, equal/unequal short,
empty, ASCII/multibyte, long, and invalid-handle cases plus hit/miss positions
in an S6C meso workload.  It observes calls/iteration, registry reads/locks,
substring publications, retain/release, allocation, instructions/cycles, and
generated loop assembly.

## Stop conditions

Return to design stop before the affected row if any implementation requires:

```text
changing the normative equality law
deriving policy/target/Home/ABI from a symbol, selector, MIR, or item order
reusing nyash.string.eq_hh or any lossy/hooked path
claiming LLVM readonly/noalloc from semantic NonFaulting
splitting or re-pairing Facts/Recipe/Join/Completion
opening ReadyEntry/Builder/session before one execution plan is sealed
using raw handles after their owned borrow/lease lifetime
selecting a second route after runtime failure
adding fallback, retry, or a language Fault edge
creating an S6C physicalizer or second Return writer
```

Missing canonical Trap consumption blocks production selection, not the
caller-zero scalar ABI.  Missing residence/lifetime proof blocks the borrowed
span route; it does not authorize an unsafe inline fast path.

## D0 evidence classification

The D0 decision is based on repository source and read-only worker census.
No runtime ABI, physical caller, benchmark keeper, or latest-HEAD integration
is claimed by this card update.  The next observable work is the R0 only.
