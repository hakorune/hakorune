//! Match exact Loop exit responsibilities against verified source scopes.
use super::*;

/// A new fallible End may use the ordinary Home suffix only when every
/// current Text carrier has no responsibility or its exact Home is present.
pub(super) fn lexical_fault_carriers_covered(
    carriers: &BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
    homes: &[LiveHomeV1],
    transferred_residences: bool,
) -> bool {
    carriers.values().all(|state| {
        let has_home = |acquisition: &crate::mir::resolved_semantics::home_new_prefix::TextResidenceAcquisitionV1| {
            homes.contains(&LiveHomeV1::Text(acquisition.clone()))
        };
        match &state.producer {
            TextProducer::Static { .. } => true,
            TextProducer::Residence(acquisition) => transferred_residences || has_home(acquisition),
            TextProducer::Header(basis) => match basis.entry_producer() {
                SourceProducer::Text(TextProducer::Residence(acquisition)) => has_home(acquisition),
                _ => false,
            },
            TextProducer::After { .. } | TextProducer::BranchNormal(_) => false,
        }
    })
}

pub(super) fn normal_port(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &Summary,
    port: usize,
    homes: &[LiveHomeV1],
    text_carriers: &BTreeMap<
        crate::mir::resolved_semantics::BindingRefV1,
        SourceLoopTextCarrierStateV2,
    >,
) -> Result<(), String> {
    let port = source
        .ports
        .get(port)
        .ok_or_else(|| reject("execution-normal-port"))?;
    if port.homes() != homes {
        return Err(reject("execution-port-homes"));
    }
    let mut unique = BTreeSet::new();
    let mut ends = Vec::new();
    let text_bindings: BTreeSet<_> = source
        .headers
        .iter()
        .filter_map(|(binding, basis)| {
            (basis.entry_producer().class() == Class::String).then_some(*binding)
        })
        .collect();
    if text_carriers.keys().copied().collect::<BTreeSet<_>>() != text_bindings {
        return Err(reject("execution-text-carrier-census"));
    }
    for home in homes.iter().rev() {
        let binding = input
            .function()
            .binding(home.binding())
            .ok_or_else(|| reject("execution-home-binding"))?;
        if !unique.insert(home.binding()) {
            return Err(reject("execution-home-duplicate"));
        }
        if !text_bindings.contains(&home.binding())
            && port.crossed_scopes().contains(&binding.owner_scope())
        {
            ends.push(home.clone());
        }
    }
    let retained: Vec<_> = homes
        .iter()
        .cloned()
        .filter(|home| !text_bindings.contains(&home.binding()) && !ends.contains(home))
        .collect();
    let prior: Vec<_> = source
        .condition
        .prior_homes()
        .iter()
        .filter(|home| !text_bindings.contains(&home.binding()))
        .cloned()
        .collect();
    if ends.as_slice()
        != port
            .normal_ends()
            .iter()
            .filter(|home| !text_bindings.contains(&home.binding()))
            .cloned()
            .collect::<Vec<_>>()
            .as_slice()
        || retained != prior
    {
        return Err(reject("execution-normal-home-closure"));
    }
    verify_text_carrier_port(input, source, &port.kind, text_carriers, homes)?;
    let function = input.function();
    if !ends.is_empty()
        && port
            .crossed_scopes()
            .iter()
            .any(|&scope| function.program_body_root(scope).is_some())
        && !lexical_fault_carriers_covered(text_carriers, homes, true)
    {
        return Err(reject("execution-lexical-fault-carrier"));
    }
    Ok(())
}

