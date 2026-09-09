//! Caller-aware Birth call matching for the compiled-entry consumer.

use super::*;

/// Match every retained Birth actual against the physical caller that owns its New site.
/// The caller function index is an internal transport coordinate; it is never a source issuer.
pub(super) fn issue_birth_calls_for_program(
    program: &super::super::physical_program::PublishedLifecyclePhysicalProgramV1<'_>,
    births: &[(
        u32,
        &super::super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    )],
    actuals: &[FinalizedBirthActualsV1],
) -> Result<Vec<CompiledEntryBirthCallV1>, String> {
    let indexed = births.to_vec();
    for (i, actual) in actuals.iter().enumerate() {
        if actuals[..i].iter().any(|previous| {
            previous.site() == actual.site() || previous.destination() == actual.destination()
        }) || actual.destination().owner() != actual.site().owner()
            || actual
                .arguments()
                .iter()
                .enumerate()
                .any(|(ordinal, argument)| {
                    argument.source().ordinal() as usize != ordinal
                        || argument.source().owner() != actual.destination().owner()
                        || argument.source().new_site() != actual.site()
                })
        {
            return Err(fault("compiled-entry-actual-membership"));
        }
    }
    let root_owner = program
        .handoff()
        .root_result()
        .map(|result| match result {
            FinalizedRootResultAbiV1::CallReturn { owner }
            | FinalizedRootResultAbiV1::I64AddReturn { owner }
            | FinalizedRootResultAbiV1::UnitReturn { owner }
            | FinalizedRootResultAbiV1::IntegerLiteralReturn { owner }
            | FinalizedRootResultAbiV1::I64FieldReturn { owner } => owner,
        })
        .ok_or_else(|| fault("compiled-entry-root-owner-missing"))?;
    let mut consumed = vec![false; actuals.len()];
    let mut referenced = vec![false; indexed.len()];
    let mut calls = Vec::new();
    for (caller_function_index, function) in program.functions().iter().enumerate() {
        let caller_function_index =
            u32::try_from(caller_function_index).map_err(|_| fault("compiled-entry-call-index"))?;
        let owner = match function.role() {
            PublishedLifecyclePhysicalFunctionRoleV1::Root { .. } => Some(root_owner),
            PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { key } => {
                let has_birth = function.blocks().iter().any(|block| {
                    block
                        .instructions()
                        .iter()
                        .copied()
                        .chain(std::iter::once(block.terminator()))
                        .any(|row| {
                            matches!(
                                row.instruction(),
                                MirInstruction::Call(crate::mir::definitions::MirCall {
                                    callee: Callee::BirthConstructor { .. },
                                    ..
                                })
                            ) || matches!(
                                row.instruction(),
                                MirInstruction::Invoke {
                                    operation: InvokeOperation::Call {
                                        call: crate::mir::definitions::MirCall {
                                            callee: Callee::BirthConstructor { .. },
                                            ..
                                        },
                                        result: InvokeCallResultKind::Unit,
                                    },
                                    ..
                                }
                            )
                        })
                });
                if !has_birth {
                    continue;
                }
                Some(
                    program
                        .handoff()
                        .callables()
                        .and_then(|callables| callables.owner_for_canonical_key(key))
                        .ok_or_else(|| fault("compiled-entry-caller-owner-missing"))?,
                )
            }
            PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { .. } => None,
        };
        let Some(owner) = owner else { continue };
        let mut owner_calls = Vec::new();
        for block in function.blocks() {
            for row in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                match row.instruction() {
                    MirInstruction::Call(call)
                        if matches!(call.callee, Callee::BirthConstructor { .. }) =>
                    {
                        owner_calls.push((call, caller_function_index));
                    }
                    MirInstruction::Invoke {
                        operation:
                            InvokeOperation::Call {
                                call,
                                result: InvokeCallResultKind::Unit,
                            },
                        ..
                    } if matches!(call.callee, Callee::BirthConstructor { .. }) => {
                        owner_calls.push((call, caller_function_index));
                    }
                    _ => {}
                }
            }
        }
        if owner_calls.is_empty() {
            continue;
        }
        let mut owner_actuals = Vec::new();
        for (index, actual) in actuals.iter().enumerate() {
            if actual.site().owner() == owner {
                owner_actuals.push((index, actual));
            }
        }
        for (call, caller_function_index) in owner_calls {
            let Callee::BirthConstructor { key, receiver } = &call.callee else {
                unreachable!()
            };
            let birth_index = indexed
                .iter()
                .position(|(_, birth)| {
                    matches!(birth.role(), PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } if abi.target() == key)
                })
                .ok_or_else(|| fault("compiled-entry-call-target"))?;
            let matching = owner_actuals
                .iter()
                .filter(|(_, actual)| {
                    actual.target() == key
                        && actual.receiver() == *receiver
                        && actual
                            .arguments()
                            .iter()
                            .map(|argument| argument.value())
                            .eq(call.args.iter().copied())
                })
                .map(|(index, _)| *index)
                .collect::<Vec<_>>();
            let [actual_index] = matching.as_slice() else {
                return Err(fault("compiled-entry-call-actual-mismatch"));
            };
            if std::mem::replace(&mut consumed[*actual_index], true) {
                return Err(fault("compiled-entry-call-duplicate"));
            }
            if call.args.len() != indexed[birth_index].1.params().len().saturating_sub(1) {
                return Err(fault("compiled-entry-call-arity"));
            }
            referenced[birth_index] = true;
            calls.push(CompiledEntryBirthCallV1 {
                caller_function_index,
                function_index: indexed[birth_index].0,
                actual: actuals[*actual_index].clone(),
            });
        }
    }
    if consumed.contains(&false) || referenced.contains(&false) {
        return Err(fault("compiled-entry-call-missing"));
    }
    Ok(calls)
}
