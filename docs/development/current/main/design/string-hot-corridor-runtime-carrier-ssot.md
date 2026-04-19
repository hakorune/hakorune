---
Status: Provisional SSOT
Decision: provisional
Date: 2026-04-19
Scope: phase-137x hot owner evidence を、public ABI を広げずに exact / meso / whole front へ通る runtime-private text carrier stack として固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/transient-text-pieces-ssot.md
  - crates/nyash_kernel/src/exports/string_plan.rs
  - crates/nyash_kernel/src/exports/string_view/substring_plan.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs
  - crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs
  - crates/nyash_kernel/src/plugin/array_string_slot.rs
---

# String Hot Corridor Runtime Carrier SSOT

## Goal

- 設計を helper 名や file 名ではなく、実際に sample が乗った hot owner から始める。
- `public handle ABI` は維持したまま、runtime-private text carrier を current string corridor の正本として固定する。
- `exact -> meso -> whole` を別 front として扱いつつ、3 front に共通する structural defect を 1 つの carrier reading に落とす。
- 次の実装カードを「どの関数を C 化するか」ではなく「どの state を hot path に残してよいか」で選べるようにする。

## Perf-Locked Owner Reading

### Exact front

- front:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 131 ms`
- top owners:
  - `string_substring_concat_hhii_export_impl 22.38%`
  - `string_concat_hh_export_impl 21.70%`
  - array string-store closure `17.34%`
  - `from_i8_string_const 13.07%`
  - `LocalKey::with 6.07%`
- read:
  - copy body が first owner ではない
  - helper body の中で `objectize -> publish -> handle/TLS` が loop ごとに残っている

### Adopted middle

- front:
  - `kilo_meso_substring_concat_array_set_loopcarry`
  - latest local reread: `Ny AOT 53 ms`
- worker audit owners:
  - `substring_hii ~26-28%`
  - `borrowed_substring_plan_from_handle ~12-13%`
  - `LocalKey::with ~13%`
  - `string_substring_concat3_hhhii_export_impl ~12-13%`
  - `execute_store_array_str_contract ~11-12%`
- read:
  - store/publication tail だけではなく、source proof を handle world から毎回引き直す cost が visible
  - exact と whole の間に、`borrowed source continuity` という別 owner が見えている

### Whole front

- front:
  - latest local reread: `kilo_kernel_small_hk = 733 ms`, `736 ms`
- worker audit owners:
  - `nyash.string.concat_hs ~10-11%`
  - `execute_store_array_str_contract ~5-7%`
  - `array_get_index_encoded_i64 ~3-4%`
  - `insert_const_mid_fallback ~2-4%`
  - libc `memmove ~15-19%`
  - `_int_malloc ~4-5%`
- read:
  - copy/alloc pressure is real, but first structural owner は `retarget/publication tail` と `const_suffix/Pieces3` publish
  - allocator/gc だけを先に C 化しても、object-world re-entry を消した証明にはならない

## Shared Diagnosis

3 front が同じ関数で熱いわけではない。
共通しているのは次の structural defect だよ。

- borrowed/view できる text が、corridor の途中で public handle/object world に戻っている
- hot path が `Arc/StringBox/handle registry/TLS publish` の都合を背負っている
- exact では helper body の publish tail として現れ、
- meso では source plan を handle world から引き直す形で現れ、
- whole では retarget/publication tail と `const_suffix/Pieces3` publish として現れる

つまり問題は「Rust が遅い」でも「Arc が絶対悪」でもなく、
`same-corridor text` を steady-state で public object world に戻していることだよ。

## Canonical Carrier Stack

この lane で固定する carrier reading は次。
これは新しい public enum の導入要求ではなく、既存 shape に対する state machine の読み方だよ。

| Corridor state | Semantic state | Current backing / adapter | Main owner front | Hot-path allowed | Hot-path forbidden |
| --- | --- | --- | --- | --- | --- |
| `BorrowedSource` | future `TextRef` / `AliasRef` | `VerifiedTextSource`, `SourceLifetimeKeep`; `BorrowedHandleBox` only as boundary/cache | meso, whole | source proof, alias retarget, source handle/epoch update | `StringBox` birth, fresh handle issue, generic publish |
| `TransientPlan` | `TextPlan<'a>` | `TextPiece<'a>`, `StringSpan` | exact, meso, whole producer sites | piece normalize, known-len carry, concat/substring planning | registry lookup as steady-state carrier, `Arc` wrapping, object identity |
| `OwnedText` | future `OwnedText` | `OwnedBytes`; `KernelTextSlot(state=OwnedBytes)` only as transport | exact, whole | freeze once, pass through caller-owned slot, same-corridor read | registry-backed unpublished carrier, immediate objectize |
| `TextCellResidence` | sink/residence only, not semantic corridor value | current `KernelTextSlot` sink seed | exact, whole | cell residence without public handle | growing `KernelTextSlot` into a public text ABI |
| `PublishedPublic` | object-world stable text | `StringBox`, `Arc<dyn NyashBox>`, fresh handle; `StringViewBox` as boundary view | true external boundary only | stable objectize, handle issue, public ABI replay | re-enter hot loop as default carrier |

Reading lock:

- do not introduce a public `TextBuf` on this lane
- do not treat `KernelTextSlot` as a user-visible string API
- do not treat `BorrowedHandleBox` as semantic `TextRef`
- do not treat `StringViewBox` as the internal substring carrier
- do not widen `TextCell` from sink/residence into a corridor value
- do not read future `TextLane` storage as semantic truth
- do not reuse the host-handle registry as the unpublished carrier
- do not move legality ownership out of MIR/lowering into runtime re-recognition

Bridge lock:

- object -> text bridge is `borrow.text_from_obj`
- text -> object bridge is `publish.text(reason, repr)`
- `reason` and `repr` are different operands and must not be collapsed into helper-name truth
- `publish.any` is deferred until string-only `publish.text` proves out

## Corridor Rules

### 1. Borrowed source must stay borrowed until proof is lost

- `execute_store_array_str_contract` and substring planning may carry verified source proof as `BorrowedSource`.
- If alias retarget succeeds, the lane stays in `BorrowedSource`.
- Retarget hit must not pay `StringBox -> Arc -> handle` just to continue in the same corridor.

### 2. `TextPlan` is planning truth, not publication truth

- `TextPlan` / `PiecesN` is the normalized transient carrier.
- It may decide lengths, pieces, and copy shape.
- It must not become an observable Box or registry-owned runtime value.

### 3. When copy is required, freeze into `OwnedBytes` first

- If the lane needs an owned result but not yet a public object, freeze into `OwnedBytes`.
- Preferred transport is caller-owned `KernelTextSlot`.
- `OwnedBytes` is the minimal unpublished text carrier on this lane.
- `TextPlan` does not publish directly; if a plan cannot stay borrowed, it freezes first.

### 4. Publish only at a real boundary

- `StringBox` / `Arc` / fresh handle issue belong to `PublishedPublic`.
- They are cold-adapter work.
- v1 bridge is `publish.text(reason, repr)` only.
- They are legal only at:
  - first true external boundary
  - stable object identity demand
  - public ABI replay that cannot stay slot-local
- typical v1 reprs are:
  - stable owned result
  - stable borrowed view result

### 4.5. Read-side alias lane stays cache-backed and cold

- common `array.get` path must stay on:
  - `TextReadOnly`
  - `EncodedAlias`
- `StableObject` is the explicit identity/public boundary branch
- stable objectize must be cache-backed and cold, not per-read fresh promotion
- if a card improves store-side continuity but reintroduces per-read stable object creation, reject it

### 5. Registry/TLS is cold adapter, not steady-state carrier

- handle registry read/write
- `LocalKey::with`
- publish counters / handle issue
- caller-latest-fresh tagging

These stay valid runtime mechanics, but the carrier design must treat them as cold boundary cost.

### 6. No shared-helper widening before the carrier contract proves out

- keep the first landing corridor-local or site-local
- do not widen generic `string_handle_from_owned`-style helpers
- do not retry registry-backed deferred carriers
- do not retry transient box/handle carriers
- first landing on this lane is explicitly narrowed further:
  - do not introduce a general-purpose `TransientText` enum yet
  - reuse `KernelTextSlot` as the first sink-local unpublished carrier
  - first substrate is `const_suffix -> KernelTextSlot`, then `KernelTextSlot -> store.array.str`
  - compiler/backend slot-consumer lowering is a separate card after the runtime substrate lands

## Front-Specific Implications

### Exact

- current owner:
  - generic publish/objectize behind `string_concat_hh` and `string_substring_concat_hhii`
- design consequence:
  - helper result should stay in `OwnedText` when the next consumer is still the same corridor
  - site-local const/substring pieces should prefer `TextPlan` and slot-local freeze over immediate publish
- success signal:
  - exact `site.string_concat_hh.*` / `site.string_substring_concat_hhii.*` publish counters fall
  - helper-body asm loses handle/TLS heavy tail without changing public ABI

### Adopted middle

- current owner:
  - `borrowed_substring_plan_from_handle`
  - `substring_hii`
  - `LocalKey::with`
- design consequence:
  - if source proof already exists as `BorrowedSource` or `OwnedText`, substring planning must consume that first
  - `handles::with_handle` becomes fallback, not the first branch
- decision role:
  - this front is the contradiction guard
  - if whole-first cards stay neutral here, the next implementation must move to direct source/slot planning

### Whole

- current owner:
  - borrowed-slot retarget/publication tail under `execute_store_array_str_contract`
  - `const_suffix`
  - `freeze_text_plan(Pieces3)`
- design consequence:
  - keep store retarget entirely in `BorrowedSource` until proof is lost
  - freeze `const_suffix/Pieces3` into `OwnedText`, then publish from a cold site adapter only when needed
  - do not bypass the existing `set_his` fast path
- current note:
  - the landed cold retirement sink in `keep_borrowed_string_slot_source_keep` is prework, not the full carrier redesign

## Code-Side Ownership Map

- `crates/nyash_kernel/src/exports/string_view/substring_plan.rs`
  - `handle -> borrowed source -> transient plan` bridge
  - should not become objectize/publish owner
- `crates/nyash_kernel/src/exports/string_plan.rs`
  - `TextPlan` / `PiecesN` normalization only
  - not a legality owner and not a public carrier
- `crates/nyash_kernel/src/exports/string_search.rs`
  - search / compare read consumers only
  - may keep `TextRef` longer inside runtime-private helper closures, but is not a carrier owner
- `crates/nyash_kernel/src/plugin/value_codec/text_carrier.rs`
  - runtime-private `TextRef` / `OwnedText` vocabulary only
  - names the semantic carrier roles without turning `KernelTextSlot` into `TextCell`
- `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`
  - `BorrowedSource` mechanics only
  - owns keep replacement / alias retarget bookkeeping
- `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
  - `OwnedText -> PublishedPublic` sink
  - owns `KernelTextSlot` transport and cold publish adapter split; slot `OwnedBytes`
    state naming remains a transport detail, not the semantic carrier name
