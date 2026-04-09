---
Status: Provisional SSOT
Decision: provisional
Date: 2026-04-09
Scope: first-class enum/sum declarations and generic surface syntax after the primitive-family keeper wave.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md
  - docs/reference/language/EBNF.md
  - docs/reference/language/types.md
  - docs/reference/language/using.md
---

# Enum / Sum And Generic Surface SSOT

## Goal

- add first-class closed sum values without collapsing them into `box` / object identity
- keep the user-facing generic surface consistent with existing `TYPE_REF<...>` syntax
- preserve the current compiler-first rule:
  - surface meaning first
  - canonical MIR next
  - runtime/codegen mechanics last

## Current Inventory

- current surface already has `match`
  - executable MVP today is literals + simple type patterns
  - there is no first-class enum declaration yet
- current type-reference surface already has angle brackets
  - `TYPE_REF := IDENT ('.' IDENT)* ('<' TYPE_REF (',' TYPE_REF)* '>')? ('[' ']')*`
  - this makes `<T>` the natural generic surface
- current type system is still dynamic at runtime
  - MIR type facts are optimization/routing facts, not semantic truth
- `::` already exists in language docs as the long-term scope/type operator
  - however, current executable static method examples still largely use `Driver.main(...)`
  - therefore `Enum::Variant(...)` must be introduced as a narrow constructor surface, not bundled with a whole static-method migration
- current MIR now has first-class sum/enum ops (`sum.make` / `sum.tag` / `sum.project`)
- current parser / AST / Stage1 inventory now lands the narrow enum surface:
  - `enum Name<T...> { ... }`
  - unit variants + single-payload tuple variants
  - first record-variant cut: `Ident { name: String }`
  - `Type::Variant(...)` and `Type::Variant { ... }` lowering to Stage1 `EnumCtor`
- current parser / AST / Stage1 inventory now also lands the narrow known-enum match lane:
  - shorthand patterns like `Some(v)` / `None`
  - first record shorthand cut: `Ident { name }`
  - exhaustiveness checking against the known enum inventory
  - Stage1 lowering to `EnumMatch`
  - record constructors / patterns must name the declared field set exactly
  - `_` does not satisfy known-enum exhaustiveness
  - guarded enum shorthand arms and block-bodied record shorthand arms are still outside this cut

## Fixed Decisions

### 1. `enum` is first-class and separate from `box`

- `box` remains the object/identity/method world
- `enum` is a closed tagged-value world
- do not emulate enum MVP with `box + kind field + manual match`

### 2. user-facing `template` keyword is rejected

- surface generics use `<T>` / `<T, E>`
- keep “template/monomorphization” as an implementation concept only
- do not introduce C++-style `template Foo<T>` syntax

### 3. construction is qualified, matching may be shorthand

Recommended surface:

```hako
enum Option<T> {
  None
  Some(T)
}

enum Result<T, E> {
  Ok(T)
  Err(E)
}

local x = Option::Some("hello")

match x {
  Some(v) => print(v)
  None => print("none")
}
```

Rules:

- construction uses `Type::Variant(...)`
- match shorthand like `Some(v)` / `None` is allowed only when the scrutinee is a known enum type
- outside that known-enum context, the language keeps current dynamic `match` behavior

### 4. accepted enum surface stays narrow

Current accepted surface includes:

- unit variants
- single-payload tuple variants
- narrow named record variants that lower through a synthetic payload box

Examples:

```hako
enum Option<T> {
  None
  Some(T)
}

enum Token {
  Ident { name: String }
  Eof
}

local tok = Token::Ident { name: "hello" }

match tok {
  Ident { name } => name
  Eof => "eof"
}
```

Still not in this cut:

- multi-payload tuple variants
- relaxed / partial record patterns
- enum methods
- generic `where` bounds
- partial specialization / template metaprogramming

### 4.1 tuple multi-payload route stays boxed unless canonical sum changes

- current tuple multi-payload is not a parser-only gap
- AST / Stage1 Program JSON / JSON v0 bridge / MIR metadata all currently assume the canonical sum lane carries at most one payload slot
- under the lifecycle-value parent, land thin-entry inventory for known user-box / enum local routes before reopening tuple multi-payload
- when tuple multi-payload work resumes, prefer the same synthetic hidden payload box route already used by record variants
- do not widen `EnumCtor` / `EnumMatch` / `SumMake` / `SumProject` in the same cut unless a separate canonical-sum decision explicitly approves it

### 5. generic semantics start as declaration/type-fact support

- generic parameters belong in declarations and type references first
- initial runtime semantics may still erase/public-box at boundaries
- do not require reified generic runtime identity in MVP

### 6. exhaustiveness is limited to known-enum matches

- when the scrutinee is a known enum type, `match` must perform exhaustiveness checking
- when the scrutinee is not known-enum, keep current dynamic `match` behavior
- this keeps the new static win local to the closed-world enum lane

## Canonical MIR Direction

The target shape is a dedicated sum lane, not “just another box”.

Preferred canonical vocabulary:

- `sum.make`
- `sum.tag`
- `sum.project`

Expected lowering shape:

