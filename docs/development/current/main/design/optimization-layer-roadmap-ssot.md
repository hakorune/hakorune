# Optimization Layer Roadmap SSOT

Status: SSOT
Date: 2026-04-12
Owner: `phase-163x optimization-resume`

## Purpose

- feature pilot と design layer を混ぜない
- `string` / `sum` / `user-box` / `array` / `map` は top-level row ではなく pilot として読む
- `.hako owner -> canonical MIR contract -> Rust mechanics -> LLVM generic optimization` の分業を roadmap に反映する

## Layer Order

1. `generic placement / effect`
   - string corridor / sum placement / thin-entry inventory-selection を pilot として fold する上位層
   - placement / publish / materialize / direct-kernel legality を generic に扱う
2. `agg_local scalarization`
   - `imm / borrow / agg_local / handle` の primary value classes を前提に、aggregate を scalar SSA に崩す
   - user-box local body / enum payload / tuple / record / closure env を同じ軸で扱う
3. `thin-entry actual consumer switch`
   - thin-entry inventory/selection を metadata に留めず、call site / entry ABI の actual consumer にする
   - known-receiver method / manifest / closure call をここへ fold する
4. `semantic simplification bundle`
   - `SCCP + SimplifyCFG + DCE + Jump Threading`
   - semantic value facts と CFG facts を使って canonical MIR を縮める
   - `DSE` は名前上の近接はあっても、この layer の owner には置かない
5. `memory-effect layer`
   - `dead store elimination`
   - `store-to-load forwarding`
   - `redundant load elimination`
   - `hoist / sink legality`
   - canonical `Store` / `Load` / future `store.array.str` / `store.map.value` をここで扱う
6. `escape / barrier -> LLVM attrs`
   - `nocapture`
   - `readonly`
   - `readnone`
   - `noalias`
   - MIR-side barrier vocabulary を LLVM attrs feed に繋ぐ
7. `numeric loop / SIMD`
   - induction normalization
   - reduction recognition
   - vectorization
   - fast-math / FMA
   - old `Float optimization` はこの layer の subtheme として読む
8. `closure split`
   - `capture classification`
   - `closure env scalarization`
   - `closure thin-entry specialization`
9. `IPO / build-time optimization`
   - `PGO`
   - `ThinLTO`
   - MIR-side semantic layersが先。ここは最後尾

## Pilot Mapping

- old `User-Box Method Dispatch`
  - top-level row ではなく `thin-entry actual consumer switch` の pilot
- old `Array Typed Slots`
  - `agg_local scalarization` の pilot
- old `MapBox Typed Value Slots`
  - `agg_local scalarization` と `memory-effect layer` の pilot
- old `DCE 強化`
  - `semantic simplification bundle` の一部
- old `LLVM Escape Analysis`
  - `escape / barrier -> LLVM attrs` の groundwork
- old `Float 最適化`
  - `numeric loop / SIMD` の subtheme
- old `Closure/Lambda 最適化`
  - `closure split` に分解して読む

## Wording Guardrails

- `DSE` は `semantic simplification bundle` の owner として書かない
  - 実装 owner は `memory-effect layer`
- `string` / `sum` / `user-box` / `array` / `map` を top-level roadmap row に戻さない
- `LLVM に頑張らせる` ではなく `LLVM が generic optimization できるように MIR の意味を揃える` と書く
- row の status は `pilot landed / partial / backlog` を分ける

## Immediate Read

- immediate code next:
  - `semantic simplification bundle` lane B2
  - overwritten `Store` pruning on definitely private carrier roots
- immediate follow-on after that:
  - lane C0 observer/control docs inventory
  - lane C1 `Debug` policy decision
- next major design lane after the current DCE slice:
  - `generic placement / effect`
  - then `agg_local scalarization`
