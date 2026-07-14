---
Status: Closed
Date: 2026-07-15
Decision: SSA-I1-COMPAT-P0a — static exact-i64 parameter ingress
Next blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-NEXT-ROW-SELECTION-DESIGN-STOP-001
Related:
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - mirbuilder-ssa-i1-compat-explicit-void-implementation-2026-07-15.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../phases/phase-296x/3481-LANGV1-TYPE-GUARANTEE-PARAMETER-ENTRY-DESIGN-STOP-001.md
  - ../phases/phase-296x/archive/3482-LANGV1-TYPE-GUARANTEE-PARAMETER-ENTRY-EXACT-NUMERIC-CONTRACT-001.md
---

# SSA-I1-COMPAT-P0a Static Exact-i64 Parameter Selection

## Decision

Select one parameter-ingress row:

```text
owner family:
  static function only

source parameter spelling:
  exact `i64` only

parameter cardinality:
  one or more
  every explicit parameter is typed `i64`

Binding SSA representation:
  InlineI64

runtime entry owner:
  existing FunctionEntryContractOwner

first supported backend:
  Rust MIR interpreter only
```

P0a adds no parameter opcode, runtime type tag, Ownership SSA operation,
caller-side check, or new backend ABI. It connects an already verified source
parameter identity and an already reserved formal ValueId to the existing
function-owned Binding SSA.

## Why exact `i64` only

Three read-only audits converge on this boundary.

```text
i64:
  existing source-to-MIR mapping = MirType::Integer
  existing ParameterEntryContractKind::ExactNumeric
  existing final-callee VM validation
  existing MIR JSON carrier
  existing unsupported-backend preflight

bool / f64:
  trivial MIR representation exists
  final-callee parameter contract does not exist

usize and other fixed-width integers:
  exact-numeric semantic contract exists
  current source-type representation mapper is not yet the complete authority

receiver:
  BoxRef identity, formal-index offset, method reroute, and ownership remain
  a separate owner-family decision
```

Admitting Bool or Float would mix a new runtime ABI with Binding SSA ingress.
Admitting `usize` by collapsing it to `i64` would violate the target-width
unsigned contract. Admitting the receiver would mix trivial parameter ingress
with Box ownership.

## Corpus evidence and claim limit

The census is intentionally recorded before implementation:

```text
lang/src typed-parameter declarations:
  static owner lines:                         48
  instance owner lines:                      367
  static i64 occurrences:                      7
  all-parameter exact-i64 static candidates:   0

apps:
  all-parameter exact-i64 static lines:      228
  files:                                      53
  strict P0a candidates:                       0
  reason: every candidate also has a nontrivial typed return

tests / tools/smokes / examples:
  strict P0a candidates:                       0
```

Therefore P0a is a production infrastructure proof, not a selfhost coverage
claim. It is still the required smallest proof before the high-volume
instance/receiver family can reuse the same formal-parameter ingress.

## Exact accepted grammar

```text
root:
  FunctionDeclaration
  is_static = true
  name != main
  is_override = false
  uses/contracts/attrs = empty

signature:
  params.len > 0
  params.len == param_decls.len
  every ParamDecl name matches params at the same index
  every declared_type_name == Some("i64")
  return_type_name == None

body:
  current SSA-I1-T grammar only
  local / assignment / BlockExpr
  fallthrough statement If
  final Return or implicit completion
  no Call / MethodCall activation
```

All-untyped parameter owners remain on the current whole-unit A+ route.
Mixed typed/untyped parameters and unsupported typed parameters fail before
Builder effects. A selected P0a owner never retries A+.

## Authority split

```text
canonical ParamDecl:
  exact source spelling, name, order, and cardinality

VerifiedResolvedFunctionV1:
  Parameter(index) -> exact BindingRef / BindingKind::Parameter

resolved trivial parameter classifier:
  the one P0a acceptance policy

VerifiedTrivialCanonicalOwnerV1:
  sealed parameter-entry rows
  Parameter BindingRef -> InlineI64 declaration profile and coverage

MirFunction:
  reserved formal ValueIds %0..%N-1

FunctionEntryContractOwner:
  exact arity and runtime i64 validation at the final callee

BindingSsaBuilderV1:
  reaching ValueId and PHI authority after entry definition

Lower:
  consumes the sealed rows and connects them
```

Non-authorities:

