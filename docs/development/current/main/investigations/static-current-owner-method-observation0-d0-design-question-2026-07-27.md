---
Status: design consultation required
Date: 2026-07-27
Decision: STATIC-CURRENT-OWNER-METHOD-OBSERVATION0-D0
ceremony_tier: T2
Blocks:
  - PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION-CORRESPONDENCE0-P0
---

# Static current-owner MethodCall observation

## Question

Choose the single source authority for `me.method(...)` inside an ordinary
method declared by a `static box`.

This is a source-observation decision only. It does not change Builder, MIR,
runtime, parser grammar, callable result semantics, or production routing.

## Correspondence evidence

The actual Parser Stage-B source currently closes:

```text
ParserBox.static_const_parse_add/2 outer static targets = 2
complete Stage-B candidates                            = 0
first unavailable stage                               = WholeSourceMethodObservation
```

The exact missing chain is:

```text
StringHelpers.to_i64/1
  Body(12).LoopBody(2).Initializer(0)
  me._digit_value(ch)
        ↓ missing current-owner static target
StringHelpers.to_i64 result
  = Unavailable(StaticCallTargetAuthorityUnavailable)
        ↓
ParserBox.static_const_eval_pos/1 result
  = Unavailable(StaticCallResultUnavailable)
        ↓
nested Integer contract = unavailable
        ↓
pre-loop candidate count = 0
```

The existing fixture had to add exactly this `_digit_value/1` target manually.
The production inventory already has the correct current-owner target sealer;
only its sole shadow traversal fails before it can issue the row.

## Root mismatch

Three catalog-backed consumers independently map callable namespace to source
receiver policy:

```text
source_call_target/whole_source_inventory.rs
source_call_target/qualified_receiver_lexical.rs
callable_result_representation/activation.rs
```

All currently map:

```text
StaticBoxMethod   -> ReceiverPolicyV1::Absent
InstanceBoxMethod -> ReceiverPolicyV1::DeclaredInstance
```

For a `static box`, an ordinary method is not a receiver-less `static`
function. Source `me` denotes the current singleton owner. Treating it as
absent loses valid current-owner MethodCall observations.

## Options

### A-prime — observation-only static current-owner context

Recommended.

Add a source-neutral policy vocabulary such as:

```rust
pub(in crate::mir) enum ReceiverPolicyV1 {
    Absent,
    DeclaredInstance,
    StaticCurrentOwner,
}
```

`StaticCurrentOwner` is issued only from the verified declaration-catalog
namespace. In the sole shadow traversal:

```text
me.method(...) receiver
  -> existing ShadowMethodCallReceiverV0::CurrentOwner
```

It does not create a lexical `ShadowBindingKindV0::Receiver`. Bare `me` in
other expression roles stays rejected unless a separate language decision
admits it.

Extract one shared namespace-to-source-receiver-policy authority and migrate
the three consumers to it.

### B — treat static current owner as DeclaredInstance

Reject.

This would make static-box methods look like instance methods, create a lexical
receiver binding, and widen every `me` use rather than only the MethodCall
receiver observation required by the source contract.

### C — inject the one missing target

Reject.

Supplying `StringHelpers.to_i64/1`, `_digit_value/1`, or its source ordinal
manually would be by-name/by-site policy and a second target authority.

### D — add a second tolerant AST walker

Reject.

The existing shadow traversal and SourcePath vocabulary are the observation
SSOT. A second walker would duplicate child order, source sites, and error
classification.

## Recommended owner chain

```text
VerifiedSameModuleCallableDeclarationCatalogV1
  + CanonicalSameModuleCallableKeyV1.namespace
        ↓ sole policy
SameModuleCallableSourceReceiverPolicyV1
  ├─ StaticBoxMethod   -> StaticCurrentOwner
  └─ InstanceBoxMethod -> DeclaredInstance
        ↓
FunctionSyntaxViewV1
        ↓ sole shadow traversal
ShadowMethodCallReceiverV0::CurrentOwner
        ↓ existing exact source call product
VerifiedSourceMethodCallSiteV1
        ↓ existing current-owner target sealer
VerifiedSourceStaticCallTargetV1
        ↓ existing callable-result proof
StringHelpers.to_i64 = ExactI64
        ↓
ParserBox.static_const_eval_pos = ExactI64
        ↓
existing nested/outer Stage-B contracts
```

