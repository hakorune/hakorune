# Hako VM / Reference Cluster Layout (Current → Target)

Status
- `lang/src/vm` is a VM/reference cluster.
- It is not the product kernel.
- It is not the day-to-day mainline owner.

Current
- `lang/src/vm/hakorune-vm/` — Hako-side VM/reference implementation
- `lang/src/vm/boxes/` — Shared helpers (op_handlers, scanners, compare, etc.)
- Mini‑VM minimal executor lives as boxes (e.g., `boxes/mir_vm_min.hako`)

Mini VM vs Hakorune VM (Roles)
- Mini VM (Hako): reference semantic executor for MIR(JSON v0). Scope is the
  minimal instruction set (const/compare/branch/jump/ret/phi). Its job in the
  system is verification: given MIR(JSON v0), compute the return value and
  map it to an exit code (Int → value, Bool → 1/0). It must not depend on
  env/get or include; inputs are passed as inline JSON strings.
- Hako VM cluster (`lang/src/vm`): VM/reference executor cluster for
  `vm-hako` and selfhost-side verification lanes. It is not the day-to-day
  mainline owner and should not be described as the product kernel/runtime.

Verify Pipeline (hakovm primary / Fail‑Fast)
1) Emit MIR(JSON v0) as a single JSON string (noise trimmed in runner).
2) Runner embeds JSON into a tiny Hako driver:
   `using selfhost.vm.entry as MiniVmEntryBox; return MiniVmEntryBox.run_min(j)`
3) The driver prints the numeric return; the runner converts it into a process
   exit code for canaries. No env.get or file I/O is required.

Resolver Policy (Modules)
- Prefer `using alias.name` with workspaces declared in `hako_module.toml` and
  aliases in `nyash.toml`.
- Implement transitive resolution (bounded depth, cycle detection, caching).
- Prelude は Runner 側で“テキスト統合（merge_prelude_text）”に一本化（AST マージは撤退）。
- Dev プロファイルでは引用付きファイルパス（"lang/…"）を bring‑up のみ限定許可。prod は alias のみ。

Target (post‑20.12b, gradual)
- `engines/hakorune/` — mainline nyvm engine
- `engines/mini/` — Mini‑VM engine (educational/minimal)
- `boxes/` — shared helpers
- `core/` — centralized execution core (value/state/reader/dispatcher + ops)

