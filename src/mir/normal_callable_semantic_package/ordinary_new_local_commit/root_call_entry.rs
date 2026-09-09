//! Existing root exit progress retains the same affine Call source row.
//! Physical observations do not issue a target, result or cleanup obligation.
use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1;
use crate::mir::ConstValue;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn terminal_call_arguments(&self) -> Option<&[i64]> {
        self.call_source_completion()
            .map(|(_, terminal)| terminal.arguments())
    }

    /// Borrow the root-owned terminal Call only when the selected lowering
    /// owner is that same source owner. Child completions may retain their
    /// own terminal relations while the root Call is still present; exposing
    /// the root arguments to those children would make the physical Call
    /// probe reject an otherwise valid child return.
    pub(crate) fn terminal_call_arguments_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&[i64]> {
        self.call_source_completion()
            .filter(|(completion, terminal)| {
                completion.owner() == owner && terminal.owner() == owner
            })
            .map(|(_, terminal)| terminal.arguments())
    }

    pub(crate) fn record_root_call_exit(
        &self,
        owner: FunctionOwnerIdV1,
        row: AppMainDirectCallDispositionRowV1,
        arguments: Vec<(BasicBlockId, MirInstruction)>,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        row.lifecycle_emission()
            .map_err(|_| freeze("call-source-mismatch"))?;
        self.record_root_home_exit_with_entry(
            owner,
            origins,
            bindings,
            RootHomeExitEntry::Call {
                row,
                arguments,
                invoke,
                projection,
                frame,
            },
        )
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn rebind_root_call_entry(
        &self,
        owner: FunctionOwnerIdV1,
        projection: &super::super::physical_boundary::FinishedBindings,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(progress) = exits.get_mut(&owner) else {
            return Err(freeze("root-call-entry-missing"));
        };
        let RootHomeExitProgress::Emitted { bindings, entry, .. } = progress else {
            return Err(freeze("root-call-entry-missing"));
        };
        let RootHomeExitEntry::Call {
            arguments,
            invoke,
            projection: result_projection,
            frame,
            ..
        } = entry
        else {
            return Ok(());
        };
        let map = |binding: &(BasicBlockId, MirInstruction)| {
            projection
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("root-call-binding-contracted"))
        };
        let mapped_arguments = arguments.iter().map(map).collect::<Result<Vec<_>, _>>()?;
        let mapped_invoke = map(invoke)?;
        let mapped_projection = map(result_projection)?;
        let mapped_frame = map(frame)?;
        let mapped_cleanup = projection.bindings(bindings)?;
        *arguments = mapped_arguments;
        *invoke = mapped_invoke;
        *result_projection = mapped_projection;
        *frame = mapped_frame;
        *bindings = mapped_cleanup;
        Ok(())
    }

    pub(crate) fn take_finalized_root_call(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<(RootHomeExitEntry, Vec<(BasicBlockId, MirInstruction)>)>, String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(progress) = exits.get_mut(&owner) else {
            return Ok(None);
        };
        let state = std::mem::replace(progress, RootHomeExitProgress::Finalized);
        match state {
            RootHomeExitProgress::Emitted {
                bindings,
                entry:
                    RootHomeExitEntry::Call {
                        row,
                        arguments,
                        invoke,
                        projection,
                        frame,
                    },
                ..
            } => Ok(Some((
                RootHomeExitEntry::Call {
                    row,
                    arguments,
                    invoke,
                    projection,
                    frame,
                },
                bindings,
            ))),
            RootHomeExitProgress::Emitted {
                origins,
                bindings,
                entry: RootHomeExitEntry::Plain,
            } => {
                *progress = RootHomeExitProgress::Emitted {
                    origins,
                    bindings,
                    entry: RootHomeExitEntry::Plain,
                };
                Ok(None)
            }
            RootHomeExitProgress::Finalized => Err(freeze("root-call-already-finalized")),
            other => {
                *progress = other;
                Err(freeze("root-call-entry-unavailable"))
            }
        }
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_call_entry(
        &self,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        entry: &RootHomeExitEntry,
        cleanup: &[(BasicBlockId, MirInstruction)],
    ) -> Result<(), String> {
        let Some((_, terminal)) = self.call_source_completion() else {
            return if matches!(entry, RootHomeExitEntry::Plain) {
                Ok(())
            } else {
                Err(freeze("call-source-missing"))
            };
        };
        let RootHomeExitEntry::Call {
            row,
            arguments,
            invoke,
            projection,
            frame,
        } = entry
        else {
            return Err(freeze("call-entry-missing"));
        };
        if arguments.len() != terminal.arguments().len()
            || row.argument_sites().len() != arguments.len()
        {
            return Err(freeze("call-argument-count"));
        }
        let mut values = Vec::with_capacity(arguments.len());
        for ((_, instruction), literal) in arguments.iter().zip(terminal.arguments()) {
            match instruction {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                } if value == literal => values.push(*dst),
                _ => return Err(freeze("call-argument-drift")),
            }
        }
        let expected = row
            .lifecycle_emission()
            .map_err(|_| freeze("call-source-mismatch"))?
            .materialize_call(None, values)
            .map_err(|_| freeze("call-projection-failed"))?;
        if !matches!(&invoke.1, MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, result: InvokeCallResultKind::I64 }, ..
        } if *call == expected)
        {
            return Err(freeze("call-target-drift"));
        }
        if !matches!((&invoke.1, &frame.1), (MirInstruction::Invoke { fault_frame, .. },
            MirInstruction::FaultFrameEnter { dst, mode: crate::mir::instruction::FaultFrameMode::RootOwned })
                if fault_frame == dst)
        {
            return Err(freeze("call-frame-drift"));
        }
        for (id, instruction) in arguments.iter().chain([invoke, projection, frame]) {
            if !super::super::physical_boundary::check_binding(
                function,
                finishing,
                *id,
                instruction,
            )? {
                return Err(freeze("call-binding-drift"));
            }
        }
        let mapped = |binding: &(BasicBlockId, MirInstruction)| match finishing {
            Some(p) => p
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("call-binding-missing")),
            None => Ok(binding.clone()),
        };
        let cleanup = match finishing {
            Some(p) => p.bindings(cleanup)?,
            None => cleanup.to_vec(),
        };
        super::super::root_cleanup_graph::call::ingress(
            function,
            &cleanup,
            &mapped(invoke)?,
            &mapped(projection)?,
        )?;
        Ok(())
    }
}

