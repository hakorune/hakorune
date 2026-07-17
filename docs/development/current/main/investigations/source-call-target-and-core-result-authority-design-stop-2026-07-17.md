---
Status: active taskboard; Q0 and Core result S0 closed
Date: 2026-07-17
Baseline: 040d2906a35b367d39f1377b8159cef020203b78
Parent: callable-result-i64-catalog0-task-2026-07-17.md
Scope: canonical source-call target projection and neutral Core method result kind
Decision: Candidate A-prime
---

# Source-call target and Core-result authority taskboard

## Decision

Candidate A-prime is selected after three read-only worker audits and a local
source/route audit authorized by the user.

```text
source-call route facts
  -> one site-indexed target catalog
       route-disjoint qualified/current-owner variants

CoreMethodContractBox
  -> one generated neutral result-kind view

bounded source receiver facts
  -> ExactStringOnSuccess

callable-result proof
  -> borrows and co-seals only the rows required at each call site
```

The target and result-kind authorities remain separate because they own
different truth. The accepted call-site proof co-seals them so no later loose
target/result join is allowed.

One universal source-call resolver is not selected. Bare, qualified-static,
and current-owner calls have different precedence inputs. They share one final
target vocabulary and catalog shape, but route-disjoint producers seal their
variants in a fixed order.

## Durable authority split

| Concern | Authority | Explicit non-authority |
| --- | --- | --- |
| same-module declarations | `VerifiedSameModuleCallableDeclarationCatalogV1` | lowering order, MIR table |
| exact call site | `SourceExprSiteV1` under caller canonical key | span text, physical symbol |
| imported static alias | verified sorted alias view co-sealed with catalog target | raw mutable `using_import_boxes` map |
| qualified selected target | `VerifiedQualifiedStaticCallTargetV1` | declaration existence alone |
| current-owner selected target | `VerifiedCurrentOwnerStaticCallTargetV1` | `current_static_box`, name split, `current_module` |
| final target vocabulary | `VerifiedSourceStaticCallTargetV1` | Builder route replay |
| final target rows | `VerifiedSourceStaticCallTargetCatalogV1` | runtime class/tag |
| Core method identity/effect/result | `CoreMethodContractBox` source row | Builder `MirType`, runtime method table |
| generated Core result view | generated JSON v1 plus generated Rust table | runtime JSON parsing in Builder |
| source String receiver | bounded `SourceCoreReceiverFactV1` | `I64ExpressionFactV1` widening |
| accepted call result | callable-result row co-sealing target and result evidence | method spelling heuristic |

## Source target product

The final catalog is site-indexed and non-Clone.

```rust
pub(crate) struct VerifiedSourceStaticCallTargetCatalogV1 {
    rows: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedSourceStaticCallTargetV1,
    >,
}

pub(crate) enum VerifiedSourceStaticCallTargetV1 {
    QualifiedStatic(VerifiedQualifiedStaticCallTargetV1),
    CurrentOwnerStatic(VerifiedCurrentOwnerStaticCallTargetV1),
    BareStaticRecovery(VerifiedBareStaticCallTargetV1),
}
```

The first row implements only `QualifiedStatic`. `BareStaticRecovery` remains
parked because its final selection requires every higher-priority FunctionCall
route to decline. A unique declaration candidate alone is not final route
authority.

### Qualified-static law

```rust
pub(crate) enum VerifiedQualifiedStaticReceiverV1 {
    ImportedAlias {
        source_alias: Box<str>,
        canonical_owner: Box<str>,
    },
    UnshadowedCanonicalOwner {
        canonical_owner: Box<str>,
    },
}
```

The sealer consumes:

```text
caller canonical key
function-relative SourceExprSiteV1
receiver spelling
verified import-alias binding, when present
exact-site lexical binding fact
reserved special-receiver decline
complete same-module declaration catalog
checked source arity
```

It produces one exact canonical target key or a typed unavailable reason. It
does not emit MIR, choose a result representation, or parse a physical symbol.

Current Builder behavior resolves imported aliases before the local-binding
check. The disconnected parity matrix must either preserve that precedence
explicitly or stop before activation; it must not silently choose conventional
lexical precedence instead.

Reserved `mem`, `__mir__`, and `__repl__` routes are not admitted as ordinary
qualified static calls when their existing special route is active.

## Core result-kind authority

No second semantic catalog is created. The existing `.hako` SSOT remains:

