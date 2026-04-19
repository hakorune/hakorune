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
- landed:
  - `kilo_micro_userbox_point_add` now exists as the narrow local user-box perf gate
  - LLVM `field_get` / `field_set` now take a typed IntegerBox path for known user-box `field_decls`
  - weak fields and non-user-box paths stay on the generic fallback
- landed:
  - LLVM `field_get` now also takes a typed BoolBox path for known user-box `field_decls`
  - LLVM `field_set` now takes a typed BoolBox path only when the set source is bool-safe (`BoolBox` handle or bool immediate)
  - ambiguous/non-boolish set sources stay on the generic fallback
- landed:
  - compare/bool expressions now lower in value context on the `.hako` builder path, so compare-as-value loop bodies are accepted structurally
  - `kilo_micro_userbox_flag_toggle` now exists as the dedicated BoolBox local perf proof
  - pure-first boundary matching now covers the narrow Flag/BoolBox toggle micro
  - `Float` surface-close on the current route:
    - Stage1 Program JSON v0 now lowers float literals, including unary-minus float literals
    - recent value-lowering now accepts float literals and preserves `MirType::Float` for float arithmetic results on the same keeper path
  - `FloatBox` fast-path pilot on the current keeper slice:
    - primitive-handle lowering now recognizes `FloatBox` handles as the float family
    - LLVM `field_get` now uses `nyash.instance.get_float_field_h` for typed non-weak `FloatBox` fields
    - LLVM `field_set` now uses `nyash.instance.set_float_field_h` only when the source is float-safe (`FloatBox` handle or actual `f64`)
  - `Float` storage-class promotion is now landed as MIR inventory only:
    - `MirType::Float` and typed `FloatBox` field facts now classify as `InlineF64`
    - dumps / MIR JSON surface the new storage fact without changing runtime behavior
- landed:
  - `ArrayBox` narrow typed-slot pilot is now landed on scalar immediate residence:
    - runtime authority stays in `ArrayBox`; `NyashValue::Array` is still not the keeper lane
    - `slot_store_i64_raw` / `slot_rmw_add1_i64_raw` can birth or preserve the narrow `InlineI64` storage
    - `nyash.array.slot_append_hh` / `nyash.array.push_hi` route integer-shaped values through the same narrow lane
    - existing generic any-store/append routes also birth or preserve narrow `InlineBool` / `InlineF64` lanes for `BoolBox` / `FloatBox` payloads
    - `array_slot_load_encoded_i64` reads `InlineI64` directly without re-boxing
    - `slot_load_hi` remains the encoded-any read contract; float slots read back as `FloatBox` handles instead of adding a new typed load ABI row
    - boxed/string/mixed routes explicitly promote back to boxed storage before mutation
    - focused ArrayBox / kernel tests pass, and `phase21_5_perf_kilo_micro_machine_lane_contract_vm` stays green
- not yet:
  - `Null` / `Void` fast paths are still conservative and low priority in this wave
  - first-class enum/sum MIR types and user-defined generics belong to the separate enum/sum design owner, not the primitive fast-path owner
    - parser / AST / Stage1 surface is now landed:
      - `enum Name<T> { ... }`
      - Stage1 `enum_decls`
      - Stage1 `EnumCtor`
    - known-enum shorthand match / exhaustiveness is now landed:
      - parser resolves `Some(v)` / `None` against known enum inventory
      - Stage1 lowers that lane as `EnumMatch`
      - `_` does not satisfy known-enum exhaustiveness
    - canonical sum MIR lowering is now landed:
      - `VariantMake` / `VariantTag` / `VariantProject`
      - JSON v0 bridge now lowers `EnumCtor` / `EnumMatch` into that lane
    - VM / LLVM / fallback runtime support for the narrow MVP variant lane is now landed
    - remaining generic semantics, `where`, enum methods, full monomorphization, and broader product-consumer parity stay backlog
    - design owner: `docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md`
  - no user-box flattening
  - no tagged pointer / NaN-boxing
  - no new `.hako` syntax or widened `@rune`

## Fixed Decisions

### 137x-B residual classification

This section is the bridge back to owner-first optimization after the
container/primitive design cleanout.

