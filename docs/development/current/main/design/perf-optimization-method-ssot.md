---
Status: SSOT
Scope: `kilo` / `micro kilo` を起点にした exe 最適化の測定順序・判断順序・止め線
Related:
- docs/development/current/main/DOCS_LAYOUT.md
- docs/development/current/main/design/optimization-task-card-os-ssot.md
- docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md
- docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md
- docs/development/current/main/design/optimization-tag-flow-ssot.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md
- docs/development/current/main/design/stage2-optimization-debug-bundle-ssot.md
- docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
- docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
- docs/development/current/main/design/stage2-string-route-split-plan.md
- docs/development/current/main/design/transient-text-pieces-ssot.md
- docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
- docs/development/current/main/design/post-store-observer-facts-ssot.md
- docs/development/current/main/design/concat3-array-store-placement-window-ssot.md
- docs/development/current/main/design/kilo-meso-benchmark-ladder-ssot.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/phases/phase-29ck/README.md
- docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-29ck/P8-PERF-REOPEN-JUDGMENT.md
- docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md
---

# Perf Optimization Method SSOT

## Goal

この文書は、`.hako` / C ABI / Rust bridge / micro leaf をまたぐ exe 最適化を、毎回同じ手順で進めるための正本だよ。

目的は 2 つだけ。

1. whole-program の差をまず安定 baseline で固定する。
2. そこから micro leaf を 1 本ずつ exact に削る。

## Task-Card OS Anchor

live optimization work is now governed by
[optimization-task-card-os-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-task-card-os-ssot.md).

This doc owns:

- measurement order
- benchmark/asm judge
- live restart card placement

It does not replace the task-card OS fields themselves.
For every live cut, read these together:

1. [optimization-task-card-os-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-task-card-os-ssot.md)
2. this document
3. [llvm-line-ownership-and-boundary-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md)

## Explicit Task Card Rule

最適化 lane は `kilo` / `micro kilo` のような曖昧な名前だけで再開してはいけない。

reopen した lane には、必ず次の task card を先に固定する。

1. `front`
2. `accept gate`
3. `whole-kilo guard`
4. `primary owner`
5. `proof delta`
6. `rewrite target`
7. `owner scope`
8. `first commands`
9. `done condition`
10. `reject condition`

運用ルール:

- `CURRENT_TASK.md` には live な task card を短く書く
- phase README には target band / historical evidence / rejected variants を残す
- `front` だけあって `first commands` や `reject condition` が無い状態では code edit を始めない
- `kilo / micro-kilo` だけの表現は pointer として不十分なので、そのままでは restart handoff に使わない

## Current Re-entry Task Card

current restart pointer after the active selfhost landing is this one:

- note: the old pre-proof runtime-executor tail-thin card below is now historical
- landed:
  - `mir-proof` first fixed `publish_now_not_required_before_first_external_boundary`
    as MIR-owned plan metadata
- next live owner:
  - `runtime-executor`

- lane owner: `phase-137x`
- front: `kilo_micro_substring_concat`
- accept gate: `kilo_micro_substring_only`
- split keeper checks:
  - `kilo_micro_substring_views_only`
  - `kilo_micro_len_substring_views`
- whole-kilo guard: `kilo_kernel_small_hk`
- route contract: `.hako -> ny-llvmc(boundary pure-first) -> C ABI`
- primary owner: `runtime-executor`
- proof delta:
  - `piecewise_publication_tail_delete`
- proof region:
  - established facts:
    - borrowed corridor may stay unmaterialized until the final consumer
    - the active corridor is non-escaping
    - the active corridor does not cross a public boundary
    - the active exact front is already 100% on the landed fast path:
      - `piecewise_subrange single_session_hit=300000`
      - `piecewise_subrange fallback_insert=0`
      - `piecewise_subrange all_three=300000`
  - region limits:
    - active `kilo_micro_substring_concat` corridor only
- publication boundary:
  - applies only to:
    - the active corridor selected by the landed MIR rewrite
  - publish as:
    - runtime-private executor only
  - require:
    - the landed MIR publication contract before deferred piecewise publication is used
  - must not touch:
    - generic `insert_hsi` / `insert_const_mid_fallback` helper body semantics
    - public ABI
    - broad callers outside the active corridor
  - must not become:
    - a generic helper rewrite
- rewrite target:
  - from: eager publication tail inside the active `piecewise_subrange_hsiii` corridor
  - to: runtime-private outcome/publication split that consumes the landed MIR contract
- generic contract term:
  - `same-corridor unpublished outcome`
- lane-specific realization:
  - see
    [string-canonical-mir-corridor-and-placement-pass-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md)
    and
    [phase-137x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-137x/README.md)
  - this lane currently realizes that contract as a string-lane unpublished text outcome; do not genericize helper/executor mechanics here