```text
lang/src/runtime/meta/core_method_contract_box.hako
```

Each canonical row gains a neutral `result_kind`. The generated manifest moves
to schema v1 and the generator also emits a static Rust data table. Builder and
source proof may later consume the same generated rows without parsing JSON at
runtime.

First neutral vocabulary:

```text
I64Value
BoolValue
StringValue
NoValue
Dynamic
```

Canonical spelling and aliases remain one source row. For String length:

```text
receiver = StringBox
canonical = length
aliases = len, size
arity = 0
core_op = StringLen
result_kind = I64Value
```

Receiver plus exact spelling plus exact arity selects a row. Duplicate alias
or canonical collisions for one receiver and arity fail generation. Same
spellings on Array/Map remain distinct receiver rows. Builder-only return-type
matches may remain temporarily only as guarded migration mirrors, never as
semantic authority.

## Bounded String receiver fact

String receiver representation does not enter the exact-i64 abstract domain.
It receives a separate bounded view:

```rust
pub(crate) enum SourceCoreReceiverFactV1 {
    ExactStringOnSuccess,
}
```

The first admitted shapes are an exact String literal and String-left `Add`.
`OnSuccess` is a result representation contract: it states the representation
when evaluation returns a value. It does not claim totality, purity, absence of
an error, or a new NonVoid fact. If implementation evidence shows this is not
the repository's result-contract law, the row stops rather than broadening the
i64 domain or adding a fallback.

## Exact task order

```text
R0-SOURCE-CALL-TARGET0-Q0 [closed]
  -> R0-CORE-METHOD-RESULT-KIND0-S0 [closed]
  -> R0-SOURCE-STRING-RECEIVER0-S0
  -> R0-SOURCE-CALL-TARGET0-M0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-S0b
  -> R0-CALLABLE-RESULT-I64-CATALOG0-P0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-G0
  -> clean HMI-S0-V0-R0-I0 resume
```

`R0-CORE-METHOD-RESULT-KIND0-S0` is the sole next code-facing row.

### Q0 — disconnected qualified target

```text
production behavior delta = 0
production producers = 0
production consumers = 0
```

Owns:

```text
qualified receiver decision
verified imported-alias view
exact lexical-shadow observation
reserved-route decline
catalog-key projection
caller-key + SourceExprSiteV1 row
typed unavailable vocabulary
```

Does not own:

```text
current-owner calls
bare calls
builtin/Core calls
result representation
argument evaluation
Builder emission
```

Required fixtures include direct canonical spelling, an imported alias,
`ParserStringUtilsBox.skip_ws/2 -> StringHelpers.skip_ws/2`, alias/local-name
collision parity, wrong arity, missing target, reserved receiver decline, and
declaration reorder parity.

Q0 closeout evidence:

```text
module:
  src/mir/source_call_target/

sealed products:
  VerifiedStaticImportAliasViewV1
  VerifiedSourceStaticCallTargetCatalogV1

focused target tests:
  10/10

existing callable-result tests:
  12/12

family structural guard:
  green

quick gate:
  66/66

production producers/consumers:
  0/0

Builder/MIR/runtime behavior delta:
  0

largest new source/check file:
  tests.rs, below 800 lines
```

The actual fixture reads the repository `StringHelpers` and
`ParserStringUtilsBox` sources at compile time. Imported alias resolution keeps
the existing alias-before-local precedence, while direct canonical spelling
requires an unbound lexical fact. The sealed target remains a structured key;
no MIR symbol is parsed or retained.

### Core result S0 — generated neutral result kinds

```text
production behavior delta = 0
production consumers = 0
```

Add `result_kind` to the existing `.hako` rows, generate JSON schema v1 and a
static Rust table, and prove canonical/alias/arity/receiver collision laws.
`String.length/len/size` is the first required `I64Value` row.

Core result S0 closeout evidence:

```text
semantic owner:
  lang/src/runtime/meta/core_method_contract_box.hako

neutral vocabulary:
  I64Value | BoolValue | StringValue | NoValue | Dynamic

generated artifacts:
  core_method_contract_manifest/v1 JSON
  src/mir/generated/core_method_contract_rows.rs

lookup key:
  receiver + exact canonical/alias spelling + exact expanded arity

StringBox.length/len/size/0:
  one canonical row, I64Value

malformed generator fixtures:
  9/9 green

Rust normalized lookup/parity fixtures:
  5/5 green

production consumers / behavior delta:
  0 / 0

runtime JSON parsing:
  0

largest modified source/check file:
  core_method_contract_box.hako, 423 lines
```