- `Type::Variant(payload)` lowers to `sum.make`
- `match enum_value { ... }` lowers to:
  - `sum.tag`
  - `switch_tag`
  - `sum.project` in matched payload arms

Policy:

- keep enum values unboxed as long as possible on the local/hot path
- box/publication only at explicit runtime boundaries
- do not force enum values into object layout as the first move

## MVP Implementation Order

### Step 1. Surface / parser / Stage1

Land:

- `enum Name<T...> { ... }`
- generic parameter lists on `enum` and `box`
- `Type::Variant(...)` constructor syntax
- narrow record declarations / constructors (`Ident { name: Type }`, `Type::Variant { name: expr }`)

Acceptance:

- parser / AST / Stage1 Program JSON can represent enum declarations and generic parameter lists
- reference docs are updated only when executable syntax lands

Status:

- landed for parser / AST / Stage1 inventory
- constructor surface is landed here; shorthand enum patterns were completed in Step 2
- the first record-variant cut is also landed on the same Stage1 route through synthetic hidden payload box declarations

### Step 2. Enum facts and known-match checking

Land:

- enum declaration registry / facts
- known-enum scrutinee recognition
- shorthand pattern resolution against the known enum
- exhaustiveness checking only for known-enum matches

Acceptance:

- dynamic `match` behavior outside known-enum contexts remains unchanged
- missing variant arms on known-enum matches fail fast

Status:

- landed for parser / AST / Stage1 known-match inventory
- current cut is intentionally narrow:
  - unit + single-payload variants only
  - guarded shorthand arms rejected
  - `_` does not satisfy exhaustiveness

### Step 3. Canonical MIR and lowering

Land:

- canonical sum ops (or a strictly equivalent canonical representation)
- match lowering via tag switch + payload project

Acceptance:

- no ad-hoc object-field encoding as the semantic truth
- MIR clearly distinguishes sum construction/tag/project from box/member operations

Status:

- landed on the narrow compiler-first lane:
  - MIR now has `SumMake` / `SumTag` / `SumProject`
  - JSON v0 bridge lowers Stage1 `EnumCtor` / `EnumMatch` into the dedicated sum lane
  - MIR JSON emit/parse preserves the same sum ops for handoff/debug
- landed on the MVP runtime/codegen lane too:
  - VM interpreter executes `SumMake` / `SumTag` / `SumProject` through a synthetic `__NySum_<Enum>` fallback `InstanceBox`
  - LLVM/Py builder registers the same synthetic runtime boxes before entry and lowers sum ops through `nyash.instance.*_field_h`
  - malformed tag projections fail fast (`[vm/sum:*]` on VM, `unreachable` on LLVM)
- still intentionally narrow in this step:
  - typed payload recovery on LLVM is only guaranteed when `payload_type` is concrete (`Integer` / `Bool` / `Float`)
  - LLVM now also recovers erased/generic payloads back to typed `Integer` / `Bool` / `Float` when `sum_make` can observe the actual payload family locally
  - unknown/genuinely dynamic payloads on LLVM still stay on boxed-handle fallback
  - product `ny-llvmc` ownership remains separate from this compat/harness slice

### Step 4. VM / LLVM / fallback runtime

Land:

- boxed/public fallback representation where needed
- VM / LLVM parity for MVP enum values and known-enum matches

Acceptance:

- current dynamic semantics outside enum paths remain stable
- enum MVP does not reopen string/primitive keeper lanes

Status:

- landed for the MVP runtime path
- backend/runtime truth stays implementation-only:
  - hidden runtime fields `__sum_tag` / `__sum_payload`
  - synthetic runtime box name `__NySum_<Enum>`
- LLVM now recovers erased/generic payloads back to typed `Integer` / `Bool` / `Float` when `sum_make` can observe a local payload fact
- unknown/genuinely dynamic payloads still stay on boxed-handle fallback
- product `ny-llvmc` ownership remains separate from this compat/harness slice

### Step 5. Record variants (first narrow cut)

Land:

- declaration surface `Ident { name: Type }`
- qualified construction `Type::Variant { name: expr }`
- known-enum shorthand record match `Ident { name } => ...`
- synthetic hidden payload box `__NyEnumPayload_<Enum>_<Variant>` on the source / JSON v0 route

Acceptance:

- enum values themselves stay on the existing sum lane (`SumMake` / `SumTag` / `SumProject`)
- Stage1 / JSON v0 root emits the hidden payload box metadata needed by VM / LLVM fallback routes
- constructors / patterns mention the declared field set exactly

Status:

- landed for the current source / JSON v0 route
- record payload lowering is implementation-only; surface truth stays the enum declaration
- block-bodied record shorthand arms and multi-payload variants remain deferred

## Deferred Backlog

- multi-payload variants
- relaxed / partial record patterns
- block-bodied record shorthand arms
- enum methods
- `where` bounds / capability constraints
- full generic monomorphization strategy
- full `::` migration for static methods outside enum constructors

## One-Line Rule

Use `enum` for closed tagged values, use `<T>` for generic surface syntax, keep `Type::Variant(...)` explicit at construction time, and delay runtime/template mechanics until after canonical sum meaning is fixed.
