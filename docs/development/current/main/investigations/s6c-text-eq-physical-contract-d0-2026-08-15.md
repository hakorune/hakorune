---
Status: accepted corrected TextEq architecture; current bounded row is ingress-surface R0
Date: 2026-08-15
Work mode: fast
Classification: closed T2 BoxShape followed by one T0 BoxShape
---

# LOOP-TEXT-EQ-PHYSICAL-CONTRACT-D0

The filename records the S6C discovery point. `TextEq` remains one leaf of
the common recursive Loop Recipe algebra. S6C is its first source adapter,
not a second Recipe family or physicalizer.

## Current six-line brief

```text
Decision: split the landed S6C ingress and replace its broad logical escape with independent TextEq, Substring-to-TextEq corridor, Completion, and input/carrier borrow views.
Source authority + canonical issuer: VerifiedS6CPrephysicalIngressV2 retains the same non-Clone Facts/Recipe/Join/Completion product; validation stays in its sole issuer and views only project retained evidence.
Non-authority: R0 issues no equality law, site receipt, ABI, route, Home, residence, lifetime, ReadyEntry, Builder, or session meaning.
Fail-fast boundary: later owners receive exact source/Recipe/control relations without raw constituent escape, source-site cloning, repeated co-seal, or cross-view authority mixing.
Smallest next slice: LOOP-TEXT-EQ-INGRESS-SURFACE-R0, one behavior-neutral file split plus sibling HRTB surfaces and existing-upstream negative proof.
Non-claims: no runtime symbol, performance keeper, CFG/SSA/PHI, production caller, fallback, retry, language Fault, or legacy retirement.
```

## Corrected owner graph

The previous census correctly found no neutral TextEq physical owner. The
accepted direction separates reusable law, one source/Recipe site, physical
carrier demand, route policy, actual session residence, and exits:

```text
types.md String/Text equality law
  -> TextEqualityLawV1                         reusable, site-free, Copy

retained S6C ingress
  -> VerifiedS6CTextEqSourceBindingV1          non-Clone; owns ingress whole
       -> LoopTextEqSiteRefV1                  neutral borrowed site view
       -> SubstringToTextEqCorridorRefV1       sibling producer-consumer view
       -> S6CCompletionRefV1                   sibling exit view

tracked route decision + route-specific lifetime demand
  -> PreparedLoopTextEqRouteDemandV1           exactly one route per site

single common physical session
  -> Ready*Residence<'session>                 actual handle/span/ValueId life
  -> ReadyLoopTextEqExecutionPlanV1<'session>
  -> exact Bool or canonical Trap

PreparedS6CPhysicalPackageV1
  owns the original ingress binding + route demand
  lends TextEq and Completion separately to the single session
```

Recipe-local keys are not a global identity. A standalone product containing
only `item/block/left/right/result` would permit re-pairing with another
Recipe. Until a common source-program owner can retain an equally strong
cohort, the S6C adapter owns the original ingress whole and lends only a
neutral per-site view. It has no `Clone`, `into_parts`, `take_ingress`, or raw
Facts/Recipe/Join/Completion getter.

No S6C physical demand family, physicalizer, CFG/SSA/PHI writer, or Return
writer is introduced. Common physical APIs consume neutral views only.

## Language law prerequisite

The exact equality law is not yet normative text. `types.md` currently says
only that same-kind primitive `String` values compare for equality;
`strings.md` owns valid UTF-8/Text representation and CP-indexed APIs, not
equality. Runtime `String == String` is conformance evidence, never the law
issuer.

`LANG-TEXT-EQUALITY-LAW-R0` must land before any site-contract code. It makes
`docs/reference/language/types.md` section 4.3 the sole equality-law owner:

```text
logical String/Text values, including the accepted StringBox-as-Text bridge:
  exact Unicode scalar-value sequence equality
  case-sensitive
  normalization-free
  locale/collation-free

String != String:
  logical negation of the same equality

ordinary non-Text BoxRef:
  identity equality remains unchanged
```

`strings.md` remains the valid UTF-8 representation owner. For valid UTF-8,
byte equality is a conforming implementation technique because the scalar
sequence has one encoding; it is not the language-level definition.

After that reference Decision, `TextEqualityLawV1::ExactUnicodeScalarSequence`
is a passive typed projection of the law. It is reusable and carries no site,
route, Home, ABI, or lifetime.

## Common TextEq site boundary

`LOOP-TEXT-EQ-SITE-CONTRACT-I0` issues no independent key ledger. Its owned
first-consumer product is:

```text
VerifiedS6CTextEqSourceBindingV1
  owns VerifiedS6CPrephysicalIngressV2 whole
  private seal: law + exact source/Recipe/control parity
  lends LoopTextEqSiteRefV1
```

