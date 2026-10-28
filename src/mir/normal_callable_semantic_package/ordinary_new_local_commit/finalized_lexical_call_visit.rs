//! Original call-tree loans joined to the same owner's finished instructions.
use super::*;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
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
    Expression {
        group_site: &'source OwnedExprSiteV1,
    },
    NewArgument {
        new_site: &'source OwnedExprSiteV1,
        ordinal: u32,
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
        self.visit_finalized_call_structure_v1(module, |owner, scope, node, arguments, function, coordinate, copies| {
            self.ledger.with_finished_projection(owner, |symbol, projection| {
                if function.signature.name != symbol { return Err(freeze("owned-array-read-finished-owner")); }
                node.validate_finished_read_arguments_v1(module, function,
                    CallValidationContextV1::Finalized(self), projection, coordinate)
            })?;
            visit(owner, scope, node, arguments, function, coordinate, copies)
        })
    }

    /// Source/actual call geometry only. This lends no read-lifetime permission.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn visit_finalized_call_structure_v1(
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
        for (owner, groups) in &self.local_calls {
            self.ledger
                .with_finished_projection(*owner, |symbol, projection| {
                    let function = module
                        .functions
                        .get(symbol)
                        .filter(|function| function.signature.name == symbol)
                        .ok_or_else(|| freeze("final-call-visit/function-missing"))?;
                    for group in groups {
                        group.validate_recorded_in_context(
                            CallValidationContextV1::Finalized(self),
                            function,
                            Some(projection),
                        )?;
                    }

                    Ok(())
                })?;
        }
        let mut source_sites = BTreeSet::new();
        let mut coordinates = BTreeSet::new();
        let mut walk = |owner,
                        context: FinalizedLexicalCallContextV1<'_>,
                        packet: &EmittedLexicalCallProjectionV1,
                        arguments: &[LocalCallArgumentV1]| {
            packet.visit_original_nodes_in_context_v1(
                owner,
                arguments,
                CallValidationContextV1::Finalized(self),
                &mut |node, args| {
                    if !source_sites.insert(node.call_site().clone()) {
                        return Err(freeze("final-call-visit/source-duplicate"));
                    }
                    let (symbol, _) = self
                        .ledger
                        .finished_binding_for_owner(owner, node.invoke_binding())?;
                    let function = module
                        .functions
                        .get(&symbol)
                        .ok_or_else(|| freeze("final-call-visit/function-missing"))?;
                    self.ledger
                        .validate_field_alias_function_v1(node.call_site(), function)?;
                    let coordinate = context.finished_binding_v1(
                        self, owner, node.call_site(), node.invoke_binding(), function, false,
                    )?;
                    if let Some(read) = node.stored_receiver_read_v1() {
                        let read_coordinate = context.finished_binding_v1(
                            self, owner, node.call_site(), read, function, false,
                        )?;
                        if !coordinates.insert((symbol.clone(), read_coordinate)) {
                            return Err(freeze("final-call-visit/coordinate-duplicate"));
                        }
                    }
                    if !coordinates.insert((symbol, coordinate)) {
                        return Err(freeze("final-call-visit/coordinate-duplicate"));
                    }
                    // This node alone owns its dependencies. Nested nodes are
                    // visited separately, while exact shared prefixes are lent once.
                    let mut copies = Vec::new();
                    for (site, original) in
                        node.copy_dependencies_in_context(CallValidationContextV1::Finalized(self))?
                    {
                        if &site != node.call_site()
                            || copies.iter().any(|(previous, _)| previous == &original)
                        {
                            continue;
                        }
                        let finished = context.finished_binding_v1(
                            self, owner, node.call_site(), &original, function, true,
                        )?;
                        copies.push((original, finished));
                    }
                    if self
                        .borrowed_packet_actuals_v1(node)?
                        .is_some_and(|actuals| !actuals.is_empty())
                    {
                        let row = node.original_source();
                        let symbol = module
                            .canonical_callable_definition_symbol(row.target())
                            .ok_or_else(|| freeze("final-call-visit/callee-definition"))?;
                        let callee = module
                            .functions
                            .get(symbol)
                            .ok_or_else(|| freeze("final-call-visit/callee-function"))?;
                        self.borrowed_ordinary_entry_source_for_function_v1(
                            row.callee_owner(),
                            callee,
                        )?;
                    }
                    visit(owner, context, node, args, function, coordinate, &copies)
                },
            )
        };
        // Every New expression is replayed, including String-only arguments
        // with no nested Call. No separate final String or Call registry exists.
        for (site, row) in self.ledger.local_commits.borrow().iter() {
            let Some(NewEmissionProgress::Emitted { arguments, .. }) = row.new_emission() else { continue };
            if arguments.iter().all(|argument| argument.expression_v1().is_none()) { continue; }
            self.ledger.with_finished_projection(site.owner(), |symbol, projection| {
                let function = module.functions.get(symbol)
                    .filter(|function| function.signature.name == symbol)
                    .ok_or_else(|| freeze("final-call-visit/function-missing"))?;
                self.ledger.validate_ordinary_new_row_emission_projected(site.owner(), site,
                    row, function, Some(projection), CallValidationContextV1::Finalized(self))?;
                for argument in arguments.iter() {
                    if let Some(expression) = argument.expression_v1() {
                        expression.visit_calls_v1(|packet, original| walk(site.owner(),
                            FinalizedLexicalCallContextV1::NewArgument {
                                new_site: site, ordinal: argument.source().ordinal(),
                            }, packet, original))?;
                    }
                }
                Ok(())
            })?;
        }
        for owner in self.local_calls.keys().copied() {
            CallValidationContextV1::Finalized(self).with_local_call_group_tree_v2(
                owner,
                |groups| {
                    for group in groups {
                        for packet in group.lexical_packets() {
                            if let Some(append) = group.owned_array_append() {
                                if append.source().site() != group.site()
                                    || append.source().candidate() != *packet.call_site()
                                {
                                    return Err(freeze("final-call-visit/append-source-identity"));
                                }
                                let row = packet.original_row()?;
                                append.source().check_candidate_v1(row)?;
                                let arguments =
                                    self.ledger.original_local_packet_arguments_v1(packet)?;
                                walk(
                                    owner,
                                    FinalizedLexicalCallContextV1::Expression {
                                        group_site: group.site(),
                                    },
                                    packet,
                                    arguments,
                                )?;
                                continue;
                            }
                            let source = self.ledger.local_call_for_owner(
                                packet.call_site().owner(), packet.call_site().site())
                                .ok_or_else(|| freeze("final-call-visit/local-source-missing"))?;
                            if source.owner() != owner || packet.call_site() != source.site() {
                                return Err(freeze("final-call-visit/local-source-identity"));
                            }
                            let context = if group.lexical().is_none() {
                                FinalizedLexicalCallContextV1::Expression {
                                    group_site: group.site(),
                                }
                            } else {
                                match source.local_binding() {
                                    Some((declaration, binding)) => {
                                        FinalizedLexicalCallContextV1::Local {
                                            group_site: group.site(),
                                            declaration,
                                            binding,
                                        }
                                    }
                                    None => FinalizedLexicalCallContextV1::Discard {
                                        group_site: group.site(),
                                    },
                                }
                            };
                            let arguments =
                                self.ledger.original_local_packet_arguments_v1(packet)?;
                            walk(owner, context, packet, arguments)?;
                        }
                    }
                    Ok(())
                },
            )?;
        }
        // Plain return expressions retain their own original call pool. They
        // do not become direct terminal Calls and are never found by MIR scan.
        {
            let exits = self.ledger.root_exits.borrow();
            for ((owner, key), progress) in exits.iter() {
                let order = match progress {
                    RootHomeExitProgress::Emitted { order, .. } | RootHomeExitProgress::Finalized(order) => order,
                    _ => continue,
                };
                let Some(expression) = order.return_expression_v1() else { continue; };
                let exit = key.explicit_site().ok_or_else(|| freeze("finished-terminal-call/exit-missing"))?;
                expression.require_source_v1(*owner, exit)?;
                expression.visit_calls_v1(|packet, arguments| walk(*owner,
                    FinalizedLexicalCallContextV1::Return { exit }, packet, arguments))?;
            }
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
                let arguments = self.ledger.validate_lexical_terminal_packet_in_context(
                    self.owner(),
                    exit,
                    packet,
                    CallValidationContextV1::Finalized(self),
                )?;
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
            let exit = exit
                .explicit_site()
                .ok_or_else(|| freeze("implicit-exit/terminal-call-drift"))?;
            self.with_terminal_call_packet_v1(*owner, exit, |packet| {
                let arguments = self.ledger.validate_lexical_terminal_packet_in_context(
                    *owner,
                    exit,
                    packet,
                    CallValidationContextV1::Finalized(self),
                )?;
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

impl FinalizedLexicalCallContextV1<'_> {
    fn finished_binding_v1(
        self, source: &FinalizedRootSourceHandoffV1, owner: FunctionOwnerIdV1,
        site: &OwnedExprSiteV1, original: &(BasicBlockId, MirInstruction),
        function: &MirFunction, copy: bool,
    ) -> Result<(BasicBlockId, usize), String> {
        match self {
            Self::NewArgument { new_site, ordinal } => source.finished_new_argument_call_binding_v1(
                owner, new_site, ordinal, site, original, function, copy),
            Self::Local { group_site, .. } | Self::Discard { group_site }
                | Self::Expression { group_site } => if copy {
                    source.finished_local_call_copy_v1(owner, group_site, site, original, function)
                } else {
                    source.finished_local_call_producer_v1(owner, group_site, site, original, function)
                },
            Self::Return { exit } => if copy {
                source.finished_terminal_call_copy_v1(owner, exit, site, original, function)
            } else {
                source.finished_terminal_call_producer_v1(owner, exit, site, original, function)
            },
        }
    }
}
