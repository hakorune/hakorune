//! Corroboration borrows original actuals; this physical view owns no domain.
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1 as Source;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

pub(super) fn value(
    projection: &LexicalCallArgumentProjectionV1,
    owner: FunctionOwnerIdV1,
    row: CallPacketSourceLoanV1<'_>,
    ordinal: u32,
    site: &SourceExprSiteV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let ledger = context.ledger();
    if let CallPacketSourceLoanV1::SelectedStaticLoop {
        packet,
        observation,
    } = row
    {
        let LexicalCallArgumentProjectionV1::SelectedCanonicalRead {
            ordinal: observed,
            site: observed_site,
            read,
        } = projection
        else {
            return Err(freeze("selected-static-packet/canonical-read-required"));
        };
        let (caller, call_site) = packet.publication_source();
        let actual = packet.selected_forwarded_actual_for_call(caller, call_site)?;
        let Source::Forwarded { binding, formal } = &actual.source else {
            return Err(freeze("selected-static-packet/forwarded-source-required"));
        };
        if !packet.corroborates_local_call(observation)
            || *observed != ordinal
            || observed_site != site
            || actual.ordinal != ordinal
            || actual.site != *site
            || actual.formal.owner() != row.callee_owner()
            || read.owner() != owner
            || read.binding() != *binding
            || formal.owner() != owner
            || packet
                .original_source()
                .argument_sites()
                .get(ordinal as usize)
                != Some(site)
        {
            return Err(freeze("selected-static-packet/canonical-source-drift"));
        }
        return Ok(read.physical_value());
    }
    let actuals = row
        .borrowed_actuals_in_context(context)?
        .ok_or_else(|| freeze("lexical-i64/borrowed-actuals-missing"))?;
    let mut matches = actuals.iter().filter(|actual| actual.ordinal == ordinal);
    let actual = matches
        .next()
        .ok_or_else(|| freeze("lexical-i64/borrowed-ordinal"))?;
    if matches.next().is_some()
        || &actual.site != site
        || actual.formal.owner() != row.callee_owner()
        || row.argument_sites().get(ordinal as usize) != Some(site)
    {
        return Err(freeze("lexical-i64/borrowed-source-identity"));
    }
    match projection {
        LexicalCallArgumentProjectionV1::Expression(_) => projection.borrowed_expression_value_v1(
            &actual.source,
            row.call_site(),
            ordinal,
            site,
            context,
        ),
        LexicalCallArgumentProjectionV1::BorrowedReadArgument {
            ordinal: observed,
            site: observed_site,
            formal,
            read,
        } if *observed == ordinal && observed_site == site && *formal == actual.formal => {
            let Source::Read(original) = &actual.source else {
                return Err(freeze("owned-array/argument-source-drift"));
            };
            read.value_with_source_v1(row.call_site(), original, context)
        }
        LexicalCallArgumentProjectionV1::BorrowedLiteral {
            ordinal: observed,
            site: observed_site,
            formal,
            binding,
        } if *observed == ordinal && observed_site == site && *formal == actual.formal => {
            match (&actual.source, &binding.1) {
                (
                    Source::Integer(expected),
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Integer(value),
                    },
                ) if expected == value => Ok(*dst),
                (
                    Source::Bool(expected),
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Bool(value),
                    },
                ) if expected == value => Ok(*dst),
                (
                    Source::Null,
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Null,
                    },
                ) => Ok(*dst),
                _ => Err(freeze("lexical-i64/borrowed-literal")),
            }
        }
        LexicalCallArgumentProjectionV1::BorrowedRead {
            ordinal: observed,
            site: observed_site,
            formal,
            read,
            entry,
        } if *observed == ordinal && observed_site == site && *formal == actual.formal => {
            let (binding, forwarded) = match &actual.source {
                Source::Scalar { binding, .. }
                | Source::TypedHome { binding, .. }
                | Source::EntryReceiver { binding, .. }
                | Source::DeclaredFormal { binding, .. }
                | Source::ReceivedNullable { binding, .. }
                | Source::ReceivedMixed { binding, .. } => (*binding, None),
                Source::Forwarded { binding, formal } => (*binding, Some(*formal)),
                _ => return Err(freeze("lexical-i64/borrowed-read-source")),
            };
            let value = read
                .value_for(owner, site.node(), binding)
                .map_err(|error| {
                    format!("[freeze:contract][lexical-i64/borrowed-read/{error:?}]")
                })?;
            match forwarded {
                Some(formal) => {
                    let rows = ledger.borrowed_ordinary_entry_values_v1(owner)?;
                    let expected = rows
                        .iter()
                        .find(|(_, root, _)| *root == formal)
                        .ok_or_else(|| freeze("lexical-i64/forwarded-entry-missing"))?;
                    if entry.as_ref() != Some(expected) {
                        return Err(freeze("lexical-i64/forwarded-entry-drift"));
                    }
                    // A distinct alias value needs the original Copy-chain proof.
                    // A raw matching type cannot stand in for that obligation.
                    if !read.proves_forwarded_entry(formal, expected.2) {
                        return Err(freeze("lexical-i64/forwarded-copy-proof-missing"));
                    }
                }
                None if entry.is_some() => return Err(freeze("lexical-i64/unexpected-entry")),
                None => {}
            }
            Ok(value)
        }
        _ => Err(freeze("lexical-i64/borrowed-projection-identity")),
    }
}

