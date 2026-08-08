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

The R6-S3B-C-I0-D0 design is accepted and its bounded implementation is
closed. The private `PreparedDelegatePostpassBatchV1` owns the staged batch:
all host/expose rows are preflighted, target method declaration/signature views
are borrowed only for forwarding AST construction, placement receipts are
computed against a staging inventory, and generated relation rows are carried
through the prepared source payload. A single consume-return commit applies
AST, inventory, and relation changes; any failure drops the whole unpublished
postpass product. C-I0 does not extend `ParserBoxSourceSealV1`; R6-S3B-D alone
may issue complete resolver-visible generated relation coverage.

The C-I0 implementation receipt is implemented by `delegate_batch.rs` and
`delegate_source_relation.rs`. `ParsedProgramWithSourceV1` exposes the
parser-private generated relation rows for D without an AST/name rescan.
Focused tests cover zero-delegate no-op, later-host atomic failure, generated
name collision, duplicate source rows, staged-vs-actual placement mismatch,
and persisted relation output. Final-seal, resolver, Recipe, Builder, MIR,
provider, and runtime authority remain closed.