Rows whose source-visible result is not represented uniquely by the first
neutral vocabulary remain `Dynamic`. In particular, Map mutator presentation
and raw-helper return carriers are not promoted into semantic result facts.
The generator rejects unknown kinds and canonical/alias collisions after
expanding arity patterns; receiver-disjoint equal spellings remain valid.

### String receiver S0 — disconnected source view

```text
production behavior delta = 0
production consumers = 0
```

Seal only String literal and String-left Add `ExactStringOnSuccess`. No general
String value domain, dynamic truthiness, result totality, or Builder type
backfeed is admitted.

### M0 — current-owner target

```text
production behavior delta = 0
production consumers = 0
```

Seal the current-owner variant from canonical source declaration identity.
The actual `StringHelpers.to_i64/1 -> me._digit_value/1` fixture is required.
Function-name splitting, `current_static_box`, `current_module`, lowering order,
and `variable_map["me"]` are forbidden target authorities.

### S0b — complete disconnected callable result

Borrow the target catalog, Core result row, and bounded String receiver view.
No production consumer is added.

Required positive rows:

```text
StringHelpers.skip_ws/2 = ExactI64 {1}
ParserStringUtilsBox.skip_ws/2 = ExactI64 {1}
StringHelpers.to_i64/1 = ExactI64 {}
StringHelpers._digit_value/1 current-owner target
provider/caller declaration reorder parity
```

Bare FunctionCall remains explicitly unavailable in this row.

### P0 / I0 / G0

P0 fixes the normalized target/result/co-seal pass and reject matrix. I0 adds
one pre-body catalog construction sequence, one selected-target emission
consumer, and one result-publication consumer. G0 fixes producer/consumer
counts, no-retry guards, line caps, and the HMI resume pointer.

## Required counters

```text
source target final vocabulary definitions = 1
qualified target producer families = 1
current-owner target producer families after M0 = 1
bare target production consumers through G0 = 0

Core result semantic catalogs = 1
Core result generated Rust tables = 1
runtime JSON parsing for Core result lookup = 0

raw Lower name reads on selected rows = 0
physical-symbol parsing = 0
current_module target identity = 0
function-name target identity = 0
runtime-tag target/result inference = 0

result proof Builder MirType reads = 0
final MirFunction metadata reads = 0
I64ExpressionFact String variants = 0

fallback / retry / re-lowering = 0
production behavior delta before I0 = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop before the affected row if any implementation requires:

1. one universal resolver replaying all Builder call precedence;
2. a second callable declaration/body catalog;
3. raw mutable `using_import_boxes` as a sealed authority;
4. current-owner identity from function names, `current_static_box`, or
   `current_module` visibility;
5. a new semantic Core-method catalog instead of extending
   `CoreMethodContractBox`;
6. Builder `MirType`, final metadata, runtime tags, or method-name whitelists
   flowing back into source proof;
7. String values entering `I64ExpressionFactV1`;
8. a totality/NonVoid claim merely to classify a successful String-left Add;
9. physical-symbol parsing, callee-first lowering, fallback, or retry;
10. a production consumer before disconnected parity;
11. a source/check file reaching 800 lines.

## Implementation may eventually claim

After the full task order is green:

```text
qualified and current-owner selected targets have canonical source identity
Core method result kinds come from one existing semantic owner
String length aliases share one receiver/arity/result row
selected call sites co-seal target and result evidence
the actual skip_ws wrapper and to_i64 chain have exact-i64 result rows
one selected production route publishes the sealed call result without retry
```

## Implementation must not claim

```text
general callable support
bare-call final authority
general String abstract interpretation
call totality or purity
general non-i64 source values
runtime type inference
physical MIR symbol identity
callee-first publication
fallback or route retry
HMI register execution before the clean resume row
```

## Docs loop breaker

```text
worker_inventory = consumed
worker_inventory_scope = read_only current source, routes, cards, and ledgers
docs_only_closeout = forbidden
next_commit_code_or_generated_artifact_delta_required = 1
```

No new consultation card is created. The next commit after this accepted
taskboard must implement or generate an artifact for Q0. A new consultation is
allowed only when an explicit stop condition is observed.
