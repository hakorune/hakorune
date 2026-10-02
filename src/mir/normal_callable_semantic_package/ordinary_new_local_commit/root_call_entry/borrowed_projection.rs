//! Corroboration borrows original actuals; this physical view owns no domain.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    BorrowedFormalActualSourceV1 as Source, OrdinaryNewClaimLedgerV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

pub(super) fn value(
    projection: &LexicalCallArgumentProjectionV1,
    owner: FunctionOwnerIdV1,
    row: &LexicalInstanceCallDispositionRowV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
    ledger: &OrdinaryNewClaimLedgerV1,
) -> Result<ValueId, String> {
    let actuals = ledger
        .borrowed_call_actuals_v1(row)?
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
                | Source::EntryReceiver { binding, .. } => (*binding, None),
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
