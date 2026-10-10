# Static CurrentOwner integer Eq D0

Status: Decision closed; selected Eq execution S0 follows
Date: 2026-10-10
Scope: MIRBUILDER-STATIC-CURRENTOWNER-I64-EQ-D0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-condition-cohort-d0-2026-10-10.md
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-two-caller-actual-s1-2026-10-10.md
  - src/mir/dynamic_operator_contract/README.md

## Selected frontier

The unchanged `SizeClassBox.accepts` condition is
`me.size_to_bin(size) == me.huge_bin()` in
`lang/src/hako_alloc/memory/size_class_box.hako`. Home already observes both
original Static calls in Lhs/Rhs order; S1 at `4a5592426d` completed the
two-caller `size_to_bin` actual/packet source cohort. Neither establishes
the integer Eq execution envelope or the linked condition CFG. The recorded
whole-app mimalloc-lite first stop remains Heap-to-Page
`source-only-object-actuals`; `accepts` is an upstream selected caller, not
the measured app first stop.

## Read-only audit and Decision space

`QualifiedStaticCallClaimIndexV1` owns both original call identities.
`home_static_eq_condition.rs` owns the exact source Eq pair and ordered Home
observations. `CallPacketSourceV1::static_i64` owns each executable call
packet. `dynamic_operator_contract/issuer.rs` is the sole **semantic
envelope** issuer and currently has no `Equal(NormalInteger, NormalInteger)`
domain. It owns no CFG or MIR. Existing `Greater`/`GreaterEqual`/`LessEqual`
integer envelopes establish the intended `TrivialBool`, `MaySuspend`,
read-only operands, Normal/Fault and no lifecycle shape. The `Equal(Dynamic,
Null)` envelope is non-suspending and cannot stand in for integer Eq.
`physical_program_json.rs` already spells `CompareOp::Eq` as `compare/eq`,
which is JSON syntax, not source or execution authorization.

Decision: one bounded Static Eq execution slice may add the complete
`Equal(NormalInteger, NormalInteger)` semantic envelope and a caller-local
source-to-MIR correspondence. The raw/default selected path is
`raw_expression_dispatch/statement_surface.rs` -> `control_flow/mod.rs`
condition descent -> `ops/binary_expression_descent.rs` ordered children ->
`ops/comparison.rs` shared append -> `if_form.rs` branch. Each child takes
its original source through `recursive_child_lowering/direct_call_disposition_port.rs`,
the sole `CallPacketSourceV1::static_i64` issuer, and
`ordinary_new_admission/selected/terminal_call/lexical_i64.rs` Invoke/
NormalResult/Fault emission. Keep those physical owners. Add only the missing
proof that the exact Home Eq pair, two source call sites and completed packets
correspond to the emitted child results, `CompareOp::Eq` original append,
and If condition destination. The formal-origin Instance borrowed-compare
loan is not this proof. The generic `located_if.rs` is a different limited
grammar and does not authorize this Static Eq. A real-source test must confirm
that unchanged `accepts` selects the identified raw/default path; if an
earlier gate intercepts it, resolve that in S0 without widening the source
shape or falling back.

Fail-fast boundary: missing/reordered RHS claim or site, wrong I64 result,
actual, Completion or packet, changed `==` to `!=` or `&&`, or swapped call
order grants no partial Eq publication. A source/class mismatch is reported
at the selected caller rather than being inferred from an unrelated callee.

Required physical acceptance: Lhs Invoke Normal -> RHS Invoke Normal -> Eq
Compare -> Branch; Lhs Fault bypasses RHS and Compare; RHS Fault bypasses
Compare. Prove both original source sites/packets, no
`borrowed_null_compare`, and source-to-published-MIR correspondence with the
existing compare publication family. An issuer-only green is explicitly a
prerequisite, not the physical acceptance. The existing one-caller and
two-caller Static routes must stay green. Do not claim app EXE advance until
the unchanged imported source is measured again.

Read-only worker audited the semantic/physical owner split and requested
ordered Fault acceptance. No shared checkout edits or Cargo were delegated.
The follow-up read-only audit identified the exact raw/default lowering
path and found no need for a separate issuer-only prerequisite slice.
