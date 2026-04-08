---
Status: Provisional SSOT
Decision: provisional
Date: 2026-04-09
Scope: primitive family と user box field access を、`.hako` surface を汚さずに compiler/MIR 主導で高速化する設計と実装順を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md
  - docs/development/current/main/design/birth-placement-ssot.md
  - docs/development/current/main/design/type-system-policy-ssot.md
  - docs/development/current/main/design/auto-specialize-box-ssot.md
  - crates/nyash_kernel/src/exports/primitive.rs
  - crates/nyash_kernel/src/exports/user_box.rs
  - crates/nyash_kernel/src/user_box_registry.rs
  - src/core/instance_v2.rs
  - src/runtime/host_handles.rs
  - crates/hakorune_mir_defs/src/call_unified.rs
---

# Primitive Family And User Box Fast Path SSOT

## Goal

- `Everything is Box` の surface は維持したまま、primitive family と user box field access を compiler/MIR 主導で薄くする
- `C/Rust が速い = 型だけ分かればよい` ではなく、`semantic type + storage class + boundary` が早く確定する状態を作る
- `.hako syntax` を増やす前に、frontend lowering / canonical MIR / storage-class facts で速い path を作る
- string で始めた `semantic primitive first` の考え方を、primitive boxes と user boxes に横展開する

## Current Reading

Current string lane already shows the right direction:

- `string` は local Rust helper rewrite より、`semantic builtin family -> canonical MIR -> placement/effect` の方が clean
- `substring(...).length()` を後段 sink だけで支えるより、早い semantic normalization の方が長期的にまっすぐ
- 同じことは primitive family と user boxes にも当てはまる

Current user-box / primitive cost reading:

- primitive scalar work is still often read back through handle/object world
- user box field access still tends to look like object lookup + downcast + value extraction
- `.hako` には field type declarations がすでにあるが、compiler がその declared type を十分に使っていない

## Current Landing Status

- landed:
  - typed `field_decls` now survive `.hako` parser -> AST -> Stage1 Program JSON -> MIR metadata -> MIR JSON
  - canonical MIR now has first-class `FieldGet` / `FieldSet`
  - MIR interpreter and LLVM/PyVM compatibility paths accept those field ops while keeping current generic field semantics
- landed:
  - declared field types now seed `FieldGet` value types on the `.hako` builder path
  - type propagation and storage-class refresh also treat `FieldGet.declared_type` as a fallback seed
- landed:
  - LLVM lowering now treats `IntegerBox` / `BoolBox` handle facts as primitive numeric inputs on `binop` / `compare`
  - numeric lowering unboxes via `nyash.integer.get_h` / `nyash.bool.get_h` before integer arithmetic or integer compare
- next:
  - pilot typed user-box field access on the internal path
- not yet:
  - no user-box flattening
  - no tagged pointer / NaN-boxing
  - no new `.hako` syntax or widened `@rune`

## Fixed Decisions

### 1. `.hako` surface stays simple

Do not add new special syntax for this wave.

Keep current `.hako` reading:

- user writes box/type declarations as today
- user writes method/field access as today
- compiler lowers the semantic family earlier

This means:

- no new primitive-only surface syntax yet
- no new `@rune` widening for field/layout hints
- no runtime-mechanics hints in `.hako`

### 2. Distinguish semantic type from storage class

For performance work, do not collapse these into one thing.

- semantic type
  - what the program means
  - e.g. `IntegerBox`, `BoolBox`, `StringBox`, user `Point`
- storage class
  - how the compiler/runtime may carry the value internally
  - e.g. `InlineI64`, `InlineBool`, `BorrowedText`, `BoxRef`, `Opaque`

The compiler should reason from declared type to storage class.
It should not force every declared type through the same `Arc<dyn NyashBox>` path.

### 3. Primitive family is a semantic builtin family

Treat primitive operations as a first-class semantic family, not as runtime rediscovery.

Examples of the intended family:

- integer arithmetic / compare
- bool logic / compare
- string scalar consumers and borrowed-text producers

This does **not** mean exposing runtime implementation details.
It means frontend/builtin lowering should normalize early to canonical ops instead of
re-discovering `IntegerBox` / `BoolBox` / `StringBox` much later.

### 4. User box field access must become canonical MIR

User box field access should not stay as “opaque dynamic method traffic” forever.

Target reading:

- `.hako` declaration says:
  - field name
  - declared field type
- frontend / MIR keeps:
  - `field.get`
  - `field.set`
  - declared type fact
  - storage class fact

This is the clean bridge from box language surface to typed/direct internal access.

### 4.5 `field_decls` is the authority

For typed field work, the authority is:

- `field_decls`

The names-only `fields` list stays only as:

- compatibility mirror
- fallback for old payloads
- legacy runtime consumers that have not been upgraded yet

