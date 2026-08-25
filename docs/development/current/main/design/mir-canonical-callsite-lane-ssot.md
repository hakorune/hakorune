---
Status: SSOT
Scope: MirInstruction の call-site 正規化（BoxShape lane, no behavior expansion）
Decision: accepted (phase29y-safe lane)
Updated: 2026-08-25 (MIR-CALL-RETIREMENT-v1 accepted)
Related:
- docs/reference/mir/INSTRUCTION_SET.md
- docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md
- src/mir/instruction.rs
- src/mir/definitions/call_unified.rs
- src/mir/contracts/backend_core_ops.rs
---

# MIR Canonical Callsite Lane (SSOT)

## 目的

MIR 命令数そのものを急いで減らすのではなく、まず「call-site 表現の入口」を 1 本化して診断距離を短くする。

- ゴール: backend 手前で call 系表現を canonical へ寄せる
- 方針: BoxShape（責務整理）を先行、BoxCount（受理拡張）はしない
- 契約: fail-fast。曖昧 fallback を増やさない

## 非目標

- 新しい受理形の追加
- AST rewrite
- NewClosure retire（NCL-2 で `Call(callee=Closure...)` の shape 契約は固定済み。`NewClosure` 自体の retire はこの lane の非目標）
- NewBox の即時統合（`Call` の legacy `func` 必須を先に整理する必要がある）

## 実装上の前提（現状）

- `MirInstruction::Call` は `func: ValueId` + `callee: Option<Callee>` の過渡形
- `used_values()` は `callee=None` でのみ `func` を使用し、`callee=Some(Method{receiver})` は receiver を使用
- `ValueId::INVALID` が sentinel として利用可能

参照:
- `src/mir/instruction.rs`
- `src/mir/instruction/methods.rs`
- `src/mir/value_id.rs`

## 最終像（この lane で到達する形）

- backend 入口では call-site を `MirInstruction::Call` で観測できる
- `BoxCall` / `ExternCall` は canonicalization 後に backend へ流れない
- `Call { callee: None }` は backend 入口で freeze（ただし `func=<const-string>` 形は MCL-5 で `Call(callee=Global)` へ正規化）
- docs SSOT と実装 ledger は常に同期（既存テストを維持）

注:
- DebugLog→Debug 統合と Nop retire は別 lane に分離する（本 lane では扱わない）



## 実行手順（1タスク=1コミット）

### MCL-0: Canonicalization pass 入口を作る（挙動不変）

- 追加:
  - `src/mir/passes/callsite_canonicalize.rs`（新規）
  - `src/mir/passes/mod.rs` へ module 追加
- ルール:
  - pass は MIR module を走査し、命令差し替えのみを担当
  - backend 判定・reject はこのコミットで入れない（次コミットに分離）
- 受け入れ:
  - ビルド緑
  - 既存 test 緑

### MCL-1: `BoxCall -> Call(callee=Method)` 変換を追加

- 変換契約:
  - `BoxCall { dst, box_val, method, args, effects }`
  - `=> Call { dst, func: ValueId::INVALID, callee: Some(Callee::Method{ box_name, method, receiver: Some(box_val), certainty, box_kind }), args, effects }`
- 注意:
  - `box_name/certainty/box_kind` が不明な場合は conservative 値を使う（runtime data + union など）
  - 変換不能なら即 freeze（silent keep しない）
- 受け入れ:
  - `used_values()` が receiver + args を維持
  - parity break を起こさない

### MCL-2: `ExternCall -> Call(callee=Extern)` 変換を追加

- 変換契約:
  - `ExternCall { dst, iface_name, method_name, args, effects }`
  - `=> Call { dst, func: ValueId::INVALID, callee: Some(Callee::Extern("<iface>.<method>")), args, effects }`
- 受け入れ:
  - extern route の結果/副作用が既存と一致
  - legacy `ExternCall` が backend 入口まで残存しない

### MCL-3: backend 入口の fail-fast 契約を固定

- 追加契約:
  - backend 入口で `Call { callee: None }` を reject（freeze）
  - backend 入口で `BoxCall` / `ExternCall` 残存を reject（freeze）
- 受け入れ:
  - reject tag が安定
  - 既存 green ケースは維持

### MCL-4: docs / tests 同期

### MCL-5: `Call(callee=None, func=<const-string>)` を `Call(callee=Global)` へ正規化

- 変換契約:
  - `Call { callee: None, func: <const-string value-id>, args, ... }`
  - `=> Call { callee: Some(Callee::Global(<string>)), func: ValueId::INVALID, args, ... }`
- 目的:
  - Program(JSON v0) runtime route で `call-missing-callee` freeze を回避し、MCL lane の backend 契約へ合流させる。
- 受け入れ:
  - `mcl5_rewrites_legacy_call_with_const_string_func_to_global_callee` が green。


- 更新:
  - `docs/reference/mir/INSTRUCTION_SET.md`（必要なら運用注記のみ）
  - `docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md`（cohort 説明）
- テスト:
  - `instruction_diet_ledger_counts_match_docs_ssot` を維持
  - canonicalization 後に legacy call-site 命令が 0 であることを確認する unit test を追加

## post-canonical retire queue（次レーン）

詳細SSOT: `docs/development/current/main/design/mir-callsite-retire-lane-ssot.md`

- 固定順序:
  - 1) Rust 側で legacy call-site を canonical call に吸収（MCL-0..5 完了）
  - 2) `.hako` mirbuilder の新規出力を canonical call-site へ移行
  - 3) `BoxCall/ExternCall` を enum から retire
- キュー:
  - RCL-0 (docs-only): done（`mir-callsite-retire-lane-ssot.md` で契約固定）
  - RCL-1 (BoxCount): done（`.hako` mirbuilder の emit を `Call(callee=Method/Extern)` へ統一）
  - RCL-2 (BoxShape): done（strict/dev の stage1 selfhost MIR 受け口で legacy emit を fail-fast reject）
  - RCL-3 (BoxShape): done（min1/min2/min3 完了。`BoxCall/ExternCall` enum retire 済み）
  - RDN-0 (separate lane): `DebugLog/Nop` retire は callsite lane と分離

## NewClosure clean path（2段階 + 契約固定）

- NCL-0: done（`Call(callee=Closure)` は canonicalization pass で `NewClosure` へ正規化し、backend 境界では `call-closure-not-canonical` を fail-fast）
- NCL-1: done（`NewClosure.body` は `body_id -> module.metadata.closure_bodies` へ外出しし、canonical 形は `body=[]` を維持）
- NCL-2: done（shape 判定を SSOT 化。`dst=Some + args=[]` のみ canonicalize、それ以外は shape-specific fail-fast）

## Core Call target retirement successor (active)

MCL/RCLはbackend入口のcanonicalizationと旧instruction variant退役を閉じた。
core `MirInstruction::Call` 自体は、なお
`func: ValueId + callee: Option<Callee>`というmigration carrierである。
次のlaneは新しいCall authorityではなく、このSSOTのretirement successorとする。

```text
MIR-CALL-LEGACY-TARGET-CENSUS-D0
  -> MIR-CALL-CANONICAL-CORRIDOR-GUARD-I0
  -> MIR-CALL-LEGACY-TARGET-RETIREMENT-R0
```

`CENSUS-D0`は全`callee: None`をcanonicalizer input、明示compatibility、test、
diagnostic、unreachableへ分類する。text countや`ValueId::INVALID`だけで
retirementを選ばない。`GUARD-I0`はまずselected native/canonical corridorだけで
`callee.is_some() == 100%`、legacy func authority/fallbackが0であることを固定する。
現状は`callee=Some`でもJoinIRやJSON transportが有効な`func`値を併記するため、
`func == ValueId::INVALID`をこのguardの不変条件にはしない。また、このcountを
証明するためだけの新しい`FunctionMetadata` semantic rowも追加しない。最初の
consumerはmodule/backend境界のpure structural censusでなければならない。

`MIR-CALL-RETIREMENT-v1`はend stateを次の一つへ固定する。

```text
Call { dst, callee: Callee, args, effects }
```

JSON-v0 compatibilityはcanonical MIR外のowner-private
`JsonV0CallInput`にだけ隔離し、exact targetを一度だけ`Callee`へ解決してからCallを
構築する。このinputはsource evidenceであって第二Call authorityではない。
`MirInstruction::LegacyCall`、`Option<Callee>`、`func`、default/sentinel Callee、
optimizer target再推論、backend string fallback/retryは最終形に残さない。

Direct MIR JSON-v0のlegacy `func`は、raw function draftから先に作る
owner-private immutable catalogだけをsource authorityにする。catalogは同一関数内の
直接`Const(String)`定義をValueIdへ対応付け、解決時に一意性・非String・foreign・
`ValueId::INVALID`を検証する。成功時は文字列をそのまま`Callee::Global`へ投影し、
module membership、arity suffix、Extern分類、alias追跡、optimizer/backend lookupは
行わない。`call`とnested `mir_call`は同じinput ownerを使い、nested nodeがtarget/
args/effects、outer nodeが`dst` overrideを所有する。

Program JSON-v0のgeneric `ExprV0::Call`は、top-level local defsから先に作る
owner-private immutable `ProgramCallTargetCatalog`を使う。import aliasのstatic/extern
producerは既存ownerとしてcatalog外にfenceし、post-lowering import mergeをmembership
authorityにしない。unique `(short name, arity)`はqualified `Callee::Global`へ一度だけ
投影し、ambiguous local candidateはtyped reject、候補のないsource nameは文字列を保った
exact `Global` terminalとする。`env.`/`nyash.`はnumeric arity suffixだけを除いて
`Callee::Extern`へ投影する。空名・重複qualified definitionはCall/Block publication前に
rejectする。catalogはmain/defs lowering前、targetはargument lowering前に確定し、
`func_map`、`maybe_resolve_calls`、target Const、Program bridge late issuer、import
merge scan、optimizer/backend/runtime lookupはauthorityではない。

このcanonical production boundaryはdirect MIR-v0、selected Rust VM/JSON/LLVMまでを
覆う。Program-v0はR3の別catalog owner、PyVM（daily route 0・diagnostic-only）、
reference-vm、Python/llvmliteは歴史/互換ownerとして`ParkedSealed`に分類する。これらは
現行R5の実装対象でもterminal closure edgeでもなく、canonical acceptanceへ再入場するのは
selected backend policyが明示的に再開した時だけとする。PyVMの物理削除は別SSOTの
removal conditionで扱う。

literal censusに加えて、`runner/mir_json_v0/call.rs`の入力依存stateも数える。
現HEADの選択境界では、Program-v0の旧literalは削除済みで、MIR-v0はresolve/reject
後にだけ発行するため、missing-callee publicationはzeroである。これは
`callee: None` の検索だけでなく、typed stale `func` と `Method(None)` の別edgeを
分類したうえで維持するzeroであり、runtime target authorityの消滅を意味しない。

field削除前に`Callee`のValueId operand ownerを一つにする。Method.receiver、
Value(value)、Closure.captures、Closure.me_captureをtarget operandとし、
Global/Extern/Constructorはtarget operandを持たない。Call argsは全variantで順序を
保ってoperandになる。escape判定はこのprojectionを再利用してよいが、別policyの
まま保つ。

R4a closed: `Callee` owns the ordered occurrence rewrite API
`rewrite_value_operands(&mut self, FnMut(&mut ValueId))`. The order is
Method.receiver, Value, Closure captures in stored order, then me; duplicates are
retained. The match is exhaustive with explicit empty Global/Extern/Constructor/
Method(None). SimplifyCFG is migrated and parity/guard evidence is green; `func`,
Call args, ownership, escape, JoinIR, and field deletion remain separate rows.

R4b closed: `used_values` has 56 direct non-test expressions across 37 files plus
one BasicBlock edge. The accepted law is typed Callee occurrences in the same
order, then args; legacy `None` keeps func once before args and duplicates remain.
The immutable exhaustive `Callee::for_each_value_operand` facet and `methods.rs`
Call arm are now green with owner/typed/legacy tests and the shared guard.

R4c closed (`4c6d9ce9a2`): `value_consumer` delegates Call membership once to
`MirInstruction::used_values`; focused refresh/fact-omission/legacy suite 5/5,
shared corridor/pointer/rustfmt/diff green, and 433 warnings remain baseline.
Its direct-set and per-instruction dedup policy stays local; typed `Callee`
targets are counted before args while legacy `None` keeps one `func` use.

R4d D0 accepted — `MIR-CALL-CANONICAL-OPERAND-ESCAPE-POLICY-D0`:
Decision: Callee enumerates target operands; escape assigns Call to Method.receiver/Value/args and Capture to Closure captures/me.
Source authority + canonical issuer: `MirInstruction::Call`/`Callee` -> `classify_escape_uses` -> DCE, escape, and FastMem consumers.
Non-authority: `used_values` generic uses, ownership SSA, FastMem allowlists, JoinIR/Query/CallLike, optimizer/backend text, PyVM/reference/Python.
Fail-fast boundary: invalid Closure shape remains the existing canonicalization reject; legacy `None.func` stays unclassified by the shared barrier and ordinary-use fail-closed in FastMem; no target re-inference.
Smallest next slice: update only `classify_escape_uses` Call membership, add the finite matrix tests, and extend the shared corridor guard; DCE/FastMem/VM policies remain consumers.
Non-claims: ownership activation, JoinIR remap, CallLike retirement, R5/R6, Method(None)/Closure/NewBox/Constructor finalization, warning cleanup; census is `classify_escape_uses` Call/Closure -> DCE/escape/FastMem, all other policies excluded.

### R4d accepted escape role matrix

| Call shape | projected values | shared escape role | compatibility note |
| --- | --- | --- | --- |
| `Some(Method { receiver: Some(v), .. })` | receiver | `Call` | instance receiver is the target operand |
| `Some(Method { receiver: None, .. })` | none | none | static form remains non-authoritative; qualified `Global` is the final form |
| `Some(Value(v))` | target value | `Call` | first-class callable target is a call barrier |
| `Some(Closure { captures, me_capture, .. })` | captures, then `me_capture` | `Capture` | pre-canonical constructor; same role as `NewClosure` |
| `Some(Global/Extern/Constructor)` | none | none for target | stored `args` still receive `Call` role |
| `None` legacy | `func` remains generic use only | no shared barrier | `used_values` preserves `func`; FastMem stays `ordinary_use` and fail-closed |
| every shape | `args` in stored order | `Call` | duplicates remain occurrences; each consumer keeps its own dedup policy |

Finite production census: one classifier Call arm, seven DCE local-field
expressions, one FastMem shared extraction plus three FastMem allowlist arms, and
one opt-in Rust escape consumer. The structural helper supplies occurrences;
each consumer retains its barrier/allowlist policy.

R4e D0 accepted — `MIR-CALL-CANONICAL-OWNERSHIP-POLICY-D0`:
Decision: do not mechanically apply generic Callee occurrences to Ownership SSA; use the explicit managed/unknown role matrix below.
Source authority + canonical issuer: `Callee` projection owns occurrences; Ownership SSA classification/ABI owns kinds and policy; no callee ABI issuer exists yet.
Non-authority: typed `func`/`INVALID`, `used_values`, variant spelling, backend/runtime inference, JoinIR/Query/CallLike, PyVM/reference/Python.
Fail-fast boundary: managed or unknown typed targets reject before liveness/witness seal; legacy `None` retains its `func` check once, typed `Some` never reads `func`; verifier activation remains zero.
Smallest next slice: replace only the Call arm in `verify_instruction_kinds`, add exact positive/negative/error-precedence tests, and extend the shared guard; no production witness installer.
Non-claims: ownership ABI/witness activation, managed-call support, R5/R6, backend changes, JoinIR/query/CallLike retirement, and warning cleanup; census is `ownership_ssa/verify.rs` Call scan -> liveness/witness and conditional backend preflight.

### R4e accepted ownership role matrix

| Call shape | target authority | ownership treatment | legacy field |
| --- | --- | --- | --- |
| `None` | legacy `func` | preserve current known-`None` requirement; args/dst keep current `None`-only policy | `func` is read once |
| `Some(Global/Extern/Constructor)` | no target operand | args/dst keep current `None`-only policy | ignored, including `INVALID` |
| `Some(Method { receiver: None, .. })` | no target operand | args/dst keep current `None`-only policy; static final form remains qualified `Global` | ignored |
| `Some(Method { receiver: Some(v), .. })` | receiver `v` | receiver, args, and dst must be known `None`; managed/unknown rejects before liveness | ignored |
| `Some(Value(v))` | target value `v` | target, args, and dst must be known `None`; managed/unknown rejects before liveness | ignored |
| `Some(Closure { captures, me_capture, .. })` | pre-canonical constructor operands | captures/me use generic `used_values` liveness; no managed-call target predicate; existing closure-shape canonicalization remains authoritative | ignored |

The matrix is a policy over the structural occurrence projection, not a new
ownership receipt. `ManagedCallOwnershipUnsupported` remains the fail-fast
terminal until a separately named managed-call ABI issuer exists.

R4f T0 selected — `MIR-CALL-CANONICAL-CALLLIKE-T0-R0`:
Decision: retire the private `CallLikeInst` metadata adapter; canonical
`MirInstruction::dst_value`/`used_values` remain the only Call metadata owner.
Source authority + canonical issuer: `MirInstruction::Call` plus the existing
`Callee` operand projection; `instruction_kinds` is observation-only.
Non-authority: `CallLikeInst`, duplicated receiver/func reconstruction, and
metadata-local target inference. Fail-fast boundary: `dst_via_meta` and
`used_via_meta` delegate/directly project canonical Call shape without changing
the Call schema or reading a typed legacy `func`. Smallest next slice: remove
the enum/impl, replace its two metadata arms with canonical delegation, add
parity tests and extend the shared guard. Non-claims: `func`/`Option<Callee>`
cutover, Query policy, JoinIR, backend activation, and warning cleanup.

Query D0 accepted — `MIR-CALL-CANONICAL-QUERY-POLICY-D0`:
Decision: treat `MirQuery` as an observation facade and do not let its Call arm
reconstruct target meaning. Source authority + canonical issuer: the existing
`MirInstruction::used_values`/`dst_value` plus `Callee` projection; Query only
projects the result to its read/write view. Non-authority: the local
`Callee::Method`/legacy-`func` match, Query variant spelling, JoinIR inference,
and backend/runtime fallback. Fail-fast boundary: `MirQueryBox::reads_of` and
`writes_of` must have one finite Call policy before R6; typed Value/Closure
operands are not optional decorations, and legacy `None.func` remains explicit
compatibility only. Smallest next slice: replace the one Query Call arm with
direct canonical delegation, add parity/negative tests, and extend the shared
guard; this is a non-production T0, not an I0 caller switch. Non-claims: R6
schema cutover, ownership ABI, JoinIR remap, PyVM/reference/Python, and warning
cleanup.

