//! Physical corroboration of the original ordered lexical call tree.
//! This root Call entry owner never classifies source or issues affine rows.
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::definitions::MirCall;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
type Binding = (BasicBlockId, MirInstruction);

/// Physical observations of the original ordered argument tree. These rows
/// cannot classify a source argument or choose a call target.
#[derive(Debug)]
pub(in crate::mir) enum LexicalCallArgumentProjectionV1 {
    Integer(Binding),
    Scalar(ExactLexicalReadV1),
    CallResult(Box<EmittedLexicalCallProjectionV1>),
    BorrowedLiteral {
        ordinal: u32,
        site: crate::mir::resolved_semantics::SourceExprSiteV1,
        formal: crate::mir::resolved_semantics::BindingRefV1,
        binding: Binding,
    },
    BorrowedRead {
        ordinal: u32,
        site: crate::mir::resolved_semantics::SourceExprSiteV1,
        formal: crate::mir::resolved_semantics::BindingRefV1,
        read: ExactLexicalReadV1,
        entry: Option<(u32, crate::mir::resolved_semantics::BindingRefV1, ValueId)>,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedLexicalCallProjectionV1 {
    receiver: ExactLexicalReadV1,
    arguments: Vec<LexicalCallArgumentProjectionV1>,
}

#[derive(Debug)]
pub(crate) struct EmittedLexicalCallProjectionV1 {
    row: LexicalInstanceCallDispositionRowV1,
    prepared: PreparedLexicalCallProjectionV1,
    invoke: Binding,
    projection: Binding,
}

impl PreparedLexicalCallProjectionV1 {
    fn validate_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        for argument in &self.arguments {
            match argument {
                LexicalCallArgumentProjectionV1::Integer(binding) => {
                    require_recorded(bindings, binding)?;
                }
                LexicalCallArgumentProjectionV1::BorrowedLiteral { binding, .. } => {
                    require_recorded(bindings, binding)?;
                }
                LexicalCallArgumentProjectionV1::BorrowedRead { .. } => {}
                LexicalCallArgumentProjectionV1::Scalar(_) => {}
                LexicalCallArgumentProjectionV1::CallResult(inner) => {
                    inner.validate_recorded(bindings)?;
                }
            }
        }
        Ok(())
    }

    pub(in crate::mir) fn new(
        receiver: ExactLexicalReadV1,
        arguments: Vec<LexicalCallArgumentProjectionV1>,
    ) -> Self {
        Self {
            receiver,
            arguments,
        }
    }

    pub(in crate::mir) fn materialize(
        &self,
        owner: FunctionOwnerIdV1,
        row: &LexicalInstanceCallDispositionRowV1,
        source: &[LocalCallArgumentV1],
    ) -> Result<MirCall, String> {
        self.materialize_using(owner, row, source, None)
    }

    pub(in crate::mir) fn materialize_with_ledger(
        &self,
        owner: FunctionOwnerIdV1,
        row: &LexicalInstanceCallDispositionRowV1,
        source: &[LocalCallArgumentV1],
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<MirCall, String> {
        self.materialize_using(owner, row, source, Some(ledger))
    }

    fn materialize_using(
        &self,
        owner: FunctionOwnerIdV1,
        row: &LexicalInstanceCallDispositionRowV1,
        source: &[LocalCallArgumentV1],
        ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    ) -> Result<MirCall, String> {
        if row.call_site().owner() != owner
            || self.arguments.len() != source.len()
            || source.len() != row.argument_sites().len()
            || source.len() != row.target().arity() as usize
        {
            return Err(freeze("lexical-i64/prepared-arity-or-owner"));
        }
        let receiver = self
            .receiver
            .value_for(owner, row.receiver_site().node(), row.receiver_binding())
            .map_err(|error| format!("[freeze:contract][lexical-i64/receiver/{error:?}]"))?;
        let mut values = Vec::with_capacity(source.len());
        for ((projection, source), site) in
            self.arguments.iter().zip(source).zip(row.argument_sites())
        {
            let value = match (projection, source) {
                (
                    LexicalCallArgumentProjectionV1::Integer((
                        _,
                        MirInstruction::Const {
                            dst,
                            value: crate::mir::ConstValue::Integer(actual),
                        },
                    )),
                    LocalCallArgumentV1::Integer(expected),
                ) if actual == expected => *dst,
                (
                    LexicalCallArgumentProjectionV1::Scalar(read),
                    LocalCallArgumentV1::Scalar(binding),
                ) => read
                    .value_for(owner, site.node(), *binding)
                    .map_err(|error| {
                        format!("[freeze:contract][lexical-i64/argument/{error:?}]")
                    })?,
                (
                    LexicalCallArgumentProjectionV1::CallResult(emitted),
                    LocalCallArgumentV1::CallResult(inner),
                ) if inner.site().owner() == owner
                    && inner.site().site() == site
                    && emitted.row.call_site() == inner.site() =>
                {
                    emitted.value_using(owner, inner.arguments(), ledger)?
                }
                (
                    projection,
                    LocalCallArgumentV1::BorrowedActual {
                        ordinal,
                        site: original,
                    },
                ) if original == site => borrowed_projection::value(
                    projection,
                    owner,
                    row,
                    *ordinal,
                    original,
                    ledger.ok_or_else(|| freeze("lexical-i64/borrowed-lender-missing"))?,
                )?,
                _ => return Err(freeze("lexical-i64/argument-projection-drift")),
            };
            values.push(value);
        }
        Ok(MirCall::new(
            None,
            crate::mir::definitions::Callee::SameModuleInstance {
                key: row.target().clone(),
                receiver,
            },
            values,
        ))
    }
}

impl EmittedLexicalCallProjectionV1 {
    pub(in crate::mir) fn copy_dependencies(
        &self,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<Vec<(crate::mir::resolved_semantics::OwnedExprSiteV1, Binding)>, String> {
        let mut dependencies = Vec::new();
        for argument in &self.prepared.arguments {
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                dependencies.extend(inner.copy_dependencies(ledger)?);
            } else {
                for copy in borrowed_projection::copies(
                    argument,
                    self.row.call_site().owner(),
                    &self.row,
                    ledger,
                )? {
                    dependencies.push((self.row.call_site().clone(), copy.clone()));
                }
            }
        }
        Ok(dependencies)
    }

    pub(in crate::mir) fn outer_bindings(&self) -> (&Binding, &Binding) {
        (&self.invoke, &self.projection)
    }

    pub(in crate::mir) fn original_row(&self) -> &LexicalInstanceCallDispositionRowV1 {
        &self.row
    }

    pub(in crate::mir) fn call_with_ledger(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<MirCall, String> {
        self.value_with_ledger(owner, source, ledger)?;
        self.prepared
            .materialize_with_ledger(owner, &self.row, source, ledger)
    }

    /// Exact original membership, including each nested node's own call site.
    pub(in crate::mir) fn has_producer_at(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        binding: &Binding,
    ) -> bool {
        if self.row.call_site() == site
            && (&self.invoke == binding
                || &self.projection == binding
                || self.prepared.arguments.iter().any(|argument| {
                    matches!(argument, LexicalCallArgumentProjectionV1::Integer(original)
                        if original == binding)
                        || matches!(argument, LexicalCallArgumentProjectionV1::BorrowedLiteral { binding: original, .. }
                            if original == binding)
                }))
        {
            return true;
        }
        self.prepared.arguments.iter().any(|argument| {
            matches!(argument, LexicalCallArgumentProjectionV1::CallResult(inner)
                if inner.has_producer_at(site, binding))
        })
    }

    pub(in crate::mir) fn call_site(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 {
        self.row.call_site()
    }

    pub(in crate::mir) fn validate_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        require_recorded(bindings, &self.invoke)?;
        require_recorded(bindings, &self.projection)?;
        self.prepared.validate_recorded(bindings)
    }

    pub(in crate::mir) fn new(
        row: LexicalInstanceCallDispositionRowV1,
        prepared: PreparedLexicalCallProjectionV1,
        invoke: Binding,
        projection: Binding,
    ) -> Self {
        Self {
            row,
            prepared,
            invoke,
            projection,
        }
    }

    pub(in crate::mir) fn value_for_source(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
    ) -> Result<ValueId, String> {
        self.value_using(owner, source, None)
    }

    pub(in crate::mir) fn value_with_ledger(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<ValueId, String> {
        self.value_using(owner, source, Some(ledger))
    }

    fn value_using(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    ) -> Result<ValueId, String> {
        let expected = self
            .prepared
            .materialize_using(owner, &self.row, source, ledger)?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
            normal_landing,
            ..
        } = &self.invoke.1
        else {
            return Err(freeze("lexical-i64/nested-invoke-shape"));
        };
        let MirInstruction::InvokeNormalResult { invoke_block, dst } = &self.projection.1 else {
            return Err(freeze("lexical-i64/nested-projection-shape"));
        };
        if self.row.result() != Some(InvokeCallResultKind::I64)
            || *call != expected
            || *invoke_block != self.invoke.0
            || *normal_landing != self.projection.0
        {
            return Err(freeze("lexical-i64/nested-producer-drift"));
        }
        Ok(*dst)
    }
}

fn require_recorded(bindings: &[Binding], expected: &Binding) -> Result<(), String> {
    if bindings
        .iter()
        .filter(|binding| *binding == expected)
        .count()
        != 1
    {
        return Err(freeze("lexical-i64/producer-record-drift"));
    }
    Ok(())
}

// Preserve the existing physical emission error vocabulary during owner move.
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/emission/{reason}]")
}

#[cfg(test)]
#[path = "lexical_projection_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "local_binding_group_tests.rs"]
mod local_group_tests;

#[path = "borrowed_projection.rs"]
mod borrowed_projection;

#[cfg(test)]
#[path = "borrowed_projection_tests.rs"]
mod borrowed_projection_tests;

#[cfg(test)]
#[path = "finished_projection_tests.rs"]
mod finished_projection_tests;

#[cfg(test)]
#[path = "lexical_terminal_tests.rs"]
mod terminal_tests;

#[cfg(test)]
#[path = "finished_copy_tests.rs"]
mod finished_copy_tests;
