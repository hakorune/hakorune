Status: design_stop__NoConnectedLoopCondSemanticProgramIssuer
Date: 2026-09-23
Task: JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D
Parent: JOINIR-LOOP-M8-LOOPV0-SCANS-S6C
NextCard: none__until_source_recipe_consumer_delete_tuple_is_decided
Implementation permission: false; design-only. No code, fixtures, semantic receipts, physicalization, or caller switch.

# LoopCond exits S6D — bounded replacement design

```text
Decision: determine whether the already-inventoried M8 S6D LoopCond cohort can be one production-replacement slice, or must stay NoSafeSlice.
Source authority + canonical issuer: resolver-owned LoopCond source sites/exits exist; a same-route issuer for typed condition values, portable Recipe keys, and JoinSig continuation is not yet identified.
Non-authority: AST/StmtRef planner recipes, route IDs, identifier names, the GenericLoop carrier issuer, and post-issuance product pairing.
Fail-fast boundary: missing or mismatched source/value/exit/continuation relations stop before Builder allocation, MIR mutation, or artifact publication.
Smallest next slice: inspect one exact S1-accepted LoopCond source candidate end to end and either name source → Facts → Recipe/JoinSig → physical consumer → old-edge deletion, or seal NoSafeSlice.
Non-claims: no new Recipe producer, selector, physicalizer, production switch, M8/M9 completion, or broad Loop cutover is authorized by this D0.
```

## Census boundary

This D0 covers one exact candidate admitted by `LOOP-FAMILY-LOOPCOND-OBSERVATION-S1`: a non-constant loop condition; one conditional body branch; explicit then/else single-exit arms; resolver-confirmed Break and Continue. It includes both predicate source sites, their typed value/effect relations, the loop continuation, the existing production caller and terminal, and the exact old edge that the same bounded series could remove. It excludes other LoopCond variants, nested LoopCond forests, all-19 closure, M9 parity, and whole-loop production selection.

## Evidence already established

- `VerifiedLoopCondBreakContinueSourceProjectionV1` retains owner/frame identity, the loop and branch predicate sites, and resolver-owned Break/Continue exit records. It does not carry typed predicate operands or portable Recipe keys.
- `LoopCondBreakContinueFacts` and its local Recipe retain an AST condition and `StmtRef`-based planner structure. They do not issue portable carrier keys or JoinSig continuation.
- `CallableLoopCondSourceFactsV1` transfers the planner outcome, source forest, source-item relation, and source port to the existing source physical consumer. That product is the current physical route, not a portable semantic-program issuer.
- The existing GenericLoop carrier issuer is route-specific. The LoopCond B3 D2 audit independently found no connected semantic-program issuer for LoopCond carrier/join relations; it cannot be borrowed or paired by owner/shape.

## D0 completion

Resolve only these finite questions:

1. Can an existing resolver/source owner provide typed values, effects, and exact reads for both predicate sites without AST re-inference?
2. Can one existing canonical issuer co-seal those facts with Recipe-owned keys and the matching JoinSig continuation for this LoopCond source owner?
3. Which named production caller reaches the proposed consumer, what is its terminal rejection boundary, and which exact old production edge becomes caller-zero in the same series?

Accept the next implementation slice only if all three answers form one identity-preserving tuple and one finite positive/negative acceptance boundary. Otherwise close this candidate as `NoSafeSlice`, retain the existing compatibility route, return selection to the family scheduler, and do not add a caller-zero S6D producer or another authority layer.
