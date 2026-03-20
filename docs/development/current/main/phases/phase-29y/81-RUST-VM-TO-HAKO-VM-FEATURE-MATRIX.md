---
Status: Active (capability matrix maintained; runtime pointer is `60-NEXT-TASK-PLAN.md`)
Decision: provisional
Date: 2026-02-18
Scope: Rust VM 依存機能と `.hako VM` 対応状況の差分を1枚で管理する。
Related:
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/phases/phase-29y/80-RUST-VM-FEATURE-AUDIT-AND-HAKO-PORT-SSOT.md
  - docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md
  - src/runner/modes/vm_hako.rs
  - lang/src/vm/boxes/mir_vm_s0.hako
---

# Rust VM -> .hako VM Feature Matrix

Lane boundary:
- この matrix は de-rust lane C（runtime port）専用。
- lane A/B の blocker はここに追加しない。

## Usage

- `status` は `blocked | queued | porting | ported` の4値のみ使う。
- 1コミットで更新してよい row は 1つだけ。
- `smoke` が未定義の row は移植着手しない。

## Matrix

| id | capability | rust-vm usage (evidence) | hako-vm status | blocker app | smoke | status | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RVP-C01 | `newbox(FileBox)` | app CLI/file read で常用 | subset-check accepted（`newbox(FileBox)`） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_filebox_newbox_vm.sh` | ported | 2026-02-17: C01 port完了 |
| RVP-C02 | args-based input routing (`args.get/length`) | app entry で常用 | subset-check + vm-hako runner accepted（main(args) bootstrap `birth/push/length`） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_args_vm.sh` | ported | 2026-02-17: `push(dst=null,args=1)` 受理 + ArrayBox length state を vm-hako S0 へ実装 |
| RVP-C03 | `FileBox.open(path, mode)` multi-arg call path | CLI系で必須 | subset-check + vm-hako runner accepted（2/3-arg open shape） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_file_error_vm.sh` | ported | 2026-02-17: open multi-arg path を vm-hako S0 へ移植（missing-file rc契約 pin） |
| RVP-C04 | MIR JSON emit contract for `Select` | APP-1 loop conditional update (`i=100` early-exit) で必須 | vm-hako route accepts `select` JSON emit + execution path | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_select_emit_block_vm.sh` | ported | 2026-02-17: port完了（emit allowlist + emitter `select` 生成を実装、emit-error契約から execution RC marker 契約へ昇格） |
| RVP-C05 | externcall `env.get/1` route | APP-1 logfile path routing（env override）で必須 | subset-check + vm-hako runtime accepted（`externcall env.get/1` shape + null/reg dst writeback） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_env_get_ported_vm.sh` | ported | 2026-02-17: port完了（env.get externcall を subset/runtime へ移植、blocked 契約を success へ昇格） |
| RVP-C06 | compare op `!=` in control-flow condition | APP-1 input/file checks (`==/!=` branches) で必須 | subset-check + vm-hako runtime accepted（`compare(!=)` operation/op_kind `Ne`） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_compare_ported_vm.sh` | ported | 2026-02-17: port完了（subset/runtimeへ `compare(!=)` 実装、blocked 契約を success へ昇格。`op_kind=Ne` runtime alias も固定） |
| RVP-C07 | `FileBox.read()` method call path | APP-1 file body readで必須 | subset-check + vm-hako runtime accepted（FileBox receiver-token state + read handle return） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_file_read_ported_vm.sh` | ported | 2026-02-17: port完了（read path を runtimeへ移植、blocked 契約を success 契約へ昇格） |
| RVP-C08 | `FileBox.close()` method call path | APP-1 file handle closeで必須 | subset-check + vm-hako runtime accepted（FileBox receiver-token state + close clear） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_file_close_ported_vm.sh` | ported | 2026-02-17: port完了（close path を runtimeへ移植、blocked 契約を success 契約へ昇格） |
| RVP-C09 | `const(type:void)` subset acceptance | APP-1 env/file route の null 初期値で発生 | subset-check + vm-hako runtime accepted（void const → scalar 0） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_const_void_ported_vm.sh` | ported | 2026-02-17: port完了（const void を subset/runtime へ移植、blocked 契約を success 契約へ昇格） |
| RVP-C10 | compare op `>=` in control-flow condition | APP-1 line-length guard（`line.length() >= 6`）で発生 | subset-check + vm-hako runtime accepted（`compare(>=)` operation/op_kind `Ge`） | `apps/tests/vm_hako_caps/compare_ge_block_min.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_compare_ge_ported_vm.sh` | ported | 2026-02-17: port完了（subset/runtimeへ `compare(>=)` 実装、blocked 契約を success へ昇格。`op_kind=Ge` runtime alias も固定） |
| RVP-C11 | non-open `boxcall(args>1)` acceptance | APP-1 line processing（`substring(start,end)` 等）で発生 | subset-check accepted（stale `boxcall(args>1)` fail-fast は不在） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_boxcall_args_gt1_ported_vm.sh` | ported | 2026-02-17: port完了（subset で non-open args>1 を受理、次 blocker を runtime contract へ前進） |
| RVP-C12 | FileBox.open path-handle propagation across `phi/copy` | APP-1 logfile path merge（env/default）で発生 | runtime ported（stale `boxcall-open-handle-missing` 不在） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_open_handle_phi_ported_vm.sh` | ported | 2026-02-17: RVP-5-min7 で phi state copy（handle/array/file）を追加し C12 を port |
| RVP-C13 | APP-1 vm-hako run completion after open-path handling | APP-1 full flow（open/read/loop/summary）で発生 | runtime ported（stale stack-overflow blocker removed; RC=0 run completion pin） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_app1_stack_overflow_after_open_ported_vm.sh` | ported | 2026-02-18: RVP-5-min8 で block recursion path を iterative 化し C13 を port。2026-02-18 min10 で fixture を `app1_summary_contract_min.txt` へ固定し、run-completion pin を維持 |
| RVP-C14 | APP-1 summary output parity after open-path run completion | APP-1 output contract（SUMMARY/FAIL_LINES/FAIL lines）で発生 | runtime ported（vm-hako output matches Rust baseline on parity fixture） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_app1_summary_contract_ported_vm.sh` | ported | 2026-02-18: RVP-5-min10 で handle-aware print/binop(compare) + `env.get` call-path + const-handle unescape を移植し、SUMMARY/FAIL_LINES/FAIL順 parity を固定 |
| RVP-C15 | APP-1 summary parity on full fixture within gate time budget | APP-1 full fixture（`sample_mixed.log`）で発生 | runtime ported（full-fixture parity fixed under promoted time budget; stale timeout blocker guarded） | `apps/tools/gate_log_summarizer/main.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_app1_summary_contract_ported_vm.sh` | ported | 2026-02-18: RVP-5-min12 で block-local instruction cache（`mir_vm_s0`）を導入し、C15 を full-fixture parity 契約へ昇格。stale timeout blocker は `vm_hako_caps_app1_summary_contract_block_vm.sh` で監視 |
| RVP-C16 | `newbox(MapBox)` subset acceptance | quick map provider / collection owner cutover の first entry で発生 | subset-check accepted（`newbox(MapBox)`） | `apps/tests/vm_hako_caps/mapbox_newbox_block_min.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_mapbox_newbox_ported_vm.sh` | ported | 2026-03-21: `newbox(MapBox)` を vm-hako subset-check へ追加し、dedicated capability smoke で ported 固定 |
| RVP-C17 | `MapBox.set(key, value)` multi-arg `boxcall` | quick map collection smoke (`new MapBox(); m.set("a", 42)`) で発生 | subset-check + vm-hako runtime accepted（stale `boxcall(set:args>1)` blocker removed; zero-RC pin） | `apps/tests/vm_hako_caps/mapbox_set_block_min.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_mapbox_set_ported_vm.sh` | ported | 2026-03-21: subset-check で `set(args=2)` を受理し、vm-hako runtime でも stale `args>1` blocker を出さず RC=0 で完走する最小契約へ昇格 |
| RVP-C18 | `MapBox.size()` zero-arg `boxcall` | quick map collection smoke (`new MapBox(); m.set(...); print(m.size())`) で発生 | subset-check + vm-hako runtime accepted（stale `op=boxcall0 method=size` blocker removed; printed `2` and RC=0 pinned） | `apps/tests/vm_hako_caps/mapbox_size_ported_min.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_mapbox_size_ported_vm.sh` | ported | 2026-03-21: vm-hako runtime now tracks minimal MapBox size semantics for distinct keys and the size route is pinned as ported |
| RVP-C19 | `MapBox.get(key)` one-arg `boxcall` value semantics | quick map collection smoke (`new MapBox(); m.set("a", 42); print(m.get("a"))`) で発生 | subset-check + vm-hako runtime accepted（stale scalar `0` result removed; printed `42` and RC=0 pinned） | `apps/tests/vm_hako_caps/mapbox_get_ported_min.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_mapbox_get_ported_vm.sh` | ported | 2026-03-21: vm-hako runtime now preserves minimal per-key MapBox scalar/handle value state for `get(key)` and the value route is pinned as ported |
| RVP-C20 | `MapBox.has(key)` one-arg `boxcall` | quick map collection smoke (`new MapBox(); m.set("a", 42); print(m.has("a"))`) で発生 | blocked（shape is accepted, but vm-hako runtime still stops at `op=boxcall1 method=has`） | `apps/tests/vm_hako_caps/mapbox_has_block_min.hako` | `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_mapbox_has_block_vm.sh` | blocked | 2026-03-21: collection owner cutover の next exact blocker is `MapBox.has(key)` runtime route |

## FileBox State Contract (RVP-C07/C08)

- vm-hako の `FileBox` receiver state は `file_boxes` map を SSOT とする。
- `open/read/close` は `file_boxes[receiver_token]` の同一 `FileBox` インスタンスへ作用させる。
- `receiver_token` は `newbox(FileBox)` で初期化し、register copy/phi でそのまま伝搬する。
- `copy/phi` で receiver alias が移動した場合、`file_boxes` state も同時にコピーする（runtime state parity）。
- `read` の戻り値は `null` なら scalar `0` とし、文字列なら `handle_regs` に保持して `kind=handle` で返す。
- `close` 実行後は receiver の `file_boxes` state を破棄する（close-after-read の再利用誤差を防ぐ）。

## Compatibility Contract Pin

- Legacy `nop` is accepted in vm-hako subset-check as a no-op (backward compatibility only).
- Contract pin test: `runner::modes::vm_hako::tests::subset_memory_gc::subset_accepts_legacy_nop_as_noop`.
- If this behavior is changed, update this matrix and `src/runner/modes/vm_hako/subset_check.rs` in the same commit.

## Next order SSOT

- Next task order は `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` を正本とする。
