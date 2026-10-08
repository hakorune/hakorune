//! Original call-tree loans joined to the same owner's finished instructions.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
use crate::mir::MirModule;
use std::collections::BTreeSet;

/// The outer source continuation. Nested nodes keep their enclosing context;
/// their own site and target always come from their original Taken row.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir) enum FinalizedLexicalCallContextV1<'source> {
    Local {
        group_site: &'source OwnedExprSiteV1,
        declaration: &'source SourceBindingSiteV1,
        binding: BindingRefV1,
    },
    Discard {
        group_site: &'source OwnedExprSiteV1,
    },
    Return {
        exit: &'source SourceStmtSiteV1,
    },
}

impl FinalizedRootSourceHandoffV1 {
    /// Only the retained original pool is enumerated, never rebound exit-prefix
    /// views. Both original sites and final instruction coordinates are unique.
    pub(in crate::mir) fn visit_finalized_lexical_call_nodes_v1(
        &self,
        module: &MirModule,
        mut visit: impl FnMut(
            FunctionOwnerIdV1,
            FinalizedLexicalCallContextV1<'_>,
            &EmittedLexicalCallProjectionV1,
            &[LocalCallArgumentV1],
            &MirFunction,
            (BasicBlockId, usize),
            &[((BasicBlockId, MirInstruction), (BasicBlockId, usize))],
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        self.validate_finalized_root_cleanup_v1(module)?;
        let mut source_sites = BTreeSet::new();
        let mut coordinates = BTreeSet::new();
        let mut walk = |owner,
                        context,
                        packet: &EmittedLexicalCallProjectionV1,
                        arguments: &[LocalCallArgumentV1]| {
            packet.visit_original_nodes_v1(owner, arguments, &self.ledger, &mut |node, args| {
                if !source_sites.insert(node.call_site().clone()) {
                    return Err(freeze("final-call-visit/source-duplicate"));
                }
                let (symbol, _) = self
                    .ledger
                    .finished_binding_for_owner(owner, node.outer_bindings().0)?;
                let function = module
                    .functions
                    .get(&symbol)
                    .ok_or_else(|| freeze("final-call-visit/function-missing"))?;
                let coordinate = match context {
                    FinalizedLexicalCallContextV1::Local { group_site, .. }
                    | FinalizedLexicalCallContextV1::Discard { group_site } => self
                        .finished_local_call_producer_v1(
                            owner,
                            group_site,
                            node.call_site(),
                            node.outer_bindings().0,
                            function,
                        )?,
                    FinalizedLexicalCallContextV1::Return { exit } => self
                        .finished_terminal_call_producer_v1(
                            owner,
                            exit,
                            node.call_site(),
                            node.outer_bindings().0,
                            function,
                        )?,
                };
                if !coordinates.insert((symbol, coordinate)) {
                    return Err(freeze("final-call-visit/coordinate-duplicate"));
                }
                // This node alone owns its dependencies. Nested nodes are
                // visited separately, while exact shared prefixes are lent once.
                let mut copies = Vec::new();
                for (site, original) in node.copy_dependencies(&self.ledger)? {
                    if &site != node.call_site()
                        || copies.iter().any(|(previous, _)| previous == &original)
                    {
                        continue;
                    }
                    let finished = match context {
                        FinalizedLexicalCallContextV1::Local { group_site, .. }
                        | FinalizedLexicalCallContextV1::Discard { group_site } => self
                            .finished_local_call_copy_v1(
                                owner,
                                group_site,
                                node.call_site(),
                                &original,
                                function,
                            )?,
                        FinalizedLexicalCallContextV1::Return { exit } => self
                            .finished_terminal_call_copy_v1(
                                owner,
                                exit,
                                node.call_site(),
                                &original,
                                function,
                            )?,
                    };
                    copies.push((original, finished));
                }
                visit(owner, context, node, args, function, coordinate, &copies)
            })
        };
        for (owner, group) in self.local_call_binding_groups() {
            let Some(packet) = group.lexical() else {
                continue;
            };
            let source = self
                .ledger
                .lexical_i64_call_source(group.site())
                .or_else(|| self.ledger.handle_call_source(group.site()))
                .or_else(|| self.ledger.nullable_call_source(group.site()))
                .ok_or_else(|| freeze("final-call-visit/local-source-missing"))?;
            if source.owner() != owner || packet.call_site() != group.site() {
                return Err(freeze("final-call-visit/local-source-identity"));
            }
            let context = match source.local_binding() {
                Some((declaration, binding)) => FinalizedLexicalCallContextV1::Local {
                    group_site: group.site(),
                    declaration,
                    binding,
                },
                None => FinalizedLexicalCallContextV1::Discard {
                    group_site: group.site(),
                },
            };
            let arguments = match packet.original_source() {
                CallPacketSourceLoanV1::Instance(row)
                    if row.source_target().is_self_receiver()
                        && row.source_target().has_object_source_requirement() =>
                {
                    self.ledger.receiver_object_packet_arguments_v1(row)?
                }
                _ => source.arguments(),
            };
            walk(owner, context, packet, arguments)?;
        }
        // The finalized root has moved its original Call entries here.
        for (exit, (entry, _)) in &self.call_entries {
            if !matches!(
                entry,
                RootHomeExitEntry::Call {
                    row: super::super::RootCallDispositionV1::Lexical(_),
                    ..
                }
            ) {
                continue;
            }
            self.with_terminal_call_packet_v1(self.owner(), exit, |packet| {
                let arguments = self
                    .ledger
                    .borrowed_terminal_arguments_v1(self.owner(), exit)?
                    .ok_or_else(|| freeze("final-call-visit/terminal-source-missing"))?;
                walk(
                    self.owner(),
                    FinalizedLexicalCallContextV1::Return { exit },
                    packet,
                    &arguments,
                )
            })?;
        }
        // Child entries remain Emitted in their existing owner. Never enumerate
        // the root's old ledger views again, or plain exits' shared prefixes.
        let exits = self.ledger.root_exits.borrow();
        for ((owner, exit), progress) in exits.iter() {
            if *owner == self.owner()
                || !matches!(
                    progress,
                    RootHomeExitProgress::Emitted {
                        entry: RootHomeExitEntry::Call {
                            row: super::super::RootCallDispositionV1::Lexical(_),
                            ..
                        },
                        ..
                    }
                )
            {
                continue;
            }
            self.with_terminal_call_packet_v1(*owner, exit, |packet| {
                let arguments = self
                    .ledger
                    .borrowed_terminal_arguments_v1(*owner, exit)?
                    .ok_or_else(|| freeze("final-call-visit/terminal-source-missing"))?;
                walk(
                    *owner,
                    FinalizedLexicalCallContextV1::Return { exit },
                    packet,
                    &arguments,
                )
            })?;
        }
        Ok(())
    }
}