- runtime executor:
  - consume the landed MIR publication contract; no new proof on this card
  - split runtime-private freeze vs publish
  - keep generic/public handle publication as cold adapter
  - exact touch set:
    - `crates/nyash_kernel/src/exports/string_helpers/concat/piecewise.rs`
    - `crates/nyash_kernel/src/exports/string_helpers/materialize.rs`
    - `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
  - must not touch:
    - `src/runtime/host_handles.rs`
    - `src/runtime/host_handles/text_read.rs`
    - public export signatures in `crates/nyash_kernel/src/exports/string.rs`
  - forbid runtime/shim route re-recognition, transient box/handle carriers, generic helper widening, registry-backed deferred carriers, or public-ABI changes on this card
- owner scope: follow `Owner Scope / Publication Boundary` below; start from `string_helpers/concat` / `string_view` / `host_handles` only if asm top still points there
- first commands:
  - `tools/checks/dev_gate.sh quick`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3`
  - `bash tools/perf/report_mir_hotops.sh kilo_micro_substring_concat`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat 'piecewise_subrange_hsiii' 3`
  - `NYASH_PERF_COUNTERS=1 <exact-front aot exe>`
- pre-probe rule:
  - if compiler sources changed, refresh release artifacts before exact/asm probes
- done condition:
  - runtime-private outcome/publication split consumes the landed MIR publication contract
  - no runtime legality or publication rediscovery is needed for the active corridor
  - exact front no longer pays eager publication as a semantic requirement
  - next-iteration pure-string consumers stay on the landed `piecewise_subrange_hsiii` route
- reject condition:
  - the slice adds a new MIR dialect, public ABI widening, or generic helper legality
  - runtime/shim starts deciding publication legality or corridor shape
  - the slice escapes its publication boundary and becomes a broad helper rewrite
  - the slice repins next-iteration consumers to generic `insert_hsi -> substring_hii`
- current status:
  - consult reviewed
  - landed `mir-proof` closed the publication contract gap
  - next live owner is `runtime-executor`
  - local executor-only thin cuts stay paused unless they preserve the landed piecewise route and the same-corridor unpublished-outcome contract

## Representation Escalation Rule

If all of these hold on the same exact front:

- route/publication counters are already stable
- fallback selection is no longer the blocker
- repeated executor-local thin cuts are non-wins
- the remaining counters stay pinned on final materialize/objectize/handle issue

then the next action is not another thin cut.

Escalate to a focused design consult on:

- handle-based public surface
- final `owned String -> boxed handle`
- runtime-private result representation / result ABI

If that consult is accepted, the return order is fixed:

1. `mir-proof`
   - lock `publish-now not required before first external boundary`
2. `runtime-executor`
   - split runtime-private freeze vs publish on the active corridor only
3. `llvm-export`
   - only after the runtime-private outcome/publication split is stable

Reading lock:

- this is not a route/proof/publication failure
- this is not permission to widen generic helper bodies
- keep the public surface fixed until a new explicit representation/ABI card is accepted

## Current Counter Reading Lock

for the active `phase-137x` front, read the current counters like this:

- `str.substring.route total=0`
- `slow_plan=0`
- `slow_plan_view_span=0`
  - old substring route is no longer the blocker
- `piecewise_subrange total=300000`
- `piecewise_subrange single_session_hit=300000`
- `piecewise_subrange fallback_insert=0`
- `piecewise_subrange all_three=300000`
  - active front already stays entirely on the landed single-session three-piece fast path
  - fallback selection and piece-shape branching are not the next delete target
- `birth.backend materialize_owned_total=300000`
- `birth.backend string_box_new_total=300000`
- `birth.backend arc_wrap_total=300000`
- `birth.backend handle_issue_total=300000`
  - next delete target is the executor-local tail:
    - final owned materialize
    - `StringBox` / `Arc` objectize
    - fresh handle issue

## Current Test Gate Note

- use `cargo test -q -p nyash_kernel --lib -- --test-threads=1` as the deterministic library acceptance gate for this lane
- treat parallel `cargo test -q -p nyash_kernel --lib` as monitor-only until cache isolation work removes cache/view test flakiness

## Current Scheduling Status

- `phase-21_5` perf reopen judgment is now landed with `reopen allowed`.
- when read from the stage axis, the parent task pack is `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md`: the first stage2+ optimization wave is `route/perf only`, and Rune optimization metadata remains `parse/noop` with backend-active consumption deferred.
- reopen order is fixed like this:
  1. confirm the pre-perf runway in `phase-29ck/P7-PRE-PERF-RUNWAY-TASK-PACK.md` is closed
  2. confirm the boundary mainline route is stable on `.hako -> ny-llvmc(boundary pure-first) -> C ABI`
  3. land the explicit `perf/kilo` reopen judgment
  4. if that judgment is green, reopen `kilo` / `micro kilo`
- current `P8/P9` evidence now allows reopen:
  - `bench_compare_c_vs_hako.sh method_call_only_small 1 1` returns `aot_status=ok`
  - `phase21_5_perf_loop_integer_hotspot_contract_vm.sh` is green
  - `phase21_5_perf_strlen_ir_contract_vm.sh` is green
- `P10-SMALL-PERF-REENTRY-TASK-PACK.md` is now closed.
- current small-entry truth is:
  - `method_call_only_small` mainline AOT IR is a pure `+5` loop
  - `box_create_destroy_small` mainline AOT IR is a pure `+1` loop
  - short microasm may still be startup/loader sensitive, but `bench_micro_aot_asm.sh` now uses a direct C runner instead of a bash loop
- startup-subtracted small-entry evidence is now `method_call_only_small=1 ms`, `box_create_destroy_small=0 ms`
- `P11-SMALL-ENTRY-STARTUP-INVENTORY.md` is now closed.
- current perf-kilo design front has moved to `transient-text-pieces-ssot.md`; the measurement snapshots below remain historical evidence until the next proof lands.
- current docs-first perf-kilo design front is the pair:
  - `post-store-observer-facts-ssot.md`
  - `concat3-array-store-placement-window-ssot.md`
  - `retained-boundary-and-birth-placement-ssot.md`
  - `string-birth-sink-ssot.md`
  `freeze.str` remains the canonical sink target, but the current parent question is now `BoundaryKind` vs `RetainedForm`, plus the separate post-store observer contract, not sink re-home.
- when the lane is on `freeze.str`, do not mix sink canonicalization with route/helper splitting in the same commit series.
- historical retained-boundary sink-local ordering below remains evidence only.
- current phase-137x re-entry order is fixed:
  1. keep the landed arm split (`slow_plan_view_span` only) as frozen evidence
  2. isolate the cold `handle_to_plan` / `plan_to_handle` adapter seam
  3. reopen only a narrow runtime-executor card on `kilo_micro_substring_concat`
  4. keep the rewrite target on a plan-native concat corridor and do not reopen recognizer/cache-first work
- reject any cache-layer growth, helper-traffic growth, or route hinting that does not first prove borrowed-lane continuity on the same artifact.
- compile-time placement helper `crates/nyash_kernel/src/exports/string_birth_placement.rs` is now landed; the next exact lane is upstream birth-density proof with `array_set` as the first `Store` boundary, rather than any further sink-local cut.
- the next large-cut owner is now compiler-local placement, not helper-local widening:
  - use `concat3-array-store-placement-window-ssot.md` as the next exact rollout contract
  - keep `array.set` as first `Store`
  - keep trailing `length()` as `PostStoreUse::LenObserver`
  - only reopen code after trace+asm bundle evidence lines up on the same artifact
- current docs-first refinement is to separate retained reason (`BoundaryKind`) from retained result (`RetainedForm`) before reopening code-side placement work.
- latest same-artifact proof after the retained-boundary parent split stayed flat (`kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 68 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 760 ms`), so code-side `RetainedForm` split remains deferred unless fresh asm evidence appears.
- landed planner cleanup: const-suffix / insert recipe helpers are isolated in `crates/nyash_kernel/src/exports/string_plan.rs`.
- rejected follow-up: moving `freeze.str` into `string_store.rs` regressed stable main (`834 ms` / `909 ms` back-to-back), so keep the shared `freeze_text_plan(...)` helper local to `string.rs` until new asm evidence appears.
- code has now landed the shared `freeze_text_plan(...)` sink helper for `concat_hs` and `insert_hsi`; keep the current proof reading as historical evidence until the next sink-local tuning lands.
- therefore the perf lane may stay reopened, `P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md` and `P13-SMALL-ENTRY-RAW-NET-REFRESH.md` are now closed, and the current small-entry lane is `none (monitor-only)`.
- current boundary-mainline `method_call_only_small` exe shape is `5,375,880` bytes / `61` relocations.
- refreshed raw 1x1 evidence is `method_call_only_small=9 ms`, `box_create_destroy_small=8 ms`.
- `llvmlite` / harness stays outside the perf judge even when the lane reopens.
- until that reopen happens, the quick chip8 crosslang smoke is monitor-only for AOT:
  - keep `[bench4]` / `[bench4-route]` shape and timing keys pinned
  - allow `aot_status=skip`
  - do not treat that smoke as proof that perf lane is open

## Owner Scope / Publication Boundary

この wave で触る owner と、keep lane として読むだけに留める owner を最初に固定する。

- this owner list applies when the perf lane is explicitly reopened; it is not the current mainline implementation front today.

- active edit owners:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/plugin/string.rs`
  - `src/runtime/host_handles.rs`
  - `lang/c-abi/shims/hako_aot.c`
  - `lang/c-abi/shims/hako_aot_shared_impl.inc`