Query D0 finite state matrix:

| Query observation | Read authority/result | Write authority/result | Local terminal/fallback |
| --- | --- | --- | --- |
| typed `Some(Global/Extern/Constructor/Method(None))` | `MirInstruction::used_values`: args in stored order | `MirInstruction::dst_value`: `dst` or empty | project only; no target inference |
| typed `Some(Method(Some(v))/Value(v))` | `used_values`: target operands, then args | `dst_value`: `dst` or empty | project only; no receiver/name retry |
| typed `Some(Closure { captures, me_capture })` | `used_values`: captures, `me_capture`, then args | `dst_value`: `dst` or empty | preserve occurrence order/duplicates; no constructor reclassification |
| legacy `None` | `used_values`: `func` once, then args | `dst_value`: `dst` or empty | compatibility observation only; no retry |
| non-Call instruction | existing canonical instruction methods/arms | existing canonical instruction methods/arms | unchanged; Query does not issue semantic facts |

Census boundary: `MirQueryBox::reads_of`/`writes_of` -> the single non-test
`loop_form_intake` reader -> `loop_to_join` and toggle-gated skip/trim probes;
includes the one production impl and test-only alternate impl, excludes JoinIR
remap/merge, PyVM/reference/Python, and R6 schema ownership. The Query methods
return observations rather than rejects: malformed or unsupported target
meaning is rejected by its upstream canonical ingress, never retried here.

Query T0 closeout (`981ec1d583`): `MirQueryBox::reads_of(Call)` now delegates
to `MirInstruction::used_values`, and `writes_of(Call)` delegates to
`dst_value`. The finite matrix above is covered by two focused tests; the
shared corridor guard also rejects a local `callee`/`func` reconstruction.
This is an observation closeout, not a production caller switch.

R5 design decision accepted — `MIR-CALL-CANONICAL-TERMINAL-CLOSURE-D0`:

```text
Decision: close every selected terminal as a typed-Callee consumer, then cut
the core schema only after R5 evidence; split R5 into optimizer, selected Rust
interpreter, printer/JSON, and selected-native-backend owners.
Source authority + canonical issuer: typed producers or one compatibility
ingress issue Callee; MirInstruction::Call stores it; terminals only dispatch
or project that stored target.
Non-authority: func/ValueId::INVALID, target Const(String), optimizer scans,
backend symbol lookup, MirCall/CallFlags before the R6 gate, PyVM/reference/
Python, and warning counts.
Fail-fast boundary: malformed/missing legacy input rejects before Call publish;
terminal unsupported-target errors do not retry another target or by-name path.
Smallest next slice: R5a bounded optimizer target-issuer census and decision;
no code or production switch until its caller/producer boundary is closed.
Non-claims: R6 field deletion, JoinIR remap, Method(None), Closure/Constructor
shape retirement, normal-root cleanup, and non-selected backend activation.
```

Worker census result: R5 is not one atomic implementation. R5a is selected as
an optimizer-only I0; R5b is split so its first row only rejects the selected
Rust VM `None -> func` execution edge; R5c printer is separable but JSON
writer remains a compatibility-mixed design stop; R5d native is a design stop
because the selected pure-first route still has pattern fallback and its C
owners are at the 760/793-line boundary. PyVM/reference/Python/WASM and
`native_driver` remain `ParkedSealed`.

R5 finite terminal task matrix:

| Task | owner boundary | old edge to delete | acceptance evidence |
| --- | --- | --- | --- |
| R5a optimizer (selected) | optimizer-only schedule, CSE key, optimizer diagnostic | optimizer canonicalizer call, typed `func` key/scan, Const/String target issuer | optimizer caller/issuer/retry = 0; post-RC/JSON compatibility issuer remains outside; key and diagnostic parity |
| R5b-B0 Rust interpreter (next) | `handlers/calls`, both instruction execution loops | `None -> func` register load and module by-name execution | `None` is typed terminal reject; typed Global/Method/Extern parity; no field deletion or method fallback claim |
| R5c printer (later) | MIR printer observers only | legacy `func` rendering and typed-call dummy display | typed target projection parity; JSON remains separate |
| R5c JSON (design stop) | v1 writer, v0 projection, legacy wire/profile switches | mixed canonical/compatibility emission and `Method(None)` receiver reuse | profile split and egress authority decision before code |
| R5d native (design stop) | selected `ny-llvmc` pure-first route | missing structured callee abort boundary and pattern fallback | route/profile decision plus C-owner split before implementation |

R5a selected brief (closed at `e36f86e869`):

```text
Decision: retire only the optimizer's target-issuing/legacy-observer edges;
shared compiler post-RC and JSON compatibility canonicalizers stay outside.
Source authority + canonical issuer: typed producer or compatibility ingress
-> Callee -> MirInstruction::Call; optimizer stores/keys/diagnoses only.
Non-authority: optimizer func/INVALID, Const(String), value-type inference,
module membership, backend/runtime lookup, and CSE legacy target keys.
Fail-fast boundary: optimizer never repairs/retries a target; residual legacy
input remains an explicit compatibility observation until R6.
Smallest next slice: remove optimizer schedule call, make CSE keys Callee-based,
and remove diagnostic func->Const scan; add matrix tests and shared guard.
Non-claims: shared canonicalizer retirement, R5b/c/d, R6 fields, Method(None),
Closure/Constructor, JoinIR, PyVM/reference/Python, and warning cleanup.
```

R5b-B0 selected brief:

```text
Decision: selected Rust interpreter treats missing Callee as a terminal typed
reject; it does not load func or retry module lookup. Existing typed dispatch
remains the only execution path for this row.
Source authority + canonical issuer: typed producer or compatibility ingress
-> Callee -> MirInstruction::Call; interpreter consumes the stored Callee.
Non-authority: func, register String values, interpreter function-map lookup,
Method(None) fallback, plugin/method recovery, and non-selected backends.
Fail-fast boundary: before any `reg_load(func)` or module `functions.get`,
`callee=None` returns the stable `call-missing-callee` terminal error.
Smallest next slice: change `handlers/calls::handle_call` only, add a direct
negative test and preserve both instruction-loop callers and current fields.
Non-claims: `func`/Option deletion, Method(None), printer/JSON/native closure,
JoinIR, PyVM/reference/Python, warning cleanup, and any VM method fallback.
```

R5a closeout evidence: `e36f86e869` removes the optimizer late schedule, makes
typed CSE keys independent of stale `func`, and removes the diagnostic
`func -> Const(String)` scan. The callsite compatibility suite is 14/14,
CSE key tests are 3/3, and the optimizer filter includes the new diagnostic
test; the shared corridor guard, pointer guard, and `git diff --check` are
green. The optimizer filter also retains two unrelated known baseline freezes
(`mir/instance-constructor-source/cohort-missing`), so they are not attributed
to this row. Warning count remains the 433-item baseline.

R5b-B0 closeout evidence: commits `95427f2cd6` and `67dd7e400a` make
`handlers/calls::handle_call` reject `None` before `reg_load(func)` or module
lookup, and update the vm-reference fixture to the typed method carrier. The
negative test is 1/1, typed Method parity is 18/18, typed Global call-contract
coverage is 8/8, and typed Extern provider coverage is 4/4. The shared
corridor/pointer guards and feature lib check are green; feature test warnings
are the 437-item vm-reference baseline (the default lane remains 433). R5b
keeps the instruction fields and all typed dispatch/method-fallback boundaries
unchanged.

R5c printer-only selected brief:

```text
Decision: printer observers project the stored typed Callee; they do not read
func or reconstruct a target. This row changes display only.
Source authority + canonical issuer: MirInstruction::Call.callee is issued by
the existing producer/compatibility ingress and is the printer's sole target.
Non-authority: func/INVALID, Const(String), JSON wire profiles, backend lookup,
and any target classification or retry.
Fail-fast boundary: typed Call display succeeds from Callee; legacy None keeps
an explicit compatibility rendering and is not silently presented as typed.
Smallest next slice: remove the display observer that always prints func,
route typed/legacy rendering through one printer projection, and add parity
tests plus the shared corridor guard.
Non-claims: JSON writer, Method(None), Closure/Constructor, interpreter,
native/other backends, core field deletion, JoinIR, PyVM/reference/Python.
```

R5c JSON egress design stop (`MIR-CALL-JSON-EGRESS-D0`):

```text
Decision: JSON egress uses typed Callee as the sole meaning source; v1 writer
and owner-private v0 compatibility projection are separate profiles.
Source authority + canonical issuer: existing producer/JsonV0 ingress issues
MirInstruction::Call.callee; writer projects it without reclassification.
Non-authority: func/INVALID, Const(String), wire strings, environment switches,
backend lookup, backend_shape post-wire reclassification, and parked backends.
Fail-fast boundary: targetless None+INVALID, Method(None), unsupported
Global, and profile-incompatible Closure/Constructor reject before JSON publish;
no func fallback or retry.
Smallest next slice: profile authority, finite input matrix,
Method(None)/Closure/Constructor boundary, and post-wire mutation census.
Non-claims: R6 field deletion, Method(None) retirement, Closure/NewBox
integration, native implementation, or PyVM/reference/Python/native_driver.
```

The census boundary is `MirInstruction::Call` producer ->
`mir_json_emit::emitters::{mod,calls,helpers}` -> `root/io` -> CLI, ny-llvmc,
and compatibility loader. The finite target matrix is:

| Stored target | v1 profile | v0 compatibility profile | D0 blocker |
| --- | --- | --- | --- |
| `Global` / `Extern` | typed `mir_call` | `call` / `externcall` projection | Global(print) and `backend_shape` reclassification |
| `Method(Some(r))` | typed receiver | typed or `boxcall` projection | profile-specific receiver authority |
| `Method(None)` | nullable receiver | `boxcall(box=func)` today | old `func` receiver reuse must reject |
| `Value` | typed `mir_call` | legacy `call(callee)` | profile round-trip only |
| `Constructor` / `Closure` | `NewBox` / `NewClosure` projection | Call-shaped compatibility | construction-versus-call boundary |
| `None + valid func` | incompatible | explicit legacy `call(func)` | compatibility ingress only |
| `None + INVALID` | incompatible | targetless legacy shape | reject before publish |

The profile census includes `root.rs` schema selection, per-call dialect
switches (`NYASH_JSON_SCHEMA_V1`, `NYASH_MIR_UNIFIED_CALL`, and methodize),
`emit_call_with_optional_func`, `Method(None)` receiver reuse, unconditional
closure projection, and `backend_shape` mutation. Existing v0/print/methodize
parity is positive evidence; typed `func` decoration ignored, `None+INVALID`,
Method(None) fallback, Constructor/Closure mismatch, and post-wire target
reclassification are negative evidence. The accepted D0 boundary keeps the
current root/profile selector and compatibility owners unchanged while
retiring one typed-path decoration edge first.

R5c JSON D0 closeout — first fast row:

```text
Decision: v0 JSON projections of an explicit Callee must not emit the stale
numeric func decoration; legacy None and Method(None) compatibility remain.
Source authority + canonical issuer: stored Call.callee -> calls.rs typed v0
projection; root profile selection remains the existing compatibility owner.
Non-authority: func/INVALID for typed Global/Extern/Constructor/Value/Closure,
wire strings, backend_shape mutation, and target reclassification.
Fail-fast boundary: typed projection never reads func; only explicit legacy
None uses emit_call_with_optional_func, while Method(None) stays R6-gated.
Completed slice: MIR-CALL-JSON-TYPED-DECORATION-I0-R0; typed helper no longer
forwards func, v0 typed/legacy/Method receiver parity is 8/8, and the guard is green.
Non-claims: profile split D1, Method(None) retirement, Closure/NewBox,
backend_shape, native, R6, and PyVM/reference/Python/native_driver.
```

Exact old edge: `emit_call_with_callee_v0` forwards `func` to
`emit_call_with_optional_func`, which emits a numeric `func` whenever the
decoration is non-`INVALID`. The I0 removes that forwarding only for explicit
typed variants. `Method(Some(receiver))` retains receiver projection;
`Method(None)` retains its compatibility `receiver <- func` edge until R6.
Positive acceptance is v0 typed Global/Extern/Constructor/Value/Closure output
without numeric `func`, plus unchanged v1 output. Negative acceptance is stale
typed `func` ignored, explicit legacy `None` still emitted, and no change to
Method(None), root profile, or backend_shape behavior.

JSON profile D1 accepted decision:

```text
Decision: root selects exactly one JsonEgressProfile; CanonicalV1 is the mainline profile and CompatibilityV0{methodize} is the owner-private legacy profile. Mixed selectors reject.
Source authority + canonical issuer: root selector parses S/U/M once; stored Call.callee remains the semantic issuer; emitters only project the selected profile.
Non-authority: func/INVALID except explicit V0 legacy ingress, wire text, independent env reads, backend_shape mutation, post-wire lookup/retry, and parked backends.
Fail-fast boundary: invalid/mixed selector, targetless Call, and unsupported profile/target combinations reject before root or instruction publication; malformed explicit targets never retry through legacy fields.
Completed slice: MIR-CALL-JSON-PROFILE-I0-R0 landed at `db350b81c9`; the immutable profile is threaded from root through emitters/mod.rs to calls.rs, per-call selector reads are gone, root tests are 2/2, calls tests are 8/8, and the shared guards are green.
Next design stop: MIR-CALL-JSON-BACKEND-SHAPE-D1-CAPABILITY-AUTHORITY; no R4c, native, or R6 implementation is implied.
Non-claims: backend_shape removal, Method(None) retirement, Closure/NewBox decision, loader fallback split, native, or R6 core schema cutover.
```

Selector state matrix (`S=NYASH_JSON_SCHEMA_V1`, `U=NYASH_MIR_UNIFIED_CALL`,
`M=HAKO_MIR_BUILDER_METHODIZE`):

| S | U | M | outcome |
| --- | --- | --- | --- |
| Unset/On | Unset/On | valid or ignored | `CanonicalV1` |
| Unset | Off | valid (`Unset` defaults true) | `CompatibilityV0{methodize}` |
| Off | Unset/Off | valid (`Unset` defaults true) | `CompatibilityV0{methodize}` |
| On | Off | any valid | typed reject `mixed_profile` |
| Off | On | any valid | typed reject `mixed_profile` |
| Invalid | any | any | typed reject `invalid_selector` |
| any | Invalid | any | typed reject `invalid_selector` |
| any | any | Invalid | typed reject `invalid_selector` |

The first row includes `S=Unset,U=Unset`, which is the canonical default. `M`
is parsed for validity once; it is semantically ignored by `CanonicalV1` and is
carried only by `CompatibilityV0`. The profile is immutable after selection.
`backend_shape` remains an explicitly separate compatibility owner and is not
silently folded into this profile decision.

I0 acceptance:

```text
positive: one root profile reaches every production Call emitter; CanonicalV1
         preserves typed mir_call output; CompatibilityV0 preserves v0
         Global/Extern/Method/Value/Constructor/Closure projections.
negative: calls.rs/helpers.rs read no selector env; mixed/invalid selectors,
          targetless production Call, and profile mismatch fail before publish.
parity: root schema kind, callee, receiver, args, dst, and existing effects
        projection are unchanged for each accepted profile; backend_shape,
        loader retry/fallback, Method(None), and construction boundaries stay
        outside the I0 claim.
```

JoinIR operand-remap D0 result (`NoSafeSlice`; next design stop:
`MIR-CALL-JOINIR-CALLER-LIFECYCLE-BOUNDARY-D1`):

```text
Decision: JoinIR Call collection/remap may delegate target ValueIds to the
  Callee operand projection, but no production edit is admitted until its
  named merge caller and shared lifecycle boundary are finite and observed.
Source authority + canonical issuer: Callee::for_each_value_operand and
  Callee::rewrite_value_operands own target occurrence order; MirInstruction
  ::Call owns args/dst; JoinIR only remaps IDs and never reclassifies targets.
Non-authority: local remap_callee matches, func/INVALID, generic used_values
  policy, target strings, backend lookup, and caller-zero/test fixtures.
Fail-fast boundary: collect/remap must be classified for every non-test
  JoinIR merge caller before delegation; an unowned or shared-lifecycle edge
  is NoSafeSlice, with no partial helper replacement or fallback.
Smallest next slice: read-only census of JoinIrIdRemapper Call arms and all
  direct merge/lifecycle callers; if the boundary closes, one R4c row replaces
  the local match and incomplete Call collection, with positive/negative/
  parity evidence and the shared corridor guard.
Non-claims: Method(None), Closure/Constructor shape, MirCall/CallFlags,
  mandatory-Callee schema, backend/native, PyVM/reference/Python, or warnings.
```

Finite D0 boundary: start at `src/mir/builder/joinir_id_remapper.rs` Call
collection/remap arms; include non-test direct consumers in
`builder/control_flow/joinir/merge/**` and value-lifecycle callers; exclude
phase test modules, disconnected/caller-zero fixtures, unrelated MIR
instructions, and all backend/native/parked routes. Completion requires a
finite owner/caller inventory, one lifecycle authority, and zero open or
reopened JoinIR remap blockers; otherwise the row remains `NoSafeSlice`.

D0 census result (read-only audit at current HEAD):

```text
collect_values_in_block -> merge/value_collector.rs plus two value-lifecycle
  owners; remap_instruction -> merge instruction and terminator rewriters.
The JoinIR merge graph has no non-test production caller (caller-zero), while
value_lifecycle.rs::verify_typed_values_are_defined and
value_lifecycle_definition.rs::prepare_transient_stale_value_facts_v1 have
different reachable/stale/retention and fail-fast semantics. Therefore a
shared Callee projection is observable, but a shared lifecycle authority is
not issued. R4c remains NoSafeSlice and no JoinIR code/test edit is allowed.
```

Next design task: `MIR-CALL-JOINIR-CALLER-LIFECYCLE-BOUNDARY-D1`.

