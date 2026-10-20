//! Physical corroboration of the original ordered lexical call tree.
//! This root Call entry owner never classifies source or issues affine rows.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalBindingReadReceiptV1;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::definitions::MirCall;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
#[path = "lexical_argument_projection.rs"]
mod argument_projection;
type Binding = (BasicBlockId, MirInstruction);
use super::super::physical_boundary::FinishedBindings;

#[path = "call_packet_source.rs"]
mod call_packet_source;
#[path = "lexical_normal_projection.rs"]
mod normal_projection;
use normal_projection::{NormalCallProjectionV1, ArgumentCallCleanupV1};
pub(crate) use call_packet_source::PreparedStaticTerminalPacketV1;
pub(in crate::mir) use call_packet_source::{CallPacketSourceLoanV1, CallPacketSourceV1};

/// Selected Static Call packet checked with a borrowed publication row.
/// The owned handoff arrives only after collector admission succeeds.
#[derive(Debug)]
pub(crate) struct PendingStaticCallProjectionV1 {
    packet: PreparedStaticTerminalPacketV1,
    prepared: PreparedLexicalCallProjectionV1,
    call: MirCall,
}

impl PendingStaticCallProjectionV1 {
    pub(crate) fn new(
        packet: PreparedStaticTerminalPacketV1,
        prepared: PreparedLexicalCallProjectionV1,
        call: MirCall,
    ) -> Self {
        Self {
            packet,
            prepared,
            call,
        }
    }

    pub(crate) fn packet(&self) -> &PreparedStaticTerminalPacketV1 {
        &self.packet
    }

    pub(crate) fn expected_call(&self) -> &MirCall {
        &self.call
    }

    pub(crate) fn validate_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        self.prepared.validate_recorded_projected(bindings, None)
    }

    pub(crate) fn into_packet(
        self,
        publication: VerifiedStaticCallResultPublicationHandoffV1,
    ) -> (CallPacketSourceV1, PreparedLexicalCallProjectionV1) {
        (self.packet.into_packet(publication), self.prepared)
    }
}

