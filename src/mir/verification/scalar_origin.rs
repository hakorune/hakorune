//! Same-definition queries over one completed, immutable MIR function.
//! Source sites and producer loans are deliberately not evidence here.
use super::{cfg, geometry::FunctionVerificationGeometry, invoke};
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::verification_types::VerificationError;
#[cfg(test)]
use crate::mir::BasicBlockId;
use crate::mir::{MirFunction, MirInstruction, MirModule, MirType, ValueId};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SameI64Definition {
    Same,
    Different,
    Unresolved,
}

/// No proof escapes this immutable function epoch. Construct fresh geometry:
/// facts cached by an issuer while it was still writing the CFG are not valid.
/// `expected` names an actual base definition, not a source position.
#[cfg(test)]
pub(in crate::mir) fn same_i64_definition(
    function: &MirFunction,
    module: Option<&MirModule>,
    value: ValueId,
    expected: ValueId,
) -> Result<SameI64Definition, Vec<VerificationError>> {
    let geometry = FunctionVerificationGeometry::new(function);
    FinishedI64OriginsV1::new(&geometry, module).same_definition(value, expected)
}

/// A borrowed immutable function/module epoch. Only structural validation is
/// shared; a query's answer still depends on its own expected definition.
/// Validation is lazy so a function with no origin obligations gains no gate.
pub(in crate::mir) struct FinishedI64OriginsV1<'geometry, 'module> {
    geometry: &'geometry FunctionVerificationGeometry<'module>,
    module: Option<&'module MirModule>,
    checked: std::cell::OnceCell<Result<(), Vec<VerificationError>>>,
}
impl<'geometry, 'module> FinishedI64OriginsV1<'geometry, 'module> {
    pub(in crate::mir) fn new(
        geometry: &'geometry FunctionVerificationGeometry<'module>,
        module: Option<&'module MirModule>,
    ) -> Self {
        Self {
            geometry,
            module,
            checked: std::cell::OnceCell::new(),
        }
    }
    pub(in crate::mir) fn geometry(&self) -> &FunctionVerificationGeometry<'module> {
        self.geometry
    }
    pub(in crate::mir) fn module(&self) -> Option<&'module MirModule> {
        self.module
    }
    pub(in crate::mir) fn same_definition(
        &self,
        value: ValueId,
        expected: ValueId,
    ) -> Result<SameI64Definition, Vec<VerificationError>> {
        self.checked.get_or_init(|| self.check_function()).clone()?;
        let query = OriginQuery {
            geometry: self.geometry,
            module: self.module,
            expected,
        };
        if !query.is_base(expected) {
            return Ok(SameI64Definition::Unresolved);
        }
        query.origin(value)
    }
    fn check_function(&self) -> Result<(), Vec<VerificationError>> {
        let geometry = self.geometry;
        let function = geometry.as_function();
        if !function.blocks.contains_key(&function.entry_block) {
            return Err(vec![VerificationError::ControlFlowError {
                block: function.entry_block,
                reason: "Missing function entry block".into(),
            }]);
        }
        // Guard the existing CFG helper's invariant before it reads terminators.
        for (block, body) in &function.blocks {
            if body
                .terminator
                .as_ref()
                .is_some_and(|row| !body.is_terminator(row))
                || body.instructions.iter().any(|row| body.is_terminator(row))
            {
                return Err(vec![VerificationError::ControlFlowError {
                    block: *block,
                    reason: "Invalid terminator placement".into(),
                }]);
            }
        }
        cfg::check_control_flow(function)?;
        let mut errors = geometry.ssa_errors().to_vec();
        errors.extend_from_slice(geometry.dominance_errors());
        if !errors.is_empty() {
            return Err(errors);
        }
        cfg::check_phi_predecessors_with(function, geometry.predecessors(), geometry.reachable())?;
        if let Some(module) = self.module {
            invoke::check_function_structure_in_module(module, geometry)?;
        } else {
            invoke::check_function_with_geometry(geometry)?;
        }
        if !geometry.strict_edge_errors().is_empty() {
            return Err(geometry.strict_edge_errors().to_vec());
        }
        Ok(())
    }
}