```text
Decision: classify caller-zero merge reachability and the two lifecycle
  owners before any Callee operand delegation; keep collection/remap policy
  separate from value retention/finalization policy.
Source authority + canonical issuer: JoinIR merge coordinator owns merge
  reachability; each lifecycle owner issues its own retention/fail-fast facts;
  Callee remains the sole target operand projection authority.
Non-authority: local remap_callee, typed func/INVALID, generic used_values,
  caller-zero fixtures, backend/runtime names, and lifecycle cross-inference.
Fail-fast boundary: before Phase 1-2 collection or Phase 4-5 rewrite, every
  non-test edge must be classified; missing caller/lifecycle issuer is
  NoSafeSlice and cannot be filled by a default or partial delegation.
Smallest next slice: finite D1 census of merge caller reachability, both
  lifecycle owners, direct remap/collection edges, and reopen triggers; only
  after D1 acceptance may R4c replace the local Callee match.
Non-claims: Method(None), Closure/Constructor shape, MirCall/CallFlags, R6,
  backend/native, PyVM/reference/Python, and warning cleanup.
```

D1 finite state matrix:

| State | Authority | Terminal | Fallback |
| --- | --- | --- | --- |
| `LiveMergeCaller` | merge coordinator | named production boundary | none |
| `CallerZero` | bounded source census | `NoSafeSlice` | no route invention |
| `VerifyLifecycle` | `value_lifecycle.rs` | separate retention contract | no stale-fact reuse |
| `StaleFactsLifecycle` | `value_lifecycle_definition.rs` | separate prepare/fail-fast contract | no verify reuse |
| `SharedBoundaryUnresolved` | D1 design owner | `NoSafeSlice` | no partial helper replacement |
| `TestOrDisconnected` | test/parked owner | `ParkedSealed` | excluded from production claim |

D1 result: the finite census is accepted. The JoinIR merge graph is
`CallerZero`, the two lifecycle owners remain separate policies, and the
Callee projection is reusable only as an occurrence helper; no production
caller or shared lifecycle issuer exists for an R4c switch. R4c is therefore
closed as `NoSafeSlice` rather than implemented. The next selected design stop
is `MIR-CALL-JSON-BACKEND-SHAPE-D1-CAPABILITY-AUTHORITY`.

Backend-shape D0 result (`NoSafeSlice`): the bounded audit found both typed
pre-wire mutation and post-wire JSON mutation, a v0/v1 profile mismatch, and
silent field/default handling. The selected terminal capability for raw
`externcall` was not proven, so deleting or moving either normalizer would
have changed authority without a safe replacement.

Feedback reconciliation at the current source boundary:

```text
used_values()       = closed by 8eca2dd048; it delegates every Callee operand
                      (Value, Method receiver, Closure captures/me) then args
escape projection   = closed by 4e71066e57; it reuses the same occurrence
                      projection while retaining its own barrier policy
JoinIR remap        = local Callee match remains; caller/lifecycle D1 census
                      closed as NoSafeSlice, so no production R4c switch exists
R6 design debt      = MirCall/CallFlags, Method(None), Closure construction,
                      and Constructor/NewBox remain one future schema gate
PyVM/reference/etc. = retired/diagnostic-only boundary; ParkedSealed and not
                      evidence for this selected production terminal
```

Backend D1 is accepted as a strict adapter boundary. `backend_shape` is not a
second semantic issuer and is not atomically retired in this row:

```text
Decision: retain one strict compatibility projection for the Program JSON-v0
  bridge, and fence the selected ny-llvmc terminal to structured canonical
  calls; raw externcall capability is not widened by this decision.
Source authority + canonical issuer: typed Callee plus root-owned
  JsonEgressProfile; `MirInstruction::call` owns the typed target, while the
  owner-local adapter only projects an explicitly validated compatibility
  payload once.
Non-authority: raw wire strings, func, backend symbol lookup, post-wire target
  inference, defaulted dst/args, dropped extra fields, retry/fallback, and
  PyVM/reference/Python/WASM behavior.
Fail-fast boundary: profile, structured callee, extern route, and arity are
  fixed before publish; malformed or profile-mismatched input rejects without
  retry or alternate target search.
Smallest next slice: `MIR-CALL-JSON-BACKEND-SHAPE-STRICT-ADAPTER-I0-R0` —
  remove typed pre-wire console mutation, require the complete v0 adapter
  shape, move the handoff projection inside the Phase0 profile fence, and add
  positive/negative/shared-guard evidence for the selected structured terminal.
Non-claims: JoinIR R4c, Method(None), Closure/Constructor, MirCall/CallFlags,
  R6 field deletion, generic raw externcall support, C backend expansion,
  native/PyVM/reference/Python/WASM, and warning cleanup.
```

Backend D1 finite state matrix:

| State | Authority | Terminal | Fallback |
| --- | --- | --- | --- |
| `TypedExternConsole` | typed Callee/profile owner | one strict bridge projection or native typed terminal | none |
| `TypedExternOther` | typed Callee | preserve typed target | no console mapping |
| `PostWireRecognized` | selected bridge owner | strict typed projection | no second mutation |
| `PostWireUnknown` | selected terminal capability | reject or preserve by explicit profile | no lookup |
| `MalformedWire` | JSON/profile validator | typed reject before publish | no default/retry |
| `RawExternCapabilityUnproven` | D1 authority owner | `NoSafeSlice` | no backend expansion |
| `DirectMirFirst` | direct route owner | unchanged, out of bridge census | excluded |

D1 result: the finite backend census is accepted. Canonical structured
`mir_call`/typed `Callee` is the selected terminal input; compatibility
`externcall` remains owner-local and is never treated as generic selected
terminal capability. The adapter must validate `op`, `func`, `dst`, and `args`
before the one console projection, preserve unknown compatibility routes
without target inference, and reject malformed/defaulted shapes.

Strict-adapter I0 is closed by `40559cceef`. Typed pre-wire backend mutation
was removed; the owner-local adapter now validates the complete v0 shape,
the handoff projection runs inside the Phase0 profile fence, and six focused
`backend_shape` tests plus runtime/source parity and the shared corridor guard
are green. The selected C terminal retains structured-callee routing, rejects
missing structured callee, and has no direct raw `externcall` arm or
fallback/retry.

The originally selected design stop was
`MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-D0-CAPABILITY-AUDIT`; its result and the
next D1 co-seal task are recorded below:

```text
Decision: audit and choose the selected ny-llvmc structured terminal's exact
  capability boundary before any native switch.
Source authority + canonical issuer: typed Callee plus root-owned
  JsonEgressProfile; the selected ny-llvmc C route table is terminal
  capability authority.
Non-authority: raw externcall wire, PyVM/reference/Python/WASM, generic
  backend guesses, symbol lookup, fallback, and retry.
Fail-fast boundary: route/profile/arity/symbol/link evidence is complete
  before publish; unknown, wrong-arity, or missing structured callee rejects.
Smallest next slice: finite native D0 census of the selected terminal route
  matrix with positive/negative/parity evidence and reopen triggers; no code.
Non-claims: JoinIR R4c, R6 field cutover, MirCall/CallFlags, Method(None),
  Closure/Constructor, warning cleanup, and non-selected backends.
```

I0 acceptance matrix:

| Input | Authority | Terminal | Outcome |
| --- | --- | --- | --- |
| typed `Callee::Extern` console route | typed Callee + profile | structured `mir_call` projection | one canonical Global projection, parity green |
| valid compatibility `externcall` with exact `op/func/dst/args` | owner-local v0 adapter | compatibility projection only | known console maps once; unknown route is preserved |
| missing/non-array `args`, missing `dst`, non-string `func`, or extra field | profile validator | before publish | typed reject, no default/drop/retry |
| raw `externcall` at selected ny-llvmc terminal | selected terminal capability | structured-call preflight | reject as profile mismatch |

Native D0 result: `MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-D0-CAPABILITY-AUDIT`
is `NoSafeSlice` for a native switch. The finite census boundary is:

```text
typed Callee + JsonEgressProfile
  -> structured call/mir_call JSON
  -> selected ny-llvmc Boundary/pure-first C dispatcher
  -> LLVM object
  -> hako_llvmc_link_obj_v2 + explicit libnyash_kernel.a
  -> executable
```

Raw `externcall`, CompatibilityV0, PyVM/reference/Python/WASM, and
`native_driver` are outside this terminal and remain `ParkedSealed`. The
selected dispatcher has structured `call`/`mir_call` arms and rejects a
missing structured callee; transport allowlists containing `externcall` are
not evidence of a C terminal capability.

Native D0 finite matrix:

| Input family | Current evidence | D0 terminal | Missing proof / fallback |
| --- | --- | --- | --- |
| exact structured Global/Extern plan | route table and several kernel exports | candidate native route | exact plan/arity/result/symbol/link co-seal required |
| `Global("print")` | direct string/scalar C surfaces | candidate native route | one owner for the arity and result contract |
| exact Extern shell route | route id/core op/arity/dst/value checks | candidate native route or explicit trap | full object→archive→executable census |
| generic Method family | helper symbols exist | `NoSafeSlice` | arity is split across family policies/registry |
| Constructor Map/Array | birth symbols exist | `NoSafeSlice` | producer/plan arity authority is not fixed |
| seven planned String routes | kernel/C surfaces exist | `NoSafeSlice` | C plan table has no rules; fallback is forbidden |
| twelve analysis routes | explicit unsupported capability | preflight reject | preserve typed reject boundary |
| missing/unknown/wrong-arity/malformed | structured preflight | typed reject | no lookup/retry |
| raw `externcall` at selected terminal | no direct C arm | profile-mismatch reject | no post-wire mutation |

The projection census currently reports Rust route specs `47`, generated C
capability rows `35`, C plan emit rules `28`, and Rust test expectations `29`.
The seven missing String plan rules are projection drift, not permission to
add a C fallback. Source/kernel exports alone do not prove that a selected
executable resolves every route: the required evidence is

```text
object undefined symbol
  -> libnyash_kernel.a defined symbol
  -> final executable defined symbol
```

for every accepted native row. Existing explicit archive linking through
`hako_llvmc_link_obj_v2` is retained, but its current smoke evidence does not
cover the full generic Method/Extern/Constructor/String set. Nearby C owners
are also at the physical boundary (`760`, `778`, `793` lines), and the generic
method owner is already `1104` lines; any future implementation requires a
path-preserving split before semantic changes.

Native D1 result: `MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-D1-CAPABILITY-COSEAL`
is accepted as the capability boundary, but it does not authorize a native
switch or make unproved routes positive. The authority split is fixed:

```text
root JsonEgressProfile
  = selects CanonicalV1 versus owner-private CompatibilityV0 only
typed Callee + GlobalCallRoute / constructor_call_routes /
generic_method_routes / ExternCallRouteSpec
  = issues route, arity, result, symbol and proof facts
selected C registry and emit table
  = validates and emits the exact issued plan; it never reclassifies or looks up
hako_llvmc_link_obj_v2 + explicit libnyash_kernel.a
  = owns the physical link edge
```

Every selected row must be `NativePositive`, `ExplicitTrap`, or
`PreflightReject`. Unknown or missing Global/Constructor/Method plans are
`PreflightReject`; only an existing plan-declared trap (currently the
hostbridge route) is `ExplicitTrap`. The seven String rows, generic Method,
Constructor, Global print arity/result ownership, and route-wide link proof
remain unaccepted until their facts are co-sealed. Rust route specs currently
enumerate 47 rows, while generated C has 35 capability rows, 28 emit rules,
and the old test observer expects 29; the generator gap and old count are
drift evidence, not acceptance.

The next execution row is the behavior-invariant
`MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-C-OWNER-SPLIT-I0-R0`:

```text
Decision: split the selected 778-line C extern shell owner path-preservingly
  into rule/validation and emission children before semantic capability work.
Source authority + canonical issuer: no authority changes; the existing Rust
  route plans and C table remain byte-for-byte behavior owners.
Non-authority: new route rows, C name lookup, arity inference, raw externcall,
  fallback/retry, and link policy changes.
Fail-fast boundary: include topology must compile with identical symbols and
  route table behavior; no semantic call output may change.
Smallest next slice: move the existing rule table/validator and emitter body
  into children, keep the parent include path, add line-cap/compile evidence.
Non-claims: native switch, route projection co-seal, seven String routes,
  Global/Method/Constructor semantics, link proof, R6, and warning cleanup.
```

The C owner split is closed by `03b06622eb`. The parent facade still includes
the same two children in the same order, the rule/validation and emitter
contents were moved without semantic edits, `build_hako_llvmc_ffi.sh` and
`cargo check -p nyash-llvm-compiler --bin ny-llvmc` are green, the existing
`env.now_ms/0` JSON route test is green, and the shared corridor guard now
checks both child owners. The facade is 2 lines; the children are 465 and 312
lines. A temporary Hako source probe was not acceptance evidence: the current
MIR compiler stopped at `PhysicalHeader(CompletionNotValue { batch_slot: 1 })`,
so no AST workaround or native link claim is made.

The next design stop is
`MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-EXTERN-NOW-MS-LINK-PROOF-D0`:

```text
Decision: design one production-generated CanonicalV1 JSON to
  object→explicit archive→executable proof for exact `extern.env.now_ms/0`;
  never hand-author a semantic fixture or bypass the route issuer.
Source authority + canonical issuer: ExternCallRouteSpec and its existing
  route metadata/lowering-plan projection; JsonEgressProfile selects
  CanonicalV1, and hako_llvmc_link_obj_v2 owns the explicit archive edge.
Non-authority: Hako probes, hand-authored JSON, the hand-built route test,
  raw externcall, C name lookup, kernel export alone, fallback/retry, parked.
Fail-fast boundary: exact route/arity=0/result_value/symbol/return shape
  must be co-sealed before C emission; malformed plan or missing archive
  rejects before object/executable publication.
Smallest next slice: D0-A capture production JSON; D0-B derive malformed
  plan inputs (row deletion and arity=1); D0-C census undefined object,
  one archive definition, and final executable definition; D0-D record
  explicit nonzero rejection with no fallback or partial publication.
NoSafeSlice until production JSON is captured, the generic publication seam
  is selected without a new semantic receipt, C validates source_symbol too,
  and missing-plan/tuple failures are route-specific rejects.
Non-claims: native switch, seven String routes, Global/Method/Constructor
  expansion, R4c/R6, warning cleanup, Hako repair, or parked backends.
```

The read-only D0 audit keeps this row `NoSafeSlice` for five finite reasons:
(1) the existing route test builds a hand-made MIR and no production-generated
CanonicalV1 JSON artifact has been captured; (2) the Dynamic-V2 static
publication receipt is descriptor-specific and cannot be reused as EnvNowMs
semantic evidence; (3) the Hako probe stops at `PhysicalHeader` and is not a
repair target; (4) the route metadata has no typed capability receipt and C
does not yet compare `source_symbol`; and (5) missing-plan handling is
nonzero only through a generic unsupported path, not a route-specific
fail-fast rejection. These are design blockers, not negative evidence against
the route. The first accepted I0 may reuse the existing object/archive/exe
publication shape, but it must not mint a new semantic receipt or broaden the
route family.

Accepted prerequisite I0: `MIR-CALL-LLVM-TRAP-DECLARATION-COSEAL-I0-R0`:

```text
Decision: give the selected C module one physical llvm.trap declaration owner
  and emit it at most once per generated LLVM module; fence C1 declarations to
  the existing selected-Dynamic profile.
Source authority + canonical issuer: the C pure-compile module prepass owns
  the declaration-once bit; `program.selected_dynamic` is the only C1 profile
  selector, while MirCallNeedFlags/typed plans only request capabilities.
Non-authority: EnvNowMs route or symbol metadata, fn_metadata presence alone,
  individual trap call sites, opt/mem2reg, kernel exports, linker behavior,
  raw externcall, and parked backends.
Fail-fast boundary: declaration census must be one (or zero when no trap call
  is emitted) before `opt -passes=mem2reg`; duplicate declarations reject and
  no object is published.
Smallest next slice: keep C1 declarations behind `program.selected_dynamic`,
  route all prescan/hostbridge/C1 trap declarations through one once-helper,
  preserve existing trap call sites and all non-trap declarations.
Non-claims: EnvNowMs link proof, route/ABI/archive changes, Dynamic activation,
  static receipts, R6, warning cleanup, or any non-selected backend.
```

The bounded owner census found two active declarations in the ordinary
EnvNowMs path: boxed-sum/prescan declarations and unconditional C1 declarations
from non-null function metadata. A third hostbridge declaration is conditional
and must use the same once-helper. The prerequisite is now closed as a
behavior-preserving physical refactor;
Before the next I0, exact Extern D0 remained unaccepted until the
route-specific acceptance evidence was co-sealed.

I0 evidence: `build_hako_llvmc_ffi.sh`, the selected C1 physicalizer smoke, the
quick `ny-llvmc` check, the shared Call corridor guard, and the pointer guard are
green. The same production-generated CanonicalV1 JSON now emits an object with
an undefined `nyash.env.now_ms`, the explicit `libnyash_kernel.a` contains one
`llvm-nm` definition, and the Boundary executable contains one final definition.
The dumped optimized module has zero trap declarations when no trap call is
needed; the helper therefore enforces the allowed zero-or-one census. Derived
missing-plan and arity=1 inputs reject nonzero with no object, and a missing
explicit archive rejects before executable publication. The exact D0 route
specificity/source-symbol acceptance decision remained separate at that point.

Accepted bounded next slice: `MIR-CALL-EXTERN-PLAN-COSEAL-I0-R0`:

```text
Decision: make the selected C generic/same-module prepasses a fail-fast
  projection of an already-issued structured Extern plan; they must not
  classify or recover a missing route by name.
Source authority + canonical issuer: Rust ExternCallRouteSpec/classifier
  issues Callee::Extern, ExternCallRoute, and the CanonicalV1 lowering-plan
  row; C reads that row and compares the original structured callee name with
  source_symbol, then checks the existing physical emit-rule tuple.
Non-authority: C alias/name lookup, hand-written EnvNowMs acceptance, runtime
  symbol lookup, key_value/INVALID inference, raw externcall, compatibility
  replay, and parked backends.
Fail-fast boundary: the generic entry prescan and
  same_module_function_prepass_call_instruction, before body emission, LLVM
  optimization, llc, object publication, or archive/executable linking.
Smallest next slice: require source_symbol in the C view; for structured
  Extern, reject missing/malformed plans with the original callee name, and
  reject source-symbol, route-tuple, arity/args, or dst/result mismatches using
  the existing unsupported-shape diagnostic; keep valid EnvNowMs emission
  unchanged.
Non-claims: EnvNowMs-specific missing-plan classification, alias projection,
  other Extern routes, link-proof expansion, native switch, R6, or warnings.
```

Finite D0-D input matrix (boundary: structured MIR `Call` Extern payload at
the generic entry prescan or `same_module_function_prepass_call_instruction`
-> prepass failure or the existing emit rule; excludes raw `externcall`,
CompatibilityV0, and parked backends):

