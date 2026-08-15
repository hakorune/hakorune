---
Status: accepted audit-hardened TextEq architecture; ingress-surface R0 is closed and the current bounded row is LANG-TEXT-EQUALITY-LAW-D0
Date: 2026-08-15
Work mode: design_stop
Classification: closed T0 BoxShape; current T2 language-law BoxShape; one later T2 common-V2 pre-session family
---

# LOOP-TEXT-EQ-PHYSICAL-CONTRACT-D0

The filename records the S6C discovery point. `TextEq` remains one leaf of
the common recursive Loop Recipe algebra. S6C is its first source adapter,
not a second Recipe family or physicalizer.

## Current six-line brief

```text
Decision: stop before TextEq site/ABI/route work until the language specification names one String/Text equality law and the admitted StringBox-as-Text bridge.
Source authority + canonical issuer: docs/reference/language/types.md §4.3 is the sole semantic-law owner; strings.md remains representation-only, and no runtime implementation may issue the law.
Non-authority: StringBox::equals, eq_vm, UTF-8 implementation details, ordinary BoxRef identity, Recipe/MIR shape, ABI, selector, benchmark, fallback, or retry cannot decide the language law.
Fail-fast boundary: reject the next site-contract/operation-contract row as NoSafeSlice until exact equality, case, normalization, locale/collation, and StringBox-as-Text admission are explicit.
Smallest next slice: LANG-TEXT-EQUALITY-LAW-D0, then a bounded reference-only I0 that updates types.md without changing runtime or compiler behavior.
Non-claims: no TextEq site receipt, physical target, ABI/wire, residence, route admission, Builder/session, production caller, fallback, retry, or legacy retirement.
```

## R0 implementation evidence

The behavior-neutral surface is now implemented in the caller-zero owner:

* semantic validation remains in `with_prephysical_source` and runs only while
  issuing `VerifiedS6CPrephysicalIngressV2`;
* later `with_ingress`, `with_text_eq_leaf`, and `with_completion` projections
  use the retained Facts/Recipe/Join cohort without re-entering validation;
* the retained source seam lives with the logical-output owner, so the output
  no longer imports the ingress module;
* the ingress no longer exposes `logical()`, detached
  `VerifiedLoopSemanticContextV1`, cloned Completion source sites, or a raw
  role-to-item table. Item identity is borrowed from the producer-owned role
  seal, and operation lookup is exact rather than `Option`/zero scrubbing;
* TextEq and Completion are sibling HRTB views. TextEq remains source/Recipe/
  control evidence only; Completion lends the original exact-two exit and Tail
  source rows. No law, ABI, route, residence, Builder, MIR, or session owner is
  issued here.

Focused evidence for this slice:

```text
cargo test --lib -q s6c_                         # 29 passed
cargo check -q                                   # green; inherited warning census unchanged
loop_pre-cutover / loop_physical-transfer guards # green
current-state pointer guard + diff check          # green
```

The ingress owner is 705 lines after the narrowing, below the 760-line design
trigger; a mechanical file split remains parked until a later bounded change
would cross that boundary.

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

tracked route decision
  -> AdmittedLoopTextEqRoutePolicyV1           site-free keeper/policy only

InstalledNormalCallableSemanticPackageV1
  -> NormalCallableSemanticPackagePortV1 target extension
       exact lowering input + parameter ABI
       exact result/header ABI issuer required by the later D0
       retained S6C source binding
       complete common Loop V2 envelope
  -> PreparedS6CPhysicalPackageV1<'loan>
       owns site-free route policy + complete preflight ledger
       borrows the sole source cohort; copies no site/source authority

single common physical session
  -> Ready*Residence<'session>                 actual handle/span/ValueId life
  -> ReadyLoopTextEqExecutionPlanV1<'session>
  -> After + Completion/Tail + DraftSeal
  -> exact Bool or canonical Trap
