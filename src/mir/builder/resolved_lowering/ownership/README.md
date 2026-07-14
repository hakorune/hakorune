# Resolved Lowering Ownership Planner

This directory owns the disconnected SSA-RC0 transition law for canonical
function-local bindings.

It may consume:

- exact owner-branded `BindingRefV1` subjects;
- already-classified `Trivial`, `Owned`, or `BorrowedStrong` values;
- current owned roots supplied by the future function Binding SSA session;
- source declaration order and the sealed function result ownership profile.

It may produce typed plans for:

- declaration installation;
- assignment replacement;
- BlockExpr scope result and reverse-order close;
- function Return/fallthrough and reverse-order root close;
- unpublished draft discard.

It must not import `MirBuilder` or `MirInstruction`, allocate `ValueId` or
`BasicBlockId`, inspect names, infer storage from raw values, keep a second
`BindingRef -> ValueId` map, or connect itself to production Lower. Upvar,
capture cell, field, index, and general place ownership remain outside this
local-binding vocabulary and must fail preflight before this planner is used.

Materialization order is encoded structurally: materialize `next`, commit the
new Binding SSA definition, then destroy `previous`. A list of freely sortable
ownership actions is intentionally not exposed.
