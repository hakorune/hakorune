---
Status: Active implementation ledger
Date: 2026-07-11
Scope: Mutable Language v1 type-contract activation, carrier, backend, and representation-consumer status.
Normative-Law: docs/reference/language/types.md
Code-Matrix: src/mir/type_contracts/guarantee_matrix.rs
Refresh-Owner: src/mir/semantic_refresh/contracts.rs
---

# Type Contract Status Ledger

This file records mutable implementation state. It does not define the meaning
of `x: T`; normative type semantics remain in `docs/reference/language/types.md`.

## Active Exact-Numeric Island

| Site | Current implementation | Carrier owner | Runtime/backend boundary |
| --- | --- | --- | --- |
| Box field write | verifier proof or runtime guard | `ExactNumericBoxFieldContract` | dynamic guard capability required |
| parameter entry | runtime checked | `FunctionEntryContractOwner` | final callee, before binding/effects |
| return exit | runtime checked | `FunctionReturnContractOwner` | final outcome, before caller publication |
| local init/reassignment | runtime checked | `LocalSlotContractOwner` | `LocalContractWrite`, before publication |
| record construction/update | exact-numeric fields runtime checked | `RecordValueContractOwner` | field check before `RecordValuePublish` |

All five families are rebuilt and validated by
`semantic_refresh::refresh_and_validate_for_boundary`. Runtime-check elision is
not active for parameter, return, or local contracts. Unsupported backends
must reject before effects.

## Remaining Annotation Sites

| Site | State | Next owner decision |
| --- | --- | --- |
| static table element | readonly U16 closeout in progress | `StaticTableElementContractOwner` |
| ordinary collection element | `Any` dynamic default | no typed activation |
| typed `Array<T>` element | representation inference only; semantic carrier inactive | write-owner convergence first |
| Weak field | builder-local `MirType` check; semantic carrier incomplete | ownership/absence row also constrains semantics |
| FFI ingress/egress | transitional non-guarantee | dedicated FFI boundary decision |
| backend preservation | capability preflight | representation boundary only |

## Representation Consumer Inventory

The classification is an API/owner contract, not a claim that every current
consumer is already migrated.

| Family | Classification | Current owner or anchor | Rule |
| --- | --- | --- | --- |
| declared parameter/return/local/field types | semantic contract source | declaration metadata + site contract owners | may rebuild semantic carriers only in `semantic_refresh` |
| `FunctionSignature` / `MirType` / `value_types` | derived representation fact | MIR function/type metadata | routing/lowering input; never contract proof |
| exact-numeric value/return facts | derived verifier fact | exact-numeric fact owners | may optimize after a check; never replace one |
| runtime type tags/specs | runtime semantic observation | `runtime_type_tag`, `runtime_type_spec` | observe runtime values; do not infer source contract activation |
| declared-type storage selection | migration debt | `declared_type_storage`, record/storage plans | must derive through an accepted representation projection |
| packed-array source-type autouse | migration debt | packed-array autouse plan owners | source spelling must not become storage authority directly |
| route/call plans reading `MirType` | derived representation fact | route-plan owners | allowed only as conservative routing evidence |
| Rune plans | explicit plan input | rune refresh/verification owners | source `:T` cannot synthesize Rune authority |
| backend layouts | derived representation fact | backend capability/layout owners | cannot satisfy a missing semantic carrier |

## Carrier Completeness

| Family | Source contract | Single owner | Semantic refresh | VM consumer | Backend preflight |
| --- | --- | --- | --- | --- | --- |
| static table readonly U16 | complete | complete | complete | complete | complete |
| typed `Array<T>` | incomplete | incomplete | incomplete | runtime methods only | incomplete |
| Weak field | declaration flag only | builder-local only | incomplete | weak operations only | incomplete |
| FFI | incomplete | incomplete | incomplete | provider-specific | incomplete |

## Known Debt Queue

```text
D1:
  audit declared_type_storage and record-state residence projections

D2:
  audit packed-array autouse decisions that read declared source type names

D3:
  keep MirType route users conservative and prohibit semantic-proof promotion

D4:
  keep typed Array inactive until ArrayElementWrite convergence lands and a
  source-owned element contract is selected
```

The null/void/Option relation, truthiness, equality compatibility, ownership,
and capability/effect rows are owned by their later Language v1 cards. They are
not redefined by this ledger.

## Implementation Anchors

| Concern | Anchor |
| --- | --- |
| guarantee matrix | `src/mir/type_contracts/guarantee_matrix.rs` |
| refresh facade | `src/mir/semantic_refresh/contracts.rs` |
| record value contract | `src/mir/type_contracts/record_value.rs` |
| static table contract | `src/mir/type_contracts/static_table.rs` |
| runtime type tags/specs | `src/backend/runtime_type_tag.rs`, `src/backend/runtime_type_spec.rs` |
| VM truthiness/equality | `src/backend/abi_util.rs` |
| MIR binary operations | `src/backend/mir_interpreter/helpers.rs` |

These paths are navigation evidence. Moving code does not change normative
semantics.