Blocking before `137x-C`:

- none for `Null` / `Void`; current conservative handling is acceptable for perf return
- none for enum/sum/generic; ownership stays with the enum/sum SSOT and later generic placement/effect work

Non-blocking residuals:

- `Null` / `Void` may later gain narrower fast paths, but current cleanup already converges safe nullish checks and the remaining perf value is low priority
- enum/sum MVP surface, canonical MIR, and narrow VM/LLVM fallback support are landed; broader generic semantics remain separate-phase backlog
- primitive/user-box field fast paths, enum/sum local aggregate keep lanes, and ArrayBox typed-slot residence are sibling proofs, not one interchangeable keeper proof

Stop-line:

- do not use ArrayBox typed-slot residence as proof for generic primitive flattening
- do not use enum/sum local aggregate evidence as proof for container residence rewrites
- do not reopen `.hako` syntax, public ABI, or full monomorphization from the 137x return gate

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
| `InlineF64` | immediate float payload | `Float` / `FloatBox` fast path | compiler/MIR fact, Rust can materialize late |
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
  - `InlineF64`
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
- add narrow local perf gates first:
  - `kilo_micro_userbox_point_add`
  - `kilo_micro_userbox_flag_toggle`

Acceptance:

- a user box field consumer can avoid repeated object/handle/downcast work on the internal path

### Current post-Step-7 queue

- keep the landed `ArrayBox` typed-slot pilot green on `InlineI64`
- move to enum Step 4 (`VM / LLVM / fallback runtime`) now that the primitive/array keeper slice is stable

### Step 7.5. Array typed-slot SSOT and pilot

- runtime authority stays in `ArrayBox`
  - do not switch the pilot authority to `NyashValue::Array`
  - `NyashValue::Array` remains a separate value-world substrate, not the keeper lane for this pilot
- public surface stays unchanged
  - keep visible `ArrayBox.{push,get,set,len/length/size,pop}` semantics
  - keep `nyash.array.*` exports as the ABI surface
  - do not add `.hako` syntax, array generics, or public benchmark-only ops
- first slot vocabulary is narrow
  - default keep lane: `Boxed`
  - first typed pilot: `InlineI64`
  - landed scalar extensions under the same pilot: `InlineBool`, `InlineF64`
  - later-only candidate: `BorrowedText`
- implementation boundary stays below the current raw seams
  - `slot_append_box_raw`
  - `slot_store_box_raw`
  - `slot_store_i64_raw`
  - `slot_store_bool_raw`
  - `slot_store_f64_raw`
  - `slot_rmw_add1_i64_raw`
  - kernel `nyash.array.slot_*` leaves remain the execution entry, not a second truth
- promotion rule must be explicit, one-way, and behavior-safe
  - typed-slot birth is opt-in for compiler/internal routes only
  - incompatible store / reflection / plugin / mixed-value routes must explicitly promote the array back to boxed storage before mutation
  - no silent reinterpretation, no hidden fallback coercion
- non-goals for the first pilot
  - no heterogeneous typed-slot lane
  - no union/variant slot layout
  - no string-lane merge into the array pilot; borrowed string stays a sibling guardrail lane
- acceptance for the first pilot
  - prove a narrow `InlineI64` array lane first
  - allow `InlineBool` / `InlineF64` only as scalar immediate residence under the same boxed-promotion rules
  - preserve current public ArrayBox behavior
  - keep plugin/reflection/mixed arrays on the boxed lane
- landed pilot facts
  - `ArrayBox` now has narrow internal `InlineI64` / `InlineBool` / `InlineF64` storage lanes beside the default boxed lane
  - internal scalar routes can birth/preserve those lanes without changing public `ArrayBox` semantics
  - boxed/string routes promote back to boxed storage before mutation
  - `array_slot_load_encoded_i64` reads `InlineI64` directly; no re-box roundtrip on the narrow integer keeper path
  - f64 readback stays on the existing encoded-any contract and may return a `FloatBox` handle; no public typed f64 load row is opened by this pilot

Resulting next cut:

- land VM / LLVM / fallback runtime support for the enum sum lane
- return to kilo/perf rereads after that sum-runtime slice is fixed

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
