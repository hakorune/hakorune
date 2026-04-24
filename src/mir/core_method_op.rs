/*!
 * MIR-side CoreMethodOp carrier vocabulary.
 *
 * The CoreMethodContract `.hako` box owns the semantic rows. This module only
 * defines the narrow MIR carrier vocabulary used after a method has already
 * been resolved to a compiler contract row.
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreMethodOp {
    ArrayLen,
    ArrayGet,
    ArraySet,
    ArrayPush,
    MapGet,
    MapSet,
    MapHas,
    MapLen,
    StringLen,
    StringSubstring,
    StringIndexOf,
}

impl CoreMethodOp {
    pub const ALL: &'static [Self] = &[
        Self::ArrayLen,
        Self::ArrayGet,
        Self::ArraySet,
        Self::ArrayPush,
        Self::MapGet,
        Self::MapSet,
        Self::MapHas,
        Self::MapLen,
        Self::StringLen,
        Self::StringSubstring,
        Self::StringIndexOf,
    ];

    pub fn as_manifest_name(self) -> &'static str {
        match self {
            Self::ArrayLen => "ArrayLen",
            Self::ArrayGet => "ArrayGet",
            Self::ArraySet => "ArraySet",
            Self::ArrayPush => "ArrayPush",
            Self::MapGet => "MapGet",
            Self::MapSet => "MapSet",
            Self::MapHas => "MapHas",
            Self::MapLen => "MapLen",
            Self::StringLen => "StringLen",
            Self::StringSubstring => "StringSubstring",
            Self::StringIndexOf => "StringIndexOf",
        }
    }

    pub fn from_manifest_name(name: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|op| op.as_manifest_name() == name)
    }
}

impl std::fmt::Display for CoreMethodOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_manifest_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreMethodOpProof {
    CoreMethodContractManifest,
}

impl std::fmt::Display for CoreMethodOpProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoreMethodContractManifest => f.write_str("core_method_contract_manifest"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreMethodLoweringTier {
    WarmDirectAbi,
    ColdFallback,
}

impl std::fmt::Display for CoreMethodLoweringTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WarmDirectAbi => f.write_str("warm_direct_abi"),
            Self::ColdFallback => f.write_str("cold_fallback"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoreMethodOpCarrier {
    pub op: CoreMethodOp,
    pub proof: CoreMethodOpProof,
    pub lowering_tier: CoreMethodLoweringTier,
}

impl CoreMethodOpCarrier {
    pub fn manifest(op: CoreMethodOp, lowering_tier: CoreMethodLoweringTier) -> Self {
        Self {
            op,
            proof: CoreMethodOpProof::CoreMethodContractManifest,
            lowering_tier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn manifest_core_ops_are_known_by_mir_carrier() {
        let manifest = include_str!(
            "../../lang/src/runtime/meta/generated/core_method_contract_manifest.json"
        );
        let parsed: serde_json::Value = serde_json::from_str(manifest).expect("manifest json");
        let rows = parsed["rows"].as_array().expect("manifest rows");
        let manifest_ops = rows
            .iter()
            .map(|row| row["core_op"].as_str().expect("core_op"))
            .collect::<BTreeSet<_>>();
        let mir_ops = CoreMethodOp::ALL
            .iter()
            .map(|op| op.as_manifest_name())
            .collect::<BTreeSet<_>>();

        assert_eq!(manifest_ops, mir_ops);
    }

    #[test]
    fn carrier_formats_stable_metadata_tokens() {
        let carrier = CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapHas,
            CoreMethodLoweringTier::WarmDirectAbi,
        );

        assert_eq!(carrier.op.to_string(), "MapHas");
        assert_eq!(carrier.proof.to_string(), "core_method_contract_manifest");
        assert_eq!(carrier.lowering_tier.to_string(), "warm_direct_abi");
        assert_eq!(
            CoreMethodOp::from_manifest_name("MapHas"),
            Some(CoreMethodOp::MapHas)
        );
        assert_eq!(CoreMethodOp::from_manifest_name("Unknown"), None);
    }
}
