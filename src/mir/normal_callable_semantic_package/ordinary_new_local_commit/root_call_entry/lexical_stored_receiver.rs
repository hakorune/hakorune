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
