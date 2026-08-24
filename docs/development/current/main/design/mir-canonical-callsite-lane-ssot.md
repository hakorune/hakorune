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

literal censusに加え、`runner/mir_json_v0/call.rs`が入力依存でmissing-callee Callを
構築できるdynamic edgeを必ず数える。retirement完了はliteral zeroだけでなく、
runtime missing-target issuance zeroを要求する。

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
Next design stop: MIR-CALL-JOINIR-OPERAND-REMAP-D0; no backend_shape, native, or R6 implementation is implied.
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

JoinIR operand-remap D0 (next design stop: `MIR-CALL-JOINIR-OPERAND-REMAP-D0`):

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

Feedback reconciliation and deferred task queue (not selected):

```text
R4b status: HEAD already delegates Call used_values to Callee::for_each_value_operand; Value, Method receiver, Closure captures/me, args order, duplicates, and legacy None parity are covered by the shared tests. Escape, value-consumer, ownership, and query now reuse the occurrence projection where their separate policies allow it.
Remaining duplicate: JoinIrIdRemapper still owns a local Callee match and its collect_values Call arm is incomplete for Value/Closure. Queue MIR-CALL-JOINIR-OPERAND-REMAP-D0 after the current JSON D1; authority is Callee::rewrite_value_operands/for_each_value_operand, but production selection stays NoSafeSlice until a named JoinIR caller and shared lifecycle boundary are proven.
R6 gate: decide MirCall/CallFlags retirement, mandatory Method receiver with static calls as qualified Global, Closure pre-canonical construction versus Callee::Value call, and Constructor/NewBox boundary in one schema decision.
Post-R7 cleanup: normal-root mode/projection sum, MainObserved naming, identity-based syntax loan, and builder.rs production/compatibility/test barrel census remain separate cleanup rows; PyVM/reference/Python/native_driver remain ParkedSealed.
```

R5 row rules: each task owns one old edge and reuses the shared corridor
guard; no new per-row shell guard, no fixture-only acceptance, and no R6 field
editing. The census includes direct callers, dynamic construction, wire
writers, and selected backend preflight; literal `callee: None = 0` is not
sufficient evidence.

R6 decision gate (not selected):

1. Prefer retiring exported `MirCall` and `CallFlags`; the canonical physical
   shape remains `Call { dst, callee, args, effects }`. Keeping them requires a
   named flag authority and backend semantics, not a compatibility re-export.
2. Make `Method { receiver: None }` impossible by issuing qualified `Global`;
   `Method` then owns a required receiver. A new `StaticMethod` variant is not
   admitted without a source-backed producer.
3. Restrict `Callee::Closure` to the pre-canonical construction input that
   becomes `NewClosure`; a closure call uses `Callee::Value(closure_value)`.
   Constructor versus `NewBox` remains an explicit boundary decision.

R6 acceptance requires the R5 matrix green, all direct/dynamic `func` readers
   classified, and no unresolved issuer. Until then, do not change
   `Option<Callee>`, `func`, `Method(None)`, `Callee::Closure`, or
   `MirCall`/`CallFlags` in code.

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