- keep-lane owners for this wave:
  - `src/llvm_py/**`
  - `tools/llvmlite_harness.py`
  - `crates/nyash-llvm-compiler/src/harness_driver.rs`
  - `crates/nyash-llvm-compiler/src/native_driver.rs`
  - explicit keep-lane selectors and their docs/tests
- operational rule:
  - start from `bench_micro_aot_asm.sh` top symbols and follow the symbol owner
  - publication boundary stays MIR-owned; runtime edits may only publish specialized executors on the active corridor
  - do not pivot into `llvm_py` just because keyword grep finds matching names
  - only reopen keep-lane owners when the route contract itself is broken

## Measurement Ladder

最適化は、必ずこの順で進める。

1. Stable baseline
   - 入口: `tools/perf/bench_compare_c_py_vs_hako_stable.sh`
   - 役割: C / Python / Hako / AOT の whole-program 差を見る
   - 使い方: `PERF_AOT_SKIP_BUILD=0` の fresh build を baseline にする
  - route contract: AOT lane is `.hako -> ny-llvmc(boundary pure-first) -> C ABI`
  - `llvmlite` / harness is a correctness/compat keep, not a perf baseline
  - `native` direct keep lane is also outside the perf judge
  - keep lanes are invalid not only as comparators but also as architecture evidence for daily perf cuts

