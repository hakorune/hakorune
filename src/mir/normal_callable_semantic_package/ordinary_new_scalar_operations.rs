//! Same scalar source evidence joined to its original ordinary Binary append.
//! This is physical correspondence, not execution or derived-reuse authority.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ExpressionScalarOperationLoanV1 as Source;
use crate::mir::{BasicBlockId, BinaryOp, ConstValue, MirFunction, MirInstruction, ValueId};

use crate::mir::builder::LocalSsaPhysicalRelationV1;

type Binding = (BasicBlockId, MirInstruction);

pub(super) type ScalarConsumerObservationV1 =
    (Rc<OriginalScalarAppendV1>, Rc<LocalSsaPhysicalRelationV1>);

#[derive(Debug)]
pub(in crate::mir) struct OriginalScalarAppendV1 {
    source: Rc<Source>,
    children: (ValueId, ValueId),
    original: Binding,
    operand_relations: [Rc<LocalSsaPhysicalRelationV1>; 2],
    divisor: Option<Binding>,
}

impl OriginalScalarAppendV1 {
    #[cfg(test)]
    pub(in crate::mir) fn fixture_for_correspondence_test(
        builder: &mut crate::mir::builder::MirBuilder,
        left: ValueId,
    ) -> Rc<Self> {
        let (ledger, sources) = Self::source_inventory_for_correspondence_test();
        let source = sources.into_iter().next().unwrap();
        let operator = match source_operator(&source).unwrap() {
            BinaryOp::Sub => crate::ast::BinaryOperator::Subtract,
            BinaryOp::Div => crate::ast::BinaryOperator::Divide,
            _ => unreachable!("scalar source fixture"),
        };
        let right = crate::mir::builder::emission::constant::emit_integer(builder, 4).unwrap();
        let (completed, relations) = builder
            .build_source_scalar_from_values_recorded(operator, left, right)
            .unwrap();
        let entry = builder.current_block_for_test().unwrap();
        let mut function = MirFunction::new(
            crate::mir::FunctionSignature {
                name: "correspondence/0".into(),
                params: vec![],
                return_type: crate::mir::MirType::Integer,
                effects: crate::mir::EffectMask::PURE,
            },
            entry,
        );
        function.blocks.get_mut(&entry).unwrap().instructions =
            builder.current_function_instructions();
        ledger
            .record_scalar_operation_append_v1(
                source,
                (left, right),
                relations,
                &completed,
                builder.local_ssa_physical_correspondence_v1(),
                &function,
            )
            .unwrap()
    }

    #[cfg(test)]
    pub(in crate::mir) fn source_inventory_for_correspondence_test(
    ) -> (Rc<OrdinaryNewClaimLedgerV1>, Vec<Rc<Source>>) {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(include_str!(
            "../../../lang/src/hako_alloc/memory/size_class_box.hako"
        )).unwrap();
        let ledger = Rc::clone(&package.ordinary_new_claim_ledger);
        let sources = package
            .batch()
            .declarations()
            .flat_map(|declaration| {
                ledger
                    .completion_for_owner(declaration.owner())
                    .and_then(|row| row.cleanup().root_flow())
                    .map(|flow| flow.scalar_operations().cloned().collect::<Vec<_>>())
                    .unwrap_or_default()
            })
            .collect();
        (ledger, sources)
    }

    pub(in crate::mir) fn source(&self) -> &Rc<Source> {
        &self.source
    }
    pub(in crate::mir) fn children(&self) -> (ValueId, ValueId) {
        self.children
    }
    pub(in crate::mir) fn original(&self) -> &Binding {
        &self.original
    }
    pub(in crate::mir) fn operand_relations(&self) -> &[Rc<LocalSsaPhysicalRelationV1>; 2] {
        &self.operand_relations
    }
    /// Expression wrappers consume the same recorded operands and their SSA
    /// dependencies; direct arithmetic equality cannot erase a pinned Copy.
    pub(in crate::mir) fn validate_expression_operands_v1(&self, children: (ValueId, ValueId),
        destination: ValueId, bindings: &[Binding],
    ) -> Result<(), String> {
        if self.children != children || self.original.1.dst_value() != Some(destination)
            || !matches!(self.original.1, MirInstruction::BinOp { op, .. } if Some(op) == self.source.integer_operator_v1())
            || !bindings.contains(&self.original) { return Err(reject("operand-correspondence")); }
        for relation in &self.operand_relations {
            if relation.target_binding().is_some_and(|target| !bindings.contains(target)) { return Err(reject("operand-correspondence")); }
        }
        Ok(())
    }
    pub(in crate::mir) fn divisor(&self) -> Option<&Binding> {
        self.divisor.as_ref()
    }
}

