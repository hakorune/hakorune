//! Selected scalar acceptance borrows the existing final view, never a permit list.
use super::*;

type Binding = (BasicBlockId, MirInstruction);

pub(super) fn validate_selected_scalar_operations(
    module: Option<&MirModule>,
    function: &MirFunction,
    view: Option<&PublishedMirBackendView<'_>>,
) -> Result<Vec<Binding>, String> {
    let actual: Vec<_> = function
        .blocks
        .iter()
        .flat_map(|(block, body)| {
            body.all_instructions().filter_map(move |row| {
                matches!(
                    row,
                    MirInstruction::BinOp {
                        op: BinaryOp::Sub | BinaryOp::Div | BinaryOp::Mod,
                        ..
                    }
                )
                .then(|| (*block, row.clone()))
            })
        })
        .collect();
    let Some(view) = view else {
        return if actual.is_empty() {
            Ok(Vec::new())
        } else {
            Err(fault("scalar-source-view-missing"))
        };
    };
    let module = module.ok_or_else(|| fault("scalar-module-missing"))?;
    if !std::ptr::eq(module, view.module())
        || !module
            .functions
            .get(&function.signature.name)
            .is_some_and(|stored| std::ptr::eq(stored, function))
    {
        return Err(fault("scalar-view-membership"));
    }
    let Some(source) = view.retained_root_source() else {
        return if actual.is_empty() {
            Ok(Vec::new())
        } else {
            Err(fault("scalar-source-handoff-missing"))
        };
    };
    let mut expected = Vec::new();
    source.visit_finished_scalar_operations_v1(module, |owner, binding, original| {
        if std::ptr::eq(owner, function) {
            if expected.contains(binding) {
                return Err(fault("scalar-source-duplicate"));
            }
            let MirInstruction::BinOp { op, rhs, .. } = &binding.1 else {
                return Err(fault("scalar-source-shape"));
            };
            if !matches!(op, BinaryOp::Sub | BinaryOp::Div | BinaryOp::Mod) {
                return Err(fault("scalar-source-operator"));
            }
            if matches!(op, BinaryOp::Div | BinaryOp::Mod) {
                let positive = original
                    .source()
                    .positive_divisor()
                    .ok_or_else(|| fault("scalar-positive-divisor-missing"))?;
                let mut definitions = function
                    .blocks
                    .values()
                    .flat_map(|block| block.all_instructions())
                    .filter(|row| row.dst_value() == Some(*rhs));
                if !matches!(definitions.next(), Some(MirInstruction::Const {
                    value: ConstValue::Integer(value), ..
                }) if *value > 0 && *value == positive)
                    || definitions.next().is_some()
                {
                    return Err(fault("scalar-positive-divisor-drift"));
                }
            }
            expected.push(binding.clone());
        }
        Ok(())
    })?;
    if actual.len() != expected.len()
        || actual.iter().any(|binding| !expected.contains(binding))
        || expected.iter().any(|binding| !actual.contains(binding))
    {
        return Err(format!("{} function={} actual={} source={} unexpected={:?} missing={:?}",
            fault("scalar-source-coverage"), function.signature.name,
            actual.len(), expected.len(),
            actual.iter().find(|binding| !expected.contains(binding)),
            expected.iter().find(|binding| !actual.contains(binding))));
    }
    Ok(expected)
}
