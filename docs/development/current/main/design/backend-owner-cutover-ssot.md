---
Status: SSOT
Decision: provisional
Date: 2026-03-28
Scope: `MIR -> backend owner` の切り直しを structure-first で進めるための正本。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
---

# Backend Owner Cutover (SSOT)

## Goal

- `AST -> LLVM` 直結ではなく、`MIR -> backend owner` の seam を置き直す。
- canonical seam は MIR のまま固定する。
- current C `.inc` mainline owner を細く compare/debug lane へ後退させる。
- future mainline owner 候補を `.hako ll emitter` に移す。

## Fixed Reading

1. `.hako` compiler authority は MIR までの semantic owner だよ。
2. backend owner cutover は `stage` 軸ではなく `owner` 軸の作業だよ。
3. current wave では distinct `AOT-Core MIR` layer を追加しない。
4. current wave では public ABI も public MIR JSON schema も増やさない。
5. compat `lang/src/llvm_ir/**` は daily owner に戻さない。

## End-State Shape

```text
.hako compiler
  -> canonical MIR
  -> analysis-only RecipeFacts
  -> runtime_decl manifest
  -> .hako ll emitter
  -> .ll text
  -> opt/llc
  -> object/exe
```

## Seam Vocabulary

- canonical seam = `MIR`
- evidence seam = `MIR JSON`
- tool seam = `.ll`

Current wave keeps all 3, but their roles are different.

- `MIR` remains the semantic SSOT.
- `MIR JSON` remains debug/fixture/export evidence.
- `.ll` is now the thin Rust/LLVM tool boundary.
- launcher/mainline daily transport is now root-first; temp MIR JSON handoff is evidence-only, not compile transport.

current C `.inc` lane は次の扱いに固定する。

- daily owner: no
- explicit compare/debug lane: yes
- compat keep lane: yes
- `route.rs` is now compare/archive-only; it no longer owns the daily hako-ll bridge lane.

## Fixed Order

1. `runtime_decl_manifest_v0`
2. `recipe_facts_v0`
3. `.hako ll emitter min v0`
4. explicit compare bridge
5. boundary-only narrow owner flip
6. archive/delete sweep
7. thin `.ll` tool boundary
8. `.hako` MIR root entry or equivalent root hydrator
9. daily `.hako ll emitter` profiles cut from `compile_json_path` to `compile_ll_text` (landed)
10. launcher/mainline daily path switches from temp MIR path handoff to root-first compile (landed)
11. `route.rs` compare/archive shrink (landed)
12. compare bridge retirement / archive decisions
13. その後に structural perf だけ reopen

## Runtime Decl Manifest Rule

- backend-private declare truth は 1 か所に寄せる。
- current SSOT file は `docs/development/current/main/design/runtime-decl-manifest-v0.toml`。
- row の最小語彙は次で固定する。
  - `symbol`
  - `args`
  - `ret`
  - `attrs`
  - `memory`
  - `lanes`
- public ABI semantics は引き続き `abi-export-manifest-v0.toml` が正本だよ。

## Recipe Facts Rule

- facts は analysis-only sidecar であり、MIR rewrite はしない。
- current v0 vocabulary は次で固定する。
  - `value_class`
  - `bool_i1`
  - `effect`
  - `cold_fallback`
  - `direct_call_ok`
  - `reject_reason`
- lowering / emitter は facts を consume するだけに寄せる。

## Compare Lane Rule

- compare lane は explicit opt-in だけで呼ぶ。
- silent fallback は禁止。
- current explicit lane は `.hako ll emitter min v0` と C `.inc` owner の比較用に使う。
- compare lane は temporary bridge であって常設 mainline route ではない。
- compare lane の minimum evidence は次で固定する。
  - `chosen_owner`
  - `accepted`
  - `first_blocker`
- route.rs now only selects explicit legacy compare/archive paths.

## Subtraction Queue

- owner cutover の primary goal は live owner surface を減らすことだよ。
- shape が daily owner flip したら、その shape の legacy C `.inc` daily route は同 commit で retired として扱う。
- boundary-only wave では、daily owner flip 済み shape を `phase29ck` default suite から外し、必要なら temporary legacy suite へ退避する。
- bridge route payload keeps `acceptance_case`, `transport_owner`, and `legacy_daily_allowed` visible until compare retirement; daily `.hako ll emitter` shapes must report `legacy_daily_allowed=no`.
- root-hydrator compat is allowed as an intermediate step, but it stays parse/coercion only and does not become a second MIR semantics owner.
- full transport cut is allowed only after launcher/mainline can hand backend work to facts/emitter without relying on a temp MIR JSON path as the daily compile transport.
- delete/archive 候補の追跡は `phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md` を正本にする。
- preservation-first SSOT を満たさない surface は、demote はしても即 delete しない。

## Non-Goals

- `AST -> LLVM IR` 直結
- `lang/src/llvm_ir/**` compat emit route の daily 復活
- broad fast-leaf widening
- full `AOT-Core MIR`
- unboxed value representation の即時導入

## Acceptance

- `CURRENT_TASK.md` / `10-Now.md` / `phase-29x/README.md` が同じ fixed order を持つ
- compare lane stays explicit bridge-only
- boundary-only wave では daily owner flip した shape が `.hako ll emitter` を使い、同 shape の legacy daily route は retired として ledger に残る
- structural perf work only reopens after compare lane is explicit and facts/manifest are visible