```

Recipe-local keys are not a global identity. A standalone product containing
only `item/block/left/right/result` would permit re-pairing with another
Recipe. Until a common source-program owner can retain an equally strong
cohort, the S6C adapter owns the original ingress whole and lends only a
neutral per-site view. It has no `Clone`, `into_parts`, `take_ingress`, or raw
Facts/Recipe/Join/Completion getter.

The route policy is deliberately site-free. A package cannot own both the
parent ingress and a detached demand that owns a site borrowed from that same
parent without either a self-reference or a copied key/source ledger. The
installed Normal-callable port therefore lends one scoped cohort; the package
co-borrows its site/corridor/Completion views and one site-free policy only
while preparing the single session.

No S6C physical demand family, physicalizer, CFG/SSA/PHI writer, or Return
writer is introduced. Common physical APIs consume neutral views only.

## Language law prerequisite

The exact equality law is not yet normative text. `types.md` currently says
only that same-kind primitive `String` values compare for equality;
`strings.md` owns valid UTF-8/Text representation and CP-indexed APIs, not
equality. Runtime `String == String` is conformance evidence, never the law
issuer.

`LANG-TEXT-EQUALITY-LAW-D0` and then `LANG-TEXT-EQUALITY-LAW-I0` must land
before any site-contract code. This is
not a wording-only fill. `types.md` currently makes ordinary `BoxRef == BoxRef`
identity-based, `eq_vm` follows that rule, and `StringBox::equals` is only an
implementation helper. D0 must explicitly decide whether the currently mapped
StringBox-as-Text lane belongs to logical Text content equality while
ordinary non-Text BoxRef identity remains unchanged. I0 then makes
`docs/reference/language/types.md` section 4.3 the sole equality-law owner and
adds the matching conformance evidence:

```text
logical String/Text values, including any StringBox-as-Text bridge explicitly
admitted by the same Decision:
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

### LANG-TEXT-EQUALITY-LAW-D0 audit receipt

The open decision is real, not a missing sentence that can be inferred from
the implementation:

* `types.md` §4.3 currently names same-kind primitive `String` equality but
  does not define scalar-vs-byte comparison, case, normalization, locale, or
  collation.
* `strings.md` defines valid UTF-8 and text indexing/representation. It does
  not own equality semantics.
* The S6C typed-input issuer requires two explicitly declared `StringBox`
  parameters and maps them to logical `Text`. That proves the accepted source
  shape, not a language-wide equality law.
* Runtime evidence is split by representation: `eq_vm` compares primitive
  `String` contents but treats ordinary `BoxRef` equality as identity, while
  `StringBox::equals` compares two `StringBox` contents. None of these
  implementations is a specification authority.

The bounded Decision must choose one of these explicit policies before the
site/operation contract opens:

```text
recommended:
  TextEqualityLawV1::ExactUnicodeScalarSequence applies to logical
  String/Text values and to the explicitly admitted StringBox-as-Text bridge;
  comparison is case-sensitive, normalization-free, and locale/collation-free;
  String != String is logical negation; ordinary non-Text BoxRef remains
  identity equality.

alternative:
  StringBox remains ordinary BoxRef identity. Then S6C must gain a separate,
  source-backed conversion/bridge contract before its TextEq can be admitted;
  the current TextEq row cannot silently choose that conversion.
```

Acceptance is one normative owner (`types.md` §4.3), one named bridge
predicate, no co-owned rule in `strings.md`, and negative coverage for
ordinary non-Text BoxRef identity, case/normalization/locale drift, and a
StringBox row that lacks explicit Text admission. Until this Decision is
accepted, `TextEqualityLawV1`, TextEq site receipts, ABI/route/residence
admission, and any Builder/session product remain `NoSafeSlice`.

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

Route selection is reusable policy, not a detached per-site product:

```text
AdmittedLoopTextEqRoutePolicyV1
  RuntimeScalar {
    scalar_keeper,
    required_lifecycle = PublishedHandlePair,
  }
  BorrowedSpanInline {
    borrowed-route keeper,
    required_lifecycle = SessionWideBorrowedTextPair,
  }
```

The policy contains no Loop/item/block/value/source key and owns no corridor.
The scoped S6C package borrows the exact site/corridor from its retained source
cohort and co-checks the policy's lifecycle class before Builder. Actual
published handles, borrowed spans, ValueIds, and borrow end points are issued
inside the sole physical session as `Ready*Residence<'session>`.

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