Do not treat `fields` as the typed design truth again.

### 5. Text world, object world, and handle world stay split

Do not fuse these worlds when generalizing beyond string.

- borrowed text corridor stays in text world
- owned text / primitive immediate stays in value world
- stable object identity stays in object world
- host publication stays in handle world

This follows current Birth / Placement reading:

- `MaterializeOwned` does not imply `StableBoxNow`
- `StableBoxNow` does not imply `FreshRegistryHandle`

### 6. Fast path order

Preferred order:

1. semantic builtin family
2. canonical MIR op / field access
3. storage class facts
4. typed fast path pilot
5. only then flattening / more aggressive representation work

Do **not** start with tagged pointer or full layout flattening.

## Storage Class Table

Initial storage-class reading for this wave:

| Storage class | Meaning | Examples now | Keep line |
| --- | --- | --- | --- |
| `InlineI64` | immediate integer payload | `IntegerBox` fast path | compiler/MIR fact, Rust can materialize late |
| `InlineBool` | immediate bool payload | `BoolBox` fast path | compiler/MIR fact, Rust can materialize late |
| `BorrowedText` | non-owning text corridor | `str.slice`, `str.len`, `str.eq` lane | MIR + placement/effect facts, Rust handles lifetime mechanics |
| `BoxRef` | stable object reference required | generic object/user box path | Rust object world |
| `Opaque` | no direct typed fast path yet | plugin / unknown / dynamic cases | fallback keep |

This table is the design anchor.
It is not a promise to flatten all rows immediately.

## Implementation Order

### Step 1. Docs-first lock

- lock this design before code motion
- keep string corridor as the current proving ground
- record that primitive/user-box optimization follows the same `semantic primitive first` rule

### Step 2. Primitive family inventory

- inventory current `IntegerBox` / `BoolBox` fast paths and rediscovery points
- identify where runtime still downcasts or handle-looks-up values that could be immediate
- keep this step behavior-preserving

Acceptance:

- docs/code map shows current primitive rediscovery points

### Step 3. User box field inventory

- inventory current field access / field storage owner path
- identify the narrowest current owner for field declarations and field reads/writes
- record where `box Point { x: IntegerBox }` type info is lost

Acceptance:

- docs/code map shows current field-declaration truth and current field-access lowering

### Step 4. Canonical MIR field access

- add canonical MIR-side `field.get` / `field.set` reading for user boxes
- attach declared field type facts without behavior change
- do not flatten layout yet

Acceptance:

- MIR/dumps show field access and declared type facts directly
- runtime behavior remains unchanged

### Step 5. Storage class facts

- add no-behavior-change storage class facts
- start with:
  - `InlineI64`
  - `InlineBool`
  - `BorrowedText`
  - `BoxRef`
  - `Opaque`

Acceptance:

- facts are visible in dumps
- no user-visible behavior change

### Step 6. Typed primitive fast path pilot

- pilot direct primitive consumer path on the narrowest useful case
- examples:
  - integer add/compare
  - bool compare
- prefer compiler-selected direct path over runtime rediscovery

Acceptance:

- exact front improves
- public surface stays unchanged

Current landing status:

- first pilot is in: LLVM lowering now unboxes `IntegerBox` / `BoolBox` handle facts on `binop` / `compare`
- this is intentionally narrow and does not yet rewrite user-box field storage or layout
- typed primitive access now precedes typed user-box field access in the execution order

### Step 7. User box typed field access pilot

- pilot typed `field.get` / `field.set` for a user box whose fields are all primitive-friendly
- avoid ABI/plugin/reflection boundaries in the first slice
- do not require full flattening
- add a narrow local perf gate first:
  - `kilo_micro_userbox_point_add`

Acceptance:

- a user box field consumer can avoid repeated object/handle/downcast work on the internal path

### Step 8. Optional flattening only later

- only after typed access wins are proven
- only when field set is stable and boundary rules are explicit
- only on boxes whose fields are all flattenable storage classes

Flattening is not the first move.
It is a later optimization once the semantic/MIR contract is already correct.

## Do Not Do Yet

- no new `.hako` special syntax for primitives or field hints
- no `@rune` widening for storage/layout hints
- no tagged pointer / NaN-boxing as the first move
- no blanket user-box flattening
- no runtime cache / epoch / provider / TLS exposure at the language surface
- no benchmark-specific fused public op such as making one micro chain its own public canonical primitive

## Active Reading

Read this design as the follow-on after the current string corridor wave:

1. keep `string` as the active proving ground
2. lock `primitive family` as the next semantic builtin family
3. lock `user box field access` as the next canonical MIR target
4. defer flattening until typed access proves its value

One-line rule:

`Everything is Box` stays true at the surface, but the compiler should not force
every box-typed path through the same object/handle machinery internally.
