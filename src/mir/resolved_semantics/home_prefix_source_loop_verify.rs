//! Validate the source control summary against its retained Recipe inputs.
use super::*;

impl ports::SourceLoopScanSummaryV1 {
    /// Co-seal the SAME source control, expression inventory and exit snapshots
    /// before Recipe assembly. Success alone still grants no Home After.
    pub(crate) fn validate_for_recipe_v1(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<(), &'static str> {
        use ports::SourceLoopPortKindV1 as Port;
        if self.refused || self.ports.iter().any(|port| port.unavailable.is_some()) {
            return Err("source Loop has an unavailable path");
        }
        if self
            .source
            .condition_source_v1(input)
            .is_none_or(|source| !Rc::ptr_eq(source, self.condition.source()))
        {
            return Err("source Loop condition identity");
        }
        self.incoming.as_ref().map_err(|reason| *reason)?;
        let conditions = self
            .bool_expressions
            .as_ref()
            .map_err(|_| "source Loop Bool projection refused")?;
        let body = self.control.as_ref().map_err(|reason| *reason)?;
        let mut roots = Vec::new();
        let mut frames = vec![&self.condition];
        let mut exits = BTreeSet::new();
        let argument_ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|_| "source Loop condition argument ledger")?;
        let condition_statement = self.condition.source().root().statement();
        let mut expected_condition_arguments = BTreeSet::new();
        for observation in self.condition.calls() {
            let original = argument_ledger
                .method_call_source_v1(observation.site().site())
                .ok_or("source Loop condition call source missing")?;
            if observation.owner() != input.owner()
                || original.owner() != input.owner()
                || !observation.is_expression_value()
                || !observation.result_source().is_some()
            {
                return Err("source Loop condition call identity");
            }
            for argument in observation.arguments() {
                let crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Expression {
                    ordinal, site,
                } = argument else { continue; };
                let key = (observation.site().site().clone(), *ordinal);
                if !expected_condition_arguments.insert(key.clone()) {
                    return Err("source Loop condition argument duplicate");
                }
                let mut matching = self.body_expressions.iter().filter_map(|plan| {
                    let frame = plan.argument_frame()?;
                    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                        call, ordinal: found_ordinal, statement,
                    } = frame.source().root() else { return None; };
                    (statement == condition_statement
                        && call.owner() == input.owner()
                        && call.site() == observation.site().site()
                        && *found_ordinal == *ordinal
                        && call.arguments().get(*found_ordinal as usize).is_some_and(|actual| {
                            actual.ordinal() == *found_ordinal && actual.site() == site
                        })
                        && frame.source().nodes().first().is_some_and(|root| root.site() == site)
                        && Rc::ptr_eq(frame.source_loan().plan(), frame.source())
                        && frame.evaluation().reference_coverage_is_exact()
                        && frame.prior_homes() == self.condition.prior_homes()
                        && argument_ledger.method_call_source_v1(call.site())
                            .is_some_and(|canonical| std::sync::Arc::ptr_eq(canonical, call)))
                    .then_some(frame)
                });
                let Some(frame) = matching.next() else {
                    return Err("source Loop condition argument frame missing");
                };
                if matching.next().is_some() {
                    return Err("source Loop condition argument duplicate");
                }
                roots.push(frame.source());
            }
        }
        let mut retained_condition_arguments = BTreeSet::new();
        for plan in self.body_expressions.iter().filter(|plan| plan.argument_frame().is_some()) {
            let frame = plan.argument_frame().ok_or("source Loop condition argument frame")?;
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                call, ordinal, statement,
            } = frame.source().root() else { continue; };
            if statement != condition_statement
                || !self.condition.source().nodes().iter().any(|node| {
                    node.site() == call.site()
                        && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall)
                })
            {
                continue;
            }
            let key = (call.site().clone(), *ordinal);
            if !expected_condition_arguments.contains(&key)
                || !retained_condition_arguments.insert(key)
            {
                return Err("source Loop condition argument identity");
            }
        }
        if retained_condition_arguments != expected_condition_arguments {
            return Err("source Loop condition argument coverage");
        }
        fn visit<'a>(
            body: &'a CheckedSourceLoopBodyV1,
            summary: &'a ports::SourceLoopScanSummaryV1,
            input: ResolvedFunctionLoweringInputV1<'_>,
            roots: &mut Vec<
                &'a Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
            >,
            frames: &mut Vec<&'a Rc<CheckedBoolConditionFrameV1>>,
            exits: &mut BTreeSet<usize>,
        ) -> Result<(), &'static str> {
            for statement in body.statements.iter() {
                match statement {
                    CheckedSourceLoopStatementV1::Return(source) => {
                        if !source.corroborates(input) {
                            return Err("source Loop Exit port identity");
                        }
                    }
                    CheckedSourceLoopStatementV1::OwnedArrayAppend { site, observation } => {
                        let NamedArrayHomeObservationV1::Append {
                            source,
                            call,
                            candidate,
                            availability,
                            prior_homes,
                            ..
                        } = observation
                        else {
                            return Err("source Loop append observation kind");
                        };
                        let ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop append ledger")?;
                        let original = ledger
                            .method_call(call.site())
                            .ok_or("source Loop append source missing")?;
                        if call.owner() != input.owner() || call.site().node() != site.node()
                            || source.binding().owner() != input.owner() || availability.is_err()
                            || source.append_candidate(call).as_ref() != Some(candidate)
                            || original.receiver() != crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(source.binding()))
                            || !matches!(original.arguments(), [argument] if argument.site() == candidate.site())
                            || ledger.resolved_loop_placement(summary.source.site(), call.site()).ok()
                                != Some(Some(crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body)) {
                            return Err("source Loop append identity");
                        }
                        // The append's argument frame owns the candidate result;
                        // frames rooted at the candidate call own its operands.
                        let mut ordinals = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let frame = plan
                                .argument_frame()
                                .ok_or("source Loop append argument frame")?;
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call: argument_call, ordinal, ..
                            } = plan.source().root() else { return Err("source Loop append argument purpose"); };
                            let argument_identity = argument_call.arguments().get(*ordinal as usize)
                                .is_some_and(|argument| argument.ordinal() == *ordinal
                                    && plan.source().nodes().first().is_some_and(|root|
                                        root.site() == argument.site()));
                            let frame_identity = Rc::ptr_eq(frame.source(), plan.source())
                                && Rc::ptr_eq(frame.source_loan().plan(), plan.source())
                                && frame.evaluation().reference_coverage_is_exact();
                            let homes_identity = frame.prior_homes() == prior_homes.as_ref();
                            let append_operand_frame = argument_call.owner() == input.owner()
                                && argument_call.site() == call.site()
                                && *ordinal == 0
                                && ledger.method_call_source_v1(call.site()).is_some_and(
                                    |original| std::sync::Arc::ptr_eq(original, argument_call),
                                )
                                && argument_identity
                                && frame.is_dynamic_carrier_append_v1()
                                && frame.source_loan().dynamic_array_carrier_append_v1()
                                    .is_some_and(|holder| holder == source);
                            let candidate_argument_frame = argument_call.owner() == input.owner()
                                && argument_call.site() == candidate.site()
                                && ledger.method_call_source_v1(candidate.site()).is_some_and(
                                    |original| std::sync::Arc::ptr_eq(original, argument_call),
                                )
                                && argument_identity;
                            let unique_ordinal = ordinals.insert((argument_call.site().clone(), *ordinal));
                            if !((append_operand_frame || candidate_argument_frame) && frame_identity
                                && homes_identity && unique_ordinal)
                            {
                                return Err("source Loop append argument identity");
                            }
                            roots.push(plan.source());
                        }
                    }
                    CheckedSourceLoopStatementV1::Call { site, observation } => {
                        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as ResultClass;
                        let ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop call ledger")?;
                        let original = ledger
                            .method_call(observation.site().site())
                            .ok_or("source Loop call source missing")?;
                        if observation.owner() != input.owner()
                            || observation.statement() != site
                            || observation.is_expression_value()
                            || !matches!(
                                observation.result(),
                                ResultClass::I64 | ResultClass::Bool | ResultClass::Unit
                                    | ResultClass::Handle | ResultClass::Nullable
                            )
                            || (observation.result() == ResultClass::Unit
                                && observation.local_binding().is_some())
                            || (matches!(observation.result(), ResultClass::Handle | ResultClass::Nullable)
                                && observation.local_binding().is_none())
                            || original.owner() != input.owner()
                            || original.arguments().len() != observation.arguments().len()
                            || ledger
                                .resolved_loop_placement(
                                    summary.source.site(),
                                    observation.site().site(),
                                )
                                .ok()
                                != Some(Some(
                                    crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
                                ))
                        {
                            return Err("source Loop call identity");
                        }
                        if let Some((declaration, binding)) = observation.local_binding() {
                            if input.function().declaration_binding(declaration) != Some(binding)
                                || !input
                                    .function()
                                    .expression_source()
                                    .initializers()
                                    .any(|row| {
                                        row.declaration_site() == declaration
                                            && row.binding() == binding
                                            && row.initializer_site()
                                                == Some(observation.site().site())
                                    })
                            {
                                return Err("source Loop call initializer identity");
                            }
                        } else if observation.site().site().node() != site.node() {
                            return Err("source Loop call statement identity");
                        }
                        let mut ordinals = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let frame = plan
                                .argument_frame()
                                .ok_or("source Loop call argument frame")?;
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call, ordinal, ..
                            } = plan.source().root() else { return Err("source Loop call argument purpose"); };
                            let owner_matches = call.owner() == input.owner();
                            let site_matches = call
                                .site()
                                .node()
                                .segments()
                                .starts_with(observation.site().site().node().segments());
                            let ledger_matches = ledger
                                .method_call_source_v1(call.site())
                                .is_some_and(|original| std::sync::Arc::ptr_eq(original, call));
                            let argument_matches = call.arguments().get(*ordinal as usize).is_some_and(|argument| {
                                argument.ordinal() == *ordinal
                                    && plan.source().nodes().first().is_some_and(|root| root.site() == argument.site())
                            });
                            let source_matches = Rc::ptr_eq(frame.source(), plan.source());
                            let loan_matches = Rc::ptr_eq(frame.source_loan().plan(), plan.source());
                            let coverage_exact = frame.evaluation().reference_coverage_is_exact();
                            let prior_homes_match = frame.prior_homes() == observation.prior_homes();
                            let unique_ordinal = ordinals.insert((
                                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    input.owner(),
                                    call.site().clone(),
                                ),
                                *ordinal,
                            ));
                            if !(owner_matches && site_matches && ledger_matches && argument_matches
                                && source_matches && loan_matches && coverage_exact
                                && prior_homes_match && unique_ordinal)
                            {
                                return Err("source Loop call argument identity");
                            }
                            roots.push(plan.source());
                        }
                    }
                    CheckedSourceLoopStatementV1::Expression { site, frame } => {
                        if frame.source().owner() != input.owner()
                            || frame.source().root().statement() != site
                        {
                            return Err("source Loop execution frame identity");
                        }
                        roots.push(frame.source());
                        let argument_ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop expression argument ledger")?;
                        let mut argument_sites = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let argument_frame = plan
                                .argument_frame()
                                .ok_or("source Loop expression argument frame")?;
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call,
                                ordinal,
                                ..
                            } = plan.source().root() else {
                                return Err("source Loop expression argument purpose");
                            };
                            let nested_call = frame
                                .source()
                                .nodes()
                                .iter()
                                .filter(|node| {
                                    node.site() == call.site()
                                        && matches!(node.source(),
                                            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall)
                                })
                                .count()
                                == 1;
                            let argument_identity = call
                                .arguments()
                                .get(*ordinal as usize)
                                .is_some_and(|argument| {
                                    argument.ordinal() == *ordinal
                                        && plan
                                            .source()
                                            .nodes()
                                            .first()
                                            .is_some_and(|root| root.site() == argument.site())
                                });
                            let owner_matches = call.owner() == input.owner();
                            let call_is_nested = call
                                .site()
                                .node()
                                .segments()
                                .starts_with(site.node().segments());
                            let ledger_matches = argument_ledger
                                .method_call_source_v1(call.site())
                                .is_some_and(|original| std::sync::Arc::ptr_eq(original, call));
                            let source_matches = Rc::ptr_eq(argument_frame.source(), plan.source());
                            let loan_matches = Rc::ptr_eq(argument_frame.source_loan().plan(), plan.source());
                            let coverage_exact = argument_frame.evaluation().reference_coverage_is_exact();
                            let unique_site = argument_sites.insert((
                                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    input.owner(),
                                    call.site().clone(),
                                ),
                                *ordinal,
                            ));
                            if !owner_matches
                                || !call_is_nested
                                || !ledger_matches
                                || !nested_call
                                || !argument_identity
                                || !source_matches
                                || !loan_matches
                                || !coverage_exact
                                || !unique_site
                            {
                                return Err("source Loop expression argument identity");
                            }
                            roots.push(plan.source());
                        }
                    }
                    CheckedSourceLoopStatementV1::If {
                        site,
                        bundle,
                        condition,
                        then_body,
                        else_body,
                    } => {
                        if input.function().if_region_bundle(site).ok() != Some(bundle)
                            || condition.source().root().statement() != site
                        {
                            return Err("source Loop If identity");
                        }
                        roots.push(condition.source());
                        frames.push(condition);
                        let ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop condition call ledger")?;
                        for observation in condition.calls() {
                            let original = ledger
                                .method_call(observation.site().site())
                                .ok_or("source Loop condition call source missing")?;
                            if observation.owner() != input.owner()
                                || observation.statement() != site
                                || !observation.is_expression_value()
                                || observation.result_source().is_none()
                                || !matches!(
                                    observation.result(),
                                    crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64
                                        | crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool
                                )
                                || original.owner() != input.owner()
                                || original.arguments().len() != observation.arguments().len()
                                || ledger
                                    .resolved_loop_placement(
                                        summary.source.site(),
                                        observation.site().site(),
                                    )
                                    .ok()
                                    != Some(Some(
                                        crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
                                    ))
                            {
                                return Err("source Loop condition call identity");
                            }
                            for plan in summary.body_expressions.iter().filter(|plan| {
                                plan.argument_frame().is_some()
                                    && plan.source().root().statement() == site
                            }) {
                                let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                    call,
                                    ordinal,
                                    ..
                                } = plan.source().root() else {
                                    return Err("source Loop condition argument purpose");
                                };
                                if call.site() != observation.site().site() {
                                    continue;
                                }
                                let frame = plan
                                    .argument_frame()
                                    .ok_or("source Loop condition argument frame")?;
                                if call.owner() != input.owner()
                                    || ledger
                                        .method_call_source_v1(call.site())
                                        .is_none_or(|source| !std::sync::Arc::ptr_eq(source, call))
                                    || call.arguments().get(*ordinal as usize).is_none_or(
                                        |argument| {
                                            argument.ordinal() != *ordinal
                                                || plan
                                                    .source()
                                                    .nodes()
                                                    .first()
                                                    .is_none_or(|root| root.site() != argument.site())
                                        },
                                    )
                                    || !Rc::ptr_eq(frame.source(), plan.source())
                                    || !Rc::ptr_eq(frame.source_loan().plan(), plan.source())
                                    || !frame.evaluation().reference_coverage_is_exact()
                                    || frame.prior_homes() != observation.prior_homes()
                                {
                                    return Err("source Loop condition argument identity");
                                }
                                roots.push(plan.source());
                            }
                        }
                        let observed_calls: BTreeSet<_> = condition
                            .calls()
                            .iter()
                            .map(|call| call.site().site().clone())
                            .collect();
                        let mut condition_argument_sites = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call,
                                ordinal,
                                ..
                            } = plan.source().root() else {
                                return Err("source Loop condition argument purpose");
                            };
                            if observed_calls.contains(call.site()) {
                                continue;
                            }
                            let argument_frame = plan
                                .argument_frame()
                                .ok_or("source Loop condition argument frame")?;
                            let nested_call = condition
                                .source()
                                .nodes()
                                .iter()
                                .filter(|node| {
                                    node.site() == call.site()
                                        && matches!(node.source(),
                                            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall)
                                })
                                .count()
                                == 1;
                            let argument_identity = call
                                .arguments()
                                .get(*ordinal as usize)
                                .is_some_and(|argument| {
                                    argument.ordinal() == *ordinal
                                        && plan
                                            .source()
                                            .nodes()
                                            .first()
                                            .is_some_and(|root| root.site() == argument.site())
                                });
                            if call.owner() != input.owner()
                                || !call
                                    .site()
                                    .node()
                                    .segments()
                                    .starts_with(site.node().segments())
                                || ledger
                                    .method_call_source_v1(call.site())
                                    .is_none_or(|original| !std::sync::Arc::ptr_eq(original, call))
                                || !nested_call
                                || !argument_identity
                                || !Rc::ptr_eq(argument_frame.source(), plan.source())
                                || !Rc::ptr_eq(argument_frame.source_loan().plan(), plan.source())
                                || !argument_frame.evaluation().reference_coverage_is_exact()
                                || argument_frame.prior_homes() != condition.prior_homes()
                                || !condition_argument_sites.insert((
                                    crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                        input.owner(),
                                        call.site().clone(),
                                    ),
                                    *ordinal,
                                ))
                            {
                                return Err("source Loop condition nested argument identity");
                            }
                            roots.push(plan.source());
                        }
                        visit(then_body, summary, input, roots, frames, exits)?;
                        if let Some(body) = else_body {
                            visit(body, summary, input, roots, frames, exits)?;
                        }
                    }
                    CheckedSourceLoopStatementV1::Loop { site, product } => {
                        if product.source.source.owner() != input.owner()
                            || product.source.source.site() != site
                        {
                            return Err("source Loop child identity");
                        }
                    }
                    CheckedSourceLoopStatementV1::Exit { site, record, port } => {
                        let original = input.function().resolved_exit(
                            &crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(
                                site.clone(),
                            ),
                        );
                        let snapshot = summary
                            .ports
                            .get(*port)
                            .ok_or("source Loop Exit port missing")?;
                        let target = input
                            .function()
                            .loop_region_bundle(summary.source.site())
                            .map_err(|_| "source Loop Exit target missing")?
                            .loop_pair()
                            .region();
                        if !matches!(record.transfer(),
                            crate::mir::resolved_semantics::ResolvedControlTransferV1::Break { target_loop }
                            | crate::mir::resolved_semantics::ResolvedControlTransferV1::Continue { target_loop } if target_loop == target)
                        {
                            return Err("source Loop Exit target identity");
                        }
                        if original != Some(record)
                            || !exits.insert(*port)
                            || !matches!((&snapshot.kind, record.transfer()),
                            (Port::Break(original), crate::mir::resolved_semantics::ResolvedControlTransferV1::Break { .. })
                            | (Port::Continue(original), crate::mir::resolved_semantics::ResolvedControlTransferV1::Continue { .. }) if original == site)
                        {
                            return Err("source Loop Exit port identity");
                        }
                    }
                }
            }
            Ok(())
        }
        visit(body, self, input, &mut roots, &mut frames, &mut exits)?;
        if roots.len() != self.body_expressions.len()
            || roots.iter().any(|source| {
                self.body_expressions
                    .iter()
                    .filter(|plan| Rc::ptr_eq(source, plan.source()))
                    .count()
                    != 1
            })
            || self.body_expressions.iter().any(|plan| {
                roots
                    .iter()
                    .filter(|source| Rc::ptr_eq(source, plan.source()))
                    .count()
                    != 1
            })
        {
            return Err("source Loop execution root census");
        }
        if frames.len() != conditions.len()
            || frames.iter().any(|frame| {
                conditions
                    .iter()
                    .filter(|condition| condition.matches_frame(frame))
                    .count()
                    != 1
            })
        {
            return Err("source Loop Bool frame census");
        }
        let mut backedges = 0;
        for (index, port) in self.ports.iter().enumerate() {
            match port.kind {
                Port::Backedge => backedges += 1,
                _ if exits.contains(&index) => (),
                _ => return Err("source Loop Exit census"),
            }
        }
        if backedges != usize::from(!body.terminated) {
            return Err("source Loop normal port census");
        }
        Ok(())
    }
}