```text
MirType alone
parameter name
call-site facts
runtime tag inference
legacy name map
encounter-order allocation
```

The sealed parameter row is explicit rather than inferred from a generic
definition during Lower:

```rust
pub struct VerifiedTrivialParameterEntryV1 {
    site: SourceBindingSiteV1,
    binding: BindingRefV1,
    formal_index: u32,
    source_name: String,
    declared_type_name: String, // exact "i64"
    representation: TrivialRepresentationV1, // InlineI64
}
```

The generic declaration definition row remains the exact-once profile coverage
subject. The parameter-entry row seals the ABI facts needed by the Builder and
prevents it from rereading or reclassifying raw source annotations.

## Exact entry algorithm

Before lowering the root body, for every source parameter in formal order:

```text
1. consume the next sealed parameter-entry row
2. claim its Parameter(index) declaration profile
3. require representation == InlineI64
4. read function.params[index]
5. require value == reserved ValueId(index)
6. require function.signature.params[index] == MirType::Integer
7. adopt the row's exact Parameter BindingRef
8. BindingSsaBuilder.define(binding, entry_block, reserved_value)
9. register MirValueKind::Parameter(index)
10. publish exact type/slot observation metadata
```

Fresh ValueId allocation for parameters is zero. No second
`BindingRef -> ValueId` map is introduced.

Parameter assignment after entry is an ordinary Binding SSA definition.
Parameter reads in If/BlockExpr use the same demand-driven PHI construction as
locals.

## Existing runtime contract reuse

P0a must reuse the already landed Language-v1 contract unchanged:

```text
ParameterEntryContractKind::ExactNumeric
exact source/formal index and reserved ValueId drift validation
final-callee validation after reroute
validation before register binding and body effects
exact arity for contracted functions
MIR JSON transport
mir-interpreter support
non-VM backend fail-fast
implicit receiver exclusion
```

P0a only proves that the canonical Binding-SSA route preserves the exact
`MirParamDecl`, so normal semantic refresh creates the existing contract row.
It does not create a second entry checker.

## Implementation order

Use one Refactor Series Mode objective with activation only in the final
commit.

### P0a-L0 — behavior-neutral exact parameter ABI facade

```text
one ExactTrivialParameterAbiV1::I64 classifier
exact "i64" -> InlineI64 -> MirType::Integer
legacy broad type mapper delegates only its exact i64 branch to this helper
passive sealed-row -> MirParamDecl installer with production callers zero
behavior/route/profile delta = 0
```

Each commit remains buildable. No accepted grammar changes in L0.

### P0a-S0 — disconnected exact parameter profile

```text
one exact i64 parameter classifier
VerifiedTrivialParameterEntryV1 rows
parameter declaration profile rows
parameter-first deterministic coverage
environment seed before body analysis
capability production activation = 0
Builder connection = 0
```

S0 retained the typed-signature capability veto until the atomic I1 commit.

### P0a-I1 — atomic production activation

```text
whole-unit capability selection
sealed parameter-entry -> exact MirParamDecl transport
reserved formal ValueId entry seed
Binding SSA publication
existing entry-contract carrier proof
VM/reference execution
unsupported-backend preflight proof
function publication gate
```

Only I1 changes production routing.

## Implementation result

P0a-I1 is closed as one atomic production activation:

```text
pre-Builder:
  exact typed parameters reach the sealed trivial profile
  unsupported/mixed typed parameters reject before Builder effects
  all-untyped owners retain the current whole-unit A+ disposition

Builder:
  sealed parameter rows install exact MirParamDecl metadata
  reserved formal ValueIds %0..%N-1 are adopted without allocation
  each exact Parameter BindingRef becomes an entry Binding SSA definition
  parameter reassignment and If merge use the same Binding SSA owner

runtime/backend:
  existing ParameterEntryContract rows refresh before draft verification
  final-callee i64 type and exact-arity checks run in the Rust interpreter
  non-VM backends fail through the existing shared capability gate
```

Production counters are fixed at:

```text
parameter fresh ValueId allocation = 0
canonical parameter legacy-name lookup = 0
typed unsupported A+ fallback = 0
source Call/MethodCall activation = 0
receiver activation = 0
Ownership SSA/opcode activation = 0
selected-route legacy RC insertion = 0
```

Focused evidence is green:

