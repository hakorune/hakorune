# Dynamic carrier ingress lifecycle

Status: ingress Decision accepted; implementation is `NoSafeSlice`
Date: 2026-08-10
Parent: `DYNAMIC-CARRIER-REBIND-TRANSACTION-D0`
Current design row: `CALLABLE-PARAMETER-TRANSFER-AUTHORITY-D0`
Exception: T2 source-authority boundary required before several implementation rows.
ParentCurrentCard: this file is the rolling card for parameter demand through carrier ingress.

## Decision

The exact initial root-carrier chain is:

```text
static ParserScanLoopBox.skip_while/4
parameter #1 pos
  -> exact Pos BindingRef
  -> PreludeInitializerPos
  -> local i / induction BindingRef
  -> Recipe input V1
  -> carrier C0 / root L0 / binding B0 / Dynamic
  -> JoinSig Enter(B0 = V1 : Dynamic)
```

A plain parameter has callable-boundary `Handle` demand. Therefore this exact
initial B0 instance is:

```text
BorrowedIngressNoEnd
```

`local i = pos` is borrowed-alias propagation. It is not a Home transfer, an
independent copy, or an owned Dynamic carrier publication. Displacing this
initial B0 instance performs carrier End zero times.

Later V17 is different. The Dynamic operator contract gives V17
`EndExactlyOnceUnlessForwarded`; after atomic forwarding into B0, the current
B0 instance is owned. Carrier flow must retain the disposition of each origin
instead of assigning one lifecycle rule to the B0 key.

## Sole authority

The Dynamic profile must not issue `plain = Handle`. The existing
`VerifiedHomeAbiV1` must not be widened directly: it is a nominal instance
method, receiver-bearing, I64/Unit cohort and cannot honestly classify the
static untyped `skip_while/4` declaration.

The selected common authority is:

```text
parser-sealed parameter transfer syntax
+ exact resolved callable declaration and parameter BindingRefs
  -> VerifiedCallableParameterDemandCatalogV1
       complete ordered rows for every parameter
       Ordinary -> Handle
       Take     -> future accepted Home-demand capability
```

The catalog owns parameter demands only. It owns no receiver, result,
Dynamic, Recipe, carrier, CFG, or physical ABI meaning. Existing and future
callable Home ABI aggregates must consume or project these rows rather than
reissue parameter demand independently.

The Dynamic ingress issuer is a one-way relational co-seal:

```text
VerifiedCallableParameterDemandCatalogV1
+ whole VerifiedDynamicOperatorCarrierLifecycleProgramV1
+ exact parameter -> prelude -> Recipe carrier -> JoinSig Enter relation
  -> VerifiedDynamicCarrierIngressLifecycleProgramV1
       private ingress row = BorrowedIngressNoEnd
```

The whole result is non-Clone and non-splittable. It exposes at most a
borrow-scoped ingress view. No public constructor accepts an ordinal, site,
demand, Recipe key, or disposition selected by the caller.

## Why implementation is still `NoSafeSlice`

The current Rust `ParamDecl` carries name and optional type but no canonical
`Ordinary | Take` transfer syntax. The current Hako parameter carrier is a
disconnected ordinary-only substrate and is not a Rust/resolver parity
authority. Absence of `take` may not be inferred from an old AST shape.

Existing source/Recipe products already prove Pos, initializer, local, V1,
C0/L0/B0, but they intentionally own no callable parameter demand. Recipe
`Dynamic`, runtime tags, selector/provider names, `MirType`, ValueId,
`ReleaseStrong`, and source names cannot fill this gap.

## Ordered tasks

### 1. `CALLABLE-PARAMETER-TRANSFER-AUTHORITY-D0` — current

Close the common Rust/Hako authority contract:

- typed closed syntax vocabulary `Ordinary | Take`;
- exact callable declaration, method/function site, parameter ordinal, and
  parser provenance;
- static and instance declarations use one parameter identity boundary;
- no raw string tag, builder-instance token, old-AST absence inference, or
  Home meaning in the parser seal;
- reuse the existing `HAKO-PARAMETER-TRANSFER-TYPED-SEAL-D0/R0` work instead
  of creating a second Hako vocabulary.

### 2. `CALLABLE-PARAMETER-TRANSFER-SOURCE-SEAL-I0`

Land the complete parser/resolver handoff and Rust/Hako parity. First active
cohort may issue only exact `Ordinary` rows; this does not activate `take`.

Required negatives: missing/duplicate/foreign ordinal, wrong parser/catalog
brand, raw `"Ordinary"` construction, builder token as identity, and
line/context drift. Compiler acceptance must be widened if the unchanged
source cannot be represented; source rewriting and fallback are forbidden.

### 3. `CALLABLE-PARAMETER-DEMAND-I0`

Issue one complete `VerifiedCallableParameterDemandCatalogV1` from the sealed
syntax and resolved declaration. First cohort maps `Ordinary -> Handle` only.
Reject partial coverage, duplicate rows, foreign BindingRefs, and declaration
arity mismatch. Do not refactor the existing instance Home ABI in this row;
record its later convergence boundary and forbid duplicate new demand owners.

### 4. `DYNAMIC-CARRIER-INGRESS-LIFECYCLE-I0`

Consume the whole parameter-demand catalog and whole Dynamic lifecycle
program. Seal parameter #1 through Pos/initializer/local/V1/C0/L0/B0 and the
exact JoinSig Enter payload. Publish one private borrowed ingress row.

Required negatives include wrong initializer BindingRef, local binding,
Recipe input, carrier owner/binding/class/entry, missing or duplicate Enter,
extra root carrier, caller-selected disposition, Clone, and split API.

### 5. Follow-on order

```text
DYNAMIC-CARRIER-REBIND-TRANSACTION-I0
-> DYNAMIC-CARRIER-FLOW-D0/I0
-> cleanup projection / Completion / exit transaction
-> physicalization
```

## Parked independent cleanup

- `CURRENT-POINTER-CROSSFIELD-CONSISTENCY-R0`: strengthen the existing guard
  and table-driven fixtures. `10-Now.md` is already reduced to a field-name
  mirror; the current pointer values are not copied there.
- `DYNAMIC-FAULT-CATALOG-EXHAUSTIVE-R0`: exhaustive operation classification
  is already closed. Only typed reject preservation and caller-zero
  visibility narrowing remain.
- recursive JoinSig topology admission, both-arm exit generalization,
  multi-root-carrier After closure, and module relocation are P2 and must not
  enter the parameter/ingress series.

## Stop / non-claims

Stop at `NoSafeSlice` if the parameter transfer seal, common demand catalog,
or exact JoinSig Enter relation cannot be issued from current source authority.

```text
no explicit Take/owned ingress activation
no Dynamic Home classification
no Home Flow or cleanup execution
no rebind or displaced-current token in the ingress rows
no CFG / SSA / PHI / ValueId
no Completion / Return / DraftSeal
no runtime/provider/physical ABI
no production activation, retry, or fallback
```

Implementation files must be responsibility-split before 760 lines and stay
below the 800-line hard limit. Tests live outside owner files.
