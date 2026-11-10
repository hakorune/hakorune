# Dynamic/String equality runtime correction

Date: 2026-11-10. Base: d7bddc9793. Scope: canonical opaque-carrier String comparison.

## Rule and implementation

The existing source contract and issuer are unchanged. Decode each operand into
Text, Null, Other valid payload, or InvalidContract before choosing equality.
Text/Text uses character order; Null/Null is equal; all other non-Text pairs are
unequal in this Text-comparison ABI. In particular Other is never treated as
Null. This is not a general Dynamic/Dynamic equality implementation.
Ordering with Null or Other remains TypeError 302. Both operands must validate
before returning a Bool or a language Fault; an invalid second operand cannot
be hidden by a valid non-Text first operand. Only Normal writes `out`.

`fault_checked_string.rs::comparable` now returns a private classification;
`compare` alone selects the operator's result/Fault. Export, ABI, source admission,
MIR, independent verifier, and carrier ownership are unchanged.

## Routes traced by read-only workers

| Stage | Existing owner and boundary |
|---|---|
| Source and target | `source_call_target/dynamic_member.rs:137–221`: Dynamic receiver preserves selector, arity and site; typed String is a different target. |
| Dynamic result | `home_local_call_flow.rs:255–286`: call results and selected nullable Text results use opaque carriers. |
| Operator contract | `dynamic_operator_contract/issuer.rs:160–184` and `model.rs:106–112`: equality of valid non-Text and Text is unequal; ordering requires Text. |
| Bool source | `home_bool_condition.rs:416–444`: retains predicate and operand order. |
| Projection | `ordinary_new_local_commit/root_call_entry/scalar_condition_operation.rs:55–110`: Text and retained call-result/local DynamicCarrier are projected; tagged Dynamic formal is not interchangeable. |
| Independent MIR | `instruction/invoke_string.rs:145–147,352–358`: validates operand domain and lifecycle. |
| Physical ABI | `hako_llvmc_ffi_published_lifecycle_instruction_schema.inc:239–245`, `hako_llvmc_ffi_lifecycle_v4_string_emit.inc:124–132,252–260`: carrier descriptor kind 5 and checked status branches. |
| Runtime | `fault_checked_string.rs::compare/comparable`; `runtime/dynamic_invocation.rs:162–191,360–377`: registered caller-owned payload and length's Integer result. |

Paths above are under `src/mir` unless otherwise named; the source target lives
under `source_call_target`, Bool/call flow under `resolved_semantics`, physical
files under `lang/c-abi/shims`, runtime export under `crates/nyash_kernel/src/exports`.

## Reproduction and evidence

Scratch inputs under `$L/probes` compare both orders of `src.length()` and Text,
and a bound length local; the integer probes stop before EXE. Direct length
stops at `ordinary_new_borrowed_static_route_selection.rs:127`
(`static-input/cohort-unfinished`); bound locals stop at
`install/lowering_port.rs:542` (`IncompleteOrdinaryNewCoverage`). Tagged formal
probes stop at `ordinary-new/root-instance-call-receiver` before runtime. No
source acceptance is broadened to reach this bug. The substring controls reach
EXE for `==`, `!=`, `<`, both orders, but MIR identifies their receiver as exact
Text and their result as OwnedString, not DynamicCarrier. They are ordinary
Text regression controls, not evidence that an opaque source route was reached.
Opaque Text and Integer carriers are exercised by the ABI matrix. All 28 scratch inputs have before/after
logs in `$L`; exact results are in the final report.

The existing ABI test file now contains one matrix covering all 36 ordered
pairs of static Text, two carrier Texts, Null, Integer from the real length
provider, and non-String Object, for all six predicates. It also checks invalid
raw/stale/reserved descriptors on both sides, invalid opcode, no output on
Fault/InvalidContract, and retained single End obligations. Before the fix,
Text/Integer equality fails with status 1 instead of Normal false. This contract
was not covered by the existing materialization test, which correctly retains
TypeError when asked to materialize an Integer as Text. No source EXE test was
added: the failing non-Text source cases have an earlier acceptance boundary;
adding only a passing substring test would not pin this defect.

