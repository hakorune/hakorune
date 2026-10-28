//! Physical observations of the same source-selected stored receiver.
//! Fault obligations are checked by the existing original Call lifetime owner.
use super::*;

#[derive(Debug)]
pub(in crate::mir) enum StoredReceiverFaultV1 {
    Call,
    Return(crate::mir::resolved_semantics::SourceStmtSiteV1),
    Constructor,
}

#[derive(Debug)]
pub(in crate::mir) enum StoredReceiverReadV1 {
    Exact(Binding),
    Checked {
        invoke: Binding,
        normal: Binding,
        fault: Vec<Binding>,
        owner: StoredReceiverFaultV1,
    },
}

impl From<Binding> for StoredReceiverReadV1 {
    fn from(read: Binding) -> Self {
        Self::Exact(read)
    }
}

impl StoredReceiverReadV1 {
    pub(super) fn primary(&self) -> &Binding {
        match self {
            Self::Exact(read) => read,
            Self::Checked { invoke, .. } => invoke,
        }
    }
    pub(super) fn producer_bindings(&self) -> Vec<&Binding> {
        match self {
            Self::Exact(read) => vec![read],
            Self::Checked { invoke, normal, .. } => vec![invoke, normal],
        }
    }
    pub(super) fn bindings(&self) -> Vec<&Binding> {
        let mut rows = self.producer_bindings();
        if let Self::Checked { fault, .. } = self {
            rows.extend(fault);
        }
        rows
    }
    pub(super) fn is_checked(&self) -> bool {
        matches!(self, Self::Checked { .. })
    }

    pub(super) fn value(
        &self,
        base: ValueId,
        target: &crate::mir::normal_callable_semantic_package::LexicalInstanceCallSourceTargetV1,
        context: Option<CallValidationContextV1<'_>>,
    ) -> Result<ValueId, String> {
        let (_, _, field, child) = target
            .stored_receiver()
            .ok_or_else(|| freeze("lexical-terminal/receiver-arm-drift"))?;
        match self {
            Self::Exact((
                _,
                MirInstruction::ObjectFieldGet {
                    dst,
                    base: actual,
                    field: actual_field,
                },
            )) if *actual == base && *actual_field == field && *dst != base => {
                if context
                    .map(|context| {
                        context
                            .ledger()
                            .stored_receiver_uses_dynamic_field_v1(target)
                    })
                    .transpose()?
                    .unwrap_or(false)
                {
                    return Err(freeze("lexical-terminal/stored-read-kind"));
                }
                Ok(*dst)
            }
            Self::Checked { invoke, normal, .. } => {
                if !context
                    .ok_or_else(|| freeze("lexical-terminal/stored-read-kind"))?
                    .ledger()
                    .stored_receiver_uses_dynamic_field_v1(target)?
                {
                    return Err(freeze("lexical-terminal/stored-read-kind"));
                }
                match (&invoke.1, &normal.1) {
                    (
                        MirInstruction::Invoke {
                            operation:
                                InvokeOperation::ObjectFieldBorrow {
                                    base: actual,
                                    field: actual_field,
                                    child: actual_child,
                                    nullable: false,
                                },
                            normal_landing,
                            fault_landing,
                            ..
                        },
                        MirInstruction::InvokeNormalResult { invoke_block, dst },
                    ) if *actual == base
                        && *actual_field == field
                        && *actual_child == child
                        && *invoke_block == invoke.0
                        && *normal_landing == normal.0
                        && normal_landing != fault_landing
                        && *dst != base =>
                    {
                        Ok(*dst)
                    }
                    _ => Err(freeze("lexical-terminal/stored-read-drift")),
                }
            }
            _ => Err(freeze("lexical-terminal/stored-read-kind")),
        }
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// Only the original lexical receiver observation, never a stored-child
    /// result or an absent Static receiver, lends this source read.
    pub(in crate::mir::normal_callable_semantic_package) fn lexical_receiver_read_v1(
        &self,
    ) -> Option<&ExactLexicalReadV1> {
        match (self.row.loan(), &self.prepared.receiver) {
            (CallPacketSourceLoanV1::Instance(row), LexicalReceiverProjectionV1::Lexical(read))
                if row.source_target().stored_receiver().is_none() => Some(read),
            _ => None,
        }
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// Independently tie the same self source to the actual implicit receiver
    /// parameter and retained Copy chain, including the finished function.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_self_entry_receiver_v1(
        &self, observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let CallPacketSourceLoanV1::Instance(row) = self.row.loan() else { return Ok(()); };
        if !row.source_target().is_self_receiver()
            || !matches!(observation.result_source(), Some(crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::Ordinary(_))) {
            return Ok(());
        }
        if !context.ledger().checked_self_entry_receiver_v1(row.source_target(), observation)? {
            return Err(freeze("lexical-terminal/receiver-arm-drift"));
        }
        let binding = row.receiver_binding()?;
        let value = *function.params.first().ok_or_else(|| freeze("lexical-terminal/receiver-arm-drift"))?;
        let read = self.lexical_receiver_read_v1().ok_or_else(|| freeze("lexical-terminal/receiver-arm-drift"))?;
        read.value_for(row.call_site().owner(), row.receiver_site().node(), binding).map_err(|issue| issue.to_string())?;
        for original in read.loan_forwarded_copies(binding, value)? {
            let actual = match finishing {
                Some(projection) => projection.relation_binding(function, original)?
                    .ok_or_else(|| freeze("lexical-terminal/receiver-arm-drift"))?,
                None => original.clone(),
            };
            if !super::super::super::physical_boundary::check_binding(function, None, actual.0, &actual.1)? {
                return Err(freeze("lexical-terminal/receiver-arm-drift"));
            }
        }
        Ok(())
    }
}