impl RootHomeExitEntry {
    pub(crate) fn call_row(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1>
    {
        match self {
            Self::Call { row, .. } => Some(row),
            Self::Plain => None,
        }
    }

    pub(crate) fn call_arguments(&self) -> Option<&[(BasicBlockId, MirInstruction)]> {
        match self {
            Self::Call { arguments, .. } => Some(arguments),
            Self::Plain => None,
        }
    }

    pub(crate) fn call_invoke(&self) -> Option<&(BasicBlockId, MirInstruction)> {
        match self {
            Self::Call { invoke, .. } => Some(invoke),
            Self::Plain => None,
        }
    }

    pub(crate) fn call_projection(&self) -> Option<&(BasicBlockId, MirInstruction)> {
        match self {
            Self::Call { projection, .. } => Some(projection),
            Self::Plain => None,
        }
    }

    pub(crate) fn call_frame(&self) -> Option<&(BasicBlockId, MirInstruction)> {
        match self {
            Self::Call { frame, .. } => Some(frame),
            Self::Plain => None,
        }
    }
}

impl RootHomeExitEntry {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn append_bindings(
        &self,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) {
        if let Self::Call {
            arguments,
            invoke,
            projection,
            frame,
            ..
        } = self
        {
            bindings.extend_from_slice(arguments);
            bindings.push(invoke.clone());
            bindings.push(projection.clone());
            bindings.push(frame.clone());
        }
    }
}