2. Leaf-proof micro ladder
   - 入口: `tools/perf/run_kilo_leaf_proof_ladder.sh`
   - 役割: 1 leaf の shape が same route で本当に薄くなったかを見る
   - 使い方:
     - old crossing が hot block から消えているかを優先して確認する
     - `leaf-proof -> micro kilo -> main kilo` の順を崩さない
   - route contract: same as stable baseline; explicit keep lanes are not valid perf comparisons here

3. Micro ladder
   - 入口: `tools/perf/run_kilo_micro_machine_ladder.sh`
   - 役割: `indexof_line` / `substring_concat` / `array_getset` の leaf 密度を比較する
   - 使い方: `ratio_cycles` と `ratio_instr` を優先して順位を決める
  - route contract: same as stable baseline; explicit keep lanes are not valid perf comparisons here

4. Meso ladder
   - 入口: `tools/perf/run_kilo_meso_machine_ladder.sh`
   - 役割: `micro` と `kilo_kernel_small_hk` の間にある `len -> array_set -> loopcarry` 境界で gap がどこで開くかを見る
   - 使い方:
     - `substring+concat+len`
     - `substring+concat+array_set`
     - `substring+concat+array_set+loopcarry`
     の順で差が開く地点を読む
   - route contract: same as stable baseline; explicit keep lanes are not valid perf comparisons here

5. ASM probe
   - 入口: `tools/perf/bench_micro_aot_asm.sh`
   - 役割: micro ladder で一番厚い leaf の原因関数を確認する
   - 使い方: `perf report --stdio --no-children` の top symbol を読む
   - runner contract: bash loop は使わず、direct C runner で exe を繰り返し起動する

6. MIR call family probe
   - 入口: `tools/perf/report_mir_hotops.sh`
   - 役割: `mir_call` がどの callee family に寄っているかを構造化表示する
   - 使い方: `[mir-shape/call]` を見て `RuntimeDataBox.substring` / `indexOf` / `get/set/length` などの次 leaf を決める

7. Optimization debug bundle
   - 入口: `tools/perf/trace_optimization_bundle.sh`
   - 役割: route trace / MIR window / IR / symbol / optional micro perf を same artifact で束ねる
   - 使い方:
     - `symbol miss` を unexplained のままにしない
     - new leaf は bundle で `route -> MIR window -> IR -> symbol` が揃ってからだけ再試行する

## Judgment Protocol

この wave の keep / reject は、測定回数と ASM の両方で決める。

- `repeat < 3` は probe-only だよ。keep / reject の判断に使わない。
- keep / reject を決める時は、最低 3 runs を取り、あわせて `bench_micro_aot_asm.sh` の top symbols を軽く見る。
- WSL の揺れが残る leaf や allocator / registry 寄りの hot path は、3-run probe のあとに `repeat=20` で再確認してから閉じる。
- 1-run の好調/不調だけで採用・不採用を決めない。probe と decision を混ぜないこと。
- same-artifact の repeat 条件を変える場合は、コード変更と同じコミットで書き残す。

## Tag Coverage

最適化 tag / knob の到達範囲は [optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md) を正本にする。

この wave での短い読みは次。

- language annotations (`@rune Hint/Contract/IntrinsicCandidate`, plus compat aliases) remain `parse/noop` and are not backend-active yet
- AotPrep knobs act before `ny-llvmc`
- boundary compile/link knobs are the only tags that truly cross into `ny-llvmc(boundary pure-first)` / object / exe generation
- perf-only knobs are measurement controls, not backend optimization tags

