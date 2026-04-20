---
Status: provisional SSOT
Decision: provisional
Date: 2026-03-29
Scope: `concat3_hhh -> array.set -> trailing length()` を、store boundary と post-store observer を混ぜずに compiler-local placement window として読む
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/post-store-observer-facts-ssot.md
  - docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
  - docs/development/current/main/design/string-birth-placement-ssot.md
  - docs/development/current/main/investigations/perf-kilo-string-birth-hotpath-summary-2026-03-28.md
  - lang/c-abi/shims/hako_llvmc_ffi_string_concat_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_string_concat_lowering.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
  - tools/perf/run_kilo_string_trace_probe.sh
  - tools/perf/run_kilo_string_trace_asm_bundle.sh
---

# Concat3 Array Store Placement Window SSOT

## Goal

`concat3_hhh` の producer chain を、`array.set` の first `Store` boundary と trailing `length()` observer を混ぜずに読む。

この文書は runtime helper の追加や sink-local tuning の正本ではなく、**compiler-local placement window をどこで観測し、どこで次の cut を入れるか** の正本だよ。

## Why this exists

今の `perf-kilo` では、小さい leaf retry が何本も reject されている。

- `array.set + trailing length()` の boundary 同化は reject
- `known_len` 伝播だけの narrow retry も reject
- `StringViewBox` / `Registry::alloc/get` / `BoxBase::new` の leaf wave も stop-line

一方で trace+asm bundle の hot tier は、まだ次を残している。

- `nyash.string.concat_hh`
- `nyash.string.concat3_hhh`
- `nyash.string.substring_hii`
- `nyash.array.set_his`
- `nyash.array.string_len_hi`
- `nyash_kernel::exports::string::string_handle_from_owned`
- `nyash_rust::box_trait::BoxBase::new`

つまり、この wave の次の本命は leaf ではなく、**compiler-local lowering が `concat3` chain をどこで handle/store/observer へ落としているか** だよ。

## Core Reading

### Semantic boundary

- `array.set` は first `Store` boundary のまま
- trailing `length()` は `PostStoreUse::LenObserver`
- 両者を同じ semantic boundary にしない

### Optimization window

この wave で 1 つの window として扱うのは、次の chain だよ。

```text
array.get / substring / concat / concat3
  -> first Store boundary (`array.set`)
  -> trailing pure observer (`length()`)
```

semantic boundary は 2 つに分かれているが、**compiler-local placement の観測窓としては 1 クラスタで追う**。

## Compiler-Local Fact Sources

この wave で next cut の入口として使うのは、次の compiler-local tables / predicates だよ。

### Producer lineage

- `remember_string_concat_pair(...)`
- `remember_string_concat_triple(...)`
- `remember_string_substring_call(...)`
- `remember_string_length_call(...)`

役割:
- `concat pair / triple`
- `substring source`
- trailing `length()` adjacency

### Store boundary detection

- MIR metadata: `value_consumer_facts[*].direct_set_consumer`
- backend reader: `hako_llvmc_value_has_direct_set_consumer(...)`

役割:
- `array.set` が first `Store` boundary か
- まだ direct-set-only guard を保てているか

### Post-store observer detection

- `analyze_array_string_len_window_candidate(...)`

役割:
- `array.set` の後ろにある `length()` を observer として読む
- semantic boundary を widen せずに adjacency fact を見る

## Current Window Facts

この wave で必要なのは、新しい runtime token ではなく compile-time facts だけだよ。

```text
PlacedCarrierFacts = {
  producer_kind: ConcatPair | Concat3 | InsertMid | Substring | Unknown,
  boundary_kind: Store,
  retained_form: ReuseHandle | NeedOwnedHandle,
  post_store_use: None | LenObserver,
  known_len: Option<usize>,
  direct_set_only: bool,
}
```

### What each field means

- `producer_kind`
  - どの string recipe family 由来か
- `boundary_kind`
  - この wave では `Store` のみ
- `retained_form`
  - reuse で済むのか、owned handle が必要か
- `post_store_use`
  - trailing observer があるか
- `known_len`
  - carrier fact から observer に答えられるか
- `direct_set_only`
  - consumer widening を reopen せずに済むか

## Current Exact Rollout

1. `array.set` を first `Store` boundary として維持する
2. trailing `length()` は `PostStoreUse::LenObserver` のまま維持する
3. compiler-local lowering で `producer_kind / post_store_use / known_len / direct_set_only` を同じ window で観測する
4. trace+asm bundle で hot path と placement reason を同じ artifact で読む
5. same-artifact 改善が見えるまで、runtime leaf や new helper を増やさない

## What this doc owns

- `concat3_hhh -> array.set -> trailing length()` を 1 つの placement window として読むこと
- compiler-local でどの helper / table / predicate を観測源に使うか
- `array.set` と `length()` を semantic boundary ではなく optimization window として並べること
- next implementation lane を lowering-side facts へ寄せること

## What this doc does not own

- `BoundaryKind` / `RetainedForm` の taxonomy そのもの
- `freeze.str` の sink implementation
- `Registry::alloc/get`
- `BoxBase::new`
- `StringViewBox` / `StringBox` の layout tuning
- bench-specific helper widening

## Non-Goals

- `array.set + length()` を同じ store boundary に再定義すること
- `set_his` / `concat3_hhh` / `string_len_from_handle` に新しい leaf helper を増やすこと
- `StoreBoundaryKind` のような runtime-visible taxonomy を増やすこと
- `repeat=1` probe で keep/reject を決めること

## Acceptance

- next cut が runtime leaf ではなく compiler-local placement window から始まる
- `array.set` は first `Store` boundary のまま保たれる
- trailing `length()` は post-store observer のまま扱われる
- trace+asm bundle で `producer -> store -> observer` の同一 artifact 読みができる
- `CURRENT_TASK.md` と `10-Now.md` がこの window を active lane として読める
