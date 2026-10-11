//! Existing retained call packets, lent without changing their source authority.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    PreparedBorrowedFormalActualV1, PreparedLoopStaticBodyDetachedPacketV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
use crate::mir::MirModule;
use std::collections::BTreeSet;

/// Transport view only: a detached LoopBody is never a Home lexical packet.
#[derive(Clone, Copy)]
pub(in crate::mir) enum FinalizedBorrowedCallPacketV1<'source> {
    Lexical(&'source EmittedLexicalCallProjectionV1),
    LoopBody(&'source PreparedLoopStaticBodyDetachedPacketV1),
}

impl<'source> FinalizedBorrowedCallPacketV1<'source> {
    pub(in crate::mir) fn call_site(self) -> &'source OwnedExprSiteV1 {
        match self {
            Self::Lexical(packet) => packet.call_site(),
            Self::LoopBody(packet) => packet.source_site(),
        }
    }

    pub(in crate::mir) fn target(self) -> &'source CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::Lexical(packet) => packet.original_source().target(),
            Self::LoopBody(packet) => packet.original_source().target(),
        }
    }

    pub(in crate::mir) fn callee_owner(self) -> FunctionOwnerIdV1 {
        match self {
            Self::Lexical(packet) => packet.original_source().callee_owner(),
            Self::LoopBody(packet) => packet.original_source().callee_owner(),
        }
    }

    pub(in crate::mir) fn selects_borrowed_source(self, arguments: &[LocalCallArgumentV1]) -> bool {
        match self {
            Self::Lexical(_) => arguments
                .iter()
                .any(|argument| matches!(argument, LocalCallArgumentV1::BorrowedActual { .. })),
            Self::LoopBody(_) => true,
        }
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// Return loans tied to the retained pool, not the visitor's callback borrow.
    pub(in crate::mir) fn borrowed_finalized_call_actuals_v1(
        &self,
        packet: FinalizedBorrowedCallPacketV1<'_>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        match packet {
            FinalizedBorrowedCallPacketV1::Lexical(packet) => {
                self.borrowed_packet_actuals_v1(packet)
            }
            FinalizedBorrowedCallPacketV1::LoopBody(packet) => {
                let retained = self
                    .loop_body_calls
                    .get(packet.source_site())
                    .ok_or_else(|| freeze("finished-loop-body/retained-source-missing"))?;
                if !std::ptr::eq(retained, packet) || !retained.corroborates_retained_scalar() {
                    return Err(freeze("finished-loop-body/retained-source-drift"));
                }
                Ok(Some(std::slice::from_ref(retained.scalar_actual())))
            }
        }
    }

    /// Both coverage consumers inspect the same original pools and final producers.
    pub(in crate::mir) fn visit_finalized_borrowed_call_nodes_v1(
        &self,
        module: &MirModule,
        mut visit: impl FnMut(
            FinalizedBorrowedCallPacketV1<'_>,
            &[LocalCallArgumentV1],
            &MirFunction,
            (BasicBlockId, usize),
            &[((BasicBlockId, MirInstruction), (BasicBlockId, usize))],
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let mut sites = BTreeSet::new();
        let mut coordinates = BTreeSet::new();
        self.visit_finalized_lexical_call_nodes_v1(
            module,
            |_, _, packet, arguments, caller, coordinate, copies| {
                if !sites.insert(packet.call_site().clone())
                    || !coordinates.insert((caller.signature.name.clone(), coordinate))
                {
                    return Err(freeze("final-borrowed-call/duplicate"));
                }
                visit(
                    FinalizedBorrowedCallPacketV1::Lexical(packet),
                    arguments,
                    caller,
                    coordinate,
                    copies,
                )
            },
        )?;
        for (site, packet) in &self.loop_body_calls {
            let (caller, coordinate) = self
                .ledger
                .finished_loop_body_call_producer_v1(site, packet, module)?;
            if !sites.insert(site.clone())
                || !coordinates.insert((caller.signature.name.clone(), coordinate))
            {
                return Err(freeze("final-borrowed-call/duplicate"));
            }
            visit(
                FinalizedBorrowedCallPacketV1::LoopBody(packet),
                &[],
                caller,
                coordinate,
                &[],
            )?;
        }
        Ok(())
    }
}
