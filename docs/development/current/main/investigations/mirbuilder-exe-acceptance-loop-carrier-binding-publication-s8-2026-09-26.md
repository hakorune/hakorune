# MIRBUILDER-EXE-ACCEPTANCE-LOOP-CARRIER-BINDING-PUBLICATION-S8

Status: open
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  MERGE-PHI-DOMINANCE-D20 decision accepted
Mode: fast — one responsibility, one production edge.

## Responsibility

Publish loop-carrier phis through the **same binding authority the
lowering reads actually resolve**. Carrier header/step/after phis
are already emitted and patched by
`LoopCondBreakContinuePhiMaterializer` + the `loop_lowering` phi
lifecycle, but loop-header/body/exit reads resolve through the
binding-id SSA store (`binding_ctx` / `MirBindingSsaAdapterV1`)
rather than the `variable_map` entries `finalize_loop_variables`
publishes — so the phi dsts are dead on arrival and
`prune_unused_phi_instructions`/DCE strips them.

## Boundary

- One bounded question first: which resolver do carrier reads use
  in the failing lane — `build_variable_access`/`variable_map` or
  `binding_ctx` BindingId→value — and where does carrier
  publication need to land for header (body reads), after (exit
  reads), and step/backedge transport.
- Both lanes converge on `lower_loop_generalized`; the fix lives in
  the shared binding publication, not in either entry port.
- No verifier change, no dialect change, no optimizer touch, no
  route addition or removal.
- The `CarriersOnly` edge-args transport stays the declared edge
  vocabulary; populated values are the missing wiring, not a new
  layout.

## Acceptance

1. Focused negative pin: minimal single-carrier `loop(cond)` with a
   post-loop read — emitted exit read binds the after-phi dst, not
   the body's last-writer.
2. Focused negative pin: header phi inputs are
   {preheader: init, backedge: step/carried}, and body reads bind
   the header phi dst — not the pre-loop value.
3. `find/3`-shape: the in-body carrier update is transported (no
   dead update statement, non-empty carrier transport on the
   backedge/step path).
4. `NYASH_BIN=target/debug/hakorune`
   `hakorune --backend mir --emit-mir-json
   apps/json-stream-aggregator/main.hako` — the 13-violation SSA
   family closes or the residual narrows to a named next family.
5. Pointer + workstream guards green; receipts/docs synced.

## Non-claims

- No claim this closes overall MirBuilder; Gate 1 evidence only.
- No verifier weakening, no LoopBuilder revival, no env-gated
  no-phi dialect.
- No new Recipe/authority minted: the binding publication wires
  into the existing read resolver, whichever the census selects.

## Evidence

(to be filled on land)