fn verify_text_carrier_port(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &Summary,
    port: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopPortKindV1,
    text_carriers: &BTreeMap<
        crate::mir::resolved_semantics::BindingRefV1,
        SourceLoopTextCarrierStateV2,
    >,
    homes: &[LiveHomeV1],
) -> Result<(), String> {
    use crate::mir::resolved_semantics::home_new_prefix::SourceLoopPortKindV1 as Port;
    use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
    let (edge, after) = match port {
        Port::Backedge => (Edge::Backedge, false),
        Port::Break(site) => (Edge::Break(site.clone()), true),
        Port::Continue(site) => (Edge::Continue(site.clone()), false),
    };
    let incoming = source.incoming.as_ref().map_err(|reason| reject(reason))?;
    for (binding, state) in text_carriers {
        let mut rows = incoming
            .iter()
            .filter(|row| row.basis.binding() == *binding && Rc::ptr_eq(&row.basis, &state.basis))
            .flat_map(|row| {
                if after {
                    row.after_incoming().iter()
                } else {
                    row.header_incoming().iter()
                }
            })
            .filter(|row| row.edge == edge);
        let row = rows
            .next()
            .ok_or_else(|| reject("execution-text-edge-missing"))?;
        if rows.next().is_some() {
            return Err(reject("execution-text-edge-duplicate"));
        }
        let SourceProducer::Text(expected) = &row.producer else {
            return Err(reject("execution-text-edge-class"));
        };
        let matches = state.producer == *expected
            || matches!(&state.producer,
            TextProducer::Header(basis) if Rc::ptr_eq(basis, &state.basis)
                && state.basis.entry_producer() == &SourceProducer::Text(expected.clone()));
        if !matches {
            return Err(reject("execution-text-edge-producer"));
        }
        let carrier_homes: Vec<_> = homes
            .iter()
            .filter(|home| home.binding() == *binding)
            .collect();
        match &state.producer {
            TextProducer::Residence(acquisition) => {
                let entry = match state.basis.entry_producer() {
                    SourceProducer::Text(TextProducer::Residence(entry)) => Some(entry),
                    _ => None,
                };
                if entry.is_some_and(|entry| acquisition.same_source(&entry)) {
                    if carrier_homes.len() != 1
                        || *carrier_homes[0] != LiveHomeV1::Text(acquisition.clone())
                    {
                        return Err(reject("execution-text-entry-home"));
                    }
                } else if !carrier_homes.is_empty() {
                    return Err(reject("execution-text-transferred-home"));
                }
            }
            TextProducer::Header(_) => match state.basis.entry_producer() {
                SourceProducer::Text(TextProducer::Residence(entry)) => {
                    if carrier_homes.len() != 1
                        || *carrier_homes[0] != LiveHomeV1::Text(entry.clone())
                    {
                        return Err(reject("execution-text-header-home"));
                    }
                }
                SourceProducer::Text(TextProducer::Static { .. }) if carrier_homes.is_empty() => {}
                SourceProducer::Text(entry @ TextProducer::After { .. })
                    if carrier_homes.is_empty()
                        && text_after_entry_matches(
                            input,
                            *binding,
                            state.basis.entry(),
                            entry,
                        ) => {}
                _ => return Err(reject("execution-text-header-entry")),
            },
            TextProducer::After { .. } => return Err(reject("execution-text-after-active")),
            TextProducer::Static { .. } if carrier_homes.is_empty() => {}
            TextProducer::Static { .. } => return Err(reject("execution-text-static-home")),
            TextProducer::BranchNormal(source)
                if source.binding() == *binding
                    && source.is_owned_residence_tree_v1()
                    && carrier_homes.is_empty() => {}
            TextProducer::BranchNormal(_) => return Err(reject("execution-text-edge-producer")),
        }
    }
    Ok(())
}

fn text_after_entry_matches(
    input: ResolvedFunctionLoweringInputV1<'_>,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    producer: &TextProducer,
) -> bool {
    use crate::mir::resolved_semantics::source_value_flow::SourceValueJoinKindV1 as Join;
    let TextProducer::After {
        product,
        binding: source_binding,
    } = producer
    else {
        return false;
    };
    if *source_binding != binding || product.source.source.owner() != input.owner() {
        return false;
    }
    let Some(basis) = product.source.headers.get(&binding) else {
        return false;
    };
    if basis.entry_producer().class() != Class::String {
        return false;
    }
    let Some(flow) = input.function().expression_source().value_flow() else {
        return false;
    };
    let Some((after, _)) = flow.loop_join(product.source.source.site(), binding, Join::LoopAfter)
    else {
        return false;
    };
    if after != value {
        return false;
    }
    let Ok(incoming) = &product.source.incoming else {
        return false;
    };
    let mut rows = incoming.iter().filter(|row| {
        row.after == after && row.basis.binding() == binding && Rc::ptr_eq(&row.basis, basis)
    });
    rows.next().is_some() && rows.next().is_none()
}