```text
resolved-lowering family:              94/94
P0a production/runtime fixtures:         5/5
resolved value profile:                 18/18
final-callee parameter contracts:         8/8
parameter backend capability:             2/2
parameter-entry filtered fixtures:        9/9
authority guard:                         green
```

The public guard also received three behavior-neutral repairs for already
landed structure: the canonical plan is an enum, nested resolved-lowering
boxes are legitimate located-carrier consumers, and historical fixture counts
are lower bounds rather than an exact ban on later tests.

## First runtime claim

The first executable fixture is direct function execution, not source-call
lowering:

```hako
static identity(x: i64) {
    return x
}
```

The compiler may execute the generated `identity/1` MIR through the Rust MIR
interpreter with an explicit argument vector. Source `Call` and `MethodCall`
remain outside the closed expression grammar.

## Required fixtures

Positive:

```text
one and multiple exact i64 parameters
reserved %0..%N-1 and first fresh ValueId non-overlap
parameter read and final return
parameter reassignment
same-name local shadow restoration
BlockExpr parameter read/rebind
one-sided and two-sided If parameter rebind
post-If demand-driven parameter PHI
exact MirParamDecl transport
one ParameterEntryContract row per parameter
valid Integer runtime arguments
verified MIR and VM/reference result
```

Negative:

```text
untyped parameter stays whole-unit A+
typed/untyped mixed signature rejects
usize/bool/f64/String/Box parameter rejects
typed return rejects
instance receiver rejects
param-decl cardinality/name/order drift rejects
missing/duplicate/foreign parameter profile rejects
fresh parameter ValueId allocation rejects
wrong runtime value rejects before body effects
missing/extra argument rejects
Call/MethodCall body rejects
non-VM backend rejects before effects
```

Authority counters:

```text
parameter fresh ValueId allocation = 0
canonical parameter legacy-name lookup = 0
typed parameter A+ fallback = 0
source call activation = 0
receiver activation = 0
Ownership SSA/opcode activation = 0
selected-route legacy RC insertion = 0
```

## Gates

```text
resolved value profile fixtures
canonical capability fixtures
resolved lowering parameter fixtures
Binding SSA contract guard
parameter-entry carrier fixtures
final-callee VM parameter fixtures
MIR JSON parameter-contract fixtures
backend parameter capability fixtures
full resolved-lowering family
authority guard
current pointer guard
release build
dev gate quick
```

No new top-level shell gate is required. Extend the existing Binding SSA
contract manifest and bounded validators.

## Implementation may claim

After I1 only:

```text
static exact-i64 parameters are exact Binding SSA entry definitions
reserved formal ValueIds are reused without allocation
parameter rebind and If PHI share the local Binding SSA authority
the existing final-callee i64 contract is preserved
unsupported backends fail before effects
```

## Implementation must not claim

```text
selfhost corpus coverage increased
source Call or MethodCall support
typed return support
usize or all exact-numeric parameter support
Bool/Float parameter support
instance receiver support
Box/String/BorrowedText parameter support
Ownership SSA activation
all backend support
Loop activation
SSA-I1-FULL
```

## Stop conditions

Stop the series if any of the following becomes necessary:

1. Receiver or instance-method setup enters P0a.
2. Bool/Float is accepted without a final-callee contract.
3. `usize` is collapsed to i64 or Box representation is accepted as proof.
4. A parameter receives a fresh ValueId instead of its reserved formal value.
5. Only part of one function's parameters uses Binding SSA.
6. Lower reclassifies source parameter types or reconstructs BindingRef.
7. Source/formal index, name, or cardinality drift is tolerated.
8. Typed unsupported signatures silently use A+.
9. P0a failure retries A+.
10. FunctionEntryContractOwner becomes binding-identity authority.
11. Runtime tags or call-site facts become entry acceptance authority.
12. Source Call/MethodCall is activated in the same slice.
13. Ownership instructions or legacy RC appear on the selected route.
14. Unsupported backends silently execute or fall back to VM.
15. Any modified source/check file reaches 800 lines.

## Follow-on order

P0a does not pretend to unlock the current corpus. The next decisions remain:

```text
P0n:
  exact-numeric type-name/MIR representation SSOT
  usize and fixed-width integer parameter expansion

P0b/P0f:
  Bool and Float final-callee contracts

P0r:
  instance receiver owner-family cutover

P0c:
  exact known-static source Call ingress

R0:
  typed return/result family
```

Each remains a separate semantic row. P0a completion returns to a selection
stop before any follow-on activation.
