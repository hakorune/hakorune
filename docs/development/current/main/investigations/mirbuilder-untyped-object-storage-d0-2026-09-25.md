# MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0

Status: Accepted target on 2026-10-19; common acceptance critical path.
Implementation permission: true for the source-backed target below; production coverage is not granted.
Scope: unchanged untyped-field sentinel and three remaining real apps; one common D0.
Authority: [RULES](../../../RULES.md), [stored-slot law](../../../../reference/language/ownership.md#untyped-strong-stored-slots),
[lifecycle](../../../../reference/language/lifecycle.md#fields-and-ordering),
[internal carriers](../../../../reference/language/failure-outcome-relations.md).

## Decision

Preserve the existing dynamic/untyped language with tagged strong stored slots.
Annotation-required rejection changes the unchanged-source acceptance goal;
source-qualified fixed storage alone does not implement dynamic slots.
The earlier ParkedSealed decision is superseded by this accepted common contract.
Representation direction alone never grants production ABI capability.

| Contract | Selected meaning |
| --- | --- |
| Initial state | Explicit internal EmptyField, distinct from null and local UninitializedSlot. Read Faults. Known-empty fields may publish; unknown/pending acquired obligations may not. |
| Ownership | Value tag and source-issued residence disposition are separate. Scalar/null have no child Home; String installs an independently supported immutable value; exact available Home/carrier transfers once; borrowed residence needs exact live support. |
| Constructor | Exact canonical birth parameter-ordinal → field demand and SAME caller capability witness. Reuse Provided relation; no hidden take for arbitrary ordinary formals or aliases. |
| Replacement | Place/RHS once; prepare and validate before commit. Failure preserves old slot/prior owner. Commit installs new payload/disposition before detached old end. Post-commit Fault never rolls back; preserve first Fault/drain outside locks. |
| Reads | Checked I64/Bool/Text/object kind; no default/coercion. Box reads borrow exact slot/holder; overwrite/end invalidates. No implicit share or borrowed self-contained Dynamic result. |
| End | Parent fini, reverse-declaration current owned field ends, native drop. Borrowed anchors live through fini and receive no owning end. Failed construction cleans initialized subset without incomplete parent fini. |

EmptyField read-Fault is an explicit new target decision, not a claim about VM
legacy defaults. String value materialization is not blanket Trivial capability.
Stored borrowed anchors must cover their entire residence, not just a read.
Same-slot borrowed Box replacement rejects unless a source-proved no-op preserves
the SAME residence; no payload-equality shortcut. Unsupported temporary/
self-dependent anchors and ownership cycles reject. String preparation acquires
independent Text residence before commit; handle copies do not acquire it.
Constructor post-install Fault drains fields, never caller/argument double-End.
Read-only worker reviewed these obligations against the existing laws.

## Source and sole consumer connection

| Existing owner | Required extension / independent check |
| --- | --- |
| instance_constructor_semantic/object_definition | Preserve original untyped declaration and constructor ordinal relation; never recover storage from MIR observation. |
| typed_object_plan/storage_inference/canonical_layout + function/object_definition | Select source-declared dynamic slot layout and destruction disposition; retain declared typed/weak profiles. |
| ordinary_new_field_write_claim + home_new_prefix_owned_field_write + ordinary_new_local_commit | Carry exact candidate capability, prepare/commit state, holder support and Normal/Fault cleanup to read/write consumers. |
| published_backend_view/physical_program_json + compiled_entry_contract | Both final consumers independently verify the SAME source/slot/kind/residence, physical write/read and cleanup relation before publication. |
| hako_llvmc_ffi lifecycle object-residence emitter | Extend existing checked residence operations; no backend inference or automatic generic-JSON permission. |
| nyash_kernel typed_object/types + typed_object_store_residence | Extend the SAME object slot storage/guard path with Empty/value tag and independent disposition. Existing install is write-once: replacement needs explicit old-residence detach. Commit coherently; end outside guard. Unsupported profiles reject. |

`dynamic_call_slot_wire` is call transport, not this field ABI. Existing
Null/Fresh/Forward object residence outcomes and Handle(0) are not a complete
dynamic slot representation. Concrete versioned ABI transport must cover these
laws before activation; no parallel object heap, tag-derived Home or fallback.
D0 acceptance authorizes implementation, not execution of an unverified profile.

## Unchanged acceptance and current evidence

- Sentinel `Holder.items = items` uses a direct owning binding, not an ordinary
  alias: transfer its available Array Home once; preserve later supported uses.
  An actual alias cannot silently consume its supporting root Home.
- TreeNode birth left/right use exact provided actual Homes; leaves store explicit
  null, value stores a scalar. Recursive call/result support still needs connection.
- BoxTorrent ContentChunk/BoxTorrentStore/BoxTorrentManifest and JSON
  UserStats/JsonStreamAggregator retain their original untyped fields.
  String formals may install independent immutable values; object fields/Map
  insertion and borrowed results require their separate source support proofs.
- These are necessary common semantics, not a claim that storage fixes every stop.
- Sentinel still fails before artifact/link/runtime; its changing first stop and
  matching binary evidence belong in [the active app card](app-json-stream-aggregator.md).
  Do not infer an I64 field declaration to bypass missing Dynamic execution.
- Keep unchanged sources, expected outputs/exit-7 sentinel and safety negatives.
  Existing suites/focused cases are reused; no new checker/test file/card selected.

## Next implementation

Connect source dynamic slot → verified layout → transactional writes/checked
reads → cleanup → both final consumers → existing C/runtime path. Start within
APP-json-stream-aggregator's common owner, use the unchanged sentinel to verify
that independent field boundary, then continue its Loop/Map connections.
No app rewrite, phase-293x MIR inference revival, Text/Array negative weakening,
or unsupported runtime activation is authorized.

Own unverified JSON Loop Call source changes remain in Git stash with message
`JSON Loop Call source connection pending common storage decision`; no build/test
ran on that half-connection. Restore only when completing its physical consumer.
Historical options/costs and earlier decisions remain in this file's Git history.
