# Checked Loop lowering

The Loop child entry is `RawLoopChildEntryPortV1::lower_loop` in
`src/mir/builder/raw_loop_child_port.rs`. Source Facts and recipe matching
supply the checked source product and its refusal diagnostics.

`lowerer/source_loop_completion.rs` opens the PHI transaction.
`lowerer/source_loop_driver.rs` lowers the condition and body through the
invocation port and emits the original control edges and their cleanup.
The completed MIR is independently checked against the original source by
`source_loop_physical_validation.rs` and then by the module verifier.

The current entry and the other syntax paths are listed in
[`mirbuilder-route-guide.md`](../../../../../docs/development/current/main/design/mirbuilder-route-guide.md).
