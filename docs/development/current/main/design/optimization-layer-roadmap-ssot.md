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
   - first landed owner seam: folded `placement_effect_routes` inventory (`phase211x`)
   - landed fold-up follow-on: placement-relevant `agg_local` proof also reads through `placement_effect_routes` (`phase212x`)
   - landed first consumer proving slice: current sum outer-box sinking now seeds from `placement_effect_routes` first (`phase213x`)
   - landed second consumer proving slice: current user-box local aggregate seed now reads folded `placement_effect_routes` first, with thin-entry subject lookup kept as fallback (`phase214x`)
   - landed third consumer proving slice: current thin-entry consumer seed now reads folded `placement_effect_routes` first, with `thin_entry_selections` kept as fallback (`phase215x`)
   - landed fourth consumer proving slice: current sum local seed metadata helper now reads folded `placement_effect_routes` first on the boundary pure-first path, with legacy thin-entry/sum/agg-local metadata kept as fallback (`phase216x`)
   - landed fifth consumer proving slice: current boundary pure-first user-box micro seed helper now reads folded `placement_effect_routes` first, with legacy `thin_entry_selections` kept as fallback (`phase217x`)
   - landed first boundary structure slice: current boundary sum and user-box helpers now share one folded `placement_effect_routes` reader/matcher seam (`phase218x`)
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
   - landed first attrs seam: compat/probe keep builder now applies conservative `readonly` / `nocapture` runtime helper attrs at finalization
7. `numeric loop / SIMD`
   - induction normalization
   - reduction recognition
   - vectorization
   - fast-math / FMA
   - old `Float optimization` はこの layer の subtheme として読む
   - landed first seam: centralized LLVM vectorization policy (`loop_vectorize` / `slp_vectorize`)
   - landed proof seam: conservative numeric induction annotation over simple while plans (`phase263x`)
   - current proof seam: conservative reduction recognition over simple while plans (`phase264x`)
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
  - `numeric loop / SIMD`
   - `phase263x` is landed: the induction proof seam over simple while plans is closed out
   - `phase264x` is active: the next proof seam is conservative reduction recognition over simple while plans
- immediate follow-on after that:
  - `escape / barrier -> LLVM attrs`