| Input state | Authority evidence | Required terminal |
|---|---|---|
| plan present, `callee.name == source_symbol`, tuple/arity/args/dst/result exact | Rust route row + C projection check | existing EnvNowMs call emission |
| plan present, source symbol differs/missing | Rust row is not the callsite identity | typed nonzero prepass reject |
| plan present, route tuple/proof/shape differs | C projection no longer matches issued route | typed nonzero prepass reject |
| plan present, arity or args count differs | plan arity and MIR args disagree | typed nonzero prepass reject |
| plan present, dst/result differs or is absent for scalar result | plan result relation disagrees | typed nonzero prepass reject |
| structured Extern plan missing or non-Extern plan at site | no route product to consume | generic `extern_call_missing_plan` reject with callee name; no C reclassification |
| raw `externcall`, CompatibilityV0, unknown non-selected route | outside selected boundary | existing reject/parked terminal; no new native path |

The missing-plan row is intentionally callsite-scoped rather than
EnvNowMs-specific: classifying it as EnvNowMs in C would require an alias
projection or a new C authority. Alias projection is a separate future slice,
not part of this I0.

Exact Extern D0 acceptance — `MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-EXTERN-NOW-MS-LINK-PROOF-D0`:

```text
Decision: accept the selected CanonicalV1 EnvNowMs route only when the Rust
  issued ExternCallRoute/lowering-plan row and the original MIR callee name
  agree at the selected C prepass; no C alias classification or fallback is
  part of the accepted route.
Boundary: production MIR JSON capture -> generic entry/same-module prepass
  -> object -> one kernel archive definition -> executable definition; derived
  malformed rows terminate at prepass before object publication.
Positive: production env.now_ms/0 reaches an object with U nyash.env.now_ms,
  the selected archive has one T nyash.env.now_ms definition, and the linked
  executable has one T definition and runs with a timestamp result.
Negative: derived missing-plan, arity=1, source_symbol mismatch, and
  result_value mismatch each return nonzero with no object; diagnostics retain
  callee_symbol=env.now_ms/0 and use extern_call_* reasons.
Guard: C shim rebuild, quick ny-llvmc check, selected Dynamic smoke, pointer
  guard, canonical corridor guard, source/check line caps, and diff check are
  green; the executable's timestamp-derived process status is not a parity
  claim.
Non-claims: native switch, other Extern route expansion, R6 field deletion,
  MirCall/CallFlags cleanup, Method(None)/Closure/Constructor redesign,
  JoinIR lifecycle, warnings, or parked backends.
```

The exact D0 finite boundary is now `Exhausted` with no open blocker. The next
design stop is the R6 core-schema decision; it must first resolve the
MirCall/CallFlags split, required Method receiver, Closure construction
boundary, and Constructor/NewBox boundary before any field deletion.

Feedback reconciliation and deferred task queue (not selected):

```text
R4b status: HEAD already delegates Call used_values to Callee::for_each_value_operand; Value, Method receiver, Closure captures/me, args order, duplicates, and legacy None parity are covered by the shared tests. Escape, value-consumer, ownership, and query now reuse the occurrence projection where their separate policies allow it.
R4c task (not selected): `MIR-CALL-JOINIR-CALLER-LIFECYCLE-BOUNDARY-D1` must first name a live merge caller, classify both lifecycle owners, and prove that `Callee::rewrite_value_operands` can replace the local remap without changing retention policy. Until then JoinIrIdRemapper's duplicate match is parked; no code or production switch.
R6 D0/D1: the four-boundary schema decision is accepted as a design stop;
`MIR-CALL-R6-CORE-SCHEMA-D1` now owns the finite reader/writer cut/compat/park
matrix. No field deletion or type cleanup is selected until D1 names every
issuer and acceptance edge.
Backend exact-Extern D0 is the selected row; its D0-A/B/C/D tasks and five NoSafeSlice conditions are recorded above. Backend-shape strict-adapter I0 and native D1/C owner split are closed.
Post-R7 cleanup: normal-root mode/projection sum, MainObserved naming, identity-based syntax loan, and builder.rs production/compatibility/test barrel census remain separate cleanup rows; PyVM/reference/Python/native_driver remain ParkedSealed.
```

R5 row rules: each task owns one old edge and reuses the shared corridor
guard; no new per-row shell guard, no fixture-only acceptance, and no R6 field
editing. The census includes direct callers, dynamic construction, wire
writers, and selected backend preflight; literal `callee: None = 0` is not
sufficient evidence.

R6 core-schema design-stop audit (D0 accepted; D1 selected):

The worker premise audit closed the four authority boundaries without opening a
production switch. The final physical core remains:

```text
Call { dst: Option<ValueId>, callee: Callee, args: Vec<ValueId>, effects }
```

Six-line brief:

```text
Decision: keep the mandatory-Callee physical Call; MirCall/CallFlags are not
  semantic authority unless a non-default flag consumer is found.
Source authority + canonical issuer: route resolver/CallTarget issues Callee
  once; MirInstruction::call is the sole physical issuer; NewBox/NewClosure
  owners issue construction, not calls.
Non-authority: func/INVALID, Option/Callee defaults, Method(None), target Const,
  flags:{}, wire text, printer output, optimizer/backend lookup or retry.
Fail-fast boundary: before block.add_instruction and before wire/object publish;
  missing receiver, malformed construction, unresolved legacy, and profile
  mismatch reject with no alternate-target retry.
Smallest next slice: MIR-CALL-R6-CORE-SCHEMA-D1, a finite reader/writer census
  and cut/compat/park matrix; no code, fixture, or field deletion.
Non-claims: R6 implementation, JoinIR remap, JSON wire change, backend switch,
  PyVM/reference/Python/native_driver, and warning cleanup.
```

Finite D0 boundary: selected Rust `CallTarget`/resolver issuance ->
`MirInstruction::Call` -> selected JSON-v1/v0 bridge -> Rust interpreter and
selected backend terminals. A fresh HEAD census finds 20 direct
`MirInstruction::Call` literals plus the one canonical helper definition
(`MirInstruction::call`): **21 writer definitions**. Helper callers are tracked
by owner family below and are not double-counted as literals. Within this
boundary, possible `callee: None` publication is **0**: the three historical
Program JSON-v0 literals were removed before HEAD and MIR JSON-v0 resolves or
rejects before publication. The old 49/4 numbers are historical ledger values,
not current completion evidence. The boundary excludes
PyVM/reference/Python/native_driver, C native D0, the JoinIR lifecycle boundary,
and the complete Constructor ABI; a literal `callee: None` search alone still
does not classify typed stale `func` or `Method(None)` edges.

Current direct-writer ledger (no duplicate helper-caller counting):

```text
canonical helper: src/mir/instruction/methods.rs:22
builder literals: builder_emit.rs:140; calls/build.rs:357;
  calls/materializer.rs:57,95; calls/unified_emitter/compat_entrypoints.rs:22,42;
  calls/unified_emitter/physical_terminal.rs:94;
  exprs_call.rs:33; normal_module_transaction/physical_thunk.rs:79;
  ssa/phi_input_materializer/edge_rematerialization.rs:240;
  utils/boxcall_emit.rs:231
shared/direct literals: ssot/method_call.rs:71; ssot/extern_call.rs:26;
  canonical_direct_call.rs:78
compat ingress literals: runner/mir_json_v0/module.rs:453;
  runner/json_v1_bridge/parse/mir_call.rs:90,159,265,290,315
helper ingress families: runtime_method_call; extern wrapper; JSON-v0
  lowering expression/statement/module helpers; each resolves an exact Callee
  before the helper publishes a Call and is counted only once at its writer.
```

Helper-family owner anchors are finite and explicit: `collection_literals.rs`,
`exprs_qmark.rs`, `rewrite/special.rs`, `calls/emit.rs`, `decls.rs`,
`control_flow/plan/lowerer/effect_emission.rs`, `new_expression.rs`,
`function_call_preflight_route.rs`, `print_stmt.rs`,
`debug_method_routing.rs`, `task_scope_stmt.rs`, `method_call_terminal.rs`,
`json_v0_bridge/lowering/expr/call_ops.rs`, `expr.rs`, `stmts.rs`,
`expr/block_expr.rs`, and `mir_json_v0/module.rs`. Any new helper caller is a
cutover blocker rather than an implicit extension of this ledger.

D1 consumer/terminal ledger (the 21 writers above are not re-counted here):

| owner | observed edge | disposition |
|---|---|---|
| `unified_emitter/physical_terminal.rs:75-103` | rebuilds `func: INVALID` + `Option<Callee>` and drops flags | cut; thin `MirInstruction::call` becomes the sole issuer |
| `instruction/methods.rs:15-28,209-225` | typed operand projection plus legacy `None -> func` | retain typed projection; cut legacy branch |
| `passes/callsite_canonicalize/pass.rs:94-143` | Closure -> `NewClosure`; `None+func` -> `Global` | retain construction boundary; cut target inference |
| `passes/simplify_cfg/flow.rs:486-498`, `ownership_ssa/verify.rs:112-147` | legacy `func` target/operand readers | cut after typed projection owner is named |
| `passes/cse.rs:102-116`, `optimizer/core.rs:243-255` | `call_legacy_{func}` key reconstruction | cut on selected path; diagnostic-only readers park |
| `query.rs:97`, `value_consumer.rs:104`, `escape_barrier.rs:63-80` | Callee occurrence projection | retain; policy remains owner-local |
| `mir_json_emit/root.rs:26-84,112-117` | schema/profile selector and contradiction rejection | retain as profile authority, never target authority |
| `mir_json_emit/emitters/mod.rs:313-340` | forwards `func` + `Option<Callee>` | V0 facade compat only; canonical path cut |
| `mir_json_emit/emitters/calls.rs:9-203` | V1 typed projection; V0 legacy wire and `receiver.unwrap_or(func)` | V1 reject/retain; V0 compat; receiver fallback reopen |
| `mir_json_emit/helpers.rs:25-102` | typed serialization, nullable receiver, unconditional `flags:{}` | typed projection retain; Method(None)/flags reopen |
| `mir_json_v0/call.rs:8-130,248-267`, `catalog.rs:6-127` | explicit/name/func one-shot resolve or typed reject | owner-private compat; no missing Call publication |
| `json_v1_bridge/parse/mir_call.rs:15-327` | typed ingress, dummy `func=0`, Constructor/NewBox, Closure split | ingress retain; dummy cut; construction reopen |
| `mir_json_v0/module.rs:420-475`, `common_util/core_bridge.rs:148-194` | compatibility Method and post-wire Const reclassification | compat/park; no selected semantic authority |
| `backend/mir_interpreter/handlers/calls/{mod,method}.rs` | missing-callee preflight; Method(None) registry/by-name recovery | preflight retain; recovery reopen/cut |
| `contracts/backend_core_ops/allowlists.rs`, `runner/product/llvm/mod.rs` | None/Closure preflight before selected native object | retain as negative guard; shrink after core cut |
| `lang/c-abi/shims/*mir_call*`, `constructor_call_route_plan.rs` | C by-name constructor/method classification; dual Constructor route | park/reopen; no new downstream authority |
| `joinir_id_remapper.rs`, `edge_rematerialization.rs` | `func`/`Option` transport and rewrite | Parked NoSafeSlice until caller/lifecycle owner exists |

This ledger is the D1 cut/compat/park inventory. A path that cannot be placed
in one row is a `CutoverBlockerOpen`, not a reason to widen the boundary or
restore a default target.

D1 is not yet accepted. The open blockers are typed stale-`func` observation
(`user_box_method_publication.rs` included), real `Method(None)` producers and
receiver recovery, public `MirCall`/`CallFlags` with lossy non-default flags,
Closure V0/runtime parity, and the dual Constructor/NewBox physical route.
JoinIR/SSA rematerialization stays `ParkedSealed` only because its caller and
lifecycle boundary is outside this selected R6 census; its observable reopen
trigger is a live merge caller plus named lifecycle owner. It is not evidence
for or against the selected core cutover.

Four-boundary design consultation (D1 policy; implementation still stopped):

| boundary | selected policy / canonical issuer | compatibility and fail-fast |
|---|---|---|
| `MirCall` / `CallFlags` | `CallTarget`/`CalleeResolver` issues target; `MirInstruction::call` issues physical Call. Keep public `MirCall`/`CallFlags` as owner-local transport until API consumers are proven absent. | default flags are discarded decoration; non-default or conflicting flags reject before `physical_terminal`; flags never classify Constructor/Closure |
| `Method(None)` | verified static callable catalog (identity + exact arity) issues qualified `Callee::Global`; instance calls issue `Method { receiver: ValueId }`. Existing `current_static_box` text and formatter are not authority. | JSON-v0 optional receiver resolves once or rejects; methodize and VM receiver/args[0] recovery are cutover blockers; no `StaticMethod` variant |
| Closure / Value | descriptor (`params/captures/me`, `dst`, empty construction args) issues `NewClosure`; an existing closure value issues `Callee::Value(ValueId)`. | Closure wire is compatibility projection only; mixed descriptor/value, missing dst, runtime args, or `Call(Callee::Closure)` at selected terminal rejects before publication |
| Constructor / NewBox | constructor syntax/resolver/JSON verifies `box_type`, `dst`, arity and args, then the existing `NewBox` owner publishes construction. `Callee::Constructor` is pre-core/compat input only. | V0 Constructor-shaped Call resolves once to `NewBox` or rejects; backend/C name scans and `Call(Constructor)` execution are forbidden |

The matrix is a policy Decision, not implementation evidence. Three physical
issuer families are still open: Method(None) has real builder/V0 producers,
Closure V0 parser/runtime parity is incomplete, and Constructor has V0 Call and
V1/NewBox routes. Public `MirCall`/`CallFlags` is now a parked transport policy,
with external use as its explicit reopen trigger. Therefore R6a remains a
`NoSafeSlice` until the following bounded rows close their own old edge and
acceptance:

```text
D1-A  CallFlags public/selected-consumer census; non-default conflict reject (design closed)
D1-B  verified static catalog -> qualified Global; Method(None) issuer count 0
D1-C  Closure descriptor/value wire discriminator and NewClosure parity
D1-D  Constructor input -> NewBox sole selected physical owner
```

Each row must keep the same fail-fast boundary (before block/wire/object
publication), add no new semantic receipt, and preserve the selected
non-closure/non-constructor call parity. A row is not accepted merely because
its local test passes: its production issuer count, old-edge deletion, and
positive/negative/parity evidence must be observable in the shared corridor
guard. If a public flag consumer, third Constructor owner, ambiguous Closure
wire, or missing static catalog is found, the row returns to `NoSafeSlice`.

### D1-A result — `MirCall`/`CallFlags` policy accepted, implementation deferred

```text
observed/current:
selected Rust production flag field readers       = 0
create_mir_call production caller                 = 1
physical terminal semantic projection             = dst/callee/args/effects only
public MirCall/CallFlags downstream consumers     = not observable in workspace
JSON flags:{}                                     = compatibility shape retained
JSON v1 ingress flag handling                      = ingress does not read/validate flags; no current reject is observed

acceptance/not-yet-landed:
non-empty/unknown flags                            = typed reject before publication
```

The public `hakorune_mir_defs` and MIR facade exports remain an owner-local
transport surface in this lane; they are not deleted or reinterpreted. Unknown
downstream crates are `ParkedSealed` with the observable reopen trigger “a
selected consumer reads a non-default flag or depends on the public fields”.
`CallFlags::constructor` is decoration only and may not classify a Callee;
`tail_call`, `no_return`, and `can_inline` have no selected semantic consumer.
The selected JSON ingress must, in a future implementation slice, reject
non-empty flag semantics rather than silently drop them; the current ingress
does not yet perform that validation. This closes D1-A as a design decision, but it does not
permit the R6a code slice: Method(None), Closure, and Constructor remain D1-B/C-D
blockers.

### D1-B design consultation — static Global issuer (not yet accepted)

```text
Decision: Method(None) is not a canonical state; static calls use a qualified
  Global(owner.method/arity), while instance calls require Method(receiver).
Source authority + canonical issuer: verified declaration/source target catalog
  -> canonical callable key -> existing static publication terminal
  -> CallTarget::Global -> CalleeResolver -> Callee::Global.
Non-authority: current_static_box/has_method, StaticMethodId/text formatting,
  func/INVALID, wire/profile/methodize, VM registry/by-name/args[0] recovery.
Fail-fast boundary: owner/method/exact arity and declaration brand before block,
  wire, or object publication; unavailable/ambiguous/foreign/overflow rejects.
Smallest next slice: design-only producer/recovery ledger and cut/compat/park
  assignment; no Method(None) code migration until the issuer is closed.
Non-claims: Method(Some) receiver recovery, backend fallback, V0 parity,
  PyVM/reference/Python/native_driver, or R6 field cutover.
```

Finite D1-B producer/recovery ledger:

| path | observed edge | disposition |
|---|---|---|
| `builder/calls/method_resolution.rs:19-55` | `current_static_box + has_method` issues `Method(None)` without verified arity | cut; verified catalog -> Global |
| `builder/calls/build.rs:307-364` | publishes the unresolved Method(None) result | reject before publication; no alternate target |
| `builder/calls/unified_emitter.rs:407-461` | `HAKO_MIR_BUILDER_METHODIZE` rewrites Global to Method(None) | cut; profile/env cannot alter core target |
| `runner/mir_json_v0/call.rs:73-113,248-267` | V0 receiver omission | owner-private compat: resolve verified key once or reject |
| `runner/modes/common_util/core_bridge.rs:101-199` | Const text -> receiverless Method, drops arity | park/cut; post-wire text is not authority |
| `mir_json_emit/emitters/calls.rs:43-56` | `receiver.unwrap_or(func)` turns static into boxcall | cut; no static->instance reclassification |
| `mir_json_emit/helpers.rs:51-64` | V1 nullable receiver serialization | V1 null reject before wire publish |
| `backend/mir_interpreter/handlers/calls/method.rs:516-532` | registry/by-name/singleton recovery | cut/reject; backend is not issuer |
| `backend/mir_interpreter/handlers/calls/mod.rs:80-91`, `global.rs:22-40` | HostBridge/static-name recovery and arity parsing | park/cut; no source authority |
| `builder/calls/call_target.rs:13-22`, `resolver.rs:78-141` | valid instance target with receiver | retain separate `Method(Some)` path |

