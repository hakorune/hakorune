//! Text carriers that disagree across an `if` join keep both ordered producers.
use super::*;

impl<'source> PrefixLocalFlow<'source> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn branch_text_join_v1(
        &self, other: &Self, binding: BindingRefV1, site: &SourceStmtSiteV1,
    ) -> Option<std::rc::Rc<BranchNormalTextSourceV1>> {
        let debug = crate::config::env::builder_debug_enabled();
        let flow = self.input.function().expression_source().value_flow()?;
        if flow.owner() != self.input.owner() || other.input.owner() != self.input.owner() {
            return None;
        }
        let Some((value, incoming)) = flow.branch_join(site, binding) else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=source-join-missing", self.input.owner()); }
            return None;
        };
        let [(then_edge, then_value), (else_edge, else_value)] = incoming else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=two-normal-arms-missing arms={incoming:?}", self.input.owner()); }
            return None;
        };
        use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
        if !matches!((then_edge, else_edge), (Edge::ThenNormal, Edge::ElseNormal)) {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=arm-order-mismatch arms={then_edge:?}/{else_edge:?}", self.input.owner()); }
            return None;
        }
        let Some(then_producer) = self.branch_text_producer_v1(binding, *then_value, flow) else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=then-producer-missing local={}", self.input.owner(), stored_local_kind(self.locals.get(&binding))); }
            return None;
        };
        let Some(else_producer) = other.branch_text_producer_v1(binding, *else_value, flow) else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=else-producer-missing local={}", self.input.owner(), stored_local_kind(other.locals.get(&binding))); }
            return None;
        };
        let result = BranchNormalTextSourceV1::new(
            std::sync::Arc::clone(flow), value, binding, site.clone(),
            Box::new([
                (then_edge.clone(), *then_value, then_producer),
                (else_edge.clone(), *else_value, else_producer),
            ]),
        );
        if result.is_none() && debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=source-witness-mismatch", self.input.owner()); }
        result
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn branch_text_producer_v1(
        &self, binding: BindingRefV1, value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
        flow: &std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
    ) -> Option<SourceLoopTextProducerV1> {
        match self.locals.get(&binding)? {
            StoredLocal::StaticText { definition } => {
                let producer = self.text_producer_for_value_v1(binding, value, flow, &[])
                    .map(|(_, producer)| producer);
                if producer.is_none() && crate::config::env::builder_debug_enabled() {
                    let basis = self.source_loop_header_basis_v1(binding);
                    let header = basis.and_then(|basis| flow.loop_join(
                        basis.source().site(), binding,
                        crate::mir::resolved_semantics::source_value_flow::SourceValueJoinKindV1::LoopHeader,
                    ));
                    eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} value={value:?} local=StaticText definition={definition:?} origin={:?} active_header={} header_value_matches={} source_header_matches={}",
                        self.input.owner(), flow.origin(value), basis.is_some(),
                        basis.is_some_and(|basis| basis.header() == value),
                        header.is_some_and(|(header, _)| header == value));
                }
                producer
            }
            StoredLocal::Text { root, acquisition }
                if *root == binding && acquisition.binding() == binding
                    && acquisition.is_text_residence() && acquisition.owns_root()
                    && acquisition.source().owner() == flow.owner() => {
                let definition = acquisition.source().nodes().first()?.site();
                let origin = flow.alias_origin_with_expression_boundary(
                    value,
                    |site| self.input.function().variable_ref(site),
                    |site| site == definition,
                )?;
                matches!(flow.origin(origin), Some(crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(site)) if site == definition)
                    .then(|| SourceLoopTextProducerV1::Residence(acquisition.clone()))
            }
            StoredLocal::TextBranch { source }
                if source.matches_source(flow, binding, value) =>
            {
                Some(SourceLoopTextProducerV1::BranchNormal(std::rc::Rc::clone(source)))
            }
            _ => None,
        }
    }
}