impl OrdinaryNewClaimLedgerV1 {
    fn scalar_operation_sources_v1(&self, owner: FunctionOwnerIdV1) -> Result<Vec<Rc<Source>>, String> {
        let flow = self.completion_for_owner(owner).and_then(|row| row.cleanup().root_flow());
        let mut sources = Vec::<Rc<Source>>::new();
        let mut lend = |source: &Rc<Source>| -> Result<(), String> {
            if source.owner() != owner { return Err(reject("source-membership")); }
            if let Some(prior) = sources.iter().find(|prior| prior.binary().site() == source.binary().site()) {
                if !Rc::ptr_eq(prior, source) { return Err(reject("source-membership")); }
            } else { sources.push(Rc::clone(source)); }
            Ok(())
        };
        for source in flow.into_iter().flat_map(|flow| flow.scalar_operations()) { lend(source)?; }
        for evaluation in flow.into_iter().flat_map(|flow| flow.expression_plans())
            .filter_map(|plan| plan.initializer_frame()).map(|frame| frame.evaluation())
            .chain(self.terminal_relations_for_owner(owner).into_iter().filter_map(|row| match row {
                TerminalRelationV1::I64Scalar(row) => row.evaluation().map(|frame| frame.as_ref()), _ => None,
            })) {
            for operation in evaluation.operations() {
                if let Some(source) = operation.scalar_source_v1() {
                    if !Rc::ptr_eq(source.plan(), evaluation.source()) { return Err(reject("source-membership")); }
                    lend(source)?;
                }
            }
        }
        Ok(sources)
    }
    pub(in crate::mir) fn has_scalar_operation_source_v1(&self, owner: FunctionOwnerIdV1) -> Result<bool, String> {
        self.scalar_operation_sources_v1(owner).map(|sources| !sources.is_empty())
    }