The existing verified catalog/key authorities are
`builder/callable_declaration_catalog/{catalog,recovery,key}.rs` and
`source_call_target/{model,qualified}.rs`. The static publication family is
`static_result_publication_physical_bridge.rs`, `method_call_terminal.rs`, and
`resolver.rs`; raw `format!("{}.{}/{}")` fallback is not an issuer. D1-B is
`NoSafeSlice` until every Method(None) producer and recovery row is either
removed, owner-private compatibility, or an explicit outside park with a
reopen trigger. Acceptance requires production Method(None) issuer count zero,
static Global key/arity parity, instance receiver parity, and pre-publication
negative coverage for absent/ambiguous/foreign/overflow targets.

Existing selected static-result/direct-static handoff is real and may be
reused without a new receipt:

```text
source MethodCall(site)
  -> same-brand declaration/source-target catalog
  -> static result publication owner
  -> static publication physical bridge / method_call_terminal
  -> CallTarget::Global(owner.method/arity)
  -> CalleeResolver -> Callee::Global -> physical Call
```

The generic `method_resolution.rs` / `unified_emitter.rs` path is not connected
to that handoff: it receives only `name`, `current_static_box`, and runtime
state, then uses `has_method` or `StaticMethodId` formatting. The source-target
module is currently disconnected for this generic route. Do not invent a new
semantic receipt or thread raw text into it; keep the generic route
`NoSafeSlice` until an existing-product handoff is proven. The bounded design
follow-up is `MIR-CALL-R6-CORE-SCHEMA-D1-B-PARK`: guard the selected static
terminal and seal the generic route as an explicit outside/reopen boundary.

### D1-B-PARK result — boundary accepted; D1-B implementation remains closed

```text
Decision: accept the design boundary only. The selected static source row may
  reuse the existing catalog/publication handoff to issue qualified Global;
  the disconnected generic Method(None) route is outside/reopen, not a new
  producer to be repaired by raw text or a new receipt.
Source authority + canonical issuer: same-brand declaration/source-target
  catalog -> existing static publication handoff -> CallTarget::Global ->
  CalleeResolver -> Callee::Global -> existing physical Call terminal.
Non-authority: current_static_box/has_method, StaticMethodId/text formatting,
  func/INVALID, JSON profile, methodize, VM registry/by-name/args[0] recovery.
Fail-fast boundary: source lineage, catalog brand, static namespace,
  owner/method/exact arity, and receipt completion before block/wire/object
  publication; absent/ambiguous/foreign/overflow rejects.
Smallest next slice: D1-C Closure construction/invocation design; no code,
  fixture, fallback, or production switch is authorized by this closure.
Non-claims: Method(None) issuer retirement, Method(Some), JSON-v0 parity,
  Closure/Constructor cutover, JoinIR, R6 field deletion, or parked backends.
```

Selected handoff (existing products only):

```text
Cataloged source context
  -> declaration catalog brand check
  -> StaticBoxMethod owner/method/arity lookup
  -> VerifiedStaticCallResultPublicationHandoffV1 (one consume)
  -> existing static physical bridge
  -> CallTarget::Global(owner.method/arity)
  -> receipt-required terminal
  -> generic MirInstruction::Call
```

The selected structural guard must assert exact owner/method/arity, catalog
brand, `Cataloged` lineage, argument-count parity, one handoff consume, and one
publication/claim completion. Negative rows are missing source context/location,
foreign lineage, absent catalog/target, namespace mismatch, ambiguity,
overflow/mismatch, failed receipt, and any rewrite/BoxCall/legacy alternate.
The selected production `Method(None)` issuer count is intentionally still a
known blocker (at least `method_resolution.rs:45-54` and
`unified_emitter.rs:435-442`); B-PARK does not claim it is zero.

Existing guard reuse is bounded as follows: the canonical Call corridor and
direct-static physical-input guard are reusable structural evidence; the
direct-static target guard currently has stale `special_handlers.rs` expectations,
and the ingress guard has a stale 775-vs-759 line-limit expectation. Those are
baseline guard debt, not negative evidence against the handoff design, and are
not repaired in this design row. D1-B as a whole remains `NoSafeSlice` until
all Method(None) producers/recovery edges are cut, owner-private compatibility,
or explicitly parked with reopen triggers.

### D1-C result — Closure construction/invocation boundary (design only)

```text
Decision: Callee::Closure is a pre-canonical construction descriptor;
  NewClosure is the construction owner and runtime closure invocation is
  Callee::Value(ValueId).
Source authority + canonical issuer: AST Lambda plus the existing ordered
  BindingRef/capture product -> NewClosure; an indirect callee value -> Call(Value).
Non-authority: CallTarget::Closure generic resolver, MirCall::closure,
  CallFlags::constructor, JSON field-presence guesses, runtime strings, and
  parked backends.
Fail-fast boundary: capture/site/dst and empty construction args are fixed
  before NewClosure publication; missing dst or runtime args on Closure Call
  reject before block/wire/backend publication.
Smallest next slice: explicit Closure wire discriminator/body-metadata policy,
  selected CallTarget::Closure caller census, and selected-backend unsupported
  guard. No code or schema switch is authorized here.
Non-claims: NewClosure retirement, closure runtime implementation, JSON schema
  expansion, or PyVM/reference/Python/native activation.
```

Observed finite edge set:

| edge | current observation | disposition |
|---|---|---|
| AST Lambda / v0 legacy lambda | existing capture product issues `NewClosure` | retain source owner |
| indirect callee value | issues `Call(Callee::Value)` with ordered args | canonical invocation |
| `CallTarget::Closure` / `MirCall::closure` | compatibility/pre-canonical arm; no selected canonical caller | park/reopen |
| v1 Closure descriptor | parses to `NewClosure`; Value parses to `Call(Value)` | retain split |
| mixed Closure descriptor + `func` | field-presence discriminator is ambiguous | typed reject until policy exists |
| v0 explicit Closure | direct parser has no accepted arm | typed reject; no implicit reclassification |
| NewClosure egress | current projection omits body/body_id wire relation | parity blocker; do not guess |
| selected Rust/native terminals | Closure Call and NewClosure are unsupported | negative guard, no activation |

NCL-1 keeps MIR closure bodies in module metadata; D1-C still requires the
selected JSON profile to preserve or explicitly reject that relation before a
core cut. D1-C is `NoSafeSlice` until the discriminator/body transport,
selected caller census, and backend policy are co-sealed.

### D1-C2 result — existing closure-body product cannot close wire parity

```text
Decision: keep D1-C as NoSafeSlice. Existing products preserve the closure
  capture descriptor, but no selected wire/profile path preserves the nonempty
  closure body identity or selected execution parity.
Source authority + canonical issuer: AST Lambda -> ordered capture product ->
  PreparedRawLambdaClosureEmissionV1 -> NewClosure; module metadata owns
  ClosureBodyId -> body, and an existing closure value issues Call(Callee::Value).
Non-authority: Callee::Closure, CallTarget::Closure, MirCall::closure,
  CallFlags::constructor, JSON field presence, func, runtime strings, and
  parked Python/VM-hako backends.
Fail-fast boundary: dst, captures, body relation, and empty construction args
  must be coherent before NewClosure/wire/backend publication; missing dst,
  runtime args, mixed Closure+func, or body/body_id mismatch rejects.
Smallest next slice: design-only choice between transporting the existing module
  body identity in the selected profile and typed-rejecting Closure wire, plus
  selected Closure caller census and backend reject guard. Do not add a schema,
  receipt, fixture, or production switch in this row.
Non-claims: NewClosure retirement, closure runtime, JSON schema expansion,
  native/PyVM/reference/Python activation, or R6 field cutover.
```

Finite wire matrix:

| input/route | current observation | disposition |
|---|---|---|
| AST Lambda with nonempty body | body externalized to `module.metadata.closure_bodies`; MIR keeps body id | wire relation currently lost |
| v1 Closure descriptor | reconstructs empty body/body_id=None | descriptor-only parity; blocker |
| v1 Closure + func/captures mix | field presence prefers construction path | ambiguity; typed reject required |
| v1 Value descriptor | issues `Call(Callee::Value)` | canonical invocation shape; selected backend unsupported |
| v0 typed Closure egress | emits `op=call` descriptor without body relation | no complete round-trip |
| selected Rust/LLVM terminals | Closure/Value call and NewClosure unsupported | retain negative guard |

The existing anchors are `raw_lambda_closure_emission.rs:29-60`,
`function/types.rs:392-397`, `mir_json_emit/emitters/calls.rs:189-203`, and
`json_v1_bridge/parse/mir_call.rs:178-240`. D1-C2 remains `NoSafeSlice` until
the body identity policy, Closure caller census, and selected-backend terminal
are co-sealed without relying on non-selected Python/VM-hako behavior.

### D1-C2-I0 design acceptance — lossless JSON egress boundary

```text
Decision: do not publish a lossy Closure construction wire. The selected JSON
  egress accepts only an empty-body NewClosure (body_id=None, inline body empty)
  and typed-rejects an external or inline body relation before wire publication.
  Runtime closure invocation remains Callee::Value; Closure construction is
  not a canonical Call terminal.
Source authority + canonical issuer: NewClosure's existing body_id/body fields
  and module closure-body metadata; the JSON egress is a projection/validator,
  not a body re-issuer.
Non-authority: JSON field presence, Callee::Closure reconstruction, func,
  runtime strings, and non-selected backends.
Fail-fast boundary: emitters/mod.rs -> emitters/calls.rs, before JSON append;
  body_id.is_some() or nonempty inline body returns the stable typed reject
  `mir-json/closure-body-wire-unavailable`.
Smallest next slice: one egress-only implementation and focused positive /
  negative tests; ingress/schema/backend behavior is outside this row.
Non-claims: Closure runtime support, wire schema expansion, NewClosure retire,
  V0/V1 full round-trip, native/PyVM/reference/Python activation, or R6 cutover.
```

Acceptance is intentionally narrow: an empty descriptor remains a positive
projection, a body-backed closure is rejected rather than silently losing its
module metadata relation, and the existing unsupported Closure/Value backend
guards remain unchanged. This is a BoxShape fail-fast cleanup, not a new
accepted closure shape.

D1-C2-I0 is closed at `873765ba0e`: `emitters/mod.rs` now passes the existing
`body_id/body` relation to the single JSON egress owner, which returns the
stable `mir-json/closure-body-wire-unavailable` reject for body-backed closures.
The empty projection and body-backed negative test are green (3/3 filtered
`new_closure` tests); the shared Call corridor, pointer, rustfmt, and diff
guards are green. No ingress/schema/backend or R6 field change was included.

### D1-D result — Constructor/NewBox boundary (design only)

```text
Decision: the selected physical construction owner is NewBox; Constructor is
  pre-core/compatibility input only. A valid constructor must carry exact
  box_type, dst, arity, ordered args, and construction effects before publish.
Source authority + canonical issuer: parser constructor source/catalog plus the
  existing normal admission -> NewBox; V0/V1 adapters resolve once or reject.
Non-authority: Callee::Constructor as a core terminal, MirCall/CallFlags,
  func/Option<Callee>, route metadata, C box_type scans, and backend by-name
  recovery.
Fail-fast boundary: validate before block.add_instruction, JSON publication,
  native/object publication, or Rust NewBox dispatch; no args/dst/type default.
Smallest next slice: keep the unproved raw-New/source handoff in design stop and
  close only a negative V0 compatibility boundary: explicit typed Constructor
  rejects before `MirInstruction::Call` publication; direct `op=newbox` remains
  the existing positive construction route.
Non-claims: native parity, NewBox unification across every backend, or R6 field
  deletion.
```

Observed finite edge set:

| edge | current observation | disposition |
|---|---|---|
| normal/raw `New` and direct collection constructors | existing `NewBox` producers | retain owner; source handoff still needs proof |
| legacy builder Constructor | some paths direct `NewBox`, unified physical terminal still publishes `Call(Constructor)` | cut/reject before Call publication |
| V0 typed Constructor | can reissue `Call(Constructor)` while V1 parses to `NewBox` | compatibility resolve once or typed reject |
| `env.box.new`, IntegerBox shortcut | separate/compatibility semantics | park; not a new selected owner |
| C/native `mir_call`/`newbox` string routes | hardcoded classification and/or dropped args | park/reject; no authority |
| Rust `Call(Constructor)` handler | unsupported negative terminal | retain until issuer count is zero |

D1-D as a whole remains `NoSafeSlice`: the raw-New source/catalog handoff,
V0/V1 parity, exact arity/dst/args validation, and third physical routes are
not co-sealed. The bounded negative compatibility row below is independently
safe because it publishes neither a Call nor a guessed NewBox.

#### D1-D source-handoff audit — `NoSafeSlice` (read-only, 2026-08-25)

The existing `VerifiedInstanceConstructorPhysicalSourceCohortV1` is a real
parser/package product, but it proves declaration rows only: final statement
ordinal, box name, parser constructor key, and opaque `ConstructorSourceIdV1`.
Its demand tickets are consumed by instance-box declaration/runtime lifecycle;
they are not a call-site target catalog. `PreparedRawNewExpressionV1` receives
only raw `class`, arguments, and field initializers, while
`BrandConstructorSourcePortV1` covers `FunctionCall` brand rows and does not
cover `ASTNode::New`. The V0 `JsonV0FunctionCatalog` proves only local
`Const(String)` legacy-function relations; it supplies no constructor
declaration/arity relation for an explicit V0 `Constructor` object.

Therefore the next design task is not to thread raw text or issue a new
receipt. It must first name an existing parser/package product that covers
`New` call sites and exact constructor arity, then classify built-in/compat
construction outside that boundary. Until that product and the V0/V1
resolve-once-or-reject matrix are co-sealed, mapping V0 `Constructor` directly
to `NewBox` would be an unproved target/arity guess and remains forbidden.
Acceptance for the next design row is a finite New-callsite census, exact
`box_type`/constructor key/arity/ordered args/dst/effects authority, and a
typed reject or one-shot `NewBox` issuer; no code or route switch is selected.

#### D1-D0 bounded negative row — V0 typed Constructor reject

```text
Decision: accept only the V0 negative compatibility edge. An explicit typed
  `Constructor` callee is rejected before `MirInstruction::Call` is built or
  added to a block. The existing direct V0 `op=newbox` owner remains positive.
Source authority + issuer: V0 call parser identifies the explicit Constructor;
  no Constructor Call issuer is permitted. `mir_json_v0/module.rs` remains the
  sole selected issuer for direct `NewBox` in this row.
Non-authority: raw `box_type`, `args.len()`, declaration catalog rows without a
  call-site relation, V1 parsing, backend/name lookup, retry, and defaults.
Fail-fast boundary: `JsonV0CallInput::resolve` returns a stable typed error
  before argument/effect construction, block publication, or runtime dispatch.
Positive: direct V0 `newbox` and existing non-Constructor V0 call variants keep
  their current parse/parity behavior.
Negative: explicit V0 `Constructor` (including `call` and `mir_call`) rejects;
  no `Call(Callee::Constructor)` can enter a block.
Non-claims: valid Constructor execution/NewBox parity, raw AST `New`, V1,
  native/backend activation, Method(None), and the R6 field cutover.
```

Acceptance is one stable reject token, one focused negative test, one direct
`newbox` positive test, and the shared corridor guard. This closes only the
publication edge; D1-D source handoff and the full Constructor/NewBox matrix
remain a design stop.

#### D1-D0 implementation acceptance — V0 publication edge closed

`JsonV0CallInput::resolve` now rejects explicit `Callee::Constructor` with
`[freeze:contract][mir-json-v0/constructor-call-requires-newbox]` before
argument/effect construction and before `mir_json_v0/module.rs` adds an
instruction. The focused typed-Constructor negative test, direct `newbox`
positive test, all 28 `mir_json_v0` tests, pointer guard, individual rustfmt,
diff check, and shared Call corridor guard are green. No V1/raw-New/native
route, core field, or positive Constructor issuer changed. The full D1-D
source/arity handoff remains `NoSafeSlice`.

#### D1-D1 bounded negative row — V1 Constructor shape reject

```text
Decision: preserve valid V1 typed Constructor -> NewBox behavior, but reject
  malformed or ambiguous Constructor shape before NewBox publication. Missing
  or non-array args, conflicting name/box_type aliases, and simultaneous flat
  and nested args are not silently defaulted or precedence-resolved.
Source authority + canonical issuer: V1 parser owns wire-shape validation;
  the existing V1 NewBox branch remains the sole positive issuer in this row.
Non-authority: name/box_type precedence, absent-args-as-empty, args.len as
  constructor arity, effects, backend registry, route metadata, and source
  declaration catalogs not connected to the V1 call site.
Fail-fast boundary: after typed Constructor recognition but before argument
  defaulting/effect publication and before `block_ref.add_instruction`.
Positive: valid Constructor with exactly one name/box_type alias and an array
  args field keeps the existing NewBox shape; direct `op=newbox` is unchanged.
Negative: missing/non-array/ambiguous args and conflicting aliases return stable
  typed rejects and publish no NewBox. Existing missing name/dst/item rejects
  remain unchanged.
Non-claims: exact constructor source/arity, V1 Constructor semantic parity,
  raw AST New, V0, native/HakoVM, Method(None), and R6 core-field cutover.
```

The finite V1 state boundary is `mir_json_v1_bridge` Constructor input through
`NewBox` publication: absent/non-V1 input is outside; missing name/box_type,
missing dst, and malformed arg items are existing rejects; valid-looking
Constructor is a retained positive; missing/non-array/dual-placement args and
conflicting aliases are the new negative states. The selected Rust core canary
family and builder Constructor producer are positive evidence, so rejecting all
valid V1 Constructors is explicitly not allowed. Acceptance is one stable
reject token family, focused positive/negative parser tests, the existing direct
NewBox test, and the shared Call corridor guard. Full D1-D remains
`NoSafeSlice` because no source/arity product reaches arbitrary `New` call sites.

#### D1-D1 implementation acceptance — V1 shape boundary closed

`parse_v1_mir_call` now rejects missing, non-array, or dual-placement `args`,
and conflicting `name`/`box_type`, with stable
`[freeze:contract][mir-json-v1/...]` errors before `NewBox` publication.
Valid typed Constructor -> `NewBox` and direct `op=newbox` remain unchanged.
The focused V1 parser suite is 9/9, the V0 compatibility suite is 28/28, and
the shared Call corridor, pointer, individual rustfmt, diff, and line-limit
checks are green in commit `640ac083a7`. No source/arity authority, R6 core
field, native route, or positive Constructor producer was added. Full D1-D
remains `NoSafeSlice`.

#### D1-D2 design Decision — ordinary `New` source relation (implementation stop)