/// Original source lending precedes access to a retained physical dependency.
pub(super) fn copies_in_context<'a>(
    projection: &'a LexicalCallArgumentProjectionV1,
    owner: FunctionOwnerIdV1,
    row: CallPacketSourceLoanV1<'_>,
    context: CallValidationContextV1<'_>,
) -> Result<&'a [Binding], String> {
    if let LexicalCallArgumentProjectionV1::BorrowedReadArgument {
        ordinal,
        site,
        read,
        ..
    } = projection
    {
        value(projection, owner, row, *ordinal, site, context)?;
        // The holder receiver is an exact source read, including its original
        // Copy chain when the common lexical-read owner requires one.
        return read.copy_dependencies_v1(context);
    }
    let LexicalCallArgumentProjectionV1::BorrowedRead {
        ordinal,
        site,
        read,
        entry,
        ..
    } = projection
    else {
        return Ok(&[]);
    };
    value(projection, owner, row, *ordinal, site, context)?;
    match entry {
        Some((_, formal, entry_value)) => read.loan_forwarded_copies(*formal, *entry_value),
        None => Ok(&[]),
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// Same formal support and original argument read, without a new source
    /// classifier or packet. Only the original formal/Copy chain can anchor it.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn field_support_projection_v1<
        'a,
    >(
        &'a self,
        support: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::ObjectReturnFieldSupportV1,
        ordinal: u32,
        context: CallValidationContextV1<'_>,
    ) -> Result<(ValueId, &'a [Binding]), String> {
        let owner = self.call_site().owner();
        let source = self.original_source();
        let actuals = source
            .borrowed_actuals_in_context(context)?
            .ok_or_else(|| freeze("field-support/actuals-missing"))?;
        let mut matching = actuals.iter().filter(|actual| actual.ordinal == ordinal);
        let actual = matching
            .next()
            .ok_or_else(|| freeze("field-support/actual-missing"))?;
        if matching.next().is_some() || support.formal.owner() != owner {
            return Err(freeze("field-support/actual-identity"));
        }
        let root = match &actual.source {
            Source::Forwarded { formal, .. } => *formal,
            Source::DeclaredFormal { root, .. } => *root,
            _ => return Err(freeze("field-support/original-formal-required")),
        };
        let entries = context.ledger().borrowed_ordinary_entry_values_v1(owner)?;
        let mut entries = entries.iter().filter(|(ordinal, binding, _)| {
            *ordinal == support.ordinal && *binding == support.formal
        });
        let entry = entries
            .next()
            .ok_or_else(|| freeze("field-support/entry-missing"))?;
        if root != support.formal || entries.next().is_some() {
            return Err(freeze("field-support/entry-identity"));
        }
        let projection = self
            .prepared
            .arguments
            .get(ordinal as usize)
            .ok_or_else(|| freeze("field-support/read-missing"))?;
        let LexicalCallArgumentProjectionV1::BorrowedRead { read, .. } = projection else {
            return Err(freeze("field-support/original-read-required"));
        };
        // Reuse the ordered argument owner's source/site/formal corroboration.
        value(projection, owner, source, ordinal, &actual.site, context)?;
        let copies = read.loan_forwarded_copies(support.formal, entry.2)?;
        Ok((entry.2, copies))
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// Borrow the exact physical actuals from this same packet; no formal rebasing.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_received_field_support_v1(
        &self,
        context: CallValidationContextV1<'_>,
        ends: Option<(
            &super::super::read_argument::IntervalViewV1<'_>,
            Option<&FinishedBindings>,
            &[Binding],
        )>,
    ) -> Result<Option<Box<[crate::mir::resolved_semantics::BindingRefV1]>>, String> {
        let ledger = context.ledger();
        let row = self.original_row()?;
        let Some(parents) = ledger.object_packet_field_support_parents_v1(row)? else {
            return Ok(None);
        };
        row.visit_object_field_support_actuals_v1(ledger, |_, actual| {
            let projection = self
                .prepared
                .arguments
                .get(actual.ordinal as usize)
                .ok_or_else(|| freeze("field-end/physical-argument-missing"))?;
            value(
                projection,
                self.call_site().owner(),
                self.original_source(),
                actual.ordinal,
                &actual.site,
                context,
            )?;
            match (&actual.source, projection) {
                (Source::Null, LexicalCallArgumentProjectionV1::BorrowedLiteral { .. }) => {}
                (
                    Source::Read(source),
                    LexicalCallArgumentProjectionV1::BorrowedReadArgument { read, .. },
                ) => {
                    if let Some((intervals, finished, bindings)) = ends {
                        read.validate_field_anchor_ends_v1(
                            self.call_site(),
                            source,
                            context,
                            intervals,
                            finished,
                            bindings,
                        )?;
                    }
                }
                _ => return Err(freeze("field-end/physical-argument-kind")),
            }
            Ok(())
        })?;
        Ok(Some(parents))
    }
}