/// Physical observations of the original ordered argument tree. These rows
/// cannot classify a source argument or choose a call target.
#[derive(Debug)]
pub(in crate::mir) enum LexicalCallArgumentProjectionV1 {
    Integer(Binding),
    Expression(Box<argument_projection::EmittedArgumentExpressionProjectionV1>),
    Scalar(ExactLexicalReadV1),
    CallResult(Box<EmittedLexicalCallProjectionV1>),
    BorrowedLiteral {
        ordinal: u32,
        site: crate::mir::resolved_semantics::SourceExprSiteV1,
        formal: crate::mir::resolved_semantics::BindingRefV1,
        binding: Binding,
    },
    BorrowedReadArgument {
        ordinal: u32,
        site: crate::mir::resolved_semantics::SourceExprSiteV1,
        formal: crate::mir::resolved_semantics::BindingRefV1,
        read: crate::mir::normal_callable_semantic_package::EmittedReadArgumentV1,
    },
    BorrowedRead {
        ordinal: u32,
        site: crate::mir::resolved_semantics::SourceExprSiteV1,
        formal: crate::mir::resolved_semantics::BindingRefV1,
        read: ExactLexicalReadV1,
        entry: Option<(u32, crate::mir::resolved_semantics::BindingRefV1, ValueId)>,
    },
    SelectedCanonicalRead {
        ordinal: u32,
        site: crate::mir::resolved_semantics::SourceExprSiteV1,
        read: CanonicalBindingReadReceiptV1,
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
    projection: NormalCallProjectionV1,
    argument_cleanup: Option<ArgumentCallCleanupV1>,
}

/// A checked physical packet held inside the unfinished caller. Only the
/// later publication terminal may exchange it for an emitted lexical packet.
pub(in crate::mir) struct PreparedSelectedStaticLoopCallProjectionV1 {
    packet: EmittedLexicalCallProjectionV1,
    fault: Binding,
}

impl PreparedSelectedStaticLoopCallProjectionV1 {
    pub(in crate::mir) fn source_site(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 {
        self.packet.call_site()
    }

    pub(in crate::mir) fn original_invoke_binding(&self) -> &(BasicBlockId, MirInstruction) {
        &self.packet.invoke
    }

    pub(in crate::mir) fn into_local_group_parts(
        self,
    ) -> Result<(
        crate::mir::resolved_semantics::OwnedExprSiteV1,
        Vec<(BasicBlockId, MirInstruction)>,
        EmittedLexicalCallProjectionV1,
    ), String> {
        let site = self.packet.call_site().clone();
        let bindings = vec![
            self.packet.invoke.clone(),
            self.packet.projection.value()?.clone(),
            self.fault,
        ];
        Ok((site, bindings, self.packet))
    }

    pub(in crate::mir) fn new(
        packet: crate::mir::normal_callable_semantic_package::VerifiedStaticLoopPacketSourceV1,
        read: CanonicalBindingReadReceiptV1,
        entry_value: ValueId,
        invoke: Binding,
        projection: Binding,
        fault: Binding,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<Self, String> {
        if !matches!((&invoke.1, &fault.1),
            (MirInstruction::Invoke { fault_frame, fault_landing, normal_landing, .. },
             MirInstruction::ReturnFault { fault_frame: returned })
                if *fault_landing == fault.0 && fault.0 != invoke.0
                    && fault.0 != *normal_landing && fault_frame == returned)
        {
            return Err(freeze("selected-static-packet/fault-binding-drift"));
        }
        let row = CallPacketSourceV1::selected_static_loop(packet, ledger)?;
        let CallPacketSourceLoanV1::SelectedStaticLoop { observation, .. } = row.loan() else {
            unreachable!("selected static source constructor");
        };
        let source = observation.arguments().to_vec();
        let [LocalCallArgumentV1::BorrowedActual { ordinal, site }] = source.as_slice() else {
            return Err(freeze("selected-static-packet/argument-shape"));
        };
        if read.physical_block() != invoke.0 || read.physical_value() != entry_value {
            return Err(freeze("selected-static-packet/read-entry-drift"));
        }
        let packet = EmittedLexicalCallProjectionV1 {
            row,
            prepared: PreparedLexicalCallProjectionV1 {
                receiver: LexicalReceiverProjectionV1::AbsentStatic,
                arguments: vec![LexicalCallArgumentProjectionV1::SelectedCanonicalRead {
                    ordinal: *ordinal,
                    site: site.clone(),
                    read,
                }],
            },
            invoke,
            projection: NormalCallProjectionV1::Value(projection),
            argument_cleanup: None,
        };
        packet.value_with_ledger(read.owner(), &source, ledger)?;
        Ok(Self { packet, fault })
    }
}

impl PreparedLexicalCallProjectionV1 {
    #[cfg(test)]
    fn validate_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        self.validate_recorded_projected(bindings, None)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded_projected(
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
                LexicalCallArgumentProjectionV1::Expression(_) => {
                    argument.validate_expression_recorded_v1(bindings, finishing)?;
                }
                LexicalCallArgumentProjectionV1::BorrowedLiteral { binding, .. } => {
                    require_recorded(bindings, binding, finishing)?;
                }
                LexicalCallArgumentProjectionV1::BorrowedReadArgument { read, .. } => {
                    for binding in read.bindings() { require_recorded(bindings, binding, finishing)?; }
                }
                LexicalCallArgumentProjectionV1::BorrowedRead { .. } => {}
                LexicalCallArgumentProjectionV1::SelectedCanonicalRead { .. } => {}
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
            (
                CallPacketSourceLoanV1::Static { .. }
                | CallPacketSourceLoanV1::SelectedStaticLoop { .. },
                None,
            ) => LexicalReceiverProjectionV1::AbsentStatic,
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
        self.materialize_in_context(owner, row, source, ledger.map(CallValidationContextV1::Lowering))
    }

    pub(in crate::mir) fn materialize_in_context(
        &self, owner: FunctionOwnerIdV1, row: CallPacketSourceLoanV1<'_>,
        source: &[LocalCallArgumentV1], context: Option<CallValidationContextV1<'_>>,
    ) -> Result<MirCall, String> {
        let ledger = context.map(CallValidationContextV1::ledger);
        if row.call_site().owner() != owner
            || self.arguments.len() != source.len()
            || source.len() != row.argument_sites().len()
            || source.len() != row.target().arity() as usize
        {
            return Err(freeze("lexical-i64/prepared-arity-or-owner"));
        }
        let callee = match row {
            CallPacketSourceLoanV1::Static { observation, .. } => {
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
            CallPacketSourceLoanV1::SelectedStaticLoop {
                packet,
                observation,
            } => {
                if !matches!(self.receiver, LexicalReceiverProjectionV1::AbsentStatic)
                    || source != observation.arguments()
                    || !packet.corroborates_local_call(observation)
                    || ledger
                        .ok_or_else(|| freeze("selected-static-packet/ledger-missing"))?
                        .local_call_for_owner(owner, observation.site().site())
                        != Some(observation)
                {
                    return Err(freeze("selected-static-packet/source-drift"));
                }
                crate::mir::definitions::Callee::Global(
                    row.target()
                        .canonical_global_target_v1()
                        .map_err(|_| freeze("selected-static-packet/global-target"))?,
                )
            }
            CallPacketSourceLoanV1::Instance(instance) => {
                if instance.source_target().stored_receiver().is_some() {
                    ledger.ok_or_else(|| freeze("stored-child/source-ledger-missing"))?
                        .recheck_stored_local_new_parent_v1(instance.source_target())?;
                }
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
                if let Some(ledger) = ledger {
                    if let Some(observation) = ledger.lexical_i64_call_source(instance.call_site()) {
                        ledger.checked_field_alias_receiver_v1(instance, observation, Some(receiver))?;
                    }
                }
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
            let value = projection.value_for_source_v1(source, owner, site, Some(row), context)?;
            values.push(value);
        }
        let mut call = MirCall::new(None, callee, values);
        if row.result() == Some(InvokeCallResultKind::OwnedString) {
            let context = context.ok_or_else(|| freeze("static-packet/ledger-missing"))?;
            call.text_argument_cleanup = self.deferred_argument_ends_v1(context)?.into_iter().map(|operation| {
                match operation {
                    InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value }) => Ok(value),
                    _ => Err(freeze("call-argument/duplicate-acquisition")),
                }
            }).collect::<Result<_, _>>()?;
        }
        Ok(call)
    }
}

impl EmittedLexicalCallProjectionV1 {
    pub(in crate::mir) fn copy_dependencies(
        &self,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<Vec<(crate::mir::resolved_semantics::OwnedExprSiteV1, Binding)>, String> {
        self.copy_dependencies_in_context(CallValidationContextV1::Lowering(ledger))
    }

    pub(in crate::mir) fn copy_dependencies_in_context(
        &self, context: CallValidationContextV1<'_>,
    ) -> Result<Vec<(crate::mir::resolved_semantics::OwnedExprSiteV1, Binding)>, String> {
        let mut dependencies = Vec::new();
        if let LexicalReceiverProjectionV1::StoredChild { parent, .. } = &self.prepared.receiver {
            dependencies.extend(parent.original_copy_dependencies_v1()?.iter()
                .map(|copy| (self.row.loan().call_site().clone(), copy.clone())));
        }
        for argument in &self.prepared.arguments {
            if let LexicalCallArgumentProjectionV1::Expression(_) = argument {
                dependencies.extend(argument.expression_copy_dependencies_v1(context)?.into_iter()
                    .map(|copy| (self.call_site().clone(), copy)));
                for call in argument.expression_calls_v1() {
                    dependencies.extend(call.copy_dependencies_in_context(context)?);
                }
            } else if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                dependencies.extend(inner.copy_dependencies_in_context(context)?);
            } else {
                for copy in borrowed_projection::copies_in_context(
                    argument,
                    self.row.loan().call_site().owner(),
                    self.row.loan(),
                    context,
                )? {
                    dependencies.push((self.row.loan().call_site().clone(), copy.clone()));
                }
            }
        }
        Ok(dependencies)
    }

    pub(in crate::mir) fn invoke_binding(&self) -> &Binding { &self.invoke }

    /// Value consumers must explicitly reject a Unit normal continuation.
    pub(in crate::mir) fn outer_bindings(&self) -> Result<(&Binding, &Binding), String> {
        Ok((&self.invoke, self.projection.value()?))
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
        self.call_in_context(owner, source, CallValidationContextV1::Lowering(ledger))
    }

    pub(in crate::mir) fn call_in_context(
        &self, owner: FunctionOwnerIdV1, source: &[LocalCallArgumentV1], context: CallValidationContextV1<'_>,
    ) -> Result<MirCall, String> {
        self.normal_result_using(owner, source, Some(context))?;
        self.prepared.materialize_in_context(owner, self.row.loan(), source, Some(context))
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
        self.visit_original_nodes_in_context_v1(owner, source, CallValidationContextV1::Lowering(ledger), visit)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn visit_original_nodes_in_context_v1(
        &self, owner: FunctionOwnerIdV1, source: &[LocalCallArgumentV1], context: CallValidationContextV1<'_>,
        visit: &mut impl FnMut(&Self, &[LocalCallArgumentV1]) -> Result<(), String>,
    ) -> Result<(), String> {
        self.call_in_context(owner, source, context)?;
        for (argument, original) in self.prepared.arguments.iter().zip(source) {
            argument.visit_expression_calls_v1(owner, context, visit)?;
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                let LocalCallArgumentV1::CallResult(original) = original else {
                    return Err(freeze("lexical-i64/nested-source-missing"));
                };
                inner.visit_original_nodes_in_context_v1(owner, original.arguments(), context, visit)?;
            }
        }
        visit(self, source)
    }

    /// Scope membership includes the SAME argument expression's cleanup graph.
    /// It is deliberately separate from the finished value-producer lender.
    pub(in crate::mir) fn has_original_scope_binding_at(
        &self, site: &crate::mir::resolved_semantics::OwnedExprSiteV1, binding: &Binding,
    ) -> bool {
        self.has_producer_at(site, binding)
            || self.call_site() == site && self.prepared.arguments.iter()
                .any(|argument| argument.expression_bindings_v1().contains(binding))
    }

    /// Exact original membership, including each nested node's own call site.
    pub(in crate::mir) fn has_producer_at(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        binding: &Binding,
    ) -> bool {
        if self.row.loan().call_site() == site
            && (&self.invoke == binding
                || matches!(&self.projection, NormalCallProjectionV1::Value(original) if original == binding)
                || matches!(&self.prepared.receiver,
                    LexicalReceiverProjectionV1::StoredChild { read, .. } if read == binding)
                || self.prepared.arguments.iter().any(|argument| {
                    matches!(argument, LexicalCallArgumentProjectionV1::Integer(original)
                        if original == binding)
                        || matches!(argument, LexicalCallArgumentProjectionV1::BorrowedLiteral { binding: original, .. }
                            if original == binding)
                        || matches!(argument, LexicalCallArgumentProjectionV1::BorrowedReadArgument { read, .. }
                            if read.producer_bindings().any(|original| original == binding))
                }))
        {
            return true;
        }
        self.prepared.arguments.iter().any(|argument| {
            matches!(argument, LexicalCallArgumentProjectionV1::CallResult(inner)
                if inner.has_producer_at(site, binding))
                || argument.expression_calls_v1().any(|call| call.has_producer_at(site, binding))
        })
    }

    pub(in crate::mir) fn call_site(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 {
        self.row.loan().call_site()
    }

    pub(in crate::mir) fn stored_receiver_read_v1(
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
        self.validate_normal_recorded(bindings, finishing)?;
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
            projection: NormalCallProjectionV1::Value(projection),
            argument_cleanup: None,
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
            projection: NormalCallProjectionV1::Value(projection),
            argument_cleanup: None,
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
        let row = CallPacketSourceV1::static_i64(original, publication, ledger)?;
        let packet = Self {
            row,
            prepared: PreparedLexicalCallProjectionV1 {
                receiver: LexicalReceiverProjectionV1::AbsentStatic,
                arguments,
            },
            invoke,
            projection: NormalCallProjectionV1::Value(projection),
            argument_cleanup: None,
        };
        let CallPacketSourceLoanV1::Static { observation, .. } = packet.row.loan() else {
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
        self.value_using(owner, source, Some(CallValidationContextV1::Lowering(ledger)))
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

#[path = "finished_read_arguments.rs"]
mod finished_read_arguments;

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