## Malformed logical payload versus invalid transport

`MalformedCarrierBeforeResultNoOperandMutationNoRebind` describes the abstract
logical-carrier contract. This export takes a physical descriptor and an opaque
registry token, not an arbitrary logical payload. Invalid descriptor, missing
brand, stale token, or a token no longer caller-owned remain InvalidContract 2.
`dynamic_invocation.rs:181–191` stores a private closed payload enum; public
publication cannot create a malformed payload variant. No reachable malformed
logical payload is demonstrated by these tests, and no new source Fault branch
is invented. Other valid payload is not malformed. No Fault/status-2 convergence
or source/issuer change is needed for this correction.

## Retained routes and next work

- Tagged Dynamic formal: projection and checked Text ABI do not accept kind 4;
  retain the named refusal, not a new raw payload interpretation.
- VM ordinary comparison uses `abi_util::eq_vm`, whose String/non-String equality
  is already false. No VM execution feature was enabled for this runtime fix.
- Generic C raw comparison and LLVM Python's `nyash.string.eq_hh` are separate
  compatibility representations; neither consumes this opaque descriptor.
  Their handle/string heuristics remain unmodified and are not proof of this fix.
- Array-get carriers use a separate source owner; direct projection into this
  comparison has not been established. Do not broaden admission by shape.
- Field-read normalization should preserve one evaluated operand and its lease;
  StringCompare can reuse this classification but still needs independent
  representation/liveness evidence. Text snapshots still copy here.
- The End clarification is path-wise: preserve acquisition/transfer/End order on
  every executable Normal/Fault path, not the number of static End instructions.
  This change removes no Length Fault edge or cleanup instruction; source/MIR
  producers are unchanged. Per-input Length cleanup deltas are therefore zero
  relative to the StringLength baseline, not an additional normalization claim.

Final verification and hashes are recorded in `$S/codex-mixed-equality-report.md`.

## Opus の確認（2026-11-10）

Codex の `44e75b3948`（runtime の分類とABIの test）と `297ea922b3`（文書）を、main `d7bddc9793` から監査して fast-forward した。

- 修正は `fault_checked_string.rs` の `comparable` だけ。operand を Text・null・それ以外の正しい値・壊れた入力（status 2）に分け、両方を検査してから演算子を選ぶ。String でない正しい値との `==` は false、`!=` は true、大小の比較は従来どおり TypeError。String でない値を null に潰していない。仕様と issuer は変えていない。
- 再現: Integer の carrier を使う source の入力は、EXE の前の受理の段で止まり、EXE には届かなかった（`static-input/cohort-unfinished`、`IncompleteOrdinaryNewCoverage` 等）。不具合の証拠は、本物の Dynamic の `length` provider で作った Integer の carrier を使う ABI の test（修正前は Fault、修正後は正しい Bool）。
- 両方とも Text でない組（null と null、それ以外とそれ以外）は、issuer の String 比較の domain（片方が exact String）の外で、canonical の source からは届かない。null と null は従来どおり等しく、それ以外同士は不等になった。`StringCompare` の回で、domain の外の組を InvalidContract にするかを決め直す。
- issuer の整数の等値（`issuer.rs:264–285`）も、`sealed()` の初期値の MaySuspend と TypeError を持つ。整数の比較は既に枠のない形で書かれているので、比較の契約の精密化の回で、約束の欄を正直にする候補。
- 検証（Codex の log を読んだ）: runtime の `checked_` 34件、C の物理 test、229入力・apps 100本・384組・固定 suite 8/8・Opus の 677本・未見の 18本・String の probe 91本で差分0。`src/` は変えていないので、全 lib と 1,788本の dump は流していない（CLI の binary は基準と同じ hash）。