This Decision narrows the remaining Constructor/NewBox blocker to the ordinary
user-defined `New` cohort. It records a source relation only; it does not issue
a new `Verified*`/`Prepared*` product, change a route, or authorize code in
`work_mode = design_stop`.

Six-line brief:

```text
Decision:
  ordinary user-defined `New` is admissible only through a source-bound
  relation to the declared `birth/N` hook; Core13, IntegerBox, builtins,
  records, and JSON compatibility remain separate owners.
Source authority + canonical issuer:
  ParserConstructorSourceCatalogV1 and existing source/semantic products own
  declaration identity; the missing New-callsite resolver must issue one
  source relation, and the existing NewBox construction owner must consume it
  once. No issuer is created in this design turn.
Non-authority:
  AST class text, `arguments.len()`, generated `<Class>.birth/N` strings,
  header lookup, FunctionSignature, MIR EffectMask, JSON name/box_type,
  runtime registry, backend symbol lookup, and post-MIR target inference.
Fail-fast boundary:
  source relation, ordered child/initializer sites, `birth/N`, Allocation
  effect, and destination must be coherent before NewBox/birth Call, Const,
  wire, block, or object publication; no retry or fallback is allowed.
Smallest next slice:
  design the relation and one-shot admission using existing SourceExprSiteV1,
  BodyEffectKindV1::Allocation, and ParserConstructorSourceCatalogV1; no code,
  fixture, receipt, or production switch until the missing issuer is accepted.
Non-claims:
  all-New coverage, Core13/IntegerBox/builtin/record routes, V0/V1 schema
  changes, Method(None), Closure, backend activation, R6 field deletion, and
  warning or physical-shelf cleanup.
```

Finite census boundary:

```text
ASTNode::New parse
  -> raw New preparation
  -> ordinary source relation
  -> existing NewBox construction admission / selected Rust handle_new_box
  -> construction publication
```

The boundary includes ordinary user-defined boxes and their declaration-side
constructor products. It excludes `mir_core13_pure()`/`env.box.new`, the
IntegerBox literal `Const` route, builtin/collection/plugin/MathBox owners,
record construction, direct JSON `newbox`, V0/V1 compatibility input, and all
PyVM/reference/Python/native_driver routes. The existing
`constructor-birth-new-lifecycle-ssot.md` remains the lifecycle authority:
allocate, declaration field initializers, matching `birth(args...)`, then
publish usable identity.

Current evidence and authority split:

| evidence | current product | design classification |
|---|---|---|
| `src/parser/expr/primary.rs:272-278` | `ASTNode::New` carries class, arguments, field initializers, and type arguments | source syntax observation only |
| `src/mir/builder/raw_expression_dispatch/mod.rs:544-557` | raw preparation forwards those same values | transport; no constructor identity |
| `src/mir/builder/new_expression.rs:154-190` | ordinary lowering builds `<Class>.birth/N` from lowered arg count and header lookup | legacy downstream inference; cut candidate |
| `src/parser/constructor_source_catalog.rs:107-205` | declaration rows and opaque `ConstructorSourceIdV1` | declaration authority; no New site relation |
| `src/mir/normal_callable_semantic_package/instance_constructor_semantic.rs:113-175` | source-bound constructor body forest/projection | declaration/body authority; not call-site target authority |
| `src/mir/resolved_semantics/shadow/expr.rs:307-327` | New site `Allocation` plus child traversal | reusable source fact; not physical effect authority |
| `src/mir/resolved_semantics/source_projection.rs:262-275` | `Argument(i)` and `Initializer(i)` projections | reusable ordered source paths |

Design relation (vocabulary only; not issued in this turn) must co-seal:

```text
FunctionOwnerIdV1                 owning source function
SourceExprSiteV1                  exact New expression site
ConstructorSourceIdV1             declaration/source identity
final box ordinal + constructor key declaration coordinates
canonical hook = birth, source arity N
ordered Argument(i) sites and Initializer(i) sites
existing BodyEffectKindV1::Allocation row
constructor origin/provenance
```

It must not carry a physical symbol, Recipe key/selector, ValueId, backend
handle, MIR `EffectMask`, or a runtime string. Source arity `N` is distinct
from the receiver-inclusive physical birth function arity `N + 1`.

The future one-shot physical admission is deliberately narrow: validate the
relation once, lower ordered child expressions and field initializers, and
co-seal the existing NewBox allocation with the matching `birth/N` call inputs
and exact destination before publishing the existing lifecycle sequence. It
must never rediscover a target from class text, argument count, or headers.
`MirInstruction::Call` may only receive an already exact `Callee`; a
`Callee::Constructor` Call is not a construction fallback. The existing
NewBox handler remains the sole selected construction terminal.

Two unresolved boundaries are intentionally left open by this design. First,
the existing products expose the New-site `Allocation` fact, but not one
single construction-wide effect row: initializer-child effects and `birth`
body effects remain their own authorities and must be referenced/co-sealed,
not reissued as MIR `EffectMask`. Second, current raw lowering writes a
`NewBox`, then a by-name `birth` Call, then field initializers
(`new_expression.rs:133-195`), while the lifecycle SSOT requires declaration
field initializers before `birth` and publication. D1-D2 makes no parity claim
for that ordering and does not silently change it; lifecycle-order correction
is a separately named blocker.

The source relation must keep `dst` and lowered `ValueId` arguments out of its
Facts. They are physical admission inputs, attached only after the relation is
borrowed once. Source validation can reject before child lowering; however,
the current `drive_legacy_expression_v1` path may append child instructions
before a later lowered-argument mismatch is known. An atomic no-block-mutation
guarantee therefore needs a named staging/transaction owner and is not claimed
by this design row.

Finite source-relation states:

| state | evidence | outcome |
|---|---|---|
| `OrdinaryReady` | unique source site/source id, user box, unique `birth/N`, ordered args/initializers, Allocation | future admission candidate; currently `NoSafeSlice` because issuer is absent |
| `Missing` | box/declaration/catalog/birth row or source site absent | typed reject before child/object publication |
| `ForeignChanged` | foreign owner/source id, changed declaration, or box/key mismatch | typed reject |
| `DuplicateAmbiguous` | repeated site/source id or multiple constructor rows | typed reject |
| `ArityOrderMismatch` | `args.len() != N`, missing/reordered child path, or initializer mismatch | typed reject before NewBox/birth publication |
| `EffectDestinationMismatch` | missing Allocation fact, construction effect disagreement, or invalid destination | typed reject before publication |
| `BirthProvenanceMismatch` | generated-birth initializer, direct birth, or `init/pack` alias is used as the ordinary hook | reject; only declared canonical `birth/N` is admissible |
| `SiteDrift` | nested/initializer New path, transformed AST site, or constructor declaration no longer matches the catalog row | typed reject before publication |
| `SpecializedCohort` | MathBox/collection/builtin NewBox or `CallTarget::Constructor` producer is observed | outside ordinary issuer census; retain its named owner |
| `Core13Pure` / `IntegerBox` / `Builtin` / `Record` | named specialized route | outside this cohort; retain its existing owner |
| `V0Compatibility` / `V1Compatibility` | typed JSON Constructor or direct `op=newbox` | owner-local compatibility; V0 reject and valid V1/direct NewBox remain unchanged |
| `MalformedAbsent` | malformed source or unresolved relation | typed reject; no default or fallback |

Old edges to retire only after this Decision is accepted and an issuer exists:

```text
ordinary raw-New class/arity inference and header lookup
ordinary `CallTarget::Constructor` / `Callee::Constructor` publication
post-lowering birth-name reconstruction or backend symbol lookup
```

Preserve the specialized Core13/IntegerBox/builtin/record owners, the current
V0 typed Constructor reject, the valid V1/direct `NewBox` compatibility
positive, the selected `handle_new_box` owner, and the unsupported Constructor
terminal until its production issuer count is proven zero.

The production count is cohort-qualified: `ordinary Constructor issuer = 0`
does not mean `all NewBox writers = 1`. Builtin/collection producers, direct
`op=newbox`, V1 compatibility, and MathBox constructor routes remain separate
rows. A future census must also include nested `New`, field-initializer `New`,
foreign/multi-module declarations, duplicate source ids, and constructor
declaration drift without re-entering a by-name resolver.

Future acceptance must show a positive ordinary New with one source relation,
source `N`, ordered arguments/initializers, Allocation, exact `dst`, one
construction publication, and the selected Rust NewBox owner; negative rows
must reject missing/foreign/duplicate/birth/arity/order/effect/destination
states before publication. A reusable guard should count relation/site
uniqueness, ordinary Constructor issuer zero, NewBox owner one, and
fallback/retry zero without counting compatibility or specialized cohorts.

This row remains `NoSafeSlice`: the parser/resolver does not yet issue the
New-callsite source relation, and the full source-effect-to-physical admission
co-seal is not present. The next implementation row may be selected only after
those two authorities are named and accepted; until then the R6 core schema,
`Option<Callee>`, `func`, `Method(None)`, `Callee::Closure`, `MirCall`, and
`CallFlags` remain untouched.

#### D1-D3 design Decision — ordinary `New` dual-producer census

D1-D2's relation shape is necessary but not sufficient. The finite census found
an ordinary-capable `PlanNormalizer` path in addition to raw New lowering. This
Decision keeps the lane at `NoSafeSlice` until both producers borrow one source
relation and converge on one physical NewBox owner.

Six-line brief:

```text
Decision:
  ordinary user-defined New covers both raw lowering and PlanNormalizer;
  neither may issue a target or NewBox without the same source relation.
Source authority + canonical issuer:
  ParserConstructorSourceCatalogV1 owns declaration identity; the semantic
  resolver owns SourceExprSiteV1 and Allocation observation; a missing shared
  New-site issuer must map each site once to catalog-backed birth/N, after
  which one existing NewBox owner performs physical admission.
Non-authority:
  class/arguments.len(), generated birth strings, header lookup,
  CoreEffectPlan::NewBox box_type/dst, FunctionSignature, MIR EffectMask,
  JSON fields, runtime/backend lookup, and either current physical path.
Fail-fast boundary:
  owner/site/catalog/birth/N/ordered paths and effect relation are checked
  before child lowering; lowered args and dst are co-sealed before NewBox,
  birth, block, wire, or object publication. No retry or alternate producer.
Smallest next slice:
  design-only raw+plan producer census, shared relation issuer, and one-owner
  convergence; do not add a receipt, route, fixture, or code in this turn.
Non-claims:
  specialized NewBox writers, lifecycle-order correction, all-New parity,
  JSON/native/PyVM/reference/Python, Method(None), Closure, or R6 cutover.
```

Finite census boundary:

```text
ASTNode::New parse/source projection
  -> raw New and PlanNormalizer ordinary-capable producers
  -> shared source relation
  -> one physical NewBox/birth admission
  -> selected construction publication
```

The census excludes Core13Pure, IntegerBox, builtin/collection/plugin/MathBox,
record, direct JSON `newbox`, V0/V1 compatibility, and non-selected backends.
Its current finite inventory is:

| edge | count | current authority/status |
|---|---:|---|
| parser `ASTNode::New` producer | 1 generic | `src/parser/expr/primary.rs:272-278`; no owner/catalog relation |
| semantic New visitor | 1 recursive branch | `src/mir/resolved_semantics/shadow/expr.rs:307-325`; can record site + `Allocation` at nested depth |
| New child paths | 2 roles + root classifier | `source_path_policy.rs:220,267` and `source_projection.rs:267,271`; `Argument(i)`/`Initializer(i)` |
| raw New ingress | 1 | `raw_expression_dispatch/mod.rs:544-557`; class/args/initializers only |
| raw ordinary path | 1 NewBox + 2 birth alternatives | `new_expression.rs:133-195`; class/arity/header inference, no source relation |
| PlanNormalizer ordinary path | 1 AST branch + 1 effect emission | `control_flow/plan/normalizer/helpers_value.rs:523-552` -> `effect_emission.rs:180-190`; `CoreEffectPlan::NewBox` has no source relation/birth |
| direct NewBox writers | 6 total | 2 ordinary candidates (raw/plan), 4 specialized; do not claim global writer count = 1 |
| catalog issue sites | 2 | `parser/source_seal/finalize.rs:203,368`; declaration rows only |
| constructor semantic batch issuer | 1 | `normal_callable_semantic_package/instance_constructor_semantic.rs:113-210`; declaration/body only |
| New-site -> catalog relation | 0 | no issuer or consumer at HEAD |

`FunctionOwnerIdV1 + SourceExprSiteV1` and `BodyEffectKindV1::Allocation` are
available to semantic inventory, but neither raw `PreparedRawNewExpressionV1`
nor plan `CoreEffectPlan::NewBox` transports them. Source arity is `N`,
`NewBox.args.len()` is `N`, and receiver-inclusive physical birth arity is
`N+1`; these values must not be collapsed.

The shared relation must be source-only: owner/site, borrowed
`ConstructorSourceIdV1`, final-box/source coordinates, declared `birth/N`,
ordered argument/initializer paths, constructor provenance, and a reference to
the existing effect observation. It must not contain `dst`, lowered `ValueId`,
physical symbol, Recipe key, selector, MIR `EffectMask`, or backend handle.
Both raw and PlanNormalizer must consume it exactly once. Their current
physical products are not independent authorities; after validation they must
feed one NewBox admission and one existing lifecycle publication owner.

Additional blockers exposed by the dual census:

```text
PlanNormalizer args-only user-box New can bypass raw source relation and birth.
raw child lowering may mutate the block before a later mismatch is known.
Allocation is not a construction-wide effect authority; child and birth-body
effects require their own existing semantic products.
NewBox -> birth -> field initializer currently disagrees with lifecycle SSOT
field initializer -> birth -> publish; D1-D3 makes no ordering claim.
```

The next design acceptance must prove relation coverage for every ordinary raw
and plan site (including nested and field-initializer New), unique catalog-backed
`birth/N`, foreign/duplicate/site-drift rejection, and one physical owner. A
future implementation still requires a named staging/transaction owner before
strict block-mutation-free fail-fast can be claimed. Until that proof exists,
the ordinary Constructor issuer count and NewBox owner count remain
cohort-qualified, and R6 field deletion is forbidden.

#### D1-D4 design Decision — shared issuer, staging, lifecycle, and effect owners

D1-D3 proves that raw and PlanNormalizer are two ordinary-capable producers,
but no current module can issue the joined New-site relation without becoming a
second authority. D1-D4 is therefore still design-only and closes the three
authorities required before a production slice: transaction/staging, lifecycle
ordering, and construction-wide effect co-seal.

Six-line brief:

```text
Decision:
  issue one borrowed ordinary-New relation at the semantic boundary, then
  route raw and PlanNormalizer through one staged physical NewBox admission.
Source authority + canonical issuer:
  ParserConstructorSourceCatalogV1 supplies declaration identity and the
  resolver supplies FunctionOwnerIdV1/SourceExprSiteV1/Allocation facts;
  a new source-bound adapter is the single future relation issuer, while the
  existing NewBox owner is the only physical issuer. Neither path owns it.
Non-authority:
  raw preparers, PlanNormalizer/CoreEffectPlan, class/arity/header lookup,
  birth strings, MIR EffectMask, dst/ValueId, Recipe keys, JSON, backend, and
  runtime registry.
Fail-fast boundary:
  relation validation precedes child lowering; a staging transaction holds
  lowered children/initializers and destination until effect/lifecycle checks
  pass, then commits NewBox -> birth/field publication exactly once.
Smallest next slice:
  design the adapter loan, staging owner, lifecycle order, and effect co-seal;
  no semantic receipt, route switch, fixture, or code is issued in D1-D4.
Non-claims:
  specialized NewBox writers, all-New parity, JSON/native/PyVM/reference/
  Python, Method(None), Closure, R6 field deletion, and warning cleanup.
```

The adapter must live at the source-bound semantic boundary, borrowing the
existing catalog row and resolver facts once. It may validate and loan
`ConstructorSourceIdV1`, `FunctionOwnerIdV1`, `SourceExprSiteV1`, source
`birth/N`, ordered `Argument(i)`/`Initializer(i)` paths, constructor
provenance, and the existing Allocation/effect products. It must not issue a
Recipe key, physical ID, `dst`, `ValueId`, MIR `EffectMask`, backend handle, or
runtime symbol. The loan is consumed and dropped at physical admission; raw and
plan code cannot reissue or retry it.

The staged admission contract is:

```text
source relation validation
  -> lower ordered arguments and field initializers into a private transaction
  -> resolve the already-declared birth body/effect product (no by-name retry)
  -> validate lifecycle order and construction-wide effects
  -> attach exact dst/ValueIds on the physical side
  -> commit one existing NewBox owner and its matching birth/field sequence
```

No block, NewBox, birth Call, field write, wire, or object publication may be
visible before commit. This is a design requirement because the current raw
`drive_legacy_expression_v1` path and PlanNormalizer effect emission can mutate
physical state while a later mismatch is still possible. A future transaction
owner must be named before this can become a fast implementation slice.

Lifecycle is an explicit authority, not a consequence of MIR instruction order.
The accepted lifecycle is declaration field initializers -> matching `birth/N`
-> publish usable identity. The current raw edge (`NewBox -> birth -> field
initializer`) and any plan edge that omits birth are negative evidence for this
row. D1-D4 may either identify an existing source-bound lifecycle product or
return `NoSafeSlice`; it may not silently reorder instructions or infer parity
from `EffectMask`.

Construction-wide effect co-seal likewise remains separate from the New-site
`BodyEffectKindV1::Allocation` observation. The adapter must name how child
initializer effects and the declared birth-body effects are borrowed and
validated together. If no existing product covers that union, the row stays
`NoSafeSlice` rather than manufacturing a global effect boolean.

After D1-D4 acceptance, the only permitted ordinary consumers are:

```text
raw PreparedRawNewExpressionV1
  -> shared relation loan -> staged admission
PlanNormalizer ordinary New
  -> same shared relation loan -> staged admission
staged admission
  -> existing NewBox physical owner exactly once
```

Retirement candidates then become explicit: raw ordinary direct NewBox and
class/arity/header birth recovery; PlanNormalizer `CoreEffectPlan::NewBox`
without a relation and its second physical writer; ordinary
`CallTarget::Constructor`/`Callee::Constructor`; and any by-name backend retry.
Specialized Array/Map/Math/record/Core13/IntegerBox/direct JSON/V0/V1 owners
remain separately counted and are not evidence for ordinary caller-zero.