The neutral site view contains the law plus exact Loop/item/block/lhs/rhs/
result relations. The source relation remains in the parent product. It also
proves `TextEq result == If condition`, but it cannot see Completion/Tail or
issue physical carrier meaning.

The same parent independently lends `SubstringToTextEqCorridorRefV1`, proving
the already-retained relation:

```text
source Substring result == source TextEq lhs
Recipe CallSlot result == Recipe TextEq left
base/start/end/needle and exact placement belong to the same program cohort
```

This is a semantic producer-consumer corridor only. It is not a span,
residence, fusion, lifetime, or physical optimization receipt.

## Strict scalar probe ABI

The first reversible correctness probe uses a one-word `BoolOrTrap` wire:

```c
typedef uint64_t nyrt_text_handle_v1;
typedef int64_t  nyrt_text_eq_outcome_v1;

#define NYRT_TEXT_EQ_FALSE_V1 ((nyrt_text_eq_outcome_v1)0)
#define NYRT_TEXT_EQ_TRUE_V1  ((nyrt_text_eq_outcome_v1)1)
#define NYRT_TEXT_EQ_TRAP_V1  ((nyrt_text_eq_outcome_v1)-1)

nyrt_text_eq_outcome_v1 nyrt_text_eq_v1(
    nyrt_text_handle_v1 lhs,
    nyrt_text_handle_v1 rhs
);
```

The raw outcome is never a Bool value. Every consumer must decode exactly:

```text
0 -> false
1 -> true
all other values -> canonical Trap
```

`icmp != 0`, validation-before-`i1` omission, or raw If-condition use is
forbidden. Both handles must be resolved as live Text before any same-handle
shortcut; two equal invalid IDs are not equal Text.

The strict implementation cannot use `nyash.string.eq_hh`: that path has a
mutable hook, compatibility fallback/hook-miss behavior, and lossy invalid-
handle-to-empty behavior. The new probe has no hook, env selector, fallback,
retry, out-param, retain, release, or language Fault edge.

Raw `u64` cannot detect a slot ID reused for another Text generation. The ABI
therefore does not claim dynamic stale-generation detection. A later physical
lifecycle owner must keep the exact operands live through the call; adding
generation to the wire would be a new D0. Unresolved/non-Text input yields the
trap sentinel. Release builds use `panic=abort`, so internal panic is process
fail-stop and never converted to the sentinel or a language Fault.

The ABI remains caller-zero and reversible until route evidence closes. A
non-keeper must be deleted, made explicit probe/test-only, or retained by a
separate oracle Decision; it does not become a permanent public surface by
default.

## Route, performance, and residence

Contract/HRTB layers cost zero at runtime. The current AOT pipeline does not
perform cross-object LTO between generated code and `libnyash_kernel.a`, so a
per-iteration C call, registry lookup, and lock remain observable.

`LOOP-TEXT-EQ-ROUTE-DECISION-R0` separates measurement from compiler policy:

```text
call-only probe
registry-only pair resolution
full strict TextEq
S6C meso: substring publication + equality + Loop
  -> artifact/target-stamped evidence
  -> tracked versioned keeper decision
  -> compiler-owned fixed route policy
```

Builder/session never reads benchmark JSON, thresholds, or environment.
Typed target admission is added only if multiple target profiles become real.
The evidence records call/lock/publication/allocation counters and generated
assembly; existing approximate counters cannot be called exact without a
focused instrumentation row.

Route and residence obligations form a sum, not independent arguments:

```text
PreparedLoopTextEqRouteDemandV1
  RuntimeScalar {
    site,
    scalar_keeper,
    published-handle lifecycle demand,
  }
  BorrowedSpanInline {
    site,
    borrowed-route keeper,
    substring-to-TextEq corridor,
    session-wide residence demand,
  }
```

Before Builder, these are demands only. Actual published handles, borrowed
spans, ValueIds, and borrow end points are issued inside the sole physical
session as `Ready*Residence<'session>`.

The scalar route still needs a named owner for substring result publication,
handle liveness through TextEq, and cleanup/release. S6C currently owns only
`StringSubstring/2 -> TextToCaller`; it has no existing Dynamic-style lease or
End authority. Dynamic TextScan lease evidence is non-authority here.

The borrowed route is opened only if scalar is rejected or a later explicit
optimization row is authorized. Its issuer consumes the producer-consumer
corridor and proves valid UTF-8 boundaries plus subject/needle lifetime. A
zero-lock plan also requires subject and needle to be resolved/pinned at Loop
entry and closed correctly on early Return and Tail. Runtime `StringSpan` is
a private carrier, not a compile-time lifetime proof. Scalar failure never
retries through the borrowed route.

## Canonical Trap boundary

Boundary code has local `llvm.trap; unreachable` examples, but there is no
common typed TextEq Trap owner. This yields the exact gate:

