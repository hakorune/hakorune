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
use super::super::physical_boundary::FinishedBindings;

#[path = "call_packet_source.rs"]
mod call_packet_source;
pub(in crate::mir) use call_packet_source::{CallPacketSourceLoanV1, CallPacketSourceV1};

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
enum LexicalReceiverProjectionV1 {
    AbsentStatic,
    Lexical(ExactLexicalReadV1),
    StoredChild {
        parent: ExactLexicalReadV1,
        read: Binding,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedLexicalCallProjectionV1 {
    receiver: LexicalReceiverProjectionV1,
    arguments: Vec<LexicalCallArgumentProjectionV1>,
}

#[derive(Debug)]
pub(crate) struct EmittedLexicalCallProjectionV1 {
    row: CallPacketSourceV1,
    prepared: PreparedLexicalCallProjectionV1,
    invoke: Binding,
    projection: Binding,
}

impl PreparedLexicalCallProjectionV1 {
    #[cfg(test)]
    fn validate_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        self.validate_recorded_projected(bindings, None)
    }

    fn validate_recorded_projected(
        &self,
        bindings: &[Binding],
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        if let LexicalReceiverProjectionV1::StoredChild { read, .. } = &self.receiver {
            require_recorded(bindings, read, finishing)?;
        }
        for argument in &self.arguments {
            match argument {
                LexicalCallArgumentProjectionV1::Integer(binding) => {
                    require_recorded(bindings, binding, finishing)?;
                }
                LexicalCallArgumentProjectionV1::BorrowedLiteral { binding, .. } => {
                    require_recorded(bindings, binding, finishing)?;
                }
                LexicalCallArgumentProjectionV1::BorrowedRead { .. } => {}
                LexicalCallArgumentProjectionV1::Scalar(_) => {}
                LexicalCallArgumentProjectionV1::CallResult(inner) => {
                    inner.validate_recorded_projected(bindings, finishing)?;
                }
            }
        }
        Ok(())
    }

    pub(in crate::mir) fn for_source(
        source: CallPacketSourceLoanV1<'_>,
        receiver: Option<ExactLexicalReadV1>,
        arguments: Vec<LexicalCallArgumentProjectionV1>,
    ) -> Result<Self, String> {
        let receiver = match (source, receiver) {
            (CallPacketSourceLoanV1::Instance(_), Some(read)) => {
                LexicalReceiverProjectionV1::Lexical(read)
            }
            (CallPacketSourceLoanV1::QualifiedStatic { .. }, None) => {
                LexicalReceiverProjectionV1::AbsentStatic
            }
            _ => return Err(freeze("lexical-i64/source-receiver-drift")),
        };
        Ok(Self {
            receiver,
            arguments,
        })
    }

    pub(in crate::mir) fn new(
        receiver: ExactLexicalReadV1,
        arguments: Vec<LexicalCallArgumentProjectionV1>,
    ) -> Self {
        Self {
            receiver: LexicalReceiverProjectionV1::Lexical(receiver),
            arguments,
        }
    }

    pub(in crate::mir) fn with_stored_receiver(mut self, read: Binding) -> Result<Self, String> {
        let LexicalReceiverProjectionV1::Lexical(parent) = self.receiver else {
            return Err(freeze("lexical-terminal/duplicate-stored-receiver"));
        };
        self.receiver = LexicalReceiverProjectionV1::StoredChild { parent, read };
        Ok(self)
    }

    pub(in crate::mir) fn materialize(
        &self,
        owner: FunctionOwnerIdV1,
        row: &LexicalInstanceCallDispositionRowV1,
        source: &[LocalCallArgumentV1],
    ) -> Result<MirCall, String> {
        self.materialize_using(owner, CallPacketSourceLoanV1::Instance(row), source, None)
    }

    pub(in crate::mir) fn materialize_with_ledger(
        &self,
        owner: FunctionOwnerIdV1,
        row: &LexicalInstanceCallDispositionRowV1,
        source: &[LocalCallArgumentV1],
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<MirCall, String> {
        self.materialize_using(
            owner,
            CallPacketSourceLoanV1::Instance(row),
            source,
            Some(ledger),
        )
    }

    pub(in crate::mir) fn materialize_using(
        &self,
        owner: FunctionOwnerIdV1,
        row: CallPacketSourceLoanV1<'_>,
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
        let callee = match row {
            CallPacketSourceLoanV1::QualifiedStatic { observation, .. } => {
                row.validate_static(ledger.ok_or_else(|| freeze("static-packet/ledger-missing"))?)?;
                if !matches!(self.receiver, LexicalReceiverProjectionV1::AbsentStatic)
                    || source != observation.arguments()
                {
                    return Err(freeze("static-packet/receiver-or-arguments"));
                }
                crate::mir::definitions::Callee::Global(
                    row.target()
                        .canonical_global_target_v1()
                        .map_err(|_| freeze("static-packet/global-target"))?,
                )
            }
            CallPacketSourceLoanV1::Instance(instance) => {
                let receiver = match (&self.receiver, instance.source_target().stored_receiver()) {
                    (LexicalReceiverProjectionV1::Lexical(read), None) => read
                        .value_for(
                            owner,
                            instance.receiver_site().node(),
                            instance.receiver_binding()?,
                        )
                        .map_err(|error| {
                            format!("[freeze:contract][lexical-i64/receiver/{error:?}]")
                        })?,
                    (
                        LexicalReceiverProjectionV1::StoredChild { parent, read },
                        Some((binding, site, field, child)),
                    ) => {
                        if field.object() == child {
                            return Err(freeze("lexical-terminal/stored-child-identity"));
                        }
                        let base =
                            parent
                                .value_for(owner, site.node(), binding)
                                .map_err(|error| {
                                    format!("[freeze:contract][lexical-terminal/parent/{error:?}]")
                                })?;
                        let MirInstruction::ObjectFieldGet {
                            dst,
                            base: actual,
                            field: actual_field,
                        } = &read.1
                        else {
                            return Err(freeze("lexical-terminal/stored-read-kind"));
                        };
                        if *actual != base || *actual_field != field || *dst == base {
                            return Err(freeze("lexical-terminal/stored-read-drift"));
                        }
                        *dst
                    }
                    _ => return Err(freeze("lexical-terminal/receiver-arm-drift")),
                };
                crate::mir::definitions::Callee::SameModuleInstance {
                    key: instance.target().clone(),
                    receiver,
                }
            }
        };
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
                    && emitted.call_site() == inner.site() =>
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
        Ok(MirCall::new(None, callee, values))
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
                    self.row.loan().call_site().owner(),
                    self.row.loan(),
                    ledger,
                )? {
                    dependencies.push((self.row.loan().call_site().clone(), copy.clone()));
                }
            }
        }
        Ok(dependencies)
    }

    pub(in crate::mir) fn outer_bindings(&self) -> (&Binding, &Binding) {
        (&self.invoke, &self.projection)
    }

    pub(in crate::mir) fn original_source(&self) -> CallPacketSourceLoanV1<'_> {
        self.row.loan()
    }

    pub(in crate::mir) fn original_row(
        &self,
    ) -> Result<&LexicalInstanceCallDispositionRowV1, String> {
        self.row.loan().require_instance()
    }

    pub(in crate::mir) fn call_with_ledger(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<MirCall, String> {
        self.value_with_ledger(owner, source, ledger)?;
        self.prepared
            .materialize_using(owner, self.row.loan(), source, Some(ledger))
    }

    /// Walk the original ordered tree against its sealed source, in evaluation
    /// order. This lends affine rows; it neither clones nor reissues them.
    pub(in crate::mir::normal_callable_semantic_package) fn visit_original_nodes_v1(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        visit: &mut impl FnMut(&Self, &[LocalCallArgumentV1]) -> Result<(), String>,
    ) -> Result<(), String> {
        self.call_with_ledger(owner, source, ledger)?;
        for (argument, original) in self.prepared.arguments.iter().zip(source) {
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                let LocalCallArgumentV1::CallResult(original) = original else {
                    return Err(freeze("lexical-i64/nested-source-missing"));
                };
                inner.visit_original_nodes_v1(owner, original.arguments(), ledger, visit)?;
            }
        }
        visit(self, source)
    }

    /// Exact original membership, including each nested node's own call site.
    pub(in crate::mir) fn has_producer_at(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        binding: &Binding,
    ) -> bool {
        if self.row.loan().call_site() == site
            && (&self.invoke == binding
                || &self.projection == binding
                || matches!(&self.prepared.receiver,
                    LexicalReceiverProjectionV1::StoredChild { read, .. } if read == binding)
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
        self.row.loan().call_site()
    }

    pub(in crate::mir::normal_callable_semantic_package) fn stored_receiver_read_v1(
        &self,
    ) -> Option<&Binding> {
        match &self.prepared.receiver {
            LexicalReceiverProjectionV1::StoredChild { read, .. } => Some(read),
            LexicalReceiverProjectionV1::Lexical(_) | LexicalReceiverProjectionV1::AbsentStatic => {
                None
            }
        }
    }

    pub(in crate::mir) fn validate_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        self.validate_recorded_projected(bindings, None)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded_projected(
        &self,
        bindings: &[Binding],
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        require_recorded(bindings, &self.invoke, finishing)?;
        require_recorded(bindings, &self.projection, finishing)?;
        self.prepared
            .validate_recorded_projected(bindings, finishing)
    }

    pub(in crate::mir) fn new(
        row: LexicalInstanceCallDispositionRowV1,
        prepared: PreparedLexicalCallProjectionV1,
        invoke: Binding,
        projection: Binding,
    ) -> Self {
        Self {
            row: CallPacketSourceV1::instance(row),
            prepared,
            invoke,
            projection,
        }
    }

    pub(in crate::mir) fn from_source(
        row: CallPacketSourceV1,
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

    pub(in crate::mir::normal_callable_semantic_package) fn new_static(
        original: std::rc::Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1>,
        publication: crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
        arguments: Vec<LexicalCallArgumentProjectionV1>,
        invoke: Binding,
        projection: Binding,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<Self, String> {
        let row = CallPacketSourceV1::qualified_static(original, publication, ledger)?;
        let packet = Self {
            row,
            prepared: PreparedLexicalCallProjectionV1 {
                receiver: LexicalReceiverProjectionV1::AbsentStatic,
                arguments,
            },
            invoke,
            projection,
        };
        let CallPacketSourceLoanV1::QualifiedStatic { observation, .. } = packet.row.loan() else {
            unreachable!("static source constructor");
        };
        packet.value_with_ledger(observation.owner(), observation.arguments(), ledger)?;
        Ok(packet)
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
            .materialize_using(owner, self.row.loan(), source, ledger)?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    call,
                    result: emitted_result,
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
        // The disposition row names the one result contract this packet may
        // carry — `I64` for the scalar lane, `NullableHandle` for the
        // checked-release nullable lane. Anything else is producer drift.
        if self.row.loan().result() != Some(*emitted_result)
            || *call != expected
            || *invoke_block != self.invoke.0
            || *normal_landing != self.projection.0
        {
            return Err(freeze("lexical-i64/nested-producer-drift"));
        }
        Ok(*dst)
    }
}

fn require_recorded(
    bindings: &[Binding],
    original: &Binding,
    finishing: Option<&FinishedBindings>,
) -> Result<(), String> {
    let mapped = finishing
        .map(|projection| {
            projection
                .binding(original.0, &original.1)?
                .ok_or_else(|| freeze("lexical-i64/producer-record-drift"))
        })
        .transpose()?;
    let expected = mapped.as_ref().unwrap_or(original);
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

#[cfg(test)]
#[path = "static_packet_tests.rs"]
mod static_packet_tests;
