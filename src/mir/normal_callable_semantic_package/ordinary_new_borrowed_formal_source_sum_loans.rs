//! Read-only source SUM loans. Original source/read identity stays with the
//! producer; these views grant no execution, payload conversion or Home.
use super::*;

/// Source-only classes for exact formal and flow points in one complete
/// original SUM. The retained flow identity prevents these facts from being
/// reused by another callable; they grant no payload conversion or execution.
#[derive(Debug, Clone)]
pub(in crate::mir) struct SourceValueClassFactsV1 {
    owner: FunctionOwnerIdV1,
    flow: std::sync::Arc<
        crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1,
    >,
    tagged_dynamic_formals: BTreeSet<BindingRefV1>,
    tagged_dynamic_points: BTreeSet<SourceValueRefV1>,
    signed_i64_formals: BTreeSet<BindingRefV1>,
}
impl SourceValueClassFactsV1 {
    pub(in crate::mir) fn proves_formal(&self, binding: BindingRefV1) -> bool {
        binding.owner() == self.owner && self.tagged_dynamic_formals.contains(&binding)
    }
    pub(in crate::mir) fn proves_signed_i64_formal(&self, binding: BindingRefV1) -> bool {
        binding.owner() == self.owner && self.signed_i64_formals.contains(&binding)
    }
    pub(in crate::mir) fn proves_value(
        &self,
        flow: &std::sync::Arc<
            crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1,
        >,
        value: SourceValueRefV1,
    ) -> bool {
        self.owner == flow.owner()
            && std::sync::Arc::ptr_eq(&self.flow, flow)
            && self.tagged_dynamic_points.contains(&value)
    }
    pub(in crate::mir) fn proves_read(
        &self,
        flow: &std::sync::Arc<
            crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1,
        >,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Option<SourceValueRefV1> {
        if binding.owner() != self.owner || !std::sync::Arc::ptr_eq(&self.flow, flow) {
            return None;
        }
        let value = flow.read(site, binding)?;
        // A formal's direct read is already anchored by the exact Entry
        // origin and the formal's Dynamic SUM. It need not also appear as a
        // derived point: alias and computed reads still require point membership.
        let exact_formal_entry = self.proves_formal(binding)
            && matches!(
                flow.origin(value),
                Some(crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Entry(
                    original,
                )) if *original == binding
            );
        (self.proves_value(flow, value) || exact_formal_entry).then_some(value)
    }
    pub(in crate::mir) fn flow(
        &self,
    ) -> &std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>
    {
        &self.flow
    }
}

impl PreparedBorrowedFormalIngressV1 {
    /// A closed class for every original field writer. No read, residence,
    /// conversion or execution permission is issued by this source fact.
    pub(in crate::mir::normal_callable_semantic_package) fn source_field_value_sum_v1(
        &self,
        claims: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        source: &Rc<FieldSource>,
    ) -> Option<Rc<SourceValueSumV1>> {
        let product = &self.original_formal_seeds;
        if !product.corroborate_original_v1(&self.source_incoming) || !source.same_inventory(claims) { return None; }
        let (original, sum) = product.source_sums.as_ref()?.fields.get(&source.field())?;
        Rc::ptr_eq(original, source).then(|| Rc::clone(sum))
    }
    /// Select an original computed argument purpose, never a legacy literal
    /// caller or an executable entry. The whole source SUM stays independent.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn original_signed_expression_argument_request_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> bool {
        fn original_expression(
            prepared: &PreparedBorrowedFormalIngressV1,
            seed: &OriginalFormalActualSeedV1,
            owner: FunctionOwnerIdV1,
            site: &SourceExprSiteV1,
            formal_signed: bool,
        ) -> bool {
            match seed {
                OriginalFormalActualSeedV1::Binary {
                    owner: original,
                    source,
                    ..
                } => *original == owner && source.site() == site && formal_signed,
                OriginalFormalActualSeedV1::MethodResult(source) => {
                    let method = source.method();
                    method.owner() == owner && method.site() == site
                        && prepared.ordinary_core_source_v1(&OwnedExprSiteV1::new(owner, site.clone()))
                            .is_some_and(|core| std::sync::Arc::ptr_eq(method, core.source().source())
                                && core.source().target().schema()
                                    == crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::StringBoxText
                                && core.source().target().result()
                                    == crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller)
                }
                OriginalFormalActualSeedV1::SourceOnly(source)
                | OriginalFormalActualSeedV1::SumUnknown(source) => {
                    original_expression(prepared, source, owner, site, formal_signed)
                }
                _ => false,
            }
        }
        let product = &self.original_formal_seeds;
        if !product.corroborate_original_v1(&self.source_incoming) {
            return false;
        }
        let Some(sums) = &product.source_sums else {
            return false;
        };
        let mut current = self.source_incoming.source_rows_v1().filter(|row| &row.call == call);
        let Some(selected) = current.next() else { return false; };
        if current.next().is_some() || selected.source.instance().is_none() { return false; }
        product
            .rows
            .iter()
            .filter(|row| &row.incoming.call == call
                && row.incoming.source.same_original_source_v1(&selected.source))
            .any(|row| {
                row.incoming
                    .arguments
                    .iter()
                    .zip(&row.seeds)
                    .any(|((_, site, formal), seed)| {
                        original_expression(self, seed, call.owner(), site,
                            sums.values.get(formal).is_some_and(|sum| sum.signed_only()))
                    })
            })
    }
}

