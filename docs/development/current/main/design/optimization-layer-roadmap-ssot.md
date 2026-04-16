# Optimization Layer Roadmap SSOT

Status: SSOT
Date: 2026-04-16
Owner: `phase-163x optimization-resume`

## Purpose

- feature pilot と design layer を混ぜない
- `string` / `sum` / `user-box` / `array` / `map` は top-level row ではなく pilot として読む
- `.hako owner -> canonical MIR contract -> Rust mechanics -> LLVM generic optimization` の分業を roadmap に反映する
- current row を 1 次元の列として読まず、`substrate / producer / exporter / consumer` の 2 軸で読む

## Two-Axis Reading

Do not read the current optimization architecture as one flat row list.

- first classify whether a row is:
  - `substrate owner`
  - `fact producer`
  - `exporter`
  - `consumer`
- then decide rollout order inside that role
- `LLVM attrs`, `C ABI corridor`, `ThinLTO`, and `PGO` are not authority rows
- `closure split` and numeric-loop proofs are first-class producers, not only late consumers

## Substrate Order

1. `canonical simplification substrate`
   - `SCCP + SimplifyCFG + DCE + Jump Threading`
   - semantic value facts と CFG facts を使って canonical MIR を縮める
   - `DSE` は名前上の近接はあっても、この layer の owner には置かない
2. `value representation / materialization / scalarization substrate`
   - old `generic placement / effect` と `agg_local scalarization` を同じ substrate family として読む
   - placement / publish / materialize / direct-kernel legality を generic に扱う
   - `imm / borrow / agg_local / handle` の primary value classes を前提に、aggregate を scalar SSA に崩す
   - user-box local body / enum payload / tuple / record / closure env を同じ軸で扱う
   - first landed owner seam: folded `placement_effect_routes` inventory (`phase211x`)
   - landed fold-up follow-on: placement-relevant `agg_local` proof also reads through `placement_effect_routes` (`phase212x`)
   - landed first consumer proving slice: current sum outer-box sinking now seeds from `placement_effect_routes` first (`phase213x`)
   - landed second consumer proving slice: current user-box local aggregate seed now reads folded `placement_effect_routes` first, with thin-entry subject lookup kept as fallback (`phase214x`)
   - landed fourth consumer proving slice: current sum local seed metadata helper now reads folded `placement_effect_routes` first on the boundary pure-first path, with legacy thin-entry/sum/agg-local metadata kept as fallback (`phase216x`)
   - landed fifth consumer proving slice: current boundary pure-first user-box micro seed helper now reads folded `placement_effect_routes` first, with legacy `thin_entry_selections` kept as fallback (`phase217x`)
   - landed first boundary structure slice: current boundary sum and user-box helpers now share one folded `placement_effect_routes` reader/matcher seam (`phase218x`)
3. `memory / effect substrate`
   - `dead store elimination`
   - `store-to-load forwarding`
   - `redundant load elimination`
   - `hoist / sink legality`
   - escape / alias / modref / barrier vocabulary はこの substrate family の入力として読む
   - canonical `Store` / `Load` / future `store.array.str` / `store.map.value` をここで扱う
4. `call surface substrate`
   - old `thin-entry actual consumer switch` と `handle ABI -> value ABI` を同じ substrate family として読む
   - thin-entry inventory/selection を metadata に留めず、call site / entry ABI の actual consumer にする
   - known-receiver method / manifest / closure call / arg-ret ABI class をここへ fold する
   - landed third consumer proving slice: current thin-entry consumer seed now reads folded `placement_effect_routes` first, with `thin_entry_selections` kept as fallback (`phase215x`)

## Fact Producers

1. `loop proof producer`
   - induction normalization
   - reduction recognition
   - vectorization legality proof
   - landed first seam: centralized LLVM vectorization policy (`loop_vectorize` / `slp_vectorize`) (`phase262x`)
   - landed proof seam: conservative numeric induction annotation over simple while plans (`phase263x`)
   - landed proof seam: conservative reduction recognition over simple while plans (`phase264x`)
   - landed contract seam: LoopSimdContract proof / policy / lowering owner split (`phase265x`)
2. `closure proof producer`
   - capture classification
   - closure env scalarization eligibility
   - closure thin-entry eligibility
   - landed owner seam: closure creation now reads a shared capture classification contract before env scalarization / thin-entry specialization (`phase269x`)
   - landed owner seam: single-capture envs are now classified as scalarizable while aggregate env lowering remains unchanged (`phase270x`)
   - landed owner seam: empty/single envs are now classified as thin-entry candidates while ctor lowering remains unchanged (`phase271x`)