## Classification

Hotspot は次の分類で読む。

- startup
  - process 起動、引数解析、runner 配線
- driver
  - `ny-llvmc` / runner / bridge の選択
- bridge
  - FFI、handle registry、dispatch、env probe
- allocation
  - `StringBox` / `ArrayBox` / `Registry::alloc`
- algorithm
  - `find_substr_byte_index` などの pure leaf
- cache
  - OnceLock / handle cache / span cache

判断に迷ったら、まず bridge と allocation を疑う。

## Stage2 Thin-Path Reading

`stage2` / `hako_alloc` / `hakozuna` 側の AOT fast-lane 検討では、最初に source layering を疑わない。

- 先に疑うもの:
  - bridge cost
  - allocation cost
  - semantic-owner cost
  - dynamic fallback cost
- 先に疑わないもの:
  - source owner/substrate layering そのもの
- route collapse は perf lane の対象にしてよいが、source relayering は別 SSOT の責務だよ。
- hot path が `HostFacade / extern_provider / plugin loader` に入るなら、それは perf miss として扱う。
- dual-lane reading:
  - `ny-llvm` / `ny-llvmc` = perf/mainline judge
  - `llvmlite` = keep lane only
  - keep lane breaks are correctness/compat issues, not performance evidence

## Stage2 Per-Slice Rule

- stage2 AOT thin-path work is measured one accepted slice at a time.
- route-table change commits and perf-retune commits do not mix.
- after each accepted slice, re-run the ladder in order:
  1. stable baseline
  2. micro ladder
  3. ASM probe
- String waves are measured separately:
  - first `String search/slice route split`
  - then `String concat route split`
- `llvmlite` remains outside the perf judge even when stage2 String waves are active.
- current scheduling consequence:
  - if `phase-29ck` reopens a new exact `ny-llvm` front, do not reopen this perf lane yet
- current preferred next owner is the boundary link owner from `P12`, not a runtime string/box leaf and not an immediate medium/full `kilo` retune

## Small-Entry Stop Line

- if the reopened small-entry lane dumps to pure-loop IR, do not edit runtime string/box leaves in that same series
- if the short asm probe is dominated by loader/startup symbols, move to startup/loader inventory before reopening medium/full `kilo`

## Stop Line

次の条件なら、その lane は「最適化より構造修正」を優先する。

- route がまだ揺れている
- contract テストが落ちている
- benchmark が startup dominated で leaf を見分けられない
- 1回の変更で 2 枚以上の leaf を同時に触りたくなった

`ratio_c_aot >= 0.95` かつ `aot_status=ok` なら、その lane は monitor-only へ落とす。

## Recommended Order

最適化の順番は原則こうだよ。

1. hot path の per-call env probe を cache 化する
2. hot bridge の dispatch / registry を薄くする
3. `substring_concat` や `array_getset` の exact leaf を 1 本だけ削る
4. それでも残るなら native leaf-local hint を exact hot leaf にだけ試す

理由:

- env probe は leaf の形を変えずに固定費を落としやすい
- dispatch / registry は多くの benchmark に横断で効く
- language-level optimization metadata (`@rune Hint(inline)`, plus compat aliases) is not backend-active in the current wave; only leaf-local native hints count here

## Current Wave Snapshot

2026-03-27 current stop-line は次だよ。

