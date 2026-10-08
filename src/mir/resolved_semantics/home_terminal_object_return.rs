//! Original source evidence for a returned owned call; no Home is acquired here.
use super::*;
use crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ObjectReturnAcquisitionV1 {
    /// Explicit Return destination, never a discarded/local continuation.
    Direct {
        argument_sites: Box<[OwnedExprSiteV1]>,
        fault_homes: Box<[BindingRefV1]>,
    },
    /// The original acquired call row, including its prior Fault homes.
    Received(LocalCallObservationV1),
}

/// Pending source obligation. Only the later completion/leaf co-seal can grant
/// a Normal handoff; current exit homes remain the original cleanup snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalObjectReturnObligationV1 {
    qualification: ObjectReturnCallQualificationV1,
    return_site: SourceStmtSiteV1,
    acquisition: ObjectReturnAcquisitionV1,
    exit_homes: Box<[BindingRefV1]>,
    arguments: ObjectCallSourceSupportV1,
}

impl TerminalObjectReturnObligationV1 {
    /// Corrupt the retained snapshot without changing its source identity.
    #[cfg(test)]
    pub(crate) fn with_arguments_for_test(&self, arguments: ObjectCallSourceSupportV1) -> Self {
        let mut changed = self.clone();
        changed.arguments = arguments;
        changed
    }

    pub(crate) fn arguments(&self) -> &ObjectCallSourceSupportV1 {
        &self.arguments
    }
    pub(crate) fn qualification(&self) -> &ObjectReturnCallQualificationV1 {
        &self.qualification
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) fn acquisition(&self) -> &ObjectReturnAcquisitionV1 {
        &self.acquisition
    }
    pub(crate) fn exit_homes(&self) -> &[BindingRefV1] {
        &self.exit_homes
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_object_return_source<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    loan: ObjectReturnCallQualificationV1,
    locals: &PrefixLocalFlow<'_>,
    calls: &[LocalCallObservationV1],
    covered: &BTreeSet<OwnedExprSiteV1>,
    homes: &[BindingRefV1],
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Result<TerminalValueReturnV1, HomePrefixUnavailableV1>, E> {
    let unavailable = || HomePrefixUnavailableV1::ReturnValueNotCovered(statement.site().clone());
    let Ok(value) = input
        .source()
        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
    else {
        return Ok(Err(unavailable()));
    };
    let value_site = OwnedExprSiteV1::new(input.owner(), value.site().clone());
    if statement.owner() != input.owner()
        || loan.value() != &value_site
        || loan.call().owner() != input.owner()
    {
        return Ok(Err(unavailable()));
    }
    // Match the terminal exit's cleanup order, newest Home first. The original
    // received-call prior_homes representation remains untouched.
    let exit_homes: Box<[BindingRefV1]> = homes.iter().rev().copied().collect();
    let acquisition = if loan.call() == loan.value() {
        let Some(call) = input.function().method_call(value.site()) else {
            return Ok(Err(unavailable()));
        };
        if call.owner() != input.owner()
            || call.site() != value.site()
            || call.selector() != loan.key().name()
            || call.arity() != loan.key().arity()
            || call.arguments().len() != call.arity() as usize
            || call.arguments().iter().enumerate().any(|(ordinal, arg)| {
                arg.ordinal() != ordinal as u32
                    || arg.site().node().segments()
                        != [
                            value.site().node().segments(),
                            &[super::SourcePathSegmentV1::Argument(ordinal as u32)],
                        ]
                        .concat()
                        .as_slice()
            })
        {
            return Ok(Err(unavailable()));
        }
        // Exact original argument sites only; no values or arity capability.
        let argument_sites = call
            .arguments()
            .iter()
            .map(|argument| OwnedExprSiteV1::new(input.owner(), argument.site().clone()))
            .collect();
        ObjectReturnAcquisitionV1::Direct {
            argument_sites,
            fault_homes: exit_homes.clone(),
        }
    } else {
        let Some(ResolvedLexicalRefV1::Local(binding)) =
            input.function().variable_ref(value.site())
        else {
            return Ok(Err(unavailable()));
        };
        let Some(original) = locals.received_call_observation(binding, calls, covered) else {
            return Ok(Err(unavailable()));
        };
        if original.site() != loan.call() || !homes.contains(&binding) {
            return Ok(Err(unavailable()));
        }
        let same_result_role = matches!(
            (loan.class(), original.result()),
            (crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::Object(_), LocalCallResultClassV1::Handle)
                | (crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(_), LocalCallResultClassV1::Nullable)
        );
        if !same_result_role {
            return Ok(Err(unavailable()));
        }
        ObjectReturnAcquisitionV1::Received(original.clone())
    };
    let arguments = match borrowed_actuals(
        loan.call(),
        BorrowedCallActualRequestV1::ObjectArguments(
            &loan,
            match &acquisition {
                ObjectReturnAcquisitionV1::Received(row) => {
                    row.local_binding().map(|(_, binding)| binding)
                }
                ObjectReturnAcquisitionV1::Direct { .. } => None,
            },
        ),
    )? {
        Some(BorrowedCallArgumentsV1::Object {
            qualification,
            arguments,
        }) if qualification == loan => arguments,
        None => ObjectCallSourceSupportV1::Unavailable,
        _ => return Ok(Err(unavailable())),
    };
    let obligation = TerminalObjectReturnObligationV1 {
        qualification: loan,
        return_site: statement.site().clone(),
        acquisition,
        exit_homes,
        arguments,
    };
    Ok(Ok(TerminalValueReturnV1::issue(
        input.owner(),
        statement.site().clone(),
        value.site().clone(),
        TerminalReturnedSourceV1::OwnedCall(Box::new(obligation)),
    )))
}