    pub(in crate::mir) fn prepare_scalar_operation_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1,
        operator: Option<BinaryOp>,
    ) -> Result<Option<Rc<Source>>, String> {
        if site.owner() != owner {
            return Err(reject("owner"));
        }
        let sources = self.scalar_operation_sources_v1(owner)?;
        let mut rows = sources.iter().filter(|row| row.binary().site() == site.site());
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some() || row.owner() != owner {
            return Err(reject("source-membership"));
        }
        if Some(source_operator(row)?) != operator {
            return Err(reject("operator"));
        }
        Ok(Some(Rc::clone(row)))
    }

    /// The already recorded append of this exact source law. No MIR search or
    /// equal-shaped source can substitute for the original affine receipt.
    pub(in crate::mir) fn scalar_operation_original_v1(&self, source: &Rc<Source>) -> Result<Rc<OriginalScalarAppendV1>, String> {
        let site = OwnedExprSiteV1::new(source.owner(), source.binary().site().clone());
        let rows = self.scalar_operation_appends.borrow();
        let row = rows.get(&site).ok_or_else(|| reject("append-missing"))?;
        if !Rc::ptr_eq(row.source(), source) { return Err(reject("source-membership")); }
        Ok(Rc::clone(row))
    }

    pub(in crate::mir) fn record_scalar_operation_append_v1(
        &self,
        source: Rc<Source>,
        children: (ValueId, ValueId),
        relations: [Rc<LocalSsaPhysicalRelationV1>; 2],
        completed: &crate::mir::builder::ops::CompletedOrdinaryBinaryV1,
        index: &crate::mir::builder::LocalSsaPhysicalIndexV1,
        function: &MirFunction,
    ) -> Result<Rc<OriginalScalarAppendV1>, String> {
        let site = OwnedExprSiteV1::new(source.owner(), source.binary().site().clone());
        let current = self
            .prepare_scalar_operation_source_v1(
                source.owner(),
                &site,
                Some(source_operator(&source)?),
            )?
            .ok_or_else(|| reject("source-missing"))?;
        if !Rc::ptr_eq(&source, &current) {
            return Err(reject("source-membership"));
        }
        let original = completed
            .arithmetic_original()
            .ok_or_else(|| reject("append-missing"))?;
        let MirInstruction::BinOp { dst, op, lhs, rhs } = &original.1 else {
            return Err(reject("append-shape"));
        };
        if *dst != completed.value()
            || *op != source_operator(&source)?
            || unique_definition(function, *dst)? != *original
        {
            return Err(reject("append-identity"));
        }
        if relations[0].values() != (children.0, *lhs)
            || relations[1].values() != (children.1, *rhs)
        {
            return Err(reject("operand-correspondence"));
        }
        for relation in &relations {
            index.require_member(relation)?;
            relation.verify(function)?;
        }
        let divisor = match source.positive_divisor() {
            Some(expected) => {
                let binding = unique_definition(function, children.1)?;
                if !matches!(&binding.1, MirInstruction::Const { value: ConstValue::Integer(actual), .. } if *actual == expected && expected > 0)
                {
                    return Err(reject("divisor-producer"));
                }
                Some(binding)
            }
            None => None,
        };
        let mut appends = self.scalar_operation_appends.borrow_mut();
        if appends.contains_key(&site) {
            return Err(reject("duplicate-original"));
        }
        let row = Rc::new(OriginalScalarAppendV1 {
            source,
            children,
            original: original.clone(),
            operand_relations: relations,
            divisor,
        });
        appends.insert(site, Rc::clone(&row));
        Ok(row)
    }

    /// Retain the complete same-frame consumer set only after all rows validate.
    pub(in crate::mir) fn record_scalar_operation_consumers_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        index: &crate::mir::builder::LocalSsaPhysicalIndexV1,
    ) -> Result<(), String> {
        let expected = self.scalar_operation_sources_v1(owner)?;
        let originals: Vec<_> = index.scalar_originals().collect();
        if originals.len() != expected.len() {
            return Err(reject("original-coverage"));
        }
        let recorded = self.scalar_operation_appends.borrow();
        let mut sites = BTreeSet::new();
        for original in originals {
            let source = original.source();
            if source.owner() != owner || !expected.iter().any(|row| Rc::ptr_eq(row, source)) {
                return Err(reject("source-membership"));
            }
            let site = OwnedExprSiteV1::new(owner, source.binary().site().clone());
            if !sites.insert(site.clone())
                || !recorded
                    .get(&site)
                    .is_some_and(|row| Rc::ptr_eq(row, original))
            {
                return Err(reject("original-membership"));
            }
            let value = original
                .original()
                .1
                .dst_value()
                .ok_or_else(|| reject("append-shape"))?;
            if unique_definition(function, value)? != *original.original() {
                return Err(reject("append-identity"));
            }
            for relation in original.operand_relations() {
                index.require_member(relation)?;
                relation.verify(function)?;
            }
        }
        let mut targets = BTreeSet::new();
        let mut collected = Vec::new();
        for (original, relation) in index.scalar_observations() {
            let source = original.source();
            let site = OwnedExprSiteV1::new(owner, source.binary().site().clone());
            if source.owner() != owner
                || !recorded
                    .get(&site)
                    .is_some_and(|row| Rc::ptr_eq(row, original))
            {
                return Err(reject("derived-source-membership"));
            }
            index.require_member(relation)?;
            relation.verify(function)?;
            let (from, to) = relation.values();
            if from == to
                || !targets.insert(to)
                || !index
                    .scalar_source_for(from)
                    .is_some_and(|row| Rc::ptr_eq(row, original))
                || !index
                    .scalar_source_for(to)
                    .is_some_and(|row| Rc::ptr_eq(row, original))
            {
                return Err(reject("derived-target"));
            }
            collected.push((Rc::clone(original), Rc::clone(relation)));
        }
        index.validate_scalar_consumer_coverage(&targets)?;
        if expected.is_empty() && collected.is_empty() {
            return Ok(());
        }
        let mut consumers = self.scalar_operation_consumers.borrow_mut();
        if consumers.contains_key(&owner) {
            return Err(reject("duplicate-consumer-handoff"));
        }
        consumers.insert(owner, collected);
        Ok(())
    }

    pub(super) fn scalar_operation_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let mut bindings = Vec::new();
        self.visit_scalar_operation_originals_v1(|original| {
            if original.source().owner() == owner {
                bindings.push(original.original().clone());
                for relation in original.operand_relations() {
                    relation.append_required_bindings(&mut bindings);
                }
            }
            Ok(())
        })?;
        self.visit_scalar_operation_consumers_v1(owner, |_, relation| {
            relation.append_required_bindings(&mut bindings);
            Ok(())
        })?;
        Ok(bindings)
    }

    pub(super) fn verify_finished_scalar_operations_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        mut project: impl FnMut(&Binding) -> Result<Option<Binding>, String>,
    ) -> Result<(), String> {
        self.visit_scalar_operation_originals_v1(|original| {
            if original.source().owner() == owner {
                let mapped = project(original.original())?
                    .ok_or_else(|| reject("mandatory-original-removed"))?;
                let value = mapped.1.dst_value().ok_or_else(|| reject("append-shape"))?;
                if unique_definition(function, value)? != mapped {
                    return Err(reject("finished-original-producer"));
                }
                for relation in original.operand_relations() {
                    relation.verify_projected(function, &mut project)?;
                }
                if let Some(divisor) = original.divisor() {
                    project(divisor)?;
                }
            }
            Ok(())
        })?;
        self.visit_scalar_operation_consumers_v1(owner, |_, relation| {
            relation.verify_projected(function, &mut project)
        })
    }

    pub(in crate::mir) fn visit_scalar_operation_consumers_v1(
        &self,
        owner: FunctionOwnerIdV1,
        mut visit: impl FnMut(
            &Rc<OriginalScalarAppendV1>,
            &Rc<LocalSsaPhysicalRelationV1>,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let consumers = self.scalar_operation_consumers.borrow();
        let Some(rows) = consumers.get(&owner) else {
            if self.has_scalar_operation_source_v1(owner)? {
                return Err(reject("consumer-handoff-missing"));
            }
            return Ok(());
        };
        for (original, relation) in rows {
            visit(original, relation)?;
        }
        Ok(())
    }

    pub(in crate::mir) fn visit_scalar_operation_originals_v1(
        &self,
        mut visit: impl FnMut(&Rc<OriginalScalarAppendV1>) -> Result<(), String>,
    ) -> Result<(), String> {
        for row in self.scalar_operation_appends.borrow().values() {
            visit(row)?;
        }
        Ok(())
    }
}

fn source_operator(source: &Source) -> Result<BinaryOp, String> {
    source.integer_operator_v1().ok_or_else(|| reject("source-operator"))
}

fn unique_definition(function: &MirFunction, value: ValueId) -> Result<Binding, String> {
    let mut rows = function
        .blocks
        .iter()
        .flat_map(|(block, row)| {
            row.all_instructions()
                .map(move |instruction| (*block, instruction))
        })
        .filter(|(_, row)| row.dst_value() == Some(value));
    let (block, row) = rows.next().ok_or_else(|| reject("producer-missing"))?;
    if rows.next().is_some() {
        return Err(reject("producer-duplicate"));
    }
    Ok((block, row.clone()))
}

fn reject(reason: &str) -> String {
    format!("[freeze:contract][scalar-original/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_scalar_operation_tests.rs"]
mod tests;