3. `language-level optimization metadata`
   - `@rune Hint/Contract/IntrinsicCandidate`
   - current reading stays `parse/noop` until MIR owner fields, consumers, and diagnostics are ready
4. `float math policy`
   - reassociation / fast-math / FMA / exception model stay above LLVM consumers
   - old `Float optimization` no longer reads as a subtheme of `numeric loop / SIMD`
   - keep this row deferred until language-level FP policy is fixed

## Exporters And Consumers

1. `LLVM fact export`
   - `readonly`
   - `nocapture`
   - future `readnone`
   - future `noalias`
   - future `alias.scope` / `parallel_accesses` / TBAA
   - exporter only; not an authority row
   - landed first attrs seam: compat/probe keep builder now applies conservative `readonly` / `nocapture` runtime helper attrs at finalization (`phase261x`)
2. `boundary / C ABI export`
   - `.hako -> ny-llvmc(boundary) -> C ABI`
   - boundary stub / runtime helper / manifest export row
   - exporter only; not an authority row
3. `numeric loop / SIMD consumer`
   - consumes loop proofs plus memory/effect facts
   - landed widening seam: integer map loops now emit the first conservative `llvm.loop` vectorization hint under LoopSimdContract (`phase266x`)
   - landed widening seam: integer sum reductions now emit the next conservative `llvm.loop` vectorization hint under LoopSimdContract (`phase267x`)
   - landed widening seam: compare/select candidates now emit the next conservative `llvm.loop` vectorization hint under LoopSimdContract (`phase268x`)
4. `IPO / build-time consumer`
   - landed owner seam: LLVM/Python build options now have one shared policy owner before `ThinLTO` / `PGO` widening (`phase272x`)
   - landed owner seam: IPO now owns callable-node facts and call-edge facts before `ThinLTO` or `PGO` consume closure-thin facts (`phase273x`)
   - landed cut: `ThinLTO` first cut now emits a companion `.thinlto.bc` when thin mode is requested and landed callable/edge seams prove direct-thin import candidates (`phase274x`)
   - consumer only; not an authority row
5. `PGO hotness overlay`
   - landed owner seam: `PGO` scaffold now has one dedicated owner while generate/use behavior remains disabled (`phase275x`)
   - landed cut: `PGO` generate/use first cut now resolves generate/use artifacts and emits a `.pgo.json` sidecar while keeping LLVM-side instrumentation/use out of scope (`phase276x`)
   - overlay only; keep after call-surface stabilization and IPO seams
6. `optimization export verifier`
   - cross-cutting verifier for MIR contract -> LLVM/C-boundary export consistency
   - needed before widening strong attrs / metadata rows such as `noalias`, `parallel_accesses`, and TBAA
   - current state: backlog

## Closeout

- optimization roadmap closeout accepted (`phase277x`)
- old flat-row roadmap is historically accepted, but current reading is now `substrate / producer / exporter / consumer`
- next mainline work returns to compiler expressivity / selfhost mirbuilder under the existing selfhost migration SSOTs

## Pilot Mapping

- old `User-Box Method Dispatch`
  - top-level row ではなく `call surface substrate` の pilot
- old `Array Typed Slots`
  - `value representation / materialization / scalarization substrate` の pilot
- old `MapBox Typed Value Slots`
  - `value representation / materialization / scalarization substrate` と `memory / effect substrate` の pilot
- old `DCE 強化`
  - `canonical simplification substrate` の一部
- old `LLVM Escape Analysis`
  - `memory / effect substrate` と `LLVM fact export` の groundwork
- old `Float 最適化`
  - `float math policy` の deferred row
- old `Closure/Lambda 最適化`
  - `closure proof producer` と `call surface substrate` に分解して読む

## Wording Guardrails

- `DSE` は `canonical simplification substrate` の owner として書かない
  - 実装 owner は `memory / effect substrate`
- `string` / `sum` / `user-box` / `array` / `map` を top-level roadmap row に戻さない
- `LLVM attrs` / `C ABI corridor` / `ThinLTO` / `PGO` を authority row として書かない
- `LLVM に頑張らせる` ではなく `LLVM が generic optimization できるように MIR contracts を揃える` と書く
- `float` を early SIMD widening の subtheme として書かない
- row の status は `substrate / producer / exporter / consumer` の role と一緒に読む

## Immediate Read

- immediate code next:
  - `compiler expressivity first`
- immediate follow-on after that:
  - `phase-29bq` failure-driven blocker capture