- `Stage0 = llvmlite` explicit compat/probe keep lane only
- `Stage1 = ny-llvmc(boundary pure-first)` daily/mainline/perf owner
- current `kilo` route is back to `pure-first + compat_replay=none + aot_status=ok`
- `Stage1 MIR dialect split` is retired for the current kilo entry
- current exact front is now `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
- future `AOT-Core MIR` is treated as `future-needed but not a new layer now`
- current action is to add proof vocabulary to the existing MIR/lowering/manifest path
  - `value_class`
  - `escape_kind`
  - `effect`
  - `cold_fallback`
- current array reject reading is now sharper:
  - adjacent fused-leaf guess was rejected
  - live no-replay route shows current semantic window is `get -> copy* -> const 1 -> add -> set`
  - next exact front is reusable route/window/IR/symbol bundle before another leaf attempt
- first concrete consumer after docs remains integer-heavy `ArrayBox.get/set/len` fast lane
- rejected array-substrate tries are now recorded in a rolling investigation ledger instead of staying as ephemeral shell history
- no-replay `kilo` が green になるまで、`src/llvm_py/**` は perf owner work に使わない
- 下の micro snapshot は historical evidence として保持するが、current exact front ではない

2026-03-22 時点の micro snapshot は次の理解だった。

- `kilo_kernel_small_hk` は whole-program baseline として固定済み
- latest fresh stable baseline is `c_ms=76`, `py_ms=105`, `ny_vm_ms=974`, `ny_aot_ms=740`, `ratio_c_aot=0.10`, `aot_status=ok`
- `kilo_micro_substring_concat` が最厚
- `kilo_micro_array_getset` が次
- `kilo_micro_indexof_line` が一番マシ
- `kilo_micro_array_getset` exact leaf has now moved to the backend boundary under the llpath canonical emit contract: `lang/c-abi/shims/hako_llvmc_ffi_common.inc` canonicalizes pure-first IR with `opt -passes=mem2reg` before `llc` in the current implementation, the Array micro seed keeps the sink honest with explicit volatile `sum` accesses, and the judged proof bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-mem2reg-v2/` shows `ny_main` registerizing the loop IV as SSA/PHI (`%i.1 = phi ...`) with the `%i` stack spill removed from asm
- `crates/nyash_kernel/src/plugin/array_index_dispatch.rs` / `array_write_dispatch.rs` remain thin wrappers, but they are no longer the primary exact leaf target
- fresh `kilo_micro_array_getset` recheck after the read-seam keep was `ny_aot_ms=44`; the current post-boundary-helper judge is `c_ms=3 / ny_aot_ms=4 / ratio_instr=1.20 / ratio_cycles=0.68 / ratio_ms=0.75`
- current stage2+ evidence bundle for `kilo_micro_array_getset` is direct-route only: `target/perf_state/optimization_bundle/stage2plus-array-wave-direct/` records `Method:RuntimeDataBox.{push,get,set}` windows, `owner_route=generic_probe first_blocker=empty`, and a hot-block scan with no `slot_load_hi` / `generic_box_call` / `hostbridge` / `runtime_data` residue
- current same-artifact microstat remains `c_ms=3 / ny_aot_ms=4 / ratio_ms=0.75`; keep that as the wave baseline until a fresh route/perf cut beats it
- refreshed 2026-03-30 stage2+ direct bundle/compare lock is:
  - `bench_micro_c_vs_aot_stat.sh kilo_micro_array_getset 1 3`
    - `c_ms=3 / ny_aot_ms=3 / ratio_instr=0.90 / ratio_cycles=0.68 / ratio_ms=1.00`
  - `run_kilo_micro_machine_ladder.sh 1 3`
    - `kilo_micro_array_getset: c_ms=4 / ny_aot_ms=4 / ratio_instr=0.90 / ratio_cycles=0.69 / ratio_ms=1.00`
  - post-boundary-helper judge bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-mem2reg-v2/`
    - `ny_main` now shows the loop IV as SSA/PHI in `lowered.ll` (`%i.1 = phi ...`)
    - asm drops the `%i` spill and keeps the hot loop to `and / inc / mov / add` shape
    - latest 3-run residue summary is `93.66% bundle / 3.02% loader / 0.56% runner`
  - refreshed direct bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-refresh/`
    - `mir_windows` stays on `Method:RuntimeDataBox.{push,get,set}`
    - `owner_route=seed first_blocker=empty`
    - `recipe_acceptance` remains empty
    - hot-block scan still has no `slot_load_hi` / `generic_box_call` / `hostbridge` / `runtime_data` residue
    - `perf_top` is still dominated by `ny_main` (`92.61%`)
  - tooling follow-up: `tools/perf/trace_optimization_bundle.sh` now auto-saves `perf_top_symbol.txt`, `perf_top_annotate.txt`, and `perf_top_objdump.txt`
  - probe bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-probe/`
    - hottest in-binary symbol resolves to `ny_main`
    - annotate/objdump now show the hot loop directly as `cmp -> load -> inc -> store -> add -> inc`
    - no surviving foreign calls appear in the hot block
  - 20-run observe bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-observe20/`
    - positive-sample instruction list still stays on `cmp` / `inc` only
    - opcode histogram is `54.45% cmp`, `45.55% inc`
  - grouped 3-run residue probe `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-groups/`
    - `perf_top_group_summary` reads `89.50% bundle / 5.98% loader / 1.47% runner`
    - this is the preferred WSL-friendly residue reading before any Array code slice
  - repeated 3-run bundles under WSL still vary materially
    - repeatA `92.66% bundle / 2.81% loader / 2.04% runner`
    - repeatB `89.84% bundle / 5.82% loader / 3.09% libc / 1.20% runner`
    - repeatC `74.02% bundle / 22.96% loader / 2.40% libc / 0.55% runner`
    - therefore `perf_top_group_summary` is a noise detector, not a sole acceptance gate
  - cold 1-run residue probe `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-cold1/`
    - `perf_top_group_summary` reads `87.25% bundle / 6.90% loader / 5.84% runner`
    - use this only as startup-residue evidence; do not replace the 3-run judge with it
  - C baseline cross-check (`benchmarks/c/bench_kilo_micro_array_getset.c` + `perf annotate --symbol main`)
    - loop body samples spread across `and / mov / inc / mov / cmp`
    - this does not reveal a missing Array route call on the Nyash side; it only confirms that the current AOT direct probe has already collapsed to a loop-shaped artifact
  - lowered IR cross-check (`target/perf_state/optimization_bundle/stage2plus-array-wave-direct-repeatA/lowered.ll`)
    - `ny_main` keeps the induction variable `%i` in an `alloca` with `load -> add -> store` every iteration
    - `sum` also remains in memory, but that is expected because the benchmark source declares it `volatile`
    - that was the actionable waste candidate before the llpath canonical emit contract landed; it is now closed by the pure-first boundary canonicalization path (current implementation: `opt -passes=mem2reg`), and the current judged bundle shows `%i` promoted to SSA/PHI with the stack spill removed from asm
- interpretation after the refresh:
  - stage2+ first wave stays `Array only`
  - fixed order stays `leaf-proof micro -> micro kilo -> main kilo`
  - the refreshed same-artifact direct artifact no longer exposes the old `%i` route/helper residue
  - any further slice must now start from a new measurable leaf below `ny_main` or a stable boundary blocker that survives the micro proof
  - do not reopen another blind helper split on the write/TLS seam while the direct bundle stays collapsed; the current direct probe no longer exposes `array_slot_store_i64` / `with_array_box` inside the hot block
- rejected probes (reverted immediately):
  - dedicated i64 write helper: `47 ms`
  - `try_set_index_i64_integer` cold-split: `48 ms`
  - `with_array_box` cache-hit inline probe: `46 ms`; asm top stayed on `array_slot_store_i64` closure + `LocalKey::with`
- fresh microasm now concentrates on `array_slot_store_i64` closure + `LocalKey::with`, so the next cut must be measurement-led rather than another blind helper split
- bridge-side `Program(JSON v0)` import-bundle trace (`NYASH_JSON_V0_IMPORT_TRACE=1`) は、compat loader observation として hot/cold の切り分けに使う。summary は default-visible、detail は `NYASH_RING0_LOG_LEVEL=DEBUG` のときだけ出す。
  - stable tag: `[json_v0/import_bundle] phase=<enter|skip|guard.set|restore|merge.begin|merge.done|fail> ...`
  - trace が perf-kilo で複数回見えるなら bundle-scope にまとめる余地を検討する
  - trace が cold なら bridge keep として閉じ、Array / String の birth lane へ戻る
- `substring_concat` の current exact leaf は kernel/runtime owner に固定する
- `crates/nyash_kernel/src/exports/string_view.rs` now owns `borrowed_substring_plan_from_handle(...)`, and `crates/nyash_kernel/src/exports/string.rs::substring_hii` is reduced to dispatch + match
- `crates/nyash_kernel/src/exports/string.rs::concat3_hhh` is now split file-locally into transient planning (`concat3_plan_from_parts`, `concat3_plan_from_fast_str`, `concat3_plan_from_spans`) plus birth sink (`freeze_concat3_plan`)
- `substring_hii` の hot path must stay on direct `with_handle(...)`; cache-backed span lookup is diagnostic-only here because it regressed `string_span_cache_get/put` back into the top symbols
- `src/runtime/host_handles.rs::Registry::alloc` now reads `policy_mode` before the write lock, keeps invariant failures in cold helpers, and folds the hot birth branch directly in the registry; `Registry::get` now uses the direct clone path; this slice is closed and documented in `docs/development/current/main/investigations/perf-kilo-string-birth-hotpath-summary-2026-03-28.md`
- current contract-change slice raises the short-slice eager materialize threshold to `<= 8 bytes`
- fresh micro recheck after the current slices is `266244455 cycles / 72 ms` for `kilo_micro_substring_concat`
- fresh stable recheck after the current slices is `740 ms` median for `kilo_kernel_small_hk` (`min=738`, `max=744`)
- rejected variant: `root StringBox <= 16 bytes` / `nested StringViewBox <= 8 bytes` improved isolated `substring_concat` to `262468757 cycles / 69 ms`, but stable `kilo_kernel_small_hk` regressed to `819 ms`; do not keep this split while stable is the primary metric
- rejected observer-only variant: `crates/nyash_kernel/src/exports/string.rs::string_len_from_handle(...)` explicit `StringBox` / `StringViewBox` downcast fast paths reached `265893951 cycles / 68 ms`, but stable `kilo_kernel_small_hk` regressed to `1066 ms` median (`min=786`, `max=1841`); revert immediately and do not reopen this cut before a stronger owner-level reason appears
- rejected structure-first variant: `BorrowedSubstringPlan::{OwnedSubstring,ViewRecipe}` moved `StringViewBox` birth from `borrowed_substring_plan_from_handle(...)` into `substring_hii`, but without a real transient carrier this only shuffled the birth site; isolated `substring_concat` landed at `267397179 cycles / 72 ms`, while stable `kilo_kernel_small_hk` regressed to `901 ms` median (`min=794`, `max=1146`); do not reopen this cut until a larger `TStr`/freeze-boundary design is ready
- current asm top is:
  - `Registry::alloc`
  - `BoxBase::new`
  - `nyash.string.substring_hii`
  - `nyash.string.concat3_hhh`
  - `string_len_from_handle` / `string_handle_from_owned`
- `BoxBase::new` remains a stop-line: it is tied to box identity via `next_box_id()`, so the next safe cut must reduce `StringViewBox::new` call count or another upstream owner instead of reusing IDs
- adopted design reading after external consultation:
  - this is a birth-density problem, not a `BoxBase::new` micro-cost problem
  - the next wave should separate `authority / transient / birth boundary / substrate`
  - read-only observer chains may stay transient; only substrate-visible / retained values should birth
- interpretation:
  - keep the short-slice materialize change if whole-program stable is the primary metric
  - do not treat isolated micro regression as automatic revert when the stable lane improves
  - next queued wave is transient string chain design-first with a future `freeze` boundary, not another threshold experiment

## Evidence To Record

最適化を 1 slice 終えたら、必ず次を更新する。

- `CURRENT_TASK.md`
- 該当 phase README
- その wave の design / investigation doc

記録する内容は最小でよい。

- baseline 数値
- top symbol
- 変更した exact leaf
- 再実行した gate / smoke

## Non-goals

- 大きな route rewrite を先にやること
- benchmark を見ずに一般論だけで最適化すること
- hint を workaround として使うこと
- Route contract for this wave:
  - perf AOT lane is `.hako -> ny-llvmc(boundary pure-first) -> C ABI`
  - `llvmlite/harness` are invalid as perf comparators but valid as explicit keep lanes
  - `native` direct keep lane is also outside the perf judge
- `kilo_micro_substring_concat`:
  - asm-guided slice first changed `SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES` from `8` to `0`, then contract-change follow-up restored eager materialize for `<= 8 bytes`
  - short `substring_hii` results now materialize under FAST lane, while mid slice still stays `StringViewBox`
  - pre-structure-first checkpoint was `266891899 cycles / 73 ms`, while stable `kilo_kernel_small_hk` sat at `804 ms`
  - accepted structure-first follow-up: `concat3_hhh` now reads `plan -> freeze -> handle` inside `string.rs`; current recheck is `266244455 cycles / 72 ms` and stable `kilo_kernel_small_hk` median `798 ms`
  - rejected observer-only follow-up: explicit `string_len_from_handle` downcast fast paths reached `265893951 cycles / 68 ms`, but stable `kilo_kernel_small_hk` regressed to `1066 ms` median, so this wave keeps the previous observer path unchanged
  - rejected structure-first follow-up: planner-side `OwnedSubstring/ViewRecipe` plus `substring_hii`-side view freeze reached `267397179 cycles / 72 ms`, but stable `kilo_kernel_small_hk` regressed to `901 ms` median, so plan/birth separation needs a real transient carrier instead of a pure birth-site shuffle
  - rejected 2026-03-28 follow-up: direct `concat_hs` / `concat3` copy materialization moved stable `kilo_kernel_small_hk` from the current `736 ms` line to `757 ms` and did not improve micro; keep `TextPlan`-backed concat routes until a new asm reason appears
  - rejected 2026-03-28 follow-up: piece-preserving `insert_inline` plus store/freeze reshaping regressed stable `kilo_kernel_small_hk` to `895 ms`; do not reopen that cut until `concat_hs` / `array_set_by_index_string_handle_value` stop being the active hot leafs
  - rejected 2026-03-28 follow-up: blanket `#[inline(always)]` on host registry / hako-forward string wrappers held stable main near `740 ms` and did not beat the current `736 ms` line, so the slice stays reverted
  - rejected 2026-03-28 follow-up: `concat_hs` duplicate span-resolution removal plus span-resolver inlining regressed stable `kilo_kernel_small_hk` to `796 ms`, so the existing `TextPlan::from_handle(...)` route stays active
  - current top symbols are:
    - `BoxBase::new`
    - `Registry::alloc`
    - `nyash.string.substring_hii`
    - `nyash.string.concat3_hhh`
    - `string_len_from_handle` / `string_handle_from_owned`
- Keep-lane diagnostic note:
  - worker inventory found likely `loop self-carry PHI` string pointer loss under `src/llvm_py/**`
  - this is diagnostic evidence only in the current wave
  - next edits still stay on kernel/runtime/C-boundary owners until asm top symbols move away from `substring_hii` / `Registry::alloc` / `BoxBase::new`