## Failure law

```text
catalog namespace mismatch
  -> typed pre-Builder rejection

StaticCurrentOwner outside catalog-backed view
  -> constructor unavailable

bare/non-receiver me under StaticCurrentOwner
  -> existing bounded source-observation rejection

duplicate/coverage/path correspondence failure
  -> typed rejection, never proof-unavailable

unsupported unrelated caller
  -> exact first unavailable caller/cause retained
  -> later callers still observed

fallback / retry / manual target injection
  -> 0
```

## Executable series after acceptance

```text
STATIC-CURRENT-OWNER-METHOD-OBSERVATION0-D0-CLOSEOUT

-> SAME-MODULE-CALLABLE-RECEIVER-POLICY0-S0
   one shared catalog namespace policy
   three duplicated mappings removed
   behavior delta = 0 until traversal consumer lands

-> STATIC-CURRENT-OWNER-METHOD-OBSERVATION0-S0
   observation-only StaticCurrentOwner context
   sole shadow traversal emits existing CurrentOwner row

-> STATIC-CURRENT-OWNER-METHOD-OBSERVATION0-P0
   static-box current-owner MethodCall positive fixture
   bare me / true static function / lambda negative fixtures
   instance-method parity

-> PRELOOP-STAGEB-STATIC-CURRENT-OWNER-TARGET0-P0
   existing target sealer yields exact `_digit_value/1`
   no new target factory

-> PRELOOP-STAGEB-SOURCE-INVENTORY0-P0b
   StringHelpers.to_i64 = ExactI64
   ParserBox.static_const_eval_pos = ExactI64
   actual Parser complete candidate count = 1
   selected row = Body(3), loop-refresh remains parked

-> PRELOOP-STAGEB-SOURCE-INVENTORY0-G0
   policy producer = 1
   traversal = existing 1
   second walker / by-name / ordinal / manual target = 0

-> PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION-CORRESPONDENCE0-P0
-> PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-D0
-> PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-S0-A/B/C
-> PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-P0/G0
-> PRELOOP-STAGEB-LEGACY-WHOLE-SOURCE-REQUEST0-I0
-> PRELOOP-STAGEB-LEGACY-ORDINARY0-P0
-> PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-I0/P0/G0
-> CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
-> PRELOOP-INNER-TYPE-PROOF-CENSUS0-P0
```

The candidate-session D0 recommendation already established by read-only
audit is:

```text
brand-free isolated Builder candidate core
+
selected-Legacy outer transaction
```

It remains parked until the actual source selector produces one exact
candidate.

## Structural gate

```text
namespace -> source receiver policy authority       = 1
StaticCurrentOwner producer                         = catalog-backed exact 1
sole shadow traversal                               = existing 1
current-owner target sealer                         = existing 1

static method lexical instance binding              = 0
second AST walker                                   = 0
manual production target row                        = 0
callee / owner name selection                       = 0
source ordinal selection                            = 0

Builder / MIR / runtime / backend delta             = 0
compile_request production consumer                 = 0
fallback / retry                                    = 0

new/modified source and check files                 < 800 lines
```

## Non-claims

```text
general bare `me` support in static-box methods
instance receiver semantics change
static function receiver support
parser or language grammar change
Builder source-site registry
new callable-result inference
production Stage-B activation
candidate Builder session implementation
loop-refresh activation
Alias / View language work
VM / LLVM / backend change
```

## Required answer

```text
Decision:
  STATIC-CURRENT-OWNER-METHOD-OBSERVATION0-prime-r1

Status:
  accepted | revised | rejected

Choice:
  A-prime | B | C | D

If A-prime:
  static current owner is observation-only
  no lexical instance receiver binding
  catalog namespace is the sole producer
  first executable row =
    SAME-MODULE-CALLABLE-RECEIVER-POLICY0-S0
```
