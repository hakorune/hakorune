# Macro owners

`default_derive.rs` owns pure default Equals/ToString selection and generated
syntax. Its caller supplies settings and the selected public-field list. It
reads no environment or registry and issues no source identity, parameter
contract, Completion or runtime ABI.

`normal_policy.rs` captures enabled/default-derive settings once at the normal
MIR/LLVM materialization caller. The normal parser borrows that snapshot and
the transform consumes it. Parser code reads neither macro environment nor
MacroBox registry. Legacy engine callers retain their existing settings path.

`engine.rs` owns compatibility expansion order and MacroBox invocation. Existing
methods take precedence and static boxes do not gain receiver methods.

The normal parser transaction invokes the shared pure producer before initial
source co-seal. It issues each generated origin from its real parent Box,
derive kind and placement, with explicit parameter coverage: Equals has one
unannotated parameter; ToString has zero. Generator labels alone do not grant
source authority. Nonempty public-field generation stops at the named missing
dynamic-field/Text-conversion contract before raw lowering.

`normal_callable_transform.rs` retains exact source and performs no second
default expansion. Residual generation, registered transformations and generated
test tails reject before authority loss. Compatibility expands once using the
same captured policy. AST-only parsing retains its separate original behavior.

Focused coverage lives in `default_derive_source_tests.rs`,
`normal_callable_default_derive_tests.rs`, existing `macro_derive` and
`normal_callable_transform_tests`. Full source acceptance, normal CLI/host
verification and transport retirement remain tracked in the
[active constructor design](../../docs/development/current/main/design/constructor-lifecycle-llvm-lowering-ssot.md).