## Common V2 pre-session boundary

The landed common Loop physicalizer is currently test-only and V1-shaped. Its
operation demand/dispatcher has no V2 Text, `CallSlot`, or `TextEq`, and the V1
layout rejects `If`/`Exit`. The task graph therefore does not jump from a
TextEq leaf policy to a physical session. One T2 family owns that missing
whole-program boundary. Its concrete rows are deliberately separate:

```text
LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
LOOP-S6C-COMMON-V2-PRESESSION-I0
```

```text
Decision: use the installed Normal callable semantic package as the sole outer owner and close exact function ABI, one complete common Loop V2 envelope, and neutral canonical-session admission before any S6C physical effect.
Source authority + canonical issuer: InstalledNormalCallableSemanticPackageV1 owns the installed catalog/batch/selection/parameter cohort; a NormalCallableSemanticPackagePortV1 extension is the only candidate issuer allowed to pair that same callable with retained S6C ingress. Exact source-backed result/header ABI issuance is still missing and is a D0 stop condition, not a landed claim.
Non-authority: Recipe-local keys, a detached TextEq site, benchmark JSON, MIR MirType/EffectMask, the V1 physicalizer, Selected-Dynamic fixed cursor, Builder state, and fixture expectations cannot issue the outer cohort or complete program.
Fail-fast boundary: before Builder/session, reject missing or foreign membership, header/result ABI, ingress identity, V2 operation/source/placement/call/control/transfer/Completion coverage, or route policy; never coerce V2 to V1 or retry another physicalizer.
Smallest next slice: D0 freezes or rejects the installed-port/result-ABI issuer and the envelope shape; only after the common semantic-program and transfer rows may the transport R0 and S6C package I0 land.
Non-claims: no Builder, ValueId, physical block/ID, runtime route activation, residence, production caller, selector switch, S6C physicalizer, or BorrowedSpan implementation.
```

The target ownership is scoped rather than self-referential:

```text
VerifiedNormalCallableSemanticPackageV1
  -> install consumes the verified product
InstalledNormalCallableSemanticPackageV1
  owns the installed source-backed catalog/batch/selection/parameter cohort
  -> NormalCallableSemanticPackagePortV1 target extension lends one cohort
       lowering input + parameter ABI
       result/header ABI only after D0 names its source-backed issuer
       retained S6C ingress/site/corridor/Completion
       complete common Loop V2 envelope

PreparedS6CPhysicalPackageV1<'loan>
  owns site-free route policy + complete preflight ledger
  borrows that one cohort only for immediate session preparation
  -> PreparedLoopOperationProgramV2
       exact 13 operations only
  -> Recipe + JoinSig + Layout-bound control subproduct
       exact If + Exit only
  -> complete envelope coverage receipt
       exact 15 placements = 13 operations + If + Exit
  -> neutral CanonicalSsaFunctionSessionV2 admission
```

`PreparedLoopOperationProgramV2` is a target contract, not a landed claim. It
retains exact source/execution/placement identity for the 13 operation items
only. The one `If`, one `Exit`, and their transfers remain a separate
Recipe/JoinSig/Layout-bound control authority; one passive envelope receipt
proves their union covers all 15 placements. The later D0 must name that
control subproduct rather than smuggle it into the operation program. None of
these targets may become a V2-to-V1 adapter, Selected-Dynamic cursor reuse, or
S6C-specific physicalizer. Until the result/header ABI issuer and those
subproducts are named, implementation is
`NoSafeSlice::MissingS6CCommonV2PreSessionIssuer`. The existing canonical
CFG/Binding-SSA/Phi/Completion/DraftSeal services remain the only physical
writers.

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
11. Delete the ingress-owned raw `role -> LoopItemKeyV1::new(number)` table.
    Project the producer-owned fixed role seal and exact Recipe row instead;
    Recipe keys have one issuer.
12. Replace `anchor_count`/`Option` summaries and hard-coded `1`/`2` values
    with exact borrowed source relations. Counts may remain diagnostics only.
