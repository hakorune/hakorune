# Macro owners

`default_derive.rs` owns pure default Equals/ToString selection and generated
syntax. Its caller supplies settings and the selected public-field list. It
reads no environment or registry and issues no source identity, parameter
contract, Completion or runtime ABI.

`engine.rs` owns expansion order, configuration reads, fixed-point/cycle bounds
and the explicit MacroBox invocation. Both preflight and actual default
expansion consume the shared pure selector; the body constructors have one
implementation. Existing methods take precedence and static boxes do not gain
receiver methods. Public-field order and generated declarations are unchanged.

`normal_callable_transform.rs` owns the current source-backed transform
boundary. Exact/no-op source is retained. Default derives that would add a
callable, registered transformations, generated test tails and unknown mutation
still reject before authority loss. This extraction does not open those routes.

The selected constructor design calls for using the pure producer inside the
existing open parser source transaction before initial co-seal. That connection
must issue generated origins and explicit parameter coverage; ToString's zero
parameters must not become missing coverage. Parser environment reads, a second
body generator and relaxed final-source equality are not substitutes.

Validation for the pure-owner extraction uses existing `macro_derive` tests
and `normal_callable_transform_tests`, serially. Source transaction, normal CLI
acceptance and legacy transport retirement have their own later gates in the
[active constructor design](../../docs/development/current/main/design/constructor-lifecycle-llvm-lowering-ssot.md).