Policy
- Engines orchestrate execution and may depend on boxes and shared/*.
- Boxes are pure helpers (no engine loop, no I/O, no plugin/ABI).
- Parser/Resolver/Emitter must not be imported from engines/boxes.
- Core provides engine‑agnostic execution primitives and should not import
  engine‑specific modules. During migration, temporary adapters may exist.

Static Box Methods（Singleton / self）
- 規約: 静的Boxのメソッドは「self（Singleton）を先頭引数」に持つ。
  - 例: `LLVMPhiInstructionBox.lower_phi(self, dst, incoming_list)`
- 互換: `HAKO_BRIDGE_INJECT_SINGLETON=1`（alias: `NYASH_BRIDGE_INJECT_SINGLETON`）で旧スタイル
  `PhiInst.lower_phi(dst, incoming)` に Singleton を注入して実行。
- Fail‑Fast: 期待 arity と不一致は静かなフォールバックをせず、安定メッセージで失敗する。
  詳細: `docs/development/architecture/llvm/static_box_singleton.md`

Bridge‑B (Ny/Core 直行)
- Wrapper 経路では `include "lang/src/vm/core/dispatcher.hako"` で Core Dispatcher を取り込み、
  `NyVmDispatcher.run(json)` を直接呼び出す。`using` は名前解決のみで実体は登録されないため、
  Core を呼ぶ目的では `include` を用いること。
  Gate‑C(Core) 直行（`NYASH_GATE_C_CORE=1`）は JSON→Core Interpreter 実行なのでこの問題の影響を受けない。

Toggles and Canaries
- Core canaries (quick profile): enable with `SMOKES_ENABLE_CORE_CANARY=1`.
  - Emit→nyvm(Core) scripts: `tools/smokes/v2/profiles/quick/core/canary_emit_nyvm_core_{return,binop,if}_vm.sh`
  - Gate‑C(Core, json→Core 直行) canaries: `tools/smokes/v2/profiles/quick/core/canary_gate_c_core_{file,pipe}_vm.sh`（既定OFF）
  - Gate‑C(Core) array sequence: `tools/smokes/v2/profiles/quick/core/canary_gate_c_core_array_mixed_vm.sh`（push→set→get をログで検証）
  - Gate‑C(Core) map sequence: `tools/smokes/v2/profiles/quick/core/canary_gate_c_core_map_{len,iterator}_vm.sh`
  - Emit→Core map len/get: `tools/smokes/v2/profiles/quick/core/canary_emit_core_map_len_get_vm.sh`
- Gate‑C Direct sanity: `tools/smokes/v2/profiles/quick/core/canary_gate_c_core_direct_string_vm.sh`

Mini‑VM Flags (20.36)
- `HAKO_VM_MIRCALL_SIZESTATE=1`
  - Array/Map の最小状態（size/len/push）を擬似的に追跡する（既定OFF）。
- `HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1`
  - 受信者（Constructor の dst＝仮ハンドル）ごとにサイズを独立管理する（既定OFF）。
- `verify_v1_inline_file()` は `HAKO_ABI_*` / `HAKO_VM_*` / `HAKO_V1_*` の canary toggles を引き継いで hv1-inline を起動する。
  - ただし実体は `src/main.rs` の `hv1_inline::run_json_v1_inline(...)` で、`.hako` `MirCallV1HandlerBox` / `MirVmMin` の owner proof そのものではない。
  - `.hako` 側 owner 変更の proof は source-contract smoke や standalone `.hako` / AOT smoke に分けて持つ。
- Canary:
  - phase2036/v1_minivm_size_state_on_canary_vm.sh → rc=2（push×2→size）
  - phase2036/v1_minivm_size_state_per_recv_on_canary_vm.sh → rc=2（A/B 各1push→size(A)+size(B)=2）

Scanners (Box化)
- MiniMirV1Scan（lang/src/vm/boxes/mini_mir_v1_scan.hako）
  - callee_name/method_name/first_arg_register/receiver_id を提供。
  - MirVmMin は本スキャナを呼ぶだけにして重複スキャンを排除（保守性の向上）。

Include Policy (quick)
- include は非推奨。quick プロファイルでは SKIP（`SMOKES_INCLUDE_POLICY=skip|warn|error`。段階的に ERROR へ移行）。
- using+alias を正道に固定（Runner 解決→Prelude テキスト統合）。
- nyash.toml の [modules] / alias）へ移行すること。

Core Route (Phase 20.34 final)
- Canaryの一部（Loop 系）は暫定的に Core 実行へ切替（Mini‑VM の緑化は 20.36 で段階的に実施）。
- verify_mir_rc は primary=core の場合、
  - MIR(JSON) → `--mir-json-file`（v1→v0 順でローダが MirModule 復元）
  - Program(JSON v0) → `--json-file`（json_v0_bridge 経由で MirModule 化）
- 代表：
  - mirbuilder_internal_core_exec_canary_vm（rc=10）
  - mirbuilder_internal_loop_core_exec_canary_vm（rc=3）
  - mirbuilder_internal_loop_count_param_core_exec_canary_vm（rc=6）
  - mirbuilder_internal_loop_sum_bc_core_exec_canary_vm（rc=8）

Loop/PHI Unification
- Runner v0 の Loop/PHI は `phi_core` に統一（SSOT）。
- トグル: `NYASH_MIR_UNIFY_LOOPFORM=1|0`（既定ON）。OFF 指定時は警告を出しつつ統一経路を継続利用（レガシー経路は削除済み）。
- 目的: 既定ONの明示化と将来のABテスト窓口の確保（挙動は不変）。
  
Exit Code Policy
- Gate‑C(Core): numeric return is mapped to process exit code。タグ付きの失敗時は安定メッセージを出し、可能な限り非0で終了。
- VM backend（Rust Interpreter）: 戻り値は標準出力に出す。プロセスの終了コードは戻り値と一致しない場合があるため、スモークは安定タグや標準出力の数値で検証する（rcは参考）。
- 推奨: CIやスクリプトでは Gate‑C(Core) を優先し rc を厳密化。開発時の対話検証は VM ルートで標準出力を検証。

Tag→RC（Core Direct）
- Core Direct（`HAKO_CORE_DIRECT=1`）では、数値行が見つからない場合は rc≠0 を返す（Fail‑Fast）。
- 代表タグ（例）
  - `[core/string/bounds]` → rc=1
  - `[core/array/oob_set]` → rc=1
  - `[core/mir_call/method_unsupported]` → rc=1

Core Direct Toggle
- `HAKO_CORE_DIRECT=1`（互換: `NYASH_CORE_DIRECT`）で、Gate‑C(Core) の JSON 実行を "Core Dispatcher 直行" 子経路に切り替える。
  - 形: 一時Hakoスクリプトに `include "lang/src/vm/core/dispatcher.hako"` を埋め込み、`NyVmDispatcher.run(json)` を実行。
  - rc: 子プロセスの終了コードを採用（数値戻り=rc、タグ/非数=rc≠0）。
  - 判定: 直行は MIR(JSON v0)（`functions` と `blocks` を含む）に限定する。Stage‑B Program(JSON v0) は v1 bridge→VM 実行に迂回し、直行は行わない。
  - 用途: Core の診断タグや rc を CI で直接検証したい時に使用。
- Runner Core toggle: `HAKO_NYVM_CORE=1` (or `NYASH_NYVM_CORE=1`) selects the
  Core bridge for the nyvm wrapper path.
- Gate‑C Core route: set `NYASH_GATE_C_CORE=1` (or `HAKO_GATE_C_CORE=1`) to
  execute MIR(JSON v0) directly via Core (interpreter path; quiet; exit code mirrors return).
  - Env: `NYASH_CORE_MAX_ITERS` or `HAKO_CORE_MAX_ITERS` overrides the Core dispatcher loop cap (default 10000).
  - Plugins: when `HAKO_GATE_C_ENABLE_PLUGINS=1` is set, the runner normalizes
    Array/Map core methods through HostHandleRouter (`HAKO_ARRAY_FORCE_HOST=1`,
    `HAKO_MAP_FORCE_HOST=1`) to keep value/return semantics stable. Plugins が OFF のときは
    ビルトインの `ArrayBox` / `MapBox` にフォールバックする。Map の `len()/size()` は extern adapter が
    ビルトイン `MapBox` の内部データ長を返すフォールバックを持つため（plugins=OFF でも）0 固定にはならない。
- Errors: VM 実行/JSON読込エラー時は非0で終了（Fail‑Fast）。

PHI Strict（既定ON）
- 目的: ループ/分岐における PHI 入力の不整合（pred 欠落や自己参照）を早期に検出し、クラッシュや未定義使用を防止する。
- 既定: ON（何も設定しない場合は厳格）
- 無効化（開発・暫定用途のみ）:
  - `HAKO_VM_PHI_STRICT=0` または `NYASH_VM_PHI_STRICT=0`
  - ON 指定: `1|true|on|yes` 等、OFF 指定: `0|false|off|no` 等を受理
- 実装: `src/backend/mir_interpreter/exec.rs`（PHI 適用時に pred 不一致を Fail‑Fast）

Quick profile opt‑in switches (smokes)
- `SMOKES_ENABLE_LOOP_COMPARE=1` — Direct↔Bridge parity for loops (sum/break/continue/nested/mixed)
- `SMOKES_ENABLE_LOOP_BRIDGE=1` — Bridge(JSON v0) loop canaries (quiet; last numeric extraction)
- `SMOKES_ENABLE_STAGEB_OOB=1` — Stage‑B OOB observation (array/map)
- `SMOKES_ENABLE_OOB_STRICT=1` — Gate‑C(Core) strict OOB fail‑fast canary (`gate_c_oob_strict_fail_vm.sh`)
- `SMOKES_ENABLE_LLVM_SELF_PARAM=1` — LLVM instruction boxes self‑param builder tests (const/binop/compare/branch/jump/ret)
- `SMOKES_ENABLE_STAGEB=1` — Stage‑B positive canaries（emit→Gate‑C）。既定OFF（安定化後に昇格予定）。
  必要に応じて各テスト内で `HAKO_STAGEB_ALLOW_FALLBACK=1` を付与（TTL; 既定OFF）。

Default quick canaries (regression)
- apps/json_lint_vm.sh — JSON Lint expected outputs (OK×10 / ERROR×6)
- core/array/array_length_vm.sh — ArrayBox.length returns 0→1→2→3 for push sequence

Dispatch policy: length()
- String 受けに対してのみ StringBox ハンドラが length() を処理する。
- Array/Map などは各 Box 専用ハンドラで length/len/size を処理する。
- これにより、配列に対して誤って文字列長を返す回帰を防止する（2025‑11 修正）。

Strictness & Tolerance (ENV policy)
- PHI strict（既定ON）
  - 既定ON（未指定時はON）。無効化は `HAKO_VM_PHI_STRICT=0`（互換: `NYASH_VM_PHI_STRICT=0`）。
  - 目的: pred不一致などPHI入力の欠落をFail‑Fastで検出。
- VMステップ上限（無限ループ対策）
  - `HAKO_VM_MAX_STEPS`（互換: `NYASH_VM_MAX_STEPS`）で1関数内の実行ステップに上限（既定: 1,000,000）。
  - `0` を指定するとステップ上限なし（開発/診断専用; 無限ループに注意）。
- OOB strict（配列/範囲外の観測）
  - `HAKO_OOB_STRICT=1`（互換: `NYASH_OOB_STRICT=1`）でOOBを観測・タグ出力。
  - `HAKO_OOB_STRICT_FAIL=1` でGate‑C(Core)の実行を非0終了にする（出力のパース不要）。
- Tolerance系（開発専用）
  - `NYASH_VM_TOLERATE_VOID=1` 等の寛容フラグは開発/診断専用。既定ではOFFで、CI/quickでは使用しないこと。


Deprecations
  - `NYASH_GATE_C_DIRECT` は移行中の互換トグル（TTL）だよ。将来は Gate‑C(Core)
  直行（`HAKO_GATE_C_CORE=1`）に統一予定。新しい導線では Core の実行仕様（数値=rc,
  安定化した診断タグ）が適用されるよ。
  - 互換トグルを使うと起動時に警告が出るよ（`HAKO_GATE_C_DIRECT_SILENCE=1` で抑止可）。
  - Stage‑B fallback TTL: 既定OFF（撤退方針）。必要な場合はテスト内限定で `HAKO_STAGEB_ALLOW_FALLBACK=1` を付与する。

Diagnostics (stable tags)
- 本フェーズでは、Gate‑C(Core) の境界で安定タグを整形して出力する:
  - `[core/binop] div by zero`
  - `[core/mir_call] array get out of bounds`
  - `[core/mir_call] array set out of bounds`
  - `[core/mir_call] modulefn unsupported: …`
  - `[core/mir_call] map iterator unsupported`
  - `[core/mir_call] map len missing arg`
  - `[core/mir_call] map set missing key|bad key|missing value|bad value`
  - `[core/mir_call] map get missing key|bad key`
  - `[core/mir_call] unsupported callee type: Closure`
  - `[map/missing] Key not found: <key>`（VM MapBox.get の既定タグ。従来文言を含むため後方互換）
  - `[array/empty/pop] empty array`（VM ArrayBox.pop 空時。strict環境（HAKO_OOB_STRICT=1）で有効）
- Gate‑C Direct では、リーダー/検証レイヤの診断をそのまま用いる（例: `unsupported callee type (expected Extern): ModuleFunction`）。

Strict OOB policy (Gate‑C)
- Enable `HAKO_OOB_STRICT=1` (alias: `NYASH_OOB_STRICT`) to tag Array OOB as stable strings
  (`[oob/array/get]…`, `[oob/array/set]…`).
- With `HAKO_OOB_STRICT_FAIL=1` (alias: `NYASH_OOB_STRICT_FAIL`), Gate‑C(Core) exits non‑zero
  if any OOB was observed during execution (no need to parse stdout in tests).

Default OOB behavior (Gate‑C/Core)
- 既定では OOB はエラー化せず、rc=0（Void/0 同等）として扱う。
- 検証を厳密化したい場合は Strict OOB policy を有効化して安定タグまたは非0終了に切替える。

Exit code differences
- Core: 数値=rc（OS仕様により 0–255 に丸められる。例: 777 → rc=9）、エラーは非0
- Direct: 数値出力のみ（rc=0）、エラーは非0
  - 数値が 255 を超えるケースは標準出力の値で検証すること（rc は下位8ビットへ丸められるため）

注: これらの整形は移行中の暫定仕様で、最終的には Core 側に移管される予定（CURRENT_TASK に TTL を記載）。

Minimal mir_call semantics (Core)
- Implemented (scoped):
  - Constructor: `ArrayBox`（サイズメタ初期化） / `MapBox`（エントリ数メタ初期化）
  - Methods (Array): `size()/push()/pop()/get()/set()`（メタデータでサイズ検証）
  - Methods (Map): `size()/len()/iterator()/set()/get()`（エントリ数メタを返す/更新する／メタから取得する）
  - ModuleFunction: `ArrayBox.len/0` / `MapBox.len/0`（メタのサイズを返す）— 他はタグ付き Fail‑Fast
  - Global/Extern: `env.console.{log|warn|error}`（数値引数のみ印字）
- Others are Fail‑Fast（安定文言を出力）。Closure 生成は v1 bridge で受理（NewClosure 発行）するが、
  実行時の呼出は未実装（VM 側は Fail‑Fast）。

See also: docs/development/architecture/collection_semantics.md（Array/Map のSSOT集約）

String helpers
- Core route（Gate‑C/Core）での最小サポート（Method）:
  - `String.size/0` — 文字列長（バイト）を返す
  - `String.indexOf/1` — 最初の一致位置、なければ -1
  - `String.lastIndexOf/1` — 最後の一致位置、なければ -1
  - `String.substring/2` — 半開区間 [start, end) を返す
- インデックス規約（bytes ベース）:
  - start/end は範囲 [0, size] にクランプされる（負の値は 0、size 超は size）。
  - start > end の場合は空文字（size==0）。
  - インデックスは UTF‑8 のバイト境界（コードポイント境界ではない）。
- ModuleFunction:
  - `StringHelpers.to_i64/1` — interpreter に inline 実装あり（数値文字列のみを許容）。
    数値結果が 255 超の場合、rc は下位8ビットに丸められるため、標準出力の値で検証すること。

Core dispatcher canaries（直行ルート）
- `profiles/quick/core/canary_core_dispatcher_*` は Gate‑C(Core) 直行へ移行済み。
  一部（大きな値や plugin‑enabled 経路）では rc 正規化が未整備のため、数値は標準出力を優先して検証し、
  rc はフォールバックとして扱う（TTL; 収束後に rc 検証に戻す）。

Aliases
- Keep existing logical module names in `hako.toml` and introduce aliases to
  new paths when transitioning.
Mini‑VM bring‑up (Phase 20.34 — notes)
- MiniMap box: minimal string‑backed register map was split out to
  `lang/src/vm/boxes/mini_map_box.hako` and is imported via
  `using selfhost.vm.helpers.mini_map as MiniMap`.
- InstructionScanner fixes: ret/const misclassification fixed. If an object
  has `op` or `dst`, it is not treated as implicit ret. Typed const detection
  tolerates spaces.
- PHI handling (dev aids):
  - Runtime trace: set `NYASH_VM_TRACE_PHI=1` to log block predecessors and
    PHI `inputs.pred` at application time. On strict mismatch, the trace logs
    `dst`, `last_pred`, `inputs`, and the block `preds` set to aid diagnosis.
  - Strictness: `HAKO_VM_PHI_STRICT=0` or `NYASH_VM_PHI_STRICT=0` can relax the
    fail‑fast during bring‑up. `NYASH_VM_PHI_TOLERATE_UNDEFINED=1` substitutes
    `Void` when a referenced input is undefined.
  - Builder (merge) policy: if/merge PHI inputs are now limited to reachable
    predecessors only. Unreachable `else_block` is no longer synthesized as an
    input. Enable `HAKO_PHI_VERIFY=1` or `NYASH_PHI_VERIFY=1` to print missing
    or duplicate predecessor inputs at build time.

Trace/verify quickstart
- Preludes (using) parse context: `NYASH_STRIP_DEBUG=1` prints surrounding
  lines for parser errors to accelerate pinpointing braces/ASI issues.
- PHI trace: `NYASH_VM_TRACE_PHI=1` alongside a Core run (`--json-file`) shows
  predecessor→incoming selection decisions.