- `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - whole-front store consumer and source-capture contract
  - must preserve `set_his` fast-path legality

## Implementation Order

### Card A. Whole-first borrowed-source continuity

- target:
  - `try_retarget_borrowed_string_slot_take_verified_text_source`
  - `keep_borrowed_string_slot_source_keep`
- goal:
  - finish the `BorrowedSource` reading so retarget hit does not drift into publish/object world
- note:
  - current ptr-eq / cold-retire landing is valid prework
  - it is not yet the proof that the carrier state is correct end-to-end

### Card B. Whole producer tail to `OwnedText`

- target:
  - `const_suffix`
  - `freeze_text_plan(Pieces3)`
- goal:
  - freeze into `OwnedBytes` / `KernelTextSlot`
  - keep objectize/handle issue in a site-local cold adapter
- narrowed first cut:
  - land `const_suffix -> KernelTextSlot` before widening to general `Pieces3` transport
  - treat this as runtime-private substrate, not full corridor completion
- reject:
  - shared-helper widening
  - registry-backed unpublished carrier

### Card C. Meso direct source/slot planning

- target:
  - `substring_hii -> borrowed_substring_plan_from_handle`
- goal:
  - if the lane already owns `BorrowedSource` or `OwnedText`, skip `handles::with_handle`
  - keep this as the next card if whole-first stays neutral on meso

### Card D. Exact confirmation only after continuity is real

- target:
  - exact helper publish tails
- goal:
  - use the same carrier continuity to shrink exact
  - do not start with helper-name-specific C rewrites

### Card E. Explicit publish bridge lock

- target:
  - `publish.text(reason, repr)` string-only contract
  - `borrow.text_from_obj` provenance lock
- goal:
  - make MIR/lowering the owner of boundary/provenance truth
  - keep runtime as boundary executor only
  - prove `substring_hii` as the first `StableView` replay site
- defer:
  - `publish.any`

## C-Rewrite Reading

If C is used as a proving tool, the right target is leaf copy/freeze work only.

Useful C probes:

- subrange copy leaf
- concat3 into reserved buffer
- site-local freeze leaf

Misleading first probes:

- `Arc<StringBox>` replacement
- handle registry
- publish/reissue bookkeeping
- alias-retarget ownership logic

Those may run faster in C while still proving the wrong thing.
The design question on this lane is carrier continuity, not “who writes the refcount code”.

## Non-Goals

- no public `TextBuf`
- no broad `Arc<StringBox>` removal campaign
- no public ABI widening
- no runtime legality re-recognition
- no generic string helper framework
- no transient box/handle carriers
- no reopening of rejected slot-store boundary probes

## Acceptance Rule

Keep using `exact -> meso -> whole` in that order.

- exact must not regress
- meso must confirm that whole-first cuts still respect borrowed-source continuity
- whole must show repeated-window improvement, not a single noisy sample

And on every card, the design review question is the same:

- which of `BorrowedSource / TransientPlan / OwnedText / PublishedPublic` is this code operating on?
- if it publishes, what exact boundary forced it?

If that answer is fuzzy, the cut is not ready.