13. Do not construct `VerifiedLoopSemanticContextV1` from detached cloned
    parts. Borrow the retained membership identity or use an issuer-private
    projection that cannot escape as standalone authority.
14. Remove cloned Completion source-site/value fields from the ingress seal.
    Keep only passive parity/count receipts needed by the issuer; sibling views
    borrow the retained Completion/Exit-Tail owner directly.
15. Narrow crate-root re-exports to the terminal issuer/product and sibling
    views required by the next owner. Owned products remain non-Clone,
    non-Default, non-splittable, and caller-zero.

## Corrected bounded DAG

```text
[closed] LOOP-TEXT-EQ-PHYSICAL-CONTRACT-D0
    -> [current] LOOP-TEXT-EQ-INGRESS-SURFACE-R0
    -> LANG-TEXT-EQUALITY-LAW-D0
    -> LANG-TEXT-EQUALITY-LAW-I0
    -> LOOP-TEXT-EQ-SITE-CONTRACT-I0
    -> LOOP-TEXT-EQ-STRICT-SCALAR-PROBE-I0
    -> LOOP-TEXT-EQ-ROUTE-DECISION-R0
    -> LOOP-TEXT-EQ-TRAP-TERMINAL-D0
    -> route-specific demand
       scalar keeper:
         LOOP-TEXT-EQ-SCALAR-LIFECYCLE-D0
         -> LOOP-TEXT-EQ-SCALAR-LIFECYCLE-I0
       scalar reject:
         SUBSTRING-TEXT-EQ-BORROWED-CORRIDOR-D0
         -> SUBSTRING-TEXT-EQ-BORROWED-CORRIDOR-I0
         -> LOOP-TEXT-EQ-SESSION-RESIDENCE-D0
         -> LOOP-TEXT-EQ-SESSION-RESIDENCE-I0
    -> AdmittedLoopTextEqRoutePolicyV1 (site-free)
    -> LOOP-SEMANTIC-PROGRAM-COSEAL-R0
    -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
    -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
    -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
    -> LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
    -> LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
       -> exact 13-operation program
       -> separate Recipe/JoinSig/Layout If+Exit control
       -> exact 15-placement envelope coverage
    -> LOOP-S6C-COMMON-V2-PRESESSION-I0
       -> scoped PreparedS6CPhysicalPackageV1<'loan>
    -> LOOP-PHYSICAL-ALWAYS-COVERAGE-I0
    -> LOOP-PHYSICAL-IF-COVERAGE-I0
    -> LOOP-PHYSICAL-EXIT-COVERAGE-I0
    -> LOOP-COMMON-V2-PHYSICAL-SESSION-I0 (S6C first adapter)
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
reissuing producer roles from raw item numbers or anchor counts
trusting a detached VerifiedLoopSemanticContextV1 built from cloned parts
letting TextEq leaf observe Completion/Tail
storing a parent-borrowed site/corridor inside a detached route demand
issuing residence before ReadyEntry/session
borrowing Dynamic lease/End evidence for S6C
using raw handle equality before live Text resolution
converting BoolOrTrap with !=0 or direct i1 truncation
claiming stale-generation detection from raw u64
claiming LLVM readonly/noalloc from semantic NonFaulting
reading benchmark/env data in Builder or changing route at runtime
selecting borrowed after scalar failure
coercing Loop V2 into V1 or using the Selected-Dynamic fixed cursor
emitting a TextEq leaf before the complete V2 operation/control envelope is sealed
creating an S6C physicalizer, fallback, retry, Fault edge, or second Return writer
```

## Evidence classification

This corrected taskization is based on repository source, six read-only worker
audits, and focused local evidence at `63d96826de`: S6C ingress `2/2`, S6C
Recipe/logical output `8/8`, Loop pre-cutover guard green, physical-transfer
guard green, and a clean worktree. S6C production callers remain zero. At the
same snapshot, `main` is 403 commits behind the working branch; main integration
is not claimed. No law edit, ABI, Trap owner, route keeper, V2 prepared program,
physical caller, benchmark result, latest-HEAD integration, or production
cutover is claimed. Current R0 is the only executable row.
