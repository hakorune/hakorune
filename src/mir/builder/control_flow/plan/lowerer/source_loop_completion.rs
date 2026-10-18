//! The SAME invocation executes and completes its original checked source Loop.
use super::PlanLowerer;
use crate::mir::builder::{MirBuilder, normal_callable_loop_source_facts::SourceLoopCondPhysicalInputV1,
    normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
    recursive_child_lowering::RawInvocationChildPortV1};
use crate::mir::ValueId;
use std::{cell::RefCell, rc::Rc};

impl PlanLowerer {
    pub(in crate::mir::builder) fn lower_checked_source_loop_v2(builder: &mut MirBuilder,
        source: &SourceLoopCondPhysicalInputV1<'_, '_>, port: &mut RawInvocationChildPortV1<'_, '_>,
        ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<ValueId, String> {
        let (graph, entry) = super::source_loop_driver::emit(builder, port, source)?;
        let marker = graph.prepare_phi_completion_v2(builder, source, &ledger.borrow())?;
        let product = Rc::clone(&graph.product);
        let result = graph.result.value();
        let claims = ledger.borrow().ordinary_new_claim_ledger().cloned()
            .ok_or_else(|| "[freeze:contract][source-loop/completion/claims]".to_owned())?;
        // Restore both existing projections, then prepare every source locator
        // before consuming the Phi transaction or installing the parent seal.
        ledger.borrow_mut().restore_source_values(entry.values);
        builder.restore_local_binding_state(entry.cache);
        let names = product.source.headers.keys().map(|&binding| {
            Ok((binding, ledger.borrow().source_loop_binding_name(binding,
                &builder.function_state.variable_ctx.variable_map)?.to_owned()))
        }).collect::<Result<Vec<_>, String>>()?;
        let function = builder.function_state.current_function.as_ref()
            .ok_or_else(|| "[freeze:contract][source-loop/completion/function]".to_owned())?;
        let prepared = claims.prepare_source_loop_physical_v2(graph, marker, function)?;
        let role = prepared.commit(builder)?;
        let proof = role.source_proof_v2().expect("source completion retains original proof");
        for (binding, name) in names {
            let point = proof.points().after_point_v2(binding)?;
            ledger.borrow_mut().publish_source_loop_point_v2(builder, &point)?;
            builder.function_state.variable_ctx.variable_map.insert(name, point.relation().values().0);
        }
        Ok(result)
    }
}