struct OriginQuery<'a, 'f> {
    geometry: &'a FunctionVerificationGeometry<'f>,
    module: Option<&'a MirModule>,
    expected: ValueId,
}
impl OriginQuery<'_, '_> {
    fn is_base(&self, value: ValueId) -> bool {
        let function = self.geometry.as_function();
        if function
            .metadata
            .value_types
            .get(&value)
            .is_some_and(|kind| *kind != MirType::Integer)
        {
            return false;
        }
        if self.geometry.definition_count(value) != 1 {
            return false;
        }
        if let Some(index) = function.params.iter().position(|old| *old == value) {
            return function.params.len() == function.signature.params.len()
                && function.signature.params[index] == MirType::Integer
                && function
                    .metadata
                    .physical_param_carriers
                    .as_deref()
                    .is_some_and(|rows| {
                        rows.len() == function.params.len()
                            && rows[index] == PhysicalCallableLaneCarrierV1::ExistingCallableI64
                    })
                && function
                    .metadata
                    .declared_param_decls
                    .get(index)
                    .is_some_and(|decl| {
                        !decl.implicit_receiver && decl.declared_type_name.as_deref() == Some("i64")
                    });
        }
        let Some((block, _, instruction)) = self.geometry.definition(value) else {
            return false;
        };
        if !self.geometry.reachable().contains(&block) {
            return false;
        }
        match instruction {
            MirInstruction::Const {
                value: crate::mir::ConstValue::Integer(_),
                ..
            } => true,
            MirInstruction::Call(call) => self.i64_call(call),
            MirInstruction::InvokeNormalResult { invoke_block, .. } => {
                use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
                matches!(self.geometry.blocks.get(invoke_block).and_then(|body| body.terminator.as_ref()),
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::Call { call, result: InvokeCallResultKind::I64 },
                        normal_landing, ..
                    }) if *normal_landing == block && self.i64_call(call))
            }
            _ => false,
        }
    }

    fn i64_call(&self, call: &crate::mir::definitions::MirCall) -> bool {
        let Some(module) = self.module else {
            return false;
        };
        let Some((callee, receiver_count)) = invoke::cataloged_call_target(module, call) else {
            return false;
        };
        callee.signature.return_type == MirType::Integer
            && callee
                .metadata
                .declared_return_type_name
                .as_deref()
                .is_none_or(|declared| declared == "i64")
            && callee.params.len() == callee.signature.params.len()
            && call.args.len() + receiver_count == callee.params.len()
    }

    /// A temporary value-dependency closure, not another CFG or proof ledger.
    /// Every Copy/Phi node must reach a real base; a closed cycle has none.
    fn origin(&self, value: ValueId) -> Result<SameI64Definition, Vec<VerificationError>> {
        let mut pending = vec![value];
        let mut nodes = HashSet::new();
        let mut reverse = HashMap::<ValueId, Vec<ValueId>>::new();
        let mut bases = Vec::new();
        let mut different = false;
        while let Some(current) = pending.pop() {
            if !nodes.insert(current) {
                continue;
            }
            if self
                .geometry
                .metadata
                .value_types
                .get(&current)
                .is_some_and(|kind| *kind != MirType::Integer)
            {
                return Ok(SameI64Definition::Unresolved);
            }
            if self.is_base(current) {
                different |= current != self.expected;
                bases.push(current);
                continue;
            }
            let Some((block, _, row)) = self.geometry.definition(current) else {
                return Ok(SameI64Definition::Unresolved);
            };
            if !self.geometry.reachable().contains(&block) {
                return Ok(SameI64Definition::Unresolved);
            }
            let inputs = match row {
                MirInstruction::Copy { src, .. } => vec![*src],
                MirInstruction::Phi {
                    inputs, type_hint, ..
                } => {
                    if type_hint
                        .as_ref()
                        .is_some_and(|kind| *kind != MirType::Integer)
                        || inputs
                            .iter()
                            .any(|(pred, _)| !self.geometry.reachable().contains(pred))
                    {
                        return Ok(SameI64Definition::Unresolved);
                    }
                    inputs.iter().map(|(_, source)| *source).collect()
                }
                _ => return Ok(SameI64Definition::Unresolved),
            };
            for input in inputs {
                reverse.entry(input).or_default().push(current);
                pending.push(input);
            }
        }
        let mut grounded = HashSet::new();
        while let Some(current) = bases.pop() {
            if grounded.insert(current) {
                if let Some(parents) = reverse.get(&current) {
                    bases.extend(parents.iter().copied());
                }
            }
        }
        Ok(if grounded.len() != nodes.len() {
            SameI64Definition::Unresolved
        } else if different {
            SameI64Definition::Different
        } else {
            SameI64Definition::Same
        })
    }
}

#[cfg(test)]
#[path = "scalar_origin_tests.rs"]
mod tests;
