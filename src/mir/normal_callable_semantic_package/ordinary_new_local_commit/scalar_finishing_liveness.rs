//! Borrow existing required scalar producers for DCE, without source permission.
use super::*;
use crate::mir::passes::dce::RequiredLivenessRoots;
use std::collections::{BTreeMap, BTreeSet};

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn scalar_finishing_liveness_v1(
        &self,
        module: &crate::mir::MirModule,
        root_symbol: &str,
    ) -> Result<RequiredLivenessRoots, String> {
        let mut roots = BTreeMap::new();
        match &*self.root_validation.borrow() {
            RootNewValidation::Unregistered => {}
            RootNewValidation::Checked(owner, boundary) => {
                self.lend_scalar_roots(module, root_symbol, *owner, boundary, &mut roots)?;
            }
            _ => return Err(freeze("scalar-liveness/root-state")),
        }
        for (owner, state) in self.child_physical_validation.borrow().iter() {
            let ChildPhysicalValidation::Checked { symbol, boundary } = state else {
                return Err(freeze("scalar-liveness/child-state"));
            };
            self.lend_scalar_roots(module, symbol, *owner, boundary, &mut roots)?;
        }
        Ok(roots)
    }

    fn lend_scalar_roots(
        &self,
        module: &crate::mir::MirModule,
        symbol: &str,
        owner: FunctionOwnerIdV1,
        boundary: &super::super::physical_boundary::PhysicalBoundary,
        roots: &mut RequiredLivenessRoots,
    ) -> Result<(), String> {
        let function = module
            .functions
            .get(symbol)
            .ok_or_else(|| freeze("scalar-liveness/function-missing"))?;
        if function.signature.name != symbol || roots.contains_key(symbol) {
            return Err(freeze("scalar-liveness/function-drift"));
        }
        let mut bindings = self.scalar_operation_bindings_v1(owner)?;
        bindings.extend(self.borrowed_mul_bindings_v1(owner)?);
        bindings.extend(self.borrowed_compare_bindings_v1(owner)?);
        roots.insert(
            symbol.to_owned(),
            validated_roots(function, boundary, bindings)?,
        );
        Ok(())
    }
}

fn validated_roots(
    function: &MirFunction,
    boundary: &super::super::physical_boundary::PhysicalBoundary,
    originals: Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BTreeSet<ValueId>, String> {
    let projection = boundary.project(function)?;
    let mut bindings = BTreeMap::new();
    for original in originals {
        if matches!(
            original.1,
            MirInstruction::Const { .. } | MirInstruction::Copy { .. }
        ) {
            continue;
        }
        let Some(dst) = original.1.dst_value() else {
            continue;
        };
        if let Some(previous) = bindings.insert(dst, original.clone()) {
            if previous != original {
                return Err(freeze("scalar-liveness/conflicting-original"));
            }
            continue;
        }
        if projection.relation_binding(function, &original)?.as_ref() != Some(&original) {
            return Err(freeze("scalar-liveness/original-drift"));
        }
    }
    Ok(bindings.keys().copied().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BinaryOp, ConstValue, EffectMask, FunctionSignature, MirType};

    #[test]
    fn required_roots_reject_missing_foreign_or_conflicting_originals() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "scalar-roots/0".into(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId(0),
        );
        let block = function.blocks.get_mut(&BasicBlockId(0)).unwrap();
        let literal = (
            BasicBlockId(0),
            MirInstruction::Const {
                dst: ValueId(1),
                value: ConstValue::Integer(2),
            },
        );
        let original = (
            BasicBlockId(0),
            MirInstruction::BinOp {
                dst: ValueId(2),
                op: BinaryOp::Mul,
                lhs: ValueId(1),
                rhs: ValueId(1),
            },
        );
        block.add_instruction(literal.1.clone());
        block.add_instruction(original.1.clone());
        block.set_terminator(MirInstruction::Return { value: None });
        let boundary = super::super::super::physical_boundary::PhysicalBoundary::capture(
            &function,
            std::slice::from_ref(&original),
        )
        .unwrap();
        assert_eq!(
            validated_roots(
                &function,
                &boundary,
                vec![literal, original.clone(), original.clone()]
            )
            .unwrap(),
            BTreeSet::from([ValueId(2)])
        );
        let mut foreign = original.clone();
        if let MirInstruction::BinOp { op, .. } = &mut foreign.1 {
            *op = BinaryOp::Add;
        }
        assert!(validated_roots(&function, &boundary, vec![foreign.clone()]).is_err());
        assert!(validated_roots(&function, &boundary, vec![original.clone(), foreign]).is_err());
        let mut missing = function.clone();
        missing
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .instructions
            .pop();
        assert!(validated_roots(&missing, &boundary, vec![original.clone()]).is_err());
        let mut duplicate = function.clone();
        duplicate
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .add_instruction(original.1.clone());
        assert!(validated_roots(&duplicate, &boundary, vec![original]).is_err());
    }
}
