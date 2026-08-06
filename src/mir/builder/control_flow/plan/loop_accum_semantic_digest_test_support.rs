//! Test-only semantic/legacy split for DirectAccum physical parity.
//!
//! PHI/SSA generation remains owned by CanonicalCfgSessionV1,
//! BindingSsaBuilderV1, and PhiTxn.  This module only stores immutable
//! observer output.  Legacy compatibility artifacts must be classified by a
//! named policy instead of being silently ignored.

#![cfg(test)]

use super::physical_digest_test_support::AlphaPhysicalMirDigestV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SemanticCoreDigestV1 {
    pub(crate) cfg_edges: Box<[String]>,
    pub(crate) operations: Box<[String]>,
    pub(crate) carriers: Box<[String]>,
    pub(crate) results: Box<[String]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LegacyAuxiliaryDigestV1 {
    pub(crate) rows: Box<[String]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AlphaPhysicalMirDigestV2 {
    pub(crate) semantic: SemanticCoreDigestV1,
    pub(crate) legacy_aux: LegacyAuxiliaryDigestV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirectAccumLegacyAuxPolicyV1;

impl DirectAccumLegacyAuxPolicyV1 {
    pub(crate) fn validate(&self, digest: &LegacyAuxiliaryDigestV1) -> Result<(), String> {
        for row in digest.rows.iter() {
            let allowed = row.starts_with("copy:")
                || row.starts_with("step-phi:")
                || row.starts_with("after-phi:")
                || row.starts_with("after-void:")
                || row.starts_with("partial-pred:");
            if !allowed {
                return Err(format!("unknown DirectAccum legacy auxiliary row: {row}"));
            }
        }
        Ok(())
    }
}

fn canonical_cfg(digest: &AlphaPhysicalMirDigestV1) -> Result<Box<[String]>, String> {
    let required = ["P->H", "H->B", "H->A", "B->S", "S->H"];
    let mut observed = Vec::new();
    for row in digest.cfg.iter() {
        let Some((_, rest)) = row.split_once(":succ=[") else {
            return Err(format!("malformed CFG observer row: {row}"));
        };
        let role = row
            .split_once(':')
            .map(|(role, _)| role)
            .ok_or_else(|| format!("malformed CFG role row: {row}"))?;
        let Some((successors, _)) = rest.split_once(']') else {
            return Err(format!("malformed CFG successor row: {row}"));
        };
        for successor in successors.split(',').filter(|value| !value.is_empty()) {
            let edge = format!("{role}->{successor}");
            if !required.contains(&edge.as_str()) {
                return Err(format!("unexpected DirectAccum CFG edge: {edge}"));
            }
            observed.push(edge);
        }
    }
    observed.sort();
    observed.dedup();
    let mut expected = required
        .iter()
        .map(|edge| (*edge).to_owned())
        .collect::<Vec<_>>();
    expected.sort();
    if observed != expected {
        return Err(format!(
            "DirectAccum required CFG mismatch: observed={observed:?} expected={expected:?}"
        ));
    }
    Ok(expected.into_boxed_slice())
}

fn semantic_rows(
    digest: &AlphaPhysicalMirDigestV1,
) -> Result<(Box<[String]>, Box<[String]>, LegacyAuxiliaryDigestV1), String> {
    let mut operations = Vec::new();
    let mut carriers = Vec::new();
    let mut auxiliary = Vec::new();
    for row in digest.instructions.iter() {
        if row.contains(":copy:") {
            auxiliary.push(format!("copy:{row}"));
            continue;
        }
        if row.contains(":phi:phi:step:join:") {
            auxiliary.push(format!("step-phi:{row}"));
            continue;
        }
        if row.contains(":phi:phi:after:") {
            auxiliary.push(format!("after-phi:{row}"));
            continue;
        }
        if row.contains(":const:const:Void=Void") {
            auxiliary.push(format!("after-void:{row}"));
            continue;
        }
        if row.contains(":phi:phi:carrier:i=") {
            carriers.push("header-carrier:i=[entry:i,backedge:i]".to_owned());
            continue;
        }
        if row.contains(":phi:phi:carrier:sum=") {
            carriers.push("header-carrier:sum=[entry:sum,backedge:sum]".to_owned());
            continue;
        }
        if !(row.contains(":const:") || row.contains(":bin:") || row.contains(":compare:")) {
            return Err(format!("unknown DirectAccum semantic/auxiliary row: {row}"));
        }
        operations.push(row.clone());
    }
    operations.sort();
    operations.dedup();
    carriers.sort();
    carriers.dedup();

    for row in digest.cfg.iter() {
        let Some((role, rest)) = row.split_once(":pred=[") else {
            return Err(format!("malformed CFG observer row: {row}"));
        };
        let Some((preds, _)) = rest.split_once(']') else {
            return Err(format!("malformed CFG predecessor row: {row}"));
        };
        let expected = match role {
            "P" => "",
            "H" => "P,S",
            "B" => "H",
            "S" => "B",
            "A" => "H",
            _ => return Err(format!("unknown DirectAccum role in CFG row: {role}")),
        };
        if preds != expected {
            auxiliary.push(format!("partial-pred:{role}=[{preds}]"));
        }
    }
    auxiliary.sort();
    auxiliary.dedup();
    Ok((
        operations.into_boxed_slice(),
        carriers.into_boxed_slice(),
        LegacyAuxiliaryDigestV1 {
            rows: auxiliary.into_boxed_slice(),
        },
    ))
}

pub(crate) fn semantic_digest(
    digest: &AlphaPhysicalMirDigestV1,
    final_rows: &[&str],
) -> Result<AlphaPhysicalMirDigestV2, String> {
    let cfg_edges = canonical_cfg(digest)?;
    let (operations, carriers, legacy_aux) = semantic_rows(digest)?;
    let mut results = final_rows
        .iter()
        .map(|row| (*row).to_owned())
        .collect::<Vec<_>>();
    results.sort();
    if results
        != [
            "final:i:carrier:i:Integer".to_owned(),
            "final:sum:carrier:sum:Integer".to_owned(),
            "result:unit:Void".to_owned(),
        ]
    {
        return Err(format!("DirectAccum final-result mismatch: {results:?}"));
    }
    Ok(AlphaPhysicalMirDigestV2 {
        semantic: SemanticCoreDigestV1 {
            cfg_edges,
            operations,
            carriers,
            results: results.into_boxed_slice(),
        },
        legacy_aux,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_accum_aux_policy_rejects_unknown_rows() {
        let digest = LegacyAuxiliaryDigestV1 {
            rows: vec!["unknown:artifact".to_owned()].into_boxed_slice(),
        };
        assert!(DirectAccumLegacyAuxPolicyV1.validate(&digest).is_err());
    }
}
