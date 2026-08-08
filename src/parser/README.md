# Parser layer boundary

The parser owns source syntax, ordered source coordinates, and parser-private
transport products. It does not resolve callable targets, issue semantic
contracts, build Recipe/CallSlot products, or emit MIR/runtime routes.

## C-S1 delegate target index

`delegate_target_index.rs` is a borrowed lookup proof over one unpublished
`OpenParserPostpassProductV1`. Exact parser invocation brand, Box declaration
path, and existing explicit method source relation are the authority. Field
type and expose method names are query selectors only. The index and its
`TargetMethodRefV1` results never mutate AST, method inventory, prepared/final
seals, or generated delegate placement.

The R6-S3B-C-I0-D0 design is now accepted but its implementation remains unopened. The
future private `PreparedDelegatePostpassBatchV1` owns the staged batch; the
next bounded row owns one private staged delegate batch: all host/expose rows
are preflighted, target method declaration/signature views are borrowed only
for forwarding AST construction, placement receipts are computed against a
staging inventory, and generated relation rows are carried through the
prepared source payload. A single consume-return commit applies AST,
inventory, and relation changes; any failure drops the whole unpublished
postpass product. The atomic commit replaces the unpublished postpass product
once. C-I0 does not extend `ParserBoxSourceSealV1`; R6-S3B-D alone
may issue complete resolver-visible generated relation coverage.
