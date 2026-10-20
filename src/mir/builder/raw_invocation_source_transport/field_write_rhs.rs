//! Observe the original recursive RHS writers, without a second evaluator.
use super::*;
use crate::mir::{BasicBlockId, MirInstruction};
use crate::mir::builder::ordinary_new_admission::selected::bool_condition::{cursor, definition};
use crate::mir::normal_callable_semantic_package::{FieldWriteSourceLoanV1, EmittedFieldWriteRhsV1,
    FieldWriteRhsNodeV1, FieldWriteRhsProofV1 as Proof};
use crate::mir::resolved_semantics::{home_new_prefix::QualifiedFieldWriteRhsV1, OwnedExprSiteV1, SourceExprSiteV1};
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct FieldWriteRhsEmissionV1 {
    recipe: Rc<QualifiedFieldWriteRhsV1>,
    nodes: Vec<FieldWriteRhsNodeV1>,
    binary: BTreeMap<SourceExprSiteV1, (BasicBlockId, MirInstruction)>,
}
impl FieldWriteRhsEmissionV1 {
    fn value_at(&self, site: &SourceExprSiteV1) -> Result<ValueId, String> {
        let index = self.recipe.source().nodes().iter().position(|node| node.site() == site)
            .ok_or("[freeze:contract][field-write-rhs/child-site]")?;
        self.nodes.iter().find(|node| node.node == index).map(|node| node.value)
            .ok_or_else(|| "[freeze:contract][field-write-rhs/child-observation]".into())
    }
}
impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn lower_whole_field_write_rhs_v1(&mut self, builder: &mut MirBuilder,
        source: &FieldWriteSourceLoanV1, input: ASTNode,
    ) -> Result<Rc<EmittedFieldWriteRhsV1>, String> {
        let recipe = source.qualified_rhs().ok_or("[freeze:contract][field-write-rhs/source-origin]")?;
        if self.field_write_rhs_emission.is_some() || self.bool_condition_emission.is_some()
            || self.callable_owner_v1() != Some(recipe.source().owner())
            || self.current_source_site_v1().as_ref() != Some(recipe.source().nodes()[0].site().node()) {
            return Err("[freeze:contract][field-write-rhs/source-scope]".into());
        }
        let ledger = self.ordinary_new_claim_ledger.clone().ok_or("[freeze:contract][field-write-rhs/no-ledger]")?;
        source.require_current_execution(&ledger)?;
        let emission = Rc::new(RefCell::new(FieldWriteRhsEmissionV1 {
            recipe: Rc::clone(recipe), nodes: Vec::new(), binary: BTreeMap::new(),
        }));
        self.field_write_rhs_emission = Some(Rc::clone(&emission));
        let result = self.lower_expression(builder, input);
        self.field_write_rhs_emission = None;
        let value = result?;
        let observed = Rc::try_unwrap(emission).map_err(|_| "[freeze:contract][field-write-rhs/escaped-scope]")?.into_inner();
        if !observed.binary.is_empty() { return Err("[freeze:contract][field-write-rhs/unconsumed-binary]".into()); }
        let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][field-write-rhs/no-function]")?;
        EmittedFieldWriteRhsV1::new(observed.recipe, observed.nodes, value, &ledger, function).map(Rc::new)
    }
    pub(super) fn observe_field_write_binary_v1(&mut self, completed: &crate::mir::builder::ops::CompletedOrdinaryBinaryV1) -> Result<(), String> {
        let Some(scope) = &self.field_write_rhs_emission else { return Ok(()) };
        let site = SourceExprSiteV1::from_node(self.current_source_site_v1().ok_or("[freeze:contract][field-write-rhs/binary-site]")?);
        let binding = completed.arithmetic_original().ok_or("[freeze:contract][field-write-rhs/binary-writer]")?.clone();
        if scope.borrow_mut().binary.insert(site, binding).is_some() {
            return Err("[freeze:contract][field-write-rhs/duplicate-binary]".into());
        }
        Ok(())
    }
    pub(super) fn lower_with_field_write_rhs_v1(&mut self, builder: &mut MirBuilder, input: ASTNode) -> Result<ValueId, String> {
        let Some(scope) = self.field_write_rhs_emission.clone() else { return self.lower_with_bool_condition_v1(builder, input) };
        let recipe = Rc::clone(&scope.borrow().recipe);
        let source = recipe.source();
        let site = SourceExprSiteV1::from_node(self.current_source_site_v1().ok_or("[freeze:contract][field-write-rhs/node-site]")?);
        let index = source.nodes().iter().position(|node| node.site() == &site).ok_or("[freeze:contract][field-write-rhs/foreign-node]")?;
        let entry = cursor(builder)?;
        let state = self.callable_ledger.clone().ok_or("[freeze:contract][field-write-rhs/no-state]")?;
        let ledger = self.ordinary_new_claim_ledger.clone().ok_or("[freeze:contract][field-write-rhs/no-ledger]")?;
        let (value, proof) = match source.nodes()[index].source() {
            Node::Literal(_) => {
                let value = self.lower_expression_unobserved_v1(builder, input)?;
                let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][field-write-rhs/no-function]")?;
                (value, Proof::Literal(definition(function, value)?.ok_or("[freeze:contract][field-write-rhs/literal-writer]")?))
            },
            Node::Reference(_) => {
                let (binding, _) = recipe.reference(index).ok_or("[freeze:contract][field-write-rhs/reference-kind]")?;
                let read = state.borrow_mut().take_exact_lexical_read(source.owner(), site.node(), binding).map_err(|issue| issue.to_string())?;
                let value = read.value_for(source.owner(), site.node(), binding).map_err(|issue| issue.to_string())?;
                let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][field-write-rhs/no-function]")?;
                (value, Proof::Reference { read: Rc::new(read), producer: definition(function, value)? })
            },
            Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. }) => {
                let owned = OwnedExprSiteV1::new(source.owner(), site.clone());
                let mut receiver = None;
                let (base, read) = ledger.take_local_field_read(&owned, |binding| {
                    let read = state.borrow_mut().take_exact_lexical_read(source.owner(), object.node(), binding)
                        .map_err(|issue| issue.to_string())?;
                    let value = read.value_for(source.owner(), object.node(), binding).map_err(|issue| issue.to_string())?;
                    receiver = Some(Rc::new(read));
                    Ok(value)
                })?.ok_or("[freeze:contract][field-write-rhs/field-source]")?;
                let field = read.field();
                let claim = if read.requires_checked_field_read_v1() {
                    crate::mir::builder::fields::PreparedExactFieldReadClaimV1::CheckedLocal { source: read,
                        read: receiver.ok_or("[freeze:contract][field-write-rhs/exact-receiver]")?, state: Rc::clone(&state) }
                } else { crate::mir::builder::fields::PreparedExactFieldReadClaimV1::Local(read) };
                let (value, _) = builder.complete_exact_field_read_v1(base, field, &owned, &ledger, claim)?;
                let rows = ledger.expression_field_read_bindings_v1(&owned, source,
                    crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(&ledger))?;
                (value, Proof::Field(rows))
            },
            Node::Binary(binary) => {
                let value = self.lower_expression_unobserved_v1(builder, input)?;
                let append = scope.borrow_mut().binary.remove(&site).ok_or("[freeze:contract][field-write-rhs/binary-observation]")?;
                let MirInstruction::BinOp { lhs, rhs, .. } = append.1 else { return Err("[freeze:contract][field-write-rhs/binary-kind]".into()) };
                let inputs = (scope.borrow().value_at(binary.lhs())?, scope.borrow().value_at(binary.rhs())?);
                let mut scalar = None;
                ledger.visit_scalar_operation_originals_v1(|original| {
                    if Rc::ptr_eq(original.source().plan(), source) && original.source().binary().site() == &site {
                        if scalar.replace(Rc::clone(original)).is_some() { return Err("[freeze:contract][field-write-rhs/duplicate-scalar]".into()); }
                    }
                    Ok(())
                })?;
                let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][field-write-rhs/no-function]")?;
                let relations = &mut builder.function_state.local_ssa_correspondence;
                let operands = [relations.loan(function, inputs.0, lhs).ok_or("[freeze:contract][field-write-rhs/lhs-relation]")?,
                    relations.loan(function, inputs.1, rhs).ok_or("[freeze:contract][field-write-rhs/rhs-relation]")?];
                (value, Proof::Binary { append, operands, scalar })
            },
            _ => return Err("[freeze:contract][field-write-rhs/node-writer-unavailable]".into()),
        };
        let exit = cursor(builder)?;
        scope.borrow_mut().nodes.push(FieldWriteRhsNodeV1 { node: index, entry, exit, value, proof });
        Ok(value)
    }
}
