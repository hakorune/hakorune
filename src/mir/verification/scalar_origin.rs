//! Same-definition queries over one completed, immutable MIR function.
//! Source sites and producer loans are deliberately not evidence here.
use super::{cfg, geometry::FunctionVerificationGeometry, invoke};
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirModule, MirType, ValueId};
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
pub(in crate::mir) fn same_i64_definition(
    function: &MirFunction,
    module: Option<&MirModule>,
    value: ValueId,
    expected: ValueId,
) -> Result<SameI64Definition, Vec<VerificationError>> {
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
    let geometry = FunctionVerificationGeometry::new(function);
    cfg::check_control_flow(function)?;
    let mut errors = geometry.ssa_errors().to_vec();
    errors.extend_from_slice(geometry.dominance_errors());
    if !errors.is_empty() {
        return Err(errors);
    }
    cfg::check_phi_predecessors_with(function, geometry.predecessors(), geometry.reachable())?;
    if let Some(module) = module {
        invoke::check_function_structure_in_module(module, &geometry)?;
    } else {
        invoke::check_function_with_geometry(&geometry)?;
    }
    let mut query = OriginQuery {
        geometry: &geometry,
        module,
        expected,
        active: HashSet::new(),
        memo: HashMap::new(),
    };
    if !geometry.strict_edge_errors().is_empty() {
        return Err(geometry.strict_edge_errors().to_vec());
    }
    if !query.is_base(expected) {
        return Ok(SameI64Definition::Unresolved);
    }
    query.origin(value)
}

struct OriginQuery<'a, 'f> {
    geometry: &'a FunctionVerificationGeometry<'f>,
    module: Option<&'a MirModule>,
    expected: ValueId,
    active: HashSet<ValueId>,
    memo: HashMap<ValueId, SameI64Definition>,
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

    fn origin(&mut self, value: ValueId) -> Result<SameI64Definition, Vec<VerificationError>> {
        if let Some(result) = self.memo.get(&value) {
            return Ok(*result);
        }
        if !self.active.insert(value) {
            return Ok(SameI64Definition::Unresolved); // Cyclic PHI is deliberately outside this query.
        }
        let result = self.trace(value);
        self.active.remove(&value);
        let result = result?;
        self.memo.insert(value, result);
        Ok(result)
    }

    fn trace(&mut self, value: ValueId) -> Result<SameI64Definition, Vec<VerificationError>> {
        if self
            .geometry
            .metadata
            .value_types
            .get(&value)
            .is_some_and(|kind| *kind != MirType::Integer)
        {
            return Ok(SameI64Definition::Unresolved);
        }
        if self.is_base(value) {
            return Ok(if value == self.expected {
                SameI64Definition::Same
            } else {
                SameI64Definition::Different
            });
        }
        let Some((block, _, row)) = self.geometry.definition(value) else {
            return Ok(SameI64Definition::Unresolved);
        };
        if !self.geometry.reachable().contains(&block) {
            return Ok(SameI64Definition::Unresolved);
        }
        match row {
            MirInstruction::Copy { src, .. } => self.origin(*src),
            MirInstruction::Phi {
                inputs, type_hint, ..
            } => {
                if type_hint
                    .as_ref()
                    .is_some_and(|kind| *kind != MirType::Integer)
                {
                    return Ok(SameI64Definition::Unresolved);
                }
                let mut result = SameI64Definition::Same;
                for (pred, source) in inputs {
                    if !self.geometry.reachable().contains(pred) {
                        result = SameI64Definition::Unresolved;
                        continue;
                    }
                    match self.origin(*source)? {
                        SameI64Definition::Unresolved => result = SameI64Definition::Unresolved,
                        SameI64Definition::Different if result == SameI64Definition::Same => {
                            result = SameI64Definition::Different
                        }
                        _ => {}
                    }
                }
                Ok(result)
            }
            _ => Ok(SameI64Definition::Unresolved), // No numeric type hint promotes handles or heap reads.
        }
    }
}

#[cfg(test)]
#[path = "scalar_origin_tests.rs"]
mod tests;