```text
ingress R0 / law R0 / site contract / caller-zero strict probe: GO

Prepared RuntimeScalar route or production physical session:
  STOP until LOOP-TEXT-EQ-TRAP-TERMINAL-D0 names the exhaustive decoder and
  canonical fail-stop owner; non-0/1 must never become Fault or truthy Bool
```

## Current R0 acceptance

`LOOP-TEXT-EQ-INGRESS-SURFACE-R0` is a T0 behavior-neutral BoxShape:

1. Split the 601-line owner:

   ```text
   s6c_prephysical_ingress.rs          facade/model
   s6c_prephysical_ingress/issuer.rs   current validation/census
   ```

2. Move the retained-source seam from the ingress consumer to its logical-
   output source owner. Make its constructor private/module-local; remove the
   backward logical-output -> ingress dependency.
3. Remove or issuer-private the broad `S6CPrephysicalIngressRefV2::logical()`
   escape and narrow crate-wide re-exports.
4. Replace it with independent sibling projections:

   ```text
   with_text_eq_leaf(...)
   with_substring_text_eq_corridor(...)
   with_completion(...)
   with_inputs_and_carrier(...)
   ```

   TextEq cannot see Completion/Tail; Completion cannot see TextEq.
5. Validation/co-seal runs only in the issuer. Post-issuance views project
   retained product/seal data directly; they do not re-enter Recipe/call/
   transfer verification and issue no new `Verified*`.
6. Completion is a structured borrow view over the retained source authority:
   exact target/count/cleanup, Loop Return site/value, Tail site/value (and
   operand only if needed). Do not create a second cloned source-site ledger.
7. TextEq view borrows exact source site/lhs/rhs/placement/classes, Recipe
   item/block/left/right/result, exact If condition, and logical NonFaulting.
8. Existing upstream source mutations prove swapped operand, wrong placement/
   If, foreign Completion, and wrong Tail rejection. Do not manufacture an
   invalid `VerifiedS6CPrephysicalIngressV2` or repeat meaning checks in views.
9. Add raw-getter absence to existing Loop guards. New top-level guards are
   zero; production caller, ReadyEntry, session, ABI, route, and physical IDs
   remain zero.
10. Do not grow the 794-line operation emitter, 757-line typed schema, or
    753-line output-row owner beyond minimal wiring.

## Corrected bounded DAG

```text
[closed] LOOP-TEXT-EQ-PHYSICAL-CONTRACT-D0
    -> [current] LOOP-TEXT-EQ-INGRESS-SURFACE-R0
    -> LANG-TEXT-EQUALITY-LAW-R0
    -> LOOP-TEXT-EQ-SITE-CONTRACT-I0
    -> LOOP-TEXT-EQ-STRICT-SCALAR-PROBE-I0
    -> LOOP-TEXT-EQ-ROUTE-DECISION-R0
    -> LOOP-TEXT-EQ-TRAP-TERMINAL-D0
    -> route-specific demand
       scalar keeper:
         LOOP-TEXT-EQ-SCALAR-LIFECYCLE-D0/I0
       scalar reject:
         SUBSTRING-TEXT-EQ-BORROWED-CORRIDOR-D0/I0
         + session-wide Text residence D0/I0
    -> PreparedLoopTextEqRouteDemandV1
    -> PreparedS6CPhysicalPackageV1
    -> S6C-PHYSICAL-SESSION-D0/I*
       entry/topology -> exact operation cursor -> After/Completion/Tail/
       DraftSeal under one common session owner
    -> parity/canary
    -> bounded selector/caller cutover
    -> exact-HEAD integration
    -> legacy caller and non-keeper probe retirement
```

The route decision may keep scalar or reject it. It cannot admit an unbuilt,
unmeasured borrowed route. Borrowed residence work is therefore parked until
the decision opens it.

## Stop conditions

Stop before the affected row if implementation requires:

```text
claiming the equality law before types.md lands
turning StringBox-as-Text into ordinary BoxRef identity
detaching Recipe-local keys from the retained program cohort
letting TextEq leaf observe Completion/Tail
issuing residence before ReadyEntry/session
borrowing Dynamic lease/End evidence for S6C
using raw handle equality before live Text resolution
converting BoolOrTrap with !=0 or direct i1 truncation
claiming stale-generation detection from raw u64
claiming LLVM readonly/noalloc from semantic NonFaulting
reading benchmark/env data in Builder or changing route at runtime
selecting borrowed after scalar failure
creating an S6C physicalizer, fallback, retry, Fault edge, or second Return writer
```

## Evidence classification

This corrected taskization is based on repository source and read-only worker
census. No law edit, ABI, Trap owner, route keeper, physical caller, benchmark
result, latest-HEAD integration, or production cutover is claimed. Current R0
is the only executable row.
