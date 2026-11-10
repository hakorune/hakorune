# Field-read normalization

Base: `e96d198b99`. Branch: `field-read`. Read-only workers trace source,
Home proofs and projection before implementation. Logs `$L`; baselines `$SL` and `$ME`.

## Rules

F1. One field read retains one original request, receiver, field, source site and evaluation; its lease and use position are never reconstructed from a name or result type.
F2. Existing LocalFieldReadRequestV1 and LocalFieldReadResultV1 own the common request/result; differences of Text, scalar, object, nullable and container remain variants and retained evidence, not context-specific replacement types.
F3. LocalFieldReadSourceV1 owns the source-qualified physical description used by the writer and source correspondence; generated-MIR verification remains independent and reads actual MIR, declarations and CFG, never the source plan.
F4. Checked versus raw field access follows original receiver/read/write/ownership evidence; a declared type, opcode or successful backend alone grants no lighter operation.
F5. HomeFieldSourceConsultV1 and its existing package implementation own field proof queries; removing callback plumbing must preserve each query's scope, census, source identity and staging responsibility.
F6. Probe queries remain read-only; executable issuance retains the same all-or-nothing batch, affine progress, exact receiver, original read and ordered consumption.
F7. Declaration spelling is classified only by DeclaredResultKindV1; scalar projection must preserve the difference between a known nonscalar result and an unsupported annotation, including caller-specific errors.
F8. All executable Normal and surviving Fault paths retain acquisition, transfer and End responsibilities and order; static cleanup counts are diagnostic, not equality gates.
F9. Existing acceptance, refusal wording and EXE results remain unchanged except the explicitly selected u16 boundary; local Text field binding remains closed.
F10. A child-path read may only emit a raw field operation when its receiver has the independently supported original object/borrow evidence; otherwise the builder gives a named refusal rather than producing invalid lifecycle MIR.
F11. Replaced types, decision branches and callback plumbing retire with their consumers in this series; no new classifier, ledger, progress type, public ABI/opcode or shape-specific producer is introduced.
F12. Completed cleanup is production net negative after formatting, including common code and all move destinations; u16 functionality, if any, is counted separately.

## u16: independent raw child borrowing

F13. A raw child view is proved only from public canonical nonweak, nonnullable user-object field definitions, a live Published owned root, and a readonly use interval; source descriptors and MirType names alone are not ownership evidence.
F14. The existing classifier and instruction-kind checks keep their order; the same raw-base liveness check may recognize the proved child view, without retrying verification or predicting OutsideModel from source syntax.
F15. Each view has one definition, is not a parameter, and every use is a later ObjectFieldGet base in the same block with the exact child owner; consumers are declared integer scalars or equally proved child views, with no intervening mutation or ownership operation and no escape through Copy, PHI, Call, Store or Return.
F16. The existing kinds map records the proved Borrowed views; no new ledger, ownership acquisition, release, MIR row or public ABI is introduced, and failed proof keeps InvalidObjectLifecycle rather than an ignored OutsideModel error.

## Existing routes and responsibilities

|Stage|Base source location|Responsibility|
|---|---|---|
|Original expression|resolved value-profile expression plan and resolved semantics field observers|Resolved receiver/root/site/path and single evaluation|
|Home source walk|`src/mir/resolved_semantics/home_new_prefix.rs:405`|12 FnMut callbacks plus HomeFieldSourceConsultV1; five FnMut are field-proof queries|
|Field proofs|`src/mir/resolved_semantics/home_field_source.rs`; `src/mir/normal_callable_semantic_package/ordinary_new_coseal_field_source.rs:30`|Existing consult and original writer/constructor/entry source owner|
|Batch issuer|`ordinary_new_coseal_issue_source_field_batch.rs:35`|Root batch, original declaration, nullable/formal/child source, staging and conflict checks|
|Physical description|`ordinary_new_field_read_source.rs:7,53,65,90`|LocalFieldReadSourceV1; checked demand and physical correspondence|
|Expression writer|`src/mir/builder/ordinary_new_admission/selected/expression_value.rs:397,487`|Path/direct field reads, retained receiver and Normal/Fault temporary cleanup|
|Final verification|`src/mir/verification`; `src/mir/ownership_ssa/verify.rs:86–93`|Actual operand/CFG/liveness and lifecycle; u16 currently fails here|
|Declared result|`src/mir/exact_trivial_scalar_abi.rs:60`; `resolved_semantics/callable_index.rs:415`; package `physical_header.rs:185`|Spelling table already one; repeated scalar projection remains|

Unqualified package paths in the table are under `src/mir/normal_callable_semantic_package`.
Detailed type/decision inventories, final metrics and validation are recorded in the report.

## Forms and governing rules

|Existing form/context|Rules and retained boundary|
|---|---|
|Declared i64, untyped scalar; return/local expression/condition/print/argument/loop|F1–6, F9; keep original integer/declaration/SUM/write proof|
|Text read and expression use|F1–6, F8–9; preserve one materialized copy and cleanup; do not open local s = t.f|
|Nullable/object/borrowed receiver and me.f|F1–6, F8–10; keep receiver identity, nonnull/read interval and ownership evidence|
|Child path n.leaf.value|F1–6, F10; preserve parent handle support or refuse before invalid MIR|
|Map/Array fields|F1–6, F8–9; preserve whole-Box element census, residence and exact operation contract|
|Declared scalar/Text/object result and unknown spelling|F7; source-resolved object proof remains distinct from spelling|

## Stages

A. Fold duplicate request/result/physical interpretation into existing field representations.
B. Consolidate field query callbacks into the existing Home field-source owner.
C. Reuse the existing single declaration classifier and scalar projection.
D. Resolve u16 using the same child-read evidence, without weakening the verifier.

Final validation uses the final source binary: 1,788 dumps, all changed compilable
inputs, fixed suite and existing EXE groups, complete lib failure names, applicable
Python physical tests and naming/pointer/rustfmt/diff checks. No source edits after it.

## A/B implementation notes

The physical factory is the existing LocalFieldReadSourceV1: raw projection and
checked operation are selected there, retaining source-field guards. The writer
and source correspondence share that answer; independent MIR verification does not
consume this descriptor. ObservedLocalFieldReadV1 and the nested raw-route wrapper
are removed, not replaced by aliases.

HomeFieldSourceConsultV1 now carries all six field queries: scan FnMut ports
fall from12 to7 and the 64 named field-FnMut signatures disappear. Original
construction/write/frame/SUM capabilities remain optional and absent on the probe;
all ten former defaults are returned before lookups, errors or diagnostics.
The two existing staging maps are retained only by the verified walk.

The child-field cohort model is separated from the source-query owner without
changing its witness conditions. Installation tests move intact out of the source
model so the touched production files stay below800 lines; test names/assertions
and literal inputs are unchanged. Neither movement is counted as a deletion.

The final A scope audit also found argument-position physical metadata outside
the descriptor. Its exact declared-i64 proof now issues that same descriptor;
take, writer and correspondence retain it. ArgumentFieldRead remains only the
distinct affine/releasable receipt, not a separate field/result classifier.

F17. C nonnull field readability may use an existing nullable-transport Birth
formal only after joining its exact provided-field writer to this root's unique
Birth call, a kind3 canonical child actual, Normal dominance and the unchanged
nonmutation suffix; nullable transport alone never proves nonnull.