fn flow_origin_missing(
    sums: &OriginalSourceSumsV1,
    owner: &FunctionOwnerIdV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
) -> bool {
    sums.flows
        .get(owner)
        .is_none_or(|flow| flow.origin(value).is_none())
}

impl PreparedBorrowedFormalIngressV1 {
    /// Complete source alternatives on Normal, never entry or execution permission.
    pub(in crate::mir::normal_callable_semantic_package) fn source_value_sum_v1(
        &self,
        formal: BindingRefV1,
    ) -> Option<&SourceValueSumV1> {
        self.original_formal_seeds
            .source_value_sum_v1(&self.source_incoming, formal)
    }
    /// Original class for one exact call argument. A stable formal anchor or
    /// reaching-value point lends only source facts; original argument plans
    /// independently corroborate the site/binding before execution.
    pub(in crate::mir::normal_callable_semantic_package) fn source_actual_value_sum_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
        expected_binding: Option<BindingRefV1>,
    ) -> Option<&SourceValueSumV1> {
        let product = &self.original_formal_seeds;
        if !product.corroborate_original_v1(&self.source_incoming) {
            return None;
        }
        let mut incoming_rows = self
            .source_incoming
            .source_rows_v1()
            .filter(|row| &row.call == call);
        let incoming = incoming_rows.next()?;
        if incoming_rows.next().is_some() {
            return None;
        }
        let target = incoming.source.instance()?;
        if incoming.callee != target.callee_owner()
            || target.call_site() != call
            || target.argument_sites().get(ordinal as usize) != Some(site)
        {
            return None;
        }
        let mut seed_rows = product.rows.iter().filter(|row| &row.incoming.call == call);
        let seed_row = seed_rows.next()?;
        if seed_rows.next().is_some()
            || seed_row.incoming.callee != incoming.callee
            || !seed_row.incoming.source.same_original_source_v1(&incoming.source)
            || seed_row.incoming.arguments != incoming.arguments
        {
            return None;
        }
        // Incoming retains only borrowed formals, so its position is not the
        // source argument ordinal when typed formals precede this argument.
        let mut actuals = seed_row.incoming.arguments.iter().enumerate()
            .filter(|(_, (actual_ordinal, _, _))| *actual_ordinal == ordinal);
        let (position, (_, actual_site, _)) = actuals.next()?;
        if actuals.next().is_some() || actual_site != site {
            return None;
        }
        // Formal anchors deliberately retain their original Forward seed,
        // rather than replacing it with a reaching-value PointValue. Lend
        // that same closed formal SUM only for its exact original read.
        let mut seed = seed_row.seeds.get(position)?;
        while let OriginalFormalActualSeedV1::SourceOnly(original) = seed { seed = original; }
        // A resolved reaching value lends only its own point SUM. Prefix gaps
        // may borrow a registered formal class; the retained argument plan
        // still owes the independent exact site/binding correspondence.
        if let OriginalFormalActualSeedV1::Scalar(
            crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer,
        ) = seed {
            let binding = expected_binding?;
            if binding.owner() != call.owner() {
                return None;
            }
            let sums = product.source_sums.as_ref()?;
            // A reaching value at this exact site is the only class this lane
            // lends when the flow saw one. Its read must name the binding.
            let read = sums.flows.get(&binding.owner()).and_then(|flow| {
                (flow.owner() == binding.owner() && flow.is_complete())
                    .then(|| flow.read(site, binding))
                    .flatten()
            });
            if let Some(value) = read {
                if flow_origin_missing(sums, &binding.owner(), value) {
                    return None;
                }
                let point = sums.points.get(&(binding.owner(), value))?;
                return point.signed_only().then_some(point);
            }
            // No flow read for this site: only the caller's own registered
            // formal can be borrowed here (a local, literal or field read is
            // not), and its corroborated closed SUM supplies the class. The
            // site-to-binding link is the candidate observation for this exact
            // argument site, never the SUM itself.
            return self.source_value_sum_v1(binding).filter(|sum| sum.signed_only());
        }
        if let OriginalFormalActualSeedV1::Forward { origin, .. } = seed {
            if expected_binding == Some(*origin) && origin.owner() == call.owner() {
                return self.source_value_sum_v1(*origin);
            }
            return None;
        }
        let OriginalFormalActualSeedV1::PointValue {
            owner,
            value: Some(value),
            ..
        } = seed
        else {
            return None;
        };
        if *owner != call.owner() {
            return None;
        }
        let sums = product.source_sums.as_ref()?;
        let flow = sums.flows.get(owner)?;
        if flow.owner() != *owner || !flow.is_complete() || flow.origin(*value).is_none() {
            return None;
        }
        if expected_binding.is_some_and(|binding| {
            binding.owner() != *owner || flow.read(site, binding) != Some(*value)
        }) {
            return None;
        }
        sums.points.get(&(*owner, *value)).map(Rc::as_ref)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn lend_source_value_class_facts_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<std::rc::Rc<SourceValueClassFactsV1>> {
        let product = &self.original_formal_seeds;
        product
            .corroborate_original_v1(&self.source_incoming)
            .then_some(())?;
        let sums = product.source_sums.as_ref()?;
        let flow = std::sync::Arc::clone(sums.flows.get(&owner)?);
        if flow.owner() != owner || !flow.is_complete() {
            return None;
        }
        let tagged_dynamic_formals: BTreeSet<BindingRefV1> = sums
            .values
            .iter()
            .filter_map(|(binding, sum)| {
                (binding.owner() == owner && sum.tagged_dynamic_only()).then_some(*binding)
            })
            .collect();
        let tagged_dynamic_points: BTreeSet<SourceValueRefV1> = sums
            .points
            .iter()
            .filter_map(|((point_owner, value), sum)| {
                (*point_owner == owner && sum.tagged_dynamic_only()).then_some(*value)
            })
            .collect();
        let signed_i64_formals: BTreeSet<BindingRefV1> = sums
            .values
            .iter()
            .filter_map(|(binding, sum)| {
                (binding.owner() == owner && sum.signed_only()).then_some(*binding)
            })
            .collect();
        let has_dynamic_points =
            !tagged_dynamic_formals.is_empty() && !tagged_dynamic_points.is_empty();
        (!signed_i64_formals.is_empty() || has_dynamic_points).then(|| {
            std::rc::Rc::new(SourceValueClassFactsV1 {
                owner,
                flow,
                tagged_dynamic_formals,
                tagged_dynamic_points,
                signed_i64_formals,
            })
        })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn lend_current_scalar_sum_v1(
        &self,
        read: &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarReadV1,
    ) -> Option<Rc<SourceValueSumV1>> {
        let product = &self.original_formal_seeds;
        let flow = product
            .source_sums
            .as_ref()?
            .flows
            .get(&read.binding().owner())?;
        read.point_for_original_flow_v1(flow)?;
        lend_original_point_sum(
            product,
            &self.source_incoming,
            flow,
            read.read(),
            read.binding(),
        )
    }
    /// Lend the SAME complete original input census without choosing a protocol.
    /// Known nonfinite alternatives are retained; missing roots remain unselected.
    pub(in crate::mir::normal_callable_semantic_package) fn lend_original_input_sums_v1(
        &self,
        parameters: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1],
    ) -> Option<Vec<Rc<SourceValueSumV1>>> {
        parameters
            .iter()
            .filter(|row| row.kind.is_ordinary_borrowed_handle())
            .map(|row| self.lend_source_value_sum_v1(row.binding))
            .collect()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn lend_source_value_sum_v1(
        &self,
        formal: BindingRefV1,
    ) -> Option<Rc<SourceValueSumV1>> {
        self.original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
            .then_some(())?;
        self.original_formal_seeds
            .source_sums
            .as_ref()?
            .values
            .get(&formal)
            .map(Rc::clone)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn closed_core_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<&Rc<ClosedOriginalCoreSourceV1>> {
        self.original_formal_seeds
            .closed_core_source_v1(&self.source_incoming, site)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn ordinary_core_source_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Option<&Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>{
        self.original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
            .then_some(())?;
        self.original_formal_seeds.ordinary_core.get(site)
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
enum OriginalIntegerBasisV1 {
    /// Issued only after exact declaration, ordinal and whole-forest stability checks.
    Declared {
        binding: BindingRefV1,
        abi: crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1,
    },
    Incoming {
        binding: BindingRefV1,
        origin: BindingRefV1,
        current_i64: bool,
    },
    IncomingSourceSum {
        binding: BindingRefV1,
        origin: BindingRefV1,
        sum: Rc<SourceValueSumV1>,
    },
    PointSourceSum {
        binding: BindingRefV1,
        site: SourceExprSiteV1,
        flow: std::sync::Arc<
            crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1,
        >,
        sum: Rc<SourceValueSumV1>,
    },
}
impl OriginalIntegerBasisV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn binding(
        &self,
    ) -> BindingRefV1 {
        match self {
            Self::Declared { binding, .. }
            | Self::Incoming { binding, .. }
            | Self::IncomingSourceSum { binding, .. }
            | Self::PointSourceSum { binding, .. } => *binding,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn i64_source(
        &self,
    ) -> bool {
        match self {
            Self::Declared { abi, .. } => abi.is_i64(),
            Self::Incoming { current_i64, .. } => *current_i64,
            Self::IncomingSourceSum { sum, .. } | Self::PointSourceSum { sum, .. } => {
                sum.signed_only()
            }
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn ready(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
    ) -> bool {
        match self {
            Self::Declared { .. } => true,
            Self::Incoming {
                origin,
                current_i64,
                ..
            } => {
                source.candidate_integer_agreement(*origin)
                    && (!*current_i64 || source.candidate_i64_source_agreement(*origin))
            }
            Self::PointSourceSum {
                binding,
                site,
                flow,
                sum,
            } => {
                sum.signed_only()
                    && lend_original_point_sum(
                        &source.original_formal_seeds,
                        &source.source_incoming,
                        flow,
                        site,
                        *binding,
                    )
                    .is_some_and(|current| Rc::ptr_eq(sum, &current))
            }
            Self::IncomingSourceSum { origin, sum, .. } => {
                sum.signed_only()
                    && source
                        .lend_source_value_sum_v1(*origin)
                        .is_some_and(|current| Rc::ptr_eq(sum, &current))
            }
        }
    }
}

impl OriginalBorrowedFormalSourceV1 {
    /// Source agreement plus the original stable use, not entry agreement alone.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn integer_origin_at_v1(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Option<OriginalIntegerBasisV1> {
        if !self
            .seeds
            .new_calls
            .current_provider_sources_match(constructors)
        {
            return None;
        }
        let binding = arg_site_binding(input, site)?;
        if value_domain::declared_integer_seed_v1(input, contracts, binding) {
            let abi = contracts
                .iter()
                .find(|row| row.owner == input.owner())?
                .parameters
                .iter()
                .find(|row| row.binding == binding)
                .and_then(|row| match row.kind {
                    CallableParameterContractKindV1::ExactTrivial(abi) => Some(abi),
                    _ => None,
                })?;
            return Some(OriginalIntegerBasisV1::Declared { binding, abi });
        }
        let origin = definitions
            .get(&input.owner())
            .and_then(|draft| draft.origins.get(&binding))
            .copied()
            .or_else(|| new_seeds::stable_constructor_formal_v1(input, site, constructors));
        let Some(origin) = origin else {
            if !super::super::super::actual_seed::uncaptured_source_binding_v1(input, binding) {
                return None;
            }
            let flow = input.function().expression_source().value_flow()?;
            let sum = lend_original_point_sum(&self.seeds, &self.inventory, flow, site, binding)?;
            return sum
                .signed_only()
                .then(|| OriginalIntegerBasisV1::PointSourceSum {
                    binding,
                    site: site.clone(),
                    flow: std::sync::Arc::clone(flow),
                    sum,
                });
        };
        if self
            .seeds
            .candidate_integer_agreement_v1(&self.inventory, origin)
        {
            return Some(OriginalIntegerBasisV1::Incoming {
                binding,
                origin,
                current_i64: self
                    .seeds
                    .candidate_i64_source_agreement_v1(&self.inventory, origin),
            });
        }
        // Borrow complete source alternatives, without changing the legacy
        // transport projection or granting an executable entry to this use.
        self.seeds
            .corroborate_original_v1(&self.inventory)
            .then_some(())?;
        let sum = self.seeds.source_sums.as_ref()?.values.get(&origin)?;
        sum.signed_only()
            .then(|| OriginalIntegerBasisV1::IncomingSourceSum {
                binding,
                origin,
                sum: Rc::clone(sum),
            })
    }
}

/// One source-only lender for original field bases and current scalar reads.
/// The original flow/read and incoming census stay bound to the SAME solver.
fn lend_original_point_sum(
    product: &OriginalFormalSeedProductV1,
    inventory: &BorrowedIncomingInventoryV1,
    flow: &std::sync::Arc<
        crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1,
    >,
    site: &SourceExprSiteV1,
    binding: BindingRefV1,
) -> Option<Rc<SourceValueSumV1>> {
    product.corroborate_original_v1(inventory).then_some(())?;
    let sums = product.source_sums.as_ref()?;
    let owner = binding.owner();
    if flow.owner() != owner
        || !flow.is_complete()
        || !std::sync::Arc::ptr_eq(flow, sums.flows.get(&owner)?)
    {
        return None;
    }
    sums.points
        .get(&(owner, flow.read(site, binding)?))
        .map(Rc::clone)
}