Acceptance for the future implementation gate is positive raw/plan parity with
one relation issue, one relation consume, source `N` versus physical `N+1`,
ordered child/initializer sites, lifecycle-correct birth, complete effects,
and one NewBox commit. Negative states include missing/foreign/duplicate/stale
relation, birth provenance or arity/order mismatch, effect/lifecycle mismatch,
relation reuse, plan field initializer without lifecycle support, and any
fallback to class text or `Callee::Constructor`; all must reject without block
mutation. A reusable guard must count raw/plan producer convergence, ordinary
writer zero outside the owner, relation issue/consume exactly once, and
fallback/retry zero while keeping specialized cohorts separate.

Until the adapter, staging owner, lifecycle authority, and effect co-seal are
all named and accepted, D1-D4 remains `NoSafeSlice`; no production code or new
semantic receipt is authorized.

#### D1-D5 design Decision — lifecycle/effect bridge and expression transaction

D1-D4's remaining authorities are now bounded. The parser has declaration
provenance, the resolver has per-owner New/child effects, and the constructor
semantic row has source identity, but no product joins them; the constructor
batch currently discards the body-shape map. D1-D5 keeps implementation stopped
and designs that bridge without pretending the lifecycle contract is already a
typed production product.

Six-line brief:

```text
Decision:
  ordinary New lifecycle/effect meaning is issued once at the semantic
  constructor boundary and borrowed by both raw and PlanNormalizer paths.
Source authority + canonical issuer:
  parser StoredFieldInitializer/ConstructorSourceIdV1 and resolver
  SourceExprSiteV1/BodyEffectShape products are the authorities; the future
  constructor-semantic bridge retains the birth owner/body-shape relation and
  issues one lifecycle/effect handoff. Builders do not issue it.
Non-authority:
  lifecycle prose alone, MIR instruction order, NewBox/CoreEffectPlan,
  EffectMask, generated birth strings, physical arity, backend publication,
  or a global boolean assembled from child effects.
Fail-fast boundary:
  distinguish declaration defaults from explicit `new Box { field: ... }`
  overrides, validate New/birth/child effects, then stage all child lowering,
  field writes, birth, and publish until one atomic commit.
Smallest next slice:
  design-only retention/reborrow of body-shape products plus the expression
  transaction owner; no semantic receipt, route switch, fixture, or code.
Non-claims:
  lifecycle implementation, construction parity, specialized NewBox routes,
  JSON/native/PyVM/reference/Python, Method(None), Closure, R6 cutover, and
  warning/physical-shelf cleanup.
```

Finite authority inventory:

| authority | current product | gap for ordinary New |
|---|---|---|
| stored declaration initializer | `parser/declarations/box_def/members/fields.rs:99-137` `StoredFieldInitializer` | provenance/order exists; no New-site relation |
| constructor provenance | `parser/source_authority/constructor_source.rs:16-25,175-220` | generated-birth trigger validation exists; no publish edge |
| constructor source row | `parser/constructor_source_catalog.rs:107-212` and semantic syntax loan | source id/key/declaration borrow exists; no lifecycle sequence |
| New site/effect | `resolved_semantics/shadow/expr.rs:311-325` and `body_shape.rs:89-104,182-188` | Allocation and per-owner effects exist; no birth/publish join |
| body-shape transport | `resolved_semantics/owner_resolver.rs:63-67,193-225,300-335` -> `compiler/lowering_input.rs:30-92` | product exists, but constructor semantic batch discards it at `instance_constructor_semantic.rs:153-156` |
| constructor semantic row | `normal_callable_semantic_package/instance_constructor_semantic.rs:28-35,113-210` | source/forest/projection exists; body-shape/lifecycle field absent |
| physical raw order | `builder/new_expression.rs:133-195` | NewBox -> birth -> explicit fields; no typed publish transaction |
| physical plan order | `builder/control_flow/plan/normalizer/helpers_value.rs:523-555` -> `effect_emission.rs:180-190` | args-only NewBox; birth/publish absent |
| construction-wide effects | no joining product | absent; MIR `EffectMask` is non-authority |

The bridge must retain or reborrow the declared birth owner's body-shape
product, the New-site Allocation, every ordered argument/initializer child
effect, and the explicit override distinction. Declaration field initializers
are not the same carrier as `new Box { field: expr }` overrides: the former
belong to the birth lifecycle, while the latter are source-site operations. A
missing birth row, `init/pack`-only alias, generated/direct provenance mismatch,
foreign/duplicate source id, site drift, or incomplete child effect coverage is
a typed reject, not a default effect.

The expression transaction is a future source/physical boundary, not a MIR
instruction wrapper. It must hold lowered child values and field writes without
publishing them, validate the borrowed lifecycle/effect handoff, attach
physical `dst`/`ValueId` operands only at admission, and commit the existing
NewBox/birth/field/publish sequence exactly once. If an existing owner cannot
provide this staging, D1-D5 stays `NoSafeSlice`; `drive_legacy_expression_v1`
and PlanNormalizer must not be treated as implicit transactions.

Lifecycle acceptance is explicit: declaration defaults -> matching `birth/N`
-> explicit construction-site overrides where the language contract permits
them -> publish usable identity. The current raw order and plan omission are
observations to reconcile, not evidence of parity. Effect acceptance requires
one joined semantic handoff over Allocation, child effects, field overrides,
and birth-body effects; it must not read instruction masks or count only the
root Allocation row.

After D1-D5, the first possible implementation slice is still limited to one
ordinary cohort only if the bridge retains body shapes, the transaction has a
named owner, and both raw/plan paths borrow the same handoff. Positive evidence
must show source id/site/birth `N`, declaration-versus-override distinction,
ordered effects, lifecycle order, one relation consume, and one NewBox commit.
Negative evidence must show pre-publication rejection without block mutation for
missing/foreign/duplicate/drifted source, birth/arity/order mismatch, effect
coverage gap, unsupported explicit override, and relation reuse.

Until those five inputs are co-sealed, D1-D5 remains `NoSafeSlice` and no
production implementation or semantic receipt is authorized.

#### D1-D6 design Decision — caller-New relation, initializer coverage, and admission snapshot

D1-D5's candidate issuer is not sufficient. The constructor semantic batch
owns declaration-side body shapes, while the resolved callable semantic batch
owns caller-side `New` shapes; the package boundary currently has both
products but issues no relation between them. D1-D6 therefore keeps the lane
in design stop and closes the missing caller-to-constructor relation before
any bridge or transaction can be implemented.

Six-line brief:

```text
Decision:
  issue one source-bound caller-New -> constructor/birth relation at the
  semantic package boundary, then let raw and PlanNormalizer borrow it.
Source authority + canonical issuer:
  ParserConstructorSourceCatalogV1/StoredFieldInitializer and the caller's
  VerifiedResolvedCallableSemanticBatchV1 body-shape loan are authorities;
  a future package-bound relation issuer co-seals them exactly once.
Non-authority:
  FunctionOwnerIdV1 alone, class/name/arity lookup, AST reparse, raw/plan
  carriers, CoreEffectPlan, EffectMask, physical IDs, N+1 arity, and sinks.
Fail-fast boundary:
  validate exact caller site, constructor source row, birth/body owners,
  Argument(i)/Initializer(i) coverage, defaults/overrides, and effects before
  child lowering; stage values/metadata and publish one selected sink.
Smallest next slice:
  design-only source-relation seed, Initializer(i) effect contract, duplicate
  body-shape loan policy, and the private admission snapshot; no code/receipt.
Non-claims:
  lifecycle implementation, raw/plan parity, Method(None), Closure, JSON,
  backend activation, R6 cutover, warning cleanup, or shelf moves.
```

Finite authority inventory:

| boundary | current authority/product | D1-D6 gap |
|---|---|---|
| constructor declaration | `parser/constructor_source_catalog.rs:107-212`, `source_authority/constructor_source.rs:16-25,175-220` | declaration `ConstructorSourceIdV1` and generated birth coverage exist; no caller New relation |
| stored defaults | `parser/declarations/box_def/members/fields.rs:99-137`, `property_emit.rs:331-357` | initializer provenance/order belongs to birth; no override carrier |
| caller semantic batch | `mir/callable_semantic_batch/issuer.rs:112-197` and resolver body shapes | caller owner/site and child effects exist; no constructor source mapping |
| package boundary | `mir/normal_callable_semantic_package/issuer.rs:130-155` | caller and constructor products coexist; no cross-owner issuer |
| constructor semantic batch | `normal_callable_semantic_package/instance_constructor_semantic.rs:146-156` | body-shape map is discarded; row cannot co-seal birth effects |
| New source shape | `parser/expr/primary.rs:259-278`, `shadow/expr.rs:307-327` | class/args/field initializers exist; field initializers have no sealed `Initializer(i)` relation |
| argument relation | `shadow/expr.rs:566-585` | `Argument(i)` relation is recorded; initializer traversal is not |
| owner identity | `resolved_semantics/ids.rs:27-65` | `FunctionOwnerIdV1` is resolver-session-local; slot equality is not a cross-pass proof |
| raw physical path | `builder/raw_expression_dispatch/mod.rs:544-556`, `new_expression.rs:133-195` | live Builder mutation, NewBox, by-name birth recovery, then fields; no expression transaction |
| plan physical path | `builder/control_flow/plan/normalizer/helpers_value.rs:523-556`, `plan/lowerer/effect_emission.rs:180-190` | args-only NewBox; field initializers reject; no birth/publish |
| future admission owner | `builder/ordinary_new_admission.rs` (does not exist at HEAD) | must be private, shared, and sink-neutral until commit |

There is no valid mapping from caller `ASTNode::New` to a constructor source
row by class text, method name, or arity. The parser/source projection must
carry either an exact `ConstructorSourceIdV1` relation seed or a typed missing
state into the semantic package. Re-discovering a declaration from the MIR,
`FunctionOwnerIdV1`, or generated `birth` text is not an issuer and cannot
repair a missing seed.

The conceptual handoff (design only; not a new receipt yet) must co-seal:

```text
caller owner + owner-branded New SourceExprSite
constructor source id + declaration/birth provenance
caller body-shape loan + birth-owner body-shape loan
source argument arity N
ordered Argument(i) sites
ordered Initializer(i) sites and explicit field names/order
declaration-default versus construction-site override provenance
root Allocation + child + override + birth-body effect coverage
```

It must exclude `dst`, `ValueId`, physical constructor arity `N+1`, MIR
instructions, `CoreEffectPlan`, `EffectMask`, Recipe keys, runtime symbols,
backend handles, and fallback state. Stored declaration defaults are consumed
by the birth owner; explicit `new Box { field: expr }` overrides are caller
operations. The handoff must not duplicate either class or silently turn an
override into a default.

The missing initializer relation is a hard boundary. Every field initializer
must receive one ordered `Initializer(i)` relation and an effect classification
before its child is lowered. Missing, duplicate, foreign, or reordered
initializer coverage is a typed reject. A child effect list that contains only
root `Allocation` or only `EffectMask` is incomplete.

The future private physical owner is tentatively named
`src/mir/builder/ordinary_new_admission.rs`; the name is a design anchor, not
an authorization to create it. It must borrow the co-sealed handoff and admit
exactly one raw or plan sink. Its private transaction snapshot must cover the
current function/block publication point, instruction lengths, `ValueId`
allocation, `TypeContext` facts, `value_origin_newbox` metadata, value-origin
caller/span facts, variable/binding/scope state, pending SSA/materialization
state, and any module metadata touched by New. A rejected child or failed
birth must restore the snapshot without leaving a block instruction, type fact,
value origin, or reserved ID behind. The existing function/module sessions and
`RawStructuredChildScopePortV1` are not implicit expression transactions.

The physical sequence remains an acceptance contract, not current parity:

```text
validate source relation and field slots
-> lower/evaluate source arguments and explicit overrides privately
-> allocate NewBox exactly once
-> consume the catalog-backed birth/N relation (declaration defaults)
-> apply explicit overrides in source order
-> publish usable identity exactly once
```

The current raw path (NewBox -> birth lookup/recovery -> fields) and current
PlanNormalizer path (args-only NewBox) are observations proving that parity is
not yet present. Positive acceptance requires one relation issue and consume,
two body-shape loans with exact owner branding, source `N` versus physical
`N+1`, complete `Argument(i)`/`Initializer(i)`/birth effect coverage, and one
NewBox/lifecycle commit through the future owner. Negative acceptance requires
pre-publication rejection with no live Builder mutation for missing source
seed, foreign/duplicate owner or ID, site drift, missing initializer relation,
birth/arity/order mismatch, effect gap, duplicate/unknown field, child failure,
or relation reuse.

Before any physical implementation, the selected plan normalizer file
`src/mir/builder/control_flow/plan/normalizer/helpers_value.rs` is already
786 lines. A separate behavior-neutral shelf split must bring each selected
production child below the 760-line trigger; no D1-D6 implementation may add
semantic growth to that file or cross the 800-line hard stop.

Until the parser relation seed, dual body-shape loan, `Initializer(i)` effect
coverage, and private admission snapshot are all accepted, D1-D6 remains
`NoSafeSlice`; no production code, fixture, route switch, or semantic receipt
is authorized.

#### D1-D7 design Decision — final parser source-seed loan and package issuer

D1-D6's final feasibility audit confirms that the existing parser products
cannot transport an exact caller-`New` to `ConstructorSourceIdV1` relation.
`ASTNode::New`, `SourceExprSiteV1`, and `ConstructorSourceCatalogV1` are each
valid in their own authority, but they are separate loans. D1-D7 fixes the
missing boundary without making the parser resolve constructor meaning or
introducing a new semantic receipt during design stop.

Six-line brief:

```text
Decision:
  the final parser source owner lends one callback-scoped seed for each
  selected ordinary New site; the semantic package joins it exactly once.
Source authority + canonical issuer:
  VerifiedFinalCallableProgramSourceV1/final transform owns the seed and
  existing ConstructorSourceCatalogV1 owns declaration identity; a future
  package issuer adjacent to normal_callable_semantic_package/issuer.rs
  co-seals seed, caller/birth body shapes, and initializer effects.
Non-authority:
  parser seed target lookup, class/name/arity re-discovery, AST rescans,
  resolver owner slots, generated birth text, MIR/CoreEffectPlan/EffectMask,
  Builder headers, physical IDs, backend lookup, and fallback state.
Fail-fast boundary:
  validate final-source brand/anchor, path coverage, catalog loan, selected
  cohort, and duplicate/drift state before semantic admission or child lower;
  unresolved/ambiguous/foreign states reject without publication.
Smallest next slice:
  design-only seed-loan field/coverage contract and package consumer join;
  no code, fixture, route switch, or new Verified*/Prepared* receipt.
Non-claims:
  parser target resolution, lifecycle/transaction implementation, raw/plan
  parity, Method(None), Closure, JSON/backend, R6 cutover, or warnings.
```

The seed is a callback-scoped loan of the final parser source owner, not an
owned semantic product and not a new Recipe key. Its conceptual fields are:

```text
parser invocation brand
exact caller declaration path + final callable slot/anchor
canonical structural New root path
ordered Argument(i) and Initializer(i) child paths
projected class/type-argument/field-name syntax observations
selected-cohort coverage for nested New and field-initializer New sites
borrow of the existing constructor catalog/declaration loan
```

The seed does not contain a resolved `ConstructorSourceIdV1` target chosen by
class text. It carries the exact source anchor and a catalog loan; the future
semantic-package issuer performs the one allowed join against the existing
catalog. A missing catalog, foreign parser brand, final-AST/catalog drift,
projection failure, duplicate/reused callsite, or an outside/generated site
is a typed state. It is never repaired by `args.len()`, `birth/N`, name lookup,
or a second parser scan.

The issuer/consumer contract is finite:

```text
VerifiedFinalCallableProgramSourceV1 seed loan
  + caller VerifiedResolvedCallableSemanticBatchV1/body-shape loan
  + birth/constructor semantic row/body-shape loan
  + existing ConstructorSourceCatalogV1
  -> one ordinary-New relation/effect handoff
  -> one selected raw or PlanNormalizer admission consumer
  -> one private ordinary_new_admission transaction
  -> one NewBox/lifecycle commit
```

The package issuer must join by exact final-source anchor and structural
child paths, then verify the catalog brand, declaration/birth row, source
arity `N`, caller/birth owner-branded body-shape loans, ordered `Argument(i)`
and `Initializer(i)` coverage, declaration-default versus explicit-override
provenance, and root/child/override/birth effect coverage. The raw and plan
paths may borrow the immutable handoff, but neither may issue or reclassify
it. One selected route consumes it once; an attempted second issue or a
relation reuse is a typed reject.

Finite source-seed states:

```text
SeedReady
SeedMissingFinalSource
SeedForeignBrandOrAnchor
SeedCatalogAbsent
SeedProjectionDrift
SeedDuplicateOrReused
SeedOutsideSelectedCohort
SeedNestedCoverageGap
SeedAmbiguousOrUnresolvedConstructor
SeedBirthOrArityMismatch
```

`Core13`, `IntegerBox`, builtin/record, JSON compatibility, direct
`op=newbox`, generated constructor-body sites, and nonselected backends are
outside the ordinary cohort. They remain explicit outside/compatibility
states, not fallback producers. All source-seed failures occur before child
lowering, block mutation, NewBox emission, or wire/object publication.

The parser issuer boundary is the existing final-source transform at
`src/parser/normal_callable_program_source/transform.rs:90-195`, after final
AST preservation and catalog validation. The semantic consumer boundary is
adjacent to `src/mir/normal_callable_semantic_package/issuer.rs:130-155`; it
is the only place allowed to combine the seed with caller and birth products.
The future `src/mir/builder/ordinary_new_admission.rs` remains a physical
consumer only: it cannot issue source identity, resolve constructors, or
re-enter parser authority.

D1-D7 design acceptance requires a finite selected-cohort seed census, exact
one-to-one anchor/catalog mapping, explicit nested/initializer coverage,
package join/reuse negatives, dual body-shape/effect co-seal, and the private
transaction snapshot from D1-D6. It must also retain the pre-implementation
`helpers_value.rs` shelf split as a separate behavior-neutral BoxShape row.
Until that evidence is recorded, D1-D7 remains `NoSafeSlice`; after it is
accepted, the next mode may switch to `fast` for the shelf split only.

#### D1-D8 design Decision — behavior-neutral PlanNormalizer helper shelf

D1-D7's source-seed contract is independent of the existing PlanNormalizer
file topology, but the selected implementation cannot start while
`helpers_value.rs` is 786 lines. D1-D8 fixes a physical BoxShape split only;
it does not alter the value normalizer, New semantics, parser authority, or
any caller contract.

Six-line brief:

```text
Decision:
  move the existing helpers_value implementation into a nested module shelf
  while preserving the normalizer::helpers_value module and public methods.
Source authority + canonical issuer:
  existing PlanNormalizer::lower_value_ast/lower_value_input signatures and
  current recursive implementation; no new semantic or physical issuer.
Non-authority:
  AST meaning, CoreEffectPlan, parser seed/package join, NewBox policy,
  effects, caller/test topology, or any `#[path]` workaround.
Fail-fast boundary:
  exact moved arm inventory, one copy of each error string, old flat module
  absent, unchanged callers, and every production child below 760/800 lines.
Smallest next slice:
  behavior-neutral file move plus facade/sibling module declarations and
  structural/parity guard; no semantic branch edit or fixture change.
Non-claims:
  ordinary-New implementation, lifecycle/effect bridge, R6 cutover,
  Method(None), Closure, JSON/backend, warning retirement, or parser seed.
```

The exact shelf is finite:

| owner | moved/retained contents | projected budget |
|---|---|---:|
| `normalizer/helpers_value/mod.rs` | old `helpers_value.rs:1-31`: imports needed by the facade, `lower_value_ast`, and `mod lower; mod variant;` | ~22 |
| `normalizer/helpers_value/lower.rs` | unchanged `lower_value_input` body from old `helpers_value.rs:32-770`; all recursive `Self::lower_value_input` calls and AST arms remain contiguous | 758 |
| `normalizer/helpers_value/variant.rs` | old `helpers_value.rs:773-786`: `enum_payload_mir_type` and `runtime_variant_box_name`, private to the shelf | ~17 |
| `normalizer/helpers_value_state.rs` | existing state helpers, unchanged and outside this move | 77 |

The old flat `normalizer/helpers_value.rs` must be replaced by the directory
`normalizer/helpers_value/mod.rs`; both forms cannot coexist. The existing
`normalizer/mod.rs:23` declaration remains unchanged, so no `#[path]` attribute
or root-level path band-aid is introduced. The child `lower.rs` adjusts only
module-relative paths (`super::super::...` where required) and imports the two
variant helpers from `super::variant`; it must not factor individual AST arms
into new dispatch helpers.

The moved arm inventory is exact and contiguous:

```text
scalar/receiver: Variable, Me/This, FieldAccess, Literal, UnaryOp
method dispatch and receiver recovery
FunctionCall, FromCall, Call
New, ArrayLiteral, MapLiteral
BlockExpr, BinaryOp, If, unsupported fallback
```

All existing callers remain untouched, including `plan/parts`,
`plan/features`, `cond_lowering_prelude`, `loop_body_lowering`,
`loop_body_lowering_associated_input`, `cond_lowering_value_expr`,
`cond_lowering_loop_header_port`, `generic_loop_body/cleanup`,
`normalizer/helpers.rs`, and the normalizer/port test suites. The facade keeps
the same `PlanNormalizer::lower_value_ast` and `PlanNormalizer::lower_value_input`
visibility and generic signature. There must be exactly one copy of every AST
arm and rejection string after the move.

Structural acceptance is required before any semantic New work:

```text
no #[path] added
normalizer/mod.rs unchanged
old flat helpers_value.rs absent; helpers_value/mod.rs present
all production children <760 and hard stop <800
caller/test diff = 0 except path-preserving module ownership
one lower_value_input implementation and one variant helper pair
```

Positive/parity evidence is the existing facade/port equivalence, nested
method and `If` lowering, map/field lowering, and unchanged direct callers.
Negative evidence is preserved pure-`If` rejection, block-prelude exit
rejection, and `New` field-initializer rejection. Cargo or fixture changes are
not part of the design row; the focused suite is selected only after the
mechanical move is applied.

#### D1-D8 path-observer census and path policy

The shelf move also has a finite physical-observer boundary. These observers
must be named before `fast` mode so the move cannot silently leave a guard or
inventory coupled to the deleted flat file:

```text
active structural observers (updated in the shelf slice):
  tools/checks/k2_wide_box_new_field_initializer_guard.sh
    -> inspect normalizer/helpers_value/lower.rs for the New rejection arm
  tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0.py
  tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0.py
    -> facade/module assertions read helpers_value/mod.rs;
       recursive-body and AST-arm assertions read helpers_value/lower.rs
  tools/checks/lib/coreplan_add_result_representation_inventory.py
    -> inspect helpers_value/lower.rs for the Add branch
  active fixtures consumed by those inventories:
    coreplan_add_result_representation_g0_inventory_v1.json
    coreplan_add_result_representation_i0_inventory_v1.json
    mirbuilder_type_fact_producer_matrix_v1.json
    -> record lower.rs as the normalizer owner

parked or historical observers (not active shelf acceptance):
  mirbuilder_fsession_direct_access_pre_s0b_v1.json
    -> historical PRE-S0B baseline provenance only
  failure_outcome_projection_binding_v0.json
  failure_outcome_semantic_site_graph_v0.json
  failure_outcome_site_inventory_v0.json
    -> generated/historical failure evidence; retained as snapshots unless a
       live guard is deliberately reopened
  archived phase/current historical design fixtures and prose references
    -> snapshot provenance only; they do not authorize an old-path guard
```

The path policy is therefore exact: `helpers_value/mod.rs` owns the facade,
`helpers_value/lower.rs` owns the recursive value arms (including `New`,
`Add`, and type-fact observations), and `helpers_value/variant.rs` owns only
the two variant helpers. No active structural guard may require
`normalizer/helpers_value.rs` after the move. Historical files may retain the
deleted path only with the explicit snapshot status above and an observable
reopen trigger: a live guard or inventory begins reading that path again.

Path-observer acceptance is part of the same behavior-neutral BoxShape row:
the finite active list is updated to the new owners, the parked list is
explicitly excluded from acceptance, no semantic branch or fixture meaning
changes, and the old flat path has zero active structural consumers. Until
this census is recorded and accepted, D1-D8 remains `NoSafeSlice`.

The remaining live generator/checker boundary is also finite and is not a
generic wildcard over every historical rust-lifecycle artifact:

```text
live rust-lifecycle source-path generators and their guards:
  tools/rust_lifecycle/mirbuilder_crate_wide_unconverted_surface_report.py
    -> tools/checks/rust_lifecycle_mirbuilder_crate_wide_unconverted_surface_report_guard.sh
  tools/rust_lifecycle/mirbuilder_crate_wide_missing_projection_policy_cluster_resolution.py
    -> tools/checks/rust_lifecycle_mirbuilder_crate_wide_missing_projection_policy_cluster_resolution_guard.sh
  tools/rust_lifecycle/mirbuilder_missing_projection_policy_joinir_plan_cluster.py
    -> tools/checks/rust_lifecycle_mirbuilder_missing_projection_policy_joinir_plan_cluster_guard.sh
  tools/rust_lifecycle/mirbuilder_native_owner_candidate_inventory.py
    -> its own --check mode against the checked-in native-owner candidate inventory
```

Their current checked-in path-bearing products are exactly:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-crate-wide-unconverted-surface-report-v0.json
  mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json
  mirbuilder-missing-projection-policy-joinir-plan-cluster-v0.json
  mirbuilder-native-owner-candidate-inventory-v0.json
```

The shelf slice must regenerate or mechanically update only the path fields
in these four live products (`helpers_value.rs` -> `helpers_value/lower.rs`)
and keep their classification, counts, anchors, and decision fields stable.
The `rust-lifecycle` generator/checker family is therefore an active observer,
not a parked prose snapshot. The FACT0 observer is a fifth live boundary:
`tools/checks/lib/mirbuilder_type_fact_partition_guard.py` (invoked by the
`mirbuilder-type-fact-partition` row in `tools/checks/guard_rows.toml`) reads
`tools/checks/fixtures/mirbuilder_type_fact_producer_matrix_v1.json`; its
normalizer rows must point at `helpers_value/lower.rs` while preserving the
partition digest/anchor meaning.

Two current design documents are policy observers rather than generated
fixtures:

```text
docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
```

Their pre-move census references and cleanup target must be
updated/annotated with the new shelf owners in the implementation slice.
They are not allowed to become unqualified active old-path guards. The
pre-S0b baseline, failure-outcome manifests, and the
archived phase documents remain explicitly `ParkedSealed` snapshot provenance.
A new live generator, checker, guard-row, or current policy document that
reads the deleted flat path is an observable reopen trigger; until the active
five-boundary set plus these two policy observers is updated and checked,
D1-D8 remains `NoSafeSlice`.

Until this exact shelf census and ownership contract are accepted, D1-D8
remains `NoSafeSlice`; no file move or fast-mode code edit is authorized.

### Post-R7 physical cleanup ledger — 2026-08-25 feedback reconciliation

This is a design-only task ledger outside the selected R6 boundary. It records
observations and reopen conditions; it does not authorize code, fixture, guard,
or production-switch work while `work_mode = design_stop`.

```text
census boundary:
  current HEAD source/docs -> named physical shelf, root-mode, empty-dir, and
  warning owners -> R7 closeout; includes tracked source and current guards;
  excludes the 2000+ line normal-root manifest, PyVM/reference/Python, and any
  broad warning-driven semantic edit.
```

#### Normal-root R0 leftovers

```text
NORMAL-ROOT-OLD-GUARD-RETIRE-R0
  observed: tools/checks/mir_root_app_mode_failfast_guard.sh still exists;
  the cutover manifest says it is a C0 integrity guard to retire in R0.
  authority: the admitted typed root projection and its selected consumers.
  non-authority: this old guard as a permanent production contract.
  acceptance: finite reader/issuer census, replacement structural guard green,
  then retire the old guard in one R0 cleanup slice. The manifest is not edited
  by this feedback reconciliation.
  reopen: a selected normal-root consumer still needs the old assertion.

NORMAL-ROOT-BOOL-PROJECTION-RETIRE-R0
  observed: MirBuilder::root_is_app_mode remains an Option<bool>, is written by
  program_root_lowering.rs:396, and is read by raw/non-main lifecycle code and
  tests; bool-shaped is_app_mode work-plan APIs remain as well. The older
  investigation line claiming production writes = 0 is historical/stale, not
  current evidence.
  authority: the admitted root execution sum/projection; non-authority is the
  legacy bool mirror and its drift checks.
  acceptance: prove the remaining readers have a typed replacement, remove the
  field and bool APIs together, and keep normal-root parity green.
  reopen: a named selected consumer cannot consume the admitted typed root.
```

#### Physical shelf and empty-directory cleanup

```text
MIR-BUILDER-NORMAL-SCRIPT-MOD-SHELF-R0
  observed: builder.rs:306-307 uses a new #[path] band-aid for
  builder/normal_script/direct_static/semantic/normal_script_direct_static_recipe.rs;
  the target directory has no mod.rs. This is a topology debt, not a semantic
  Call decision.
  issuer/owner: normal_script module tree; no new authority is issued.
  acceptance: add the path-complete normal_script/mod.rs shelf and remove only
  this root #[path] while preserving module paths, tests, and behavior; update
  the path guard in the same bounded BoxShape slice.
  reopen: another owner or privacy edge requires a root-level path attribute.

MIR-EMPTY-DIR-CLEANUP-R0
  observed: six empty directories have no current entries:
  src/box_callable/generated;
  src/mir/builder/normal_default_root_catalog_lifecycle;
  src/mir/builder/resolved_lowering/dynamic_v2_aot_activation;
  src/mir/callable_parameter_demand;
  src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/operator_carrier_lifecycle;
  src/parser/callable_parameter_source/normal_root_preservation.
  acceptance: rerun finite path/reference census, remove only empty physical
  shelves, and record the six-path result; no recursive cleanup.
  reopen: a generated-file rule or source owner recreates one of these shelves.

MIR-COMMON-V2-SHELF-R0
  observed: the reported flat 15-file set is not reproduced at HEAD. The
  selected builder shelf has 14 flat common_v2*.rs files under
  src/mir/builder/resolved_lowering/ plus 13 files under its
  common_v2_session/ child (including mod.rs); the separate
  src/mir/loop_recipe_contract/ family has 16 common_v2*.rs files.
  Treat the reported 15 as an inventory discrepancy until the intended family,
  missing member, and production/test ownership are named.
  acceptance: reconcile the finite selected-family inventory, then house only
  the named common_v2 family behind one mod.rs without changing imports or
  semantics.
  reopen: a fifteenth file or external path consumer is found.
```

#### Warning retirement lane

```text
MIRBUILDER-WARNING-RETIREMENT-R0 (existing T5-D7 row)
  observed baseline in the current card/docs = 433 warnings (422 dead_code +
  11 private_interfaces). The feedback's “17 unexplained allows” is not yet an
  exact diagnostic census; a broad attribute grep is not equivalent to warning
  count. Keep both claims separate until an exact HEAD diagnostic ledger names
  the 17 or corrects the number.
  authority: cargo diagnostics plus each live module owner; non-authority:
  blanket #[allow] counts, retired-module noise, or warning-driven guesses.
  acceptance: classify A mechanical unused import/re-export rows as one batch
  with one end guard, skip B already-retiring modules, and give each C active
  definition one semantic remove/retain decision with reason. No broad purge.
  reopen: a new warning class or a live owner appears during the bounded batch.
```

| state / edge | source authority | D0 terminal classification |
|---|---|---|
| `Global`, `Extern`, `Value`, `Method(Some(receiver))` | route resolver / `CallTarget` | canonical callable; preserve target operands then ordered args |
| `Method(None)` | legacy/static producers and v0 projection | compatibility/transitional only; static must become qualified `Global`; core reject until migrated |
| `Callee::Closure` with `dst=Some,args=[]` | closure construction owner | pre-core input -> `NewClosure`; runtime closure call is `Callee::Value` |
| Closure with runtime args or missing destination | no valid construction relation | typed reject before publication |
| `Constructor` | constructor resolver / route family | pre-core input until one `NewBox` or typed-Call owner is selected; no generic reclassification |
| `NewBox`, `NewClosure` | existing construction owners | independent physical instructions; not Call target fallback |
| `None + legacy name/func` | JSON-v0 owner-local catalog | resolve once to exact `Callee` or typed reject; never publish missing Call |
| missing, `INVALID`, duplicate, foreign, non-String, malformed explicit target | none | typed reject before block/wire/object publication; no retry |
| `MirCall.flags` / `CallFlags` | current builder intermediate only | physical terminal consumes no flag; non-default consumer would reopen the design |

Exact edge classification for D1:

```text
cut       = semantic func readers, Option<Callee> terminal readers, target-Const
            reconstruction, Method(None) recovery, and any backend/optimizer
            by-name fallback once their canonical replacement is named;
compat    = owner-local JSON-v0 raw draft/catalog only, with one resolve point;
park      = JoinIR remap until a live caller and both lifecycle owners are named;
reopen    = public MirCall/CallFlags use, non-default flag semantics, a second
            Constructor/NewBox issuer, or a Closure wire/runtime consumer.
```

The audit found `MirCall.flags` is created and then discarded by the physical
terminal; no production field reader was found. This permits a future private
staging cleanup, but does not authorize deleting the public `MirCall`/
`CallFlags` exports in D1. `Method(None)` still has real producers and a
by-name/recovery consumer, while Closure and Constructor have split V0/V1
construction paths. Those are schema blockers, not mechanical compiler fixes.

Selected next design task is
`MIR-CALL-R6-CORE-SCHEMA-D1-D8-ORDINARY-NEW-HELPER-SHELF-DESIGN`.
D1-B-PARK accepted the existing static handoff/outside boundary and D1-C2-I0
closed the lossy Closure body egress edge; D1-D0/D1-D1 closed only negative
V0/V1 publication and shape edges. The D1-D8 helper shelf, D1-D7 final parser
source seed and package join, initializer/effect coverage, dual body-shape
loan, private admission snapshot, generic Method(None) issuer, and
Constructor/NewBox dual route remain blockers. Until the full D1-D8 and
remaining Method(None) edges are accepted,
do not change `Option<Callee>`,
`func`, `Method(None)`, `Callee::Closure`, `MirCall`, or `CallFlags` in code.

Post-R7 normal-root cleanup (parked, separate lane):

```text
NORMAL-ROOT-PROJECTION-SUM-D0
  combine mode + root projection into one App/ProgramRuntime sum; remove bool drift checks
NORMAL-ROOT-SOURCE-NOMENCLATURE-D0
  Parser MainObserved -> compiler static-main admission -> App; rename the source-backed lowering view
NORMAL-ROOT-SYNTAX-LOAN-D0
  replace index/name AST rediscovery with an admitted opaque identity loan
```

These rows do not reopen normal-root production or the 2000-line manifest.

retirement seriesの固定順は次。

```text
R1  exact qualified Program JSON-v0 producers
R2  owner-private MIR JSON-v0 input state
R3  Program/MIR JSON-v0 exact pre-core resolution; late issuer retirement
R4  Callee operand/remap SSOT and semantic consumer migration
R5  optimizer/interpreter/printer/JSON/selected backend terminal closure
R6  Call core schema atomic cutover
R7  impossible-state guard and docs/reference closeout
```

loader、interpreter、optimizer、printer/JSON、backend、fixture/reference callerが
R2-R5で閉じるまでR6を選ばない。valid explicit calleeはlegacy decorationより優先し、
malformed explicit callee、missing/ambiguous/non-String/foreign legacy relationはCall
publication前にtyped rejectする。reject後に別target sourceをretryしてはならない。

active rowの選択は`CURRENT_STATE.toml`とrolling workstream cardだけが行う。
R1より後の行は前行のevidenceなしに自動起動しない。cleanup上位順は
`mirbuilder-cleanup-retirement0-d0-task-map-2026-08-04.md`が保持する。

## 作業分離ルール

- BoxShape lane と BoxCount lane を混ぜない
- DebugLog/Nop/NewBox/NewClosure はこの lane で触らない
- 1コミットに 1変換（または 1契約）だけを入れる
- fast gate FAIL 状態で新しい fixture/case を増やさない

## 受け入れコマンド（最小）

```bash
cargo test instruction_diet_ledger_counts_match_docs_ssot -- --nocapture
cargo test mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture
cargo check --bin hakorune
```

必要に応じて:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```

## 作業役へのハンドオフ文（そのまま貼れる版）

「MIR call-site canonicalization lane を MCL-0 から順に実施してください。  
各コミットは 1タスクのみ。BoxShape 専用で、受理拡張（BoxCount）は禁止。  
`BoxCall/ExternCall -> Call(callee=...)` を backend 手前で正規化し、backend 入口で legacy 残存を fail-fast 化してください。  
NewBox/NewClosure/DebugLog/Nop の整理はこの lane の対象外です。」
